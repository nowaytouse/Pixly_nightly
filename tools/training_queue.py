"""
训练队列管理系统

Phase 47.21 (EX-002): 从废弃Go代码提取
源文件: core/@deprecated/go_ai_service_2025_11_11/ai 2/training_queue.go

核心功能:
- 增量训练Pipeline自动化
- 样本数阈值触发训练
- 基于性能提升的自动部署
- 训练批次管理和失败重试

作者: Pixly Team
日期: 2025-11-12
"""

from typing import List, Dict, Any, Optional
from dataclasses import dataclass, field
from datetime import datetime, timedelta
from pathlib import Path
import asyncio
import json
import glob
import subprocess


@dataclass
class TrainingConfig:
    """训练配置"""
    min_samples: int = 100                          # 触发训练的最小样本数
    max_samples: int = 1000                         # 单批次最大样本数
    check_interval: timedelta = timedelta(hours=1)  # 检查间隔
    auto_deploy: bool = False                       # 自动部署新模型
    performance_threshold: float = 0.05             # 性能提升阈值(5%)
    max_retries: int = 3                            # 最大重试次数
    model_type: str = "ppo"                         # 模型类型
    
    def to_dict(self) -> dict:
        """转换为字典"""
        return {
            "min_samples": self.min_samples,
            "max_samples": self.max_samples,
            "check_interval_seconds": self.check_interval.total_seconds(),
            "auto_deploy": self.auto_deploy,
            "performance_threshold": self.performance_threshold,
            "max_retries": self.max_retries,
            "model_type": self.model_type
        }
    
    @classmethod
    def from_dict(cls, data: dict) -> 'TrainingConfig':
        """从字典创建"""
        return cls(
            min_samples=data.get("min_samples", 100),
            max_samples=data.get("max_samples", 1000),
            check_interval=timedelta(seconds=data.get("check_interval_seconds", 3600)),
            auto_deploy=data.get("auto_deploy", False),
            performance_threshold=data.get("performance_threshold", 0.05),
            max_retries=data.get("max_retries", 3),
            model_type=data.get("model_type", "ppo")
        )


@dataclass
class TrainingBatch:
    """训练批次"""
    batch_id: int                                   # 批次ID
    model_type: str                                 # 模型类型
    start_time: datetime = field(default_factory=datetime.now)
    end_time: Optional[datetime] = None
    status: str = "pending"                         # pending, training, completed, failed
    samples: List[str] = field(default_factory=list)  # 样本文件路径列表
    metrics: Dict[str, float] = field(default_factory=dict)
    new_version: str = ""                           # 新版本号
    error_message: str = ""                         # 错误信息
    retry_count: int = 0                            # 重试次数
    
    def to_dict(self) -> dict:
        """转换为字典"""
        return {
            "batch_id": self.batch_id,
            "model_type": self.model_type,
            "start_time": self.start_time.isoformat(),
            "end_time": self.end_time.isoformat() if self.end_time else None,
            "status": self.status,
            "sample_count": len(self.samples),
            "metrics": self.metrics,
            "new_version": self.new_version,
            "error_message": self.error_message,
            "retry_count": self.retry_count
        }


