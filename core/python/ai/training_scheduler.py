"""
🧠 PIXLY v3.0 本地化训练调度器

完全替代Go训练队列的本地化实现：
- 废弃HTTP训练端点 → 本地训练函数
- 废弃网络反馈收集 → 本地文件/SQLite存储
- 集成PPO本地训练 → 直接Python调用
- 零网络依赖训练调度

基于废弃Go training_queue.go完全重新设计
"""

import time
import threading
import sqlite3
from pathlib import Path
from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass, asdict
from enum import Enum
import json

try:
    import torch
    PYTORCH_AVAILABLE = True
except ImportError:
    PYTORCH_AVAILABLE = False


class TrainingStatus(Enum):
    """训练状态"""
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"


@dataclass
class TrainingTask:
    """训练任务"""
    task_id: str
    model_type: str  # lightgbm, ppo, transformer
    data_path: str
    config: Dict[str, Any]
    status: TrainingStatus = TrainingStatus.PENDING
    created_at: float = 0.0
    started_at: Optional[float] = None
    completed_at: Optional[float] = None
    progress: float = 0.0
    loss: Optional[float] = None
    metrics: Dict[str, float] = None
    error_message: Optional[str] = None
    
    def __post_init__(self):
        if self.created_at == 0.0:
            self.created_at = time.time()
        if self.metrics is None:
            self.metrics = {}