class TrainingQueue:
    """
    训练队列管理器 - 增量训练Pipeline
    
    功能:
    - 自动检测新样本数量
    - 达到阈值自动触发训练
    - 训练批次管理
    - 失败重试机制
    - 基于性能提升的自动部署
    """
    
    def __init__(self, config: TrainingConfig, 
                 observations_dir: Path = Path("data/observations"),
                 models_dir: Path = Path("models")):
        """
        初始化训练队列管理器
        
        Args:
            config: 训练配置
            observations_dir: 观测数据目录
            models_dir: 模型目录
        """
        self.config = config
        self.observations_dir = observations_dir
        self.models_dir = models_dir
        
        self.current_batch: Optional[TrainingBatch] = None
        self.batch_history: List[TrainingBatch] = []
        self.is_running = False
        self.next_batch_id = 1
        
        # 加载历史
        self.load_history()
    
    async def start(self):
        """启动训练队列（异步循环）"""
        if self.is_running:
            print("⚠️ 训练队列已在运行")
            return
        
        self.is_running = True
        print(f"🚀 训练队列已启动")
        print(f"   检查间隔: {self.config.check_interval}")
        print(f"   最小样本数: {self.config.min_samples}")
        print(f"   自动部署: {'✅' if self.config.auto_deploy else '❌'}")
        
        while self.is_running:
            await self.check_and_trigger_training()
            await asyncio.sleep(self.config.check_interval.total_seconds())
    
    def stop(self):
        """停止训练队列"""
        self.is_running = False
        print("⏹️ 训练队列已停止")
    
    async def check_and_trigger_training(self):
        """检查并触发训练"""
        # 如果有正在训练的批次，跳过
        if self.current_batch and self.current_batch.status == "training":
            print(f"⏳ 批次 #{self.current_batch.batch_id} 正在训练中...")
            return
        
        # 统计新样本数
        sample_count = await self.count_new_samples()
        print(f"📊 当前未训练样本数: {sample_count}")
        
        if sample_count >= self.config.min_samples:
            print(f"✅ 达到训练阈值 ({self.config.min_samples})，触发训练")
            await self.start_training_batch()
        else:
            remaining = self.config.min_samples - sample_count
            print(f"   还需 {remaining} 个样本才能触发训练")
    
    async def count_new_samples(self) -> int:
        """统计新样本数量（未用于训练的）"""
        if not self.observations_dir.exists():
            return 0
        
        # 获取所有观测文件
        pattern = str(self.observations_dir / "*_obs.json")
        all_samples = glob.glob(pattern)
        
        # 获取已训练的样本
        trained_samples = set()
        for batch in self.batch_history:
            if batch.status == "completed":
                trained_samples.update(batch.samples)
        
        # 计算新样本
        new_samples = [s for s in all_samples if s not in trained_samples]
        return len(new_samples)
    
    async def load_samples(self) -> List[str]:
        """加载待训练样本"""
        pattern = str(self.observations_dir / "*_obs.json")
        all_samples = glob.glob(pattern)
        
        # 过滤已训练的样本
        trained_samples = set()
        for batch in self.batch_history:
            if batch.status == "completed":
                trained_samples.update(batch.samples)
        
        new_samples = [s for s in all_samples if s not in trained_samples]
        
        # 限制最大样本数
        if len(new_samples) > self.config.max_samples:
            new_samples = new_samples[:self.config.max_samples]
        
        return new_samples
    
    async def start_training_batch(self):
        """开始训练批次"""
        # 创建新批次
        batch = TrainingBatch(
            batch_id=self.next_batch_id,
            model_type=self.config.model_type,
            start_time=datetime.now(),
            status="pending"
        )
        self.next_batch_id += 1
        
        # 加载样本
        batch.samples = await self.load_samples()
        if not batch.samples:
            print("⚠️ 没有可用的训练样本")
            return
        
        print(f"\n{'='*60}")
        print(f"🎯 开始训练批次 #{batch.batch_id}")
        print(f"{'='*60}")
        print(f"样本数量: {len(batch.samples)}")
        print(f"模型类型: {batch.model_type}")
        
        self.current_batch = batch
        batch.status = "training"
        
        # 执行训练
        success = await self.run_training(batch)
        
        if success:
            batch.status = "completed"
            batch.end_time = datetime.now()
            duration = (batch.end_time - batch.start_time).total_seconds()
            print(f"✅ 训练完成! 耗时: {duration:.1f}秒")
            
            # 自动部署
            if self.config.auto_deploy:
                await self.auto_deploy_model(batch)
        else:
            # 失败处理
            batch.status = "failed"
            batch.retry_count += 1
            
            if batch.retry_count < self.config.max_retries:
                print(f"⚠️ 训练失败，将重试 ({batch.retry_count}/{self.config.max_retries})")
                # 可以在这里安排重试
            else:
                print(f"❌ 训练失败，已达最大重试次数")
        
        # 保存到历史
        self.batch_history.append(batch)
        self.save_history()
        self.current_batch = None
    
    async def run_training(self, batch: TrainingBatch) -> bool:
        """
        执行训练
        
        Returns:
            训练是否成功
        """
        try:
            # 调用训练脚本
            if batch.model_type == "ppo":
                cmd = [
                    "python3", "tools/train_ppo.py",
                    "--episodes", "100",
                    "--save-path", str(self.models_dir / "ppo")
                ]
            elif batch.model_type == "lightgbm":
                cmd = [
                    "python3", "tools/train_model.py",
                    "--samples", str(len(batch.samples))
                ]
            else:
                print(f"❌ 不支持的模型类型: {batch.model_type}")
                return False
            
            print(f"🔧 执行命令: {' '.join(cmd)}")
            
            # 异步执行
            process = await asyncio.create_subprocess_exec(
                *cmd,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE
            )
            
            stdout, stderr = await process.communicate()
            
            if process.returncode == 0:
                # 解析输出获取指标
                output = stdout.decode()
                print(f"📊 训练输出:\n{output[-500:]}")  # 最后500字符
                
                # 提取版本号（示例）
                batch.new_version = f"v{datetime.now().strftime('%Y%m%d_%H%M%S')}"
                
                return True
            else:
                error = stderr.decode()
                batch.error_message = error
                print(f"❌ 训练失败: {error}")
                return False
                
        except Exception as e:
            batch.error_message = str(e)
            print(f"❌ 训练异常: {e}")
            return False
    
    async def auto_deploy_model(self, batch: TrainingBatch):
        """
        自动部署模型（基于性能提升）
        
        Args:
            batch: 训练批次
        """
        print(f"\n🤖 评估新模型性能...")
        
        # 这里应该加载新旧模型并比较性能
        # 示例：假设从batch.metrics获取
        new_accuracy = batch.metrics.get("accuracy", 0)
        old_accuracy = 0.85  # 应该从模型路由器获取当前活跃模型的准确率
        
        improvement = new_accuracy - old_accuracy
        
        if improvement >= self.config.performance_threshold:
            print(f"✅ 性能提升 {improvement:.2%} (阈值: {self.config.performance_threshold:.2%})")
            print(f"🚀 自动部署新模型版本: {batch.new_version}")
            
            # 这里应该调用模型路由器注册新版本
            # router.register_model(...)
            
        else:
            print(f"⚠️ 性能提升不足 {improvement:.2%} < {self.config.performance_threshold:.2%}")
            print(f"   新模型不会被部署")
    
    def get_status(self) -> Dict[str, Any]:
        """获取队列状态"""
        return {
            "is_running": self.is_running,
            "current_batch": self.current_batch.to_dict() if self.current_batch else None,
            "total_batches": len(self.batch_history),
            "completed_batches": len([b for b in self.batch_history if b.status == "completed"]),
            "failed_batches": len([b for b in self.batch_history if b.status == "failed"]),
            "config": self.config.to_dict()
        }
    
    def save_history(self, path: Optional[Path] = None):
        """保存训练历史"""
        save_path = path or (self.models_dir / "training_history.json")
        
        history_data = {
            "batches": [b.to_dict() for b in self.batch_history],
            "next_batch_id": self.next_batch_id
        }
        
        try:
            save_path.parent.mkdir(parents=True, exist_ok=True)
            with open(save_path, 'w', encoding='utf-8') as f:
                json.dump(history_data, f, indent=2, ensure_ascii=False)
        except Exception as e:
            print(f"⚠️ 保存历史失败: {e}")
    
    def load_history(self, path: Optional[Path] = None):
        """加载训练历史"""
        load_path = path or (self.models_dir / "training_history.json")
        
        if not load_path.exists():
            return
        
        try:
            with open(load_path, 'r', encoding='utf-8') as f:
                history_data = json.load(f)
            
            self.next_batch_id = history_data.get("next_batch_id", 1)
            
            for batch_data in history_data.get("batches", []):
                batch = TrainingBatch(
                    batch_id=batch_data["batch_id"],
                    model_type=batch_data["model_type"],
                    start_time=datetime.fromisoformat(batch_data["start_time"]),
                    end_time=datetime.fromisoformat(batch_data["end_time"]) if batch_data.get("end_time") else None,
                    status=batch_data["status"],
                    samples=[],  # 不加载样本列表以节省内存
                    metrics=batch_data.get("metrics", {}),
                    new_version=batch_data.get("new_version", ""),
                    error_message=batch_data.get("error_message", ""),
                    retry_count=batch_data.get("retry_count", 0)
                )
                self.batch_history.append(batch)
            
            print(f"✅ 加载了 {len(self.batch_history)} 个历史批次")
        except Exception as e:
            print(f"⚠️ 加载历史失败: {e}")
    
    def print_summary(self):
        """打印队列摘要"""
        status = self.get_status()
        
        print("\n" + "="*60)
        print("📊 训练队列状态")
        print("="*60)
        print(f"运行状态: {'🟢 运行中' if status['is_running'] else '🔴 已停止'}")
        print(f"总批次数: {status['total_batches']}")
        print(f"  - 已完成: {status['completed_batches']}")
        print(f"  - 失败: {status['failed_batches']}")
        
        if self.current_batch:
            print(f"\n当前批次: #{self.current_batch.batch_id}")
            print(f"  状态: {self.current_batch.status}")
            print(f"  样本数: {len(self.current_batch.samples)}")
        
        if self.batch_history:
            print(f"\n最近5个批次:")
            for batch in reversed(self.batch_history[-5:]):
                status_icon = {"completed": "✅", "failed": "❌", "training": "⏳"}.get(batch.status, "❓")
                duration = ""
                if batch.end_time:
                    duration = f" ({(batch.end_time - batch.start_time).total_seconds():.1f}s)"
                print(f"  {status_icon} #{batch.batch_id}: {batch.status}{duration}")
        
        print("="*60 + "\n")


# 示例使用
async def main():
    # 创建配置
    config = TrainingConfig(
        min_samples=50,  # 降低阈值用于测试
        max_samples=200,
        check_interval=timedelta(seconds=30),  # 30秒检查一次
        auto_deploy=True,
        performance_threshold=0.03
    )
    
    # 创建训练队列
    queue = TrainingQueue(config)
    
    # 打印初始状态
    queue.print_summary()
    
    # 手动触发一次检查
    print("🔍 手动检查训练条件...")
    await queue.check_and_trigger_training()
    
    queue.print_summary()


if __name__ == "__main__":
    asyncio.run(main())