class TrainingScheduler:
    """
    🧠 本地化训练调度器
    
    替代Go TrainingQueue的核心功能：
    - 本地训练任务调度 (替代/api/v1/training/*)
    - PPO在线学习管理
    - 本地反馈数据收集
    - SQLite训练历史存储
    """
    
    def __init__(self, models_dir: str = "models", 
                 data_dir: str = "data", 
                 db_path: str = "training.db"):
        
        self.models_dir = Path(models_dir)
        self.data_dir = Path(data_dir)
        self.db_path = Path(db_path)
        
        # 确保目录存在
        for directory in [self.models_dir, self.data_dir]:
            directory.mkdir(parents=True, exist_ok=True)
        
        # 任务队列
        self.task_queue: List[TrainingTask] = []
        self.active_task: Optional[TrainingTask] = None
        self.task_history: Dict[str, TrainingTask] = {}
        
        # 线程管理
        self._lock = threading.RLock()
        self._scheduler_thread = None
        self._running = False
        
        # 训练器实例
        self.trainers: Dict[str, Any] = {}
        
        # 初始化
        self._init_database()
        self._load_task_history()
        self._init_trainers()
    
    def _init_database(self):
        """初始化SQLite数据库"""
        with sqlite3.connect(self.db_path) as conn:
            conn.execute("""
                CREATE TABLE IF NOT EXISTS training_tasks (
                    task_id TEXT PRIMARY KEY,
                    model_type TEXT NOT NULL,
                    data_path TEXT NOT NULL,
                    config TEXT NOT NULL,
                    status TEXT NOT NULL,
                    created_at REAL NOT NULL,
                    started_at REAL,
                    completed_at REAL,
                    progress REAL DEFAULT 0.0,
                    loss REAL,
                    metrics TEXT,
                    error_message TEXT
                )
            """)
            conn.commit()
    
    def _init_trainers(self):
        """初始化训练器"""
        # PPO训练器
        if PYTORCH_AVAILABLE:
            try:
                from .ppo_trainer import PPOTrainer
                self.trainers["ppo"] = PPOTrainer(self.models_dir)
                print("✅ PPO训练器已初始化")
            except ImportError:
                print("⚠️ PPO训练器不可用")
        
        # LightGBM训练器
        try:
            from .lightgbm_trainer import LightGBMTrainer
            self.trainers["lightgbm"] = LightGBMTrainer(self.models_dir)
            print("✅ LightGBM训练器已初始化")
        except ImportError:
            print("⚠️ LightGBM训练器不可用")
    
    def start_scheduler(self):
        """启动训练调度器"""
        with self._lock:
            if self._running:
                return
            
            self._running = True
            self._scheduler_thread = threading.Thread(target=self._scheduler_loop, daemon=True)
            self._scheduler_thread.start()
            print("✅ 训练调度器已启动")
    
    def stop_scheduler(self):
        """停止训练调度器"""
        with self._lock:
            self._running = False
            if self._scheduler_thread:
                self._scheduler_thread.join(timeout=5)
            print("✅ 训练调度器已停止")
    
    def submit_training_task(self, model_type: str, data_path: str, 
                           config: Dict[str, Any]) -> str:
        """提交训练任务"""
        import uuid
        
        task_id = str(uuid.uuid4())[:8]
        task = TrainingTask(
            task_id=task_id,
            model_type=model_type,
            data_path=data_path,
            config=config
        )
        
        with self._lock:
            self.task_queue.append(task)
            self.task_history[task_id] = task
            self._save_task_to_db(task)
        
        print(f"✅ 提交训练任务: {task_id} ({model_type})")
        return task_id
    
    def get_task_status(self, task_id: str) -> Optional[Dict[str, Any]]:
        """获取任务状态"""
        with self._lock:
            task = self.task_history.get(task_id)
            if task:
                return asdict(task)
            return None
    
    def cancel_task(self, task_id: str) -> bool:
        """取消训练任务"""
        with self._lock:
            # 从队列中移除
            self.task_queue = [t for t in self.task_queue if t.task_id != task_id]
            
            # 更新状态
            if task_id in self.task_history:
                task = self.task_history[task_id]
                task.status = TrainingStatus.CANCELLED
                self._save_task_to_db(task)
                print(f"✅ 取消训练任务: {task_id}")
                return True
            
            return False
    
    def get_training_stats(self) -> Dict[str, Any]:
        """获取训练统计"""
        with self._lock:
            stats = {
                "total_tasks": len(self.task_history),
                "pending_tasks": len(self.task_queue),
                "active_task": self.active_task.task_id if self.active_task else None,
                "scheduler_running": self._running,
                "model_stats": {}
            }
            
            # 按模型类型统计
            for task in self.task_history.values():
                model_type = task.model_type
                if model_type not in stats["model_stats"]:
                    stats["model_stats"][model_type] = {
                        "total": 0,
                        "completed": 0,
                        "failed": 0,
                        "avg_duration": 0.0
                    }
                
                stats["model_stats"][model_type]["total"] += 1
                
                if task.status == TrainingStatus.COMPLETED:
                    stats["model_stats"][model_type]["completed"] += 1
                    if task.started_at and task.completed_at:
                        duration = task.completed_at - task.started_at
                        current_avg = stats["model_stats"][model_type]["avg_duration"]
                        completed_count = stats["model_stats"][model_type]["completed"]
                        stats["model_stats"][model_type]["avg_duration"] = (current_avg * (completed_count - 1) + duration) / completed_count
                elif task.status == TrainingStatus.FAILED:
                    stats["model_stats"][model_type]["failed"] += 1
            
            return stats
    
    def _scheduler_loop(self):
        """调度器主循环"""
        while self._running:
            try:
                with self._lock:
                    # 检查是否有待处理任务
                    if not self.task_queue or self.active_task:
                        time.sleep(1)
                        continue
                    
                    # 获取下一个任务
                    task = self.task_queue.pop(0)
                    self.active_task = task
                
                # 执行训练任务
                self._execute_training_task(task)
                
                with self._lock:
                    self.active_task = None
                    
            except Exception as e:
                print(f"❌ 调度器错误: {e}")
                time.sleep(5)
    
    def _execute_training_task(self, task: TrainingTask):
        """执行训练任务"""
        try:
            task.status = TrainingStatus.RUNNING
            task.started_at = time.time()
            self._save_task_to_db(task)
            
            print(f"🚀 开始训练: {task.task_id} ({task.model_type})")
            
            # 获取对应训练器
            trainer = self.trainers.get(task.model_type)
            if trainer is None:
                raise RuntimeError(f"不支持的模型类型: {task.model_type}")
            
            # 执行训练
            if hasattr(trainer, 'train'):
                result = trainer.train(
                    data_path=task.data_path,
                    config=task.config,
                    progress_callback=lambda p: self._update_task_progress(task, p)
                )
                
                task.loss = result.get('final_loss')
                task.metrics = result.get('metrics', {})
                task.status = TrainingStatus.COMPLETED
                
                print(f"✅ 训练完成: {task.task_id}")
            else:
                raise RuntimeError(f"训练器 {task.model_type} 不支持训练方法")
                
        except Exception as e:
            task.status = TrainingStatus.FAILED
            task.error_message = str(e)
            print(f"❌ 训练失败: {task.task_id} - {e}")
        
        finally:
            task.completed_at = time.time()
            task.progress = 100.0 if task.status == TrainingStatus.COMPLETED else task.progress
            self._save_task_to_db(task)
    
    def _update_task_progress(self, task: TrainingTask, progress: float):
        """更新任务进度"""
        task.progress = progress
        # 定期保存进度到数据库
        if int(progress) % 10 == 0:  # 每10%保存一次
            self._save_task_to_db(task)
    
    def _save_task_to_db(self, task: TrainingTask):
        """保存任务到数据库"""
        with sqlite3.connect(self.db_path) as conn:
            conn.execute("""
                INSERT OR REPLACE INTO training_tasks 
                (task_id, model_type, data_path, config, status, created_at, 
                 started_at, completed_at, progress, loss, metrics, error_message)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """, (
                task.task_id, task.model_type, task.data_path, 
                json.dumps(task.config), task.status.value,
                task.created_at, task.started_at, task.completed_at,
                task.progress, task.loss, json.dumps(task.metrics),
                task.error_message
            ))
            conn.commit()
    
    def _load_task_history(self):
        """从数据库加载任务历史"""
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.execute("SELECT * FROM training_tasks")
                for row in cursor.fetchall():
                    task = TrainingTask(
                        task_id=row[0],
                        model_type=row[1],
                        data_path=row[2],
                        config=json.loads(row[3]),
                        status=TrainingStatus(row[4]),
                        created_at=row[5],
                        started_at=row[6],
                        completed_at=row[7],
                        progress=row[8],
                        loss=row[9],
                        metrics=json.loads(row[10]) if row[10] else {},
                        error_message=row[11]
                    )
                    self.task_history[task.task_id] = task
            
            print(f"✅ 加载训练历史: {len(self.task_history)} 个任务")
        except Exception as e:
            print(f"⚠️ 加载训练历史失败: {e}")


# 全局训练调度器实例
_global_scheduler: Optional[TrainingScheduler] = None
_scheduler_lock = threading.Lock()

def get_training_scheduler() -> TrainingScheduler:
    """获取全局训练调度器实例（单例模式）"""
    global _global_scheduler
    
    with _scheduler_lock:
        if _global_scheduler is None:
            _global_scheduler = TrainingScheduler()
            _global_scheduler.start_scheduler()
        return _global_scheduler
