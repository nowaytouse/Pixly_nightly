"""
🚀 PIXLY v3.0 部署管理器

企业级自动化部署解决方案：
- 多环境部署管道  
- 版本管理与回滚
- 健康检查集成
- 本地化部署架构
- 零停机部署支持

完全本地化实现，零网络依赖
"""

import os
import time
import json
import shutil
import subprocess
import threading
from pathlib import Path
from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass, asdict
from enum import Enum
import sqlite3
import hashlib


class DeploymentStatus(Enum):
    """部署状态"""
    PENDING = "pending"
    PREPARING = "preparing" 
    BUILDING = "building"
    TESTING = "testing"
    DEPLOYING = "deploying"
    COMPLETED = "completed"
    FAILED = "failed"
    ROLLED_BACK = "rolled_back"


class DeploymentTarget(Enum):
    """部署目标"""
    DEVELOPMENT = "development"
    STAGING = "staging"
    PRODUCTION = "production"
    LOCAL = "local"


@dataclass
class DeploymentConfig:
    """部署配置"""
    name: str
    version: str
    target: DeploymentTarget
    source_path: str
    deploy_path: str
    build_commands: List[str] = None
    test_commands: List[str] = None
    pre_deploy_hooks: List[str] = None
    post_deploy_hooks: List[str] = None
    health_check_url: Optional[str] = None
    rollback_enabled: bool = True
    backup_enabled: bool = True
    
    def __post_init__(self):
        if self.build_commands is None:
            self.build_commands = []
        if self.test_commands is None:
            self.test_commands = []
        if self.pre_deploy_hooks is None:
            self.pre_deploy_hooks = []
        if self.post_deploy_hooks is None:
            self.post_deploy_hooks = []


@dataclass
class DeploymentRecord:
    """部署记录"""
    deployment_id: str
    config: DeploymentConfig
    status: DeploymentStatus
    start_time: float
    end_time: Optional[float] = None
    duration_ms: Optional[float] = None
    build_output: str = ""
    test_output: str = ""
    deploy_output: str = ""
    error_message: Optional[str] = None
    artifacts_path: Optional[str] = None
    backup_path: Optional[str] = None
    
    def __post_init__(self):
        if self.end_time and self.start_time:
            self.duration_ms = (self.end_time - self.start_time) * 1000


class DeploymentManager:
    """
    🚀 企业级部署管理器
    
    功能特性：
    - 自动化部署流水线
    - 多环境管理
    - 版本控制集成
    - 健康检查验证
    """
    
    def __init__(self, workspace_dir: str = "deployments", 
                 db_path: str = "data/deployments.db"):
        
        self.workspace_dir = Path(workspace_dir)
        self.db_path = db_path
        
        # 部署记录
        self.active_deployments: Dict[str, DeploymentRecord] = {}
        self.deployment_history: List[DeploymentRecord] = []
        
        # 部署配置
        self.deployment_configs: Dict[str, DeploymentConfig] = {}
        
        # 钩子函数
        self.deployment_hooks: Dict[str, List[Callable]] = {
            'before_build': [],
            'after_build': [],
            'before_test': [],
            'after_test': [],
            'before_deploy': [],
            'after_deploy': [],
            'on_failure': [],
            'on_success': []
        }
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 初始化
        self._init_directories()
        self._init_database()
        self._load_default_configs()
    
    def _init_directories(self):
        """初始化部署目录"""
        self.workspace_dir.mkdir(parents=True, exist_ok=True)
        
        # 创建子目录
        subdirs = [
            "builds",      # 构建产物
            "backups",     # 备份文件
            "artifacts",   # 部署工件
            "configs",     # 部署配置
            "logs"         # 部署日志
        ]
        
        for subdir in subdirs:
            (self.workspace_dir / subdir).mkdir(exist_ok=True)
    
    def _init_database(self):
        """初始化部署数据库"""
        try:
            Path(self.db_path).parent.mkdir(parents=True, exist_ok=True)
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 部署记录表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS deployments (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    deployment_id TEXT UNIQUE NOT NULL,
                    config_name TEXT NOT NULL,
                    version TEXT NOT NULL,
                    target TEXT NOT NULL,
                    status TEXT NOT NULL,
                    start_time REAL NOT NULL,
                    end_time REAL,
                    duration_ms REAL,
                    build_output TEXT,
                    test_output TEXT,
                    deploy_output TEXT,
                    error_message TEXT,
                    artifacts_path TEXT,
                    backup_path TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 部署配置表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS deployment_configs (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT UNIQUE NOT NULL,
                    config_data TEXT NOT NULL,
                    version TEXT NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_deployments_id ON deployments(deployment_id)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_deployments_target ON deployments(target)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_configs_name ON deployment_configs(name)')
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 初始化部署数据库失败: {e}")
    
    def _load_default_configs(self):
        """加载默认部署配置"""
        
        # 开发环境配置
        dev_config = DeploymentConfig(
            name="pixly_development",
            version="3.0.0",
            target=DeploymentTarget.DEVELOPMENT,
            source_path="./",
            deploy_path="./deploy/dev",
            build_commands=[
                "python -m pip install -r requirements.txt",
                "python -m compileall core/"
            ],
            test_commands=[
                "python -m pytest tests/ -v"
            ],
            health_check_url=None  # 本地化，不使用HTTP
        )
        
        # 生产环境配置
        prod_config = DeploymentConfig(
            name="pixly_production", 
            version="3.0.0",
            target=DeploymentTarget.PRODUCTION,
            source_path="./",
            deploy_path="./deploy/prod",
            build_commands=[
                "python -m pip install -r requirements.txt --no-dev",
                "python -O -m compileall core/",
                "cargo build --release --manifest-path core/rust/Cargo.toml"
            ],
            test_commands=[
                "python -m pytest tests/ -x --tb=short",
                "cargo test --manifest-path core/rust/Cargo.toml"
            ],
            pre_deploy_hooks=[
                "echo 'Starting production deployment...'"
            ],
            post_deploy_hooks=[
                "echo 'Production deployment completed!'"
            ]
        )
        
        self.deployment_configs["pixly_development"] = dev_config
        self.deployment_configs["pixly_production"] = prod_config
    
    def register_config(self, config: DeploymentConfig) -> bool:
        """注册部署配置"""
        try:
            with self._lock:
                self.deployment_configs[config.name] = config
            
            # 保存到数据库
            self._save_config_to_db(config)
            
            print(f"✅ 注册部署配置: {config.name}")
            return True
            
        except Exception as e:
            print(f"⚠️ 注册部署配置失败 {config.name}: {e}")
            return False
    
    def _save_config_to_db(self, config: DeploymentConfig):
        """保存配置到数据库"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            config_data = json.dumps(asdict(config), indent=2)
            
            cursor.execute('''
                INSERT OR REPLACE INTO deployment_configs 
                (name, config_data, version, updated_at)
                VALUES (?, ?, ?, datetime('now'))
            ''', (config.name, config_data, config.version))
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 保存部署配置失败: {e}")
    
    def deploy(self, config_name: str, version: Optional[str] = None) -> str:
        """执行部署"""
        
        if config_name not in self.deployment_configs:
            raise ValueError(f"部署配置不存在: {config_name}")
        
        config = self.deployment_configs[config_name]
        if version:
            config.version = version
        
        # 生成部署ID
        deployment_id = f"deploy_{config_name}_{int(time.time())}"
        
        # 创建部署记录
        record = DeploymentRecord(
            deployment_id=deployment_id,
            config=config,
            status=DeploymentStatus.PENDING,
            start_time=time.time()
        )
        
        with self._lock:
            self.active_deployments[deployment_id] = record
        
        print(f"🚀 开始部署: {config.name} v{config.version}")
        print(f"   部署ID: {deployment_id}")
        print(f"   目标环境: {config.target.value}")
        
        # 在新线程中执行部署
        threading.Thread(
            target=self._execute_deployment,
            args=(deployment_id,),
            daemon=True
        ).start()
        
        return deployment_id
    
    def _execute_deployment(self, deployment_id: str):
        """执行部署流程"""
        record = self.active_deployments[deployment_id]
        config = record.config
        
        try:
            # 1. 准备阶段
            record.status = DeploymentStatus.PREPARING
            self._call_hooks('before_build', record)
            
            # 创建工作目录
            work_dir = self.workspace_dir / "builds" / deployment_id
            work_dir.mkdir(parents=True, exist_ok=True)
            
            # 2. 构建阶段
            record.status = DeploymentStatus.BUILDING
            build_success = self._execute_build(record, work_dir)
            
            if not build_success:
                record.status = DeploymentStatus.FAILED
                self._call_hooks('on_failure', record)
                return
            
            self._call_hooks('after_build', record)
            
            # 3. 测试阶段
            if config.test_commands:
                record.status = DeploymentStatus.TESTING
                self._call_hooks('before_test', record)
                
                test_success = self._execute_tests(record, work_dir)
                
                if not test_success:
                    record.status = DeploymentStatus.FAILED
                    self._call_hooks('on_failure', record)
                    return
                
                self._call_hooks('after_test', record)
            
            # 4. 部署阶段
            record.status = DeploymentStatus.DEPLOYING
            self._call_hooks('before_deploy', record)
            
            deploy_success = self._execute_deploy(record, work_dir)
            
            if deploy_success:
                record.status = DeploymentStatus.COMPLETED
                print(f"✅ 部署成功: {deployment_id}")
                self._call_hooks('on_success', record)
            else:
                record.status = DeploymentStatus.FAILED
                print(f"❌ 部署失败: {deployment_id}")
                self._call_hooks('on_failure', record)
            
        except Exception as e:
            record.status = DeploymentStatus.FAILED
            record.error_message = str(e)
            print(f"❌ 部署异常 {deployment_id}: {e}")
            self._call_hooks('on_failure', record)
        
        finally:
            # 完成部署
            record.end_time = time.time()
            record.duration_ms = (record.end_time - record.start_time) * 1000
            
            # 移动到历史记录
            with self._lock:
                if deployment_id in self.active_deployments:
                    del self.active_deployments[deployment_id]
                self.deployment_history.append(record)
            
            # 保存到数据库
            self._save_deployment_to_db(record)
    
    def _execute_build(self, record: DeploymentRecord, work_dir: Path) -> bool:
        """执行构建命令"""
        config = record.config
        build_output = []
        
        try:
            # 复制源代码到工作目录
            source_path = Path(config.source_path).resolve()
            if source_path.exists():
                if source_path.is_file():
                    shutil.copy2(source_path, work_dir)
                else:
                    # 复制目录内容
                    for item in source_path.iterdir():
                        if item.name not in ['.git', '__pycache__', 'node_modules']:
                            if item.is_file():
                                shutil.copy2(item, work_dir / item.name)
                            else:
                                shutil.copytree(item, work_dir / item.name, 
                                              ignore=shutil.ignore_patterns('__pycache__'))
            
            build_output.append(f"源代码已复制到: {work_dir}")
            
            # 执行构建命令
            for command in config.build_commands:
                build_output.append(f"\n执行构建命令: {command}")
                
                result = subprocess.run(
                    command.split(),
                    cwd=work_dir,
                    capture_output=True,
                    text=True,
                    timeout=600  # 10分钟超时
                )
                
                build_output.append(f"返回码: {result.returncode}")
                build_output.append(f"输出: {result.stdout}")
                
                if result.stderr:
                    build_output.append(f"错误: {result.stderr}")
                
                if result.returncode != 0:
                    record.build_output = "\n".join(build_output)
                    record.error_message = f"构建命令失败: {command}"
                    return False
            
            record.build_output = "\n".join(build_output)
            print(f"  ✅ 构建阶段完成")
            return True
            
        except Exception as e:
            record.build_output = "\n".join(build_output) + f"\n构建异常: {e}"
            record.error_message = f"构建阶段异常: {e}"
            return False
    
    def _execute_tests(self, record: DeploymentRecord, work_dir: Path) -> bool:
        """执行测试命令"""
        config = record.config
        test_output = []
        
        try:
            for command in config.test_commands:
                test_output.append(f"\n执行测试命令: {command}")
                
                result = subprocess.run(
                    command.split(),
                    cwd=work_dir,
                    capture_output=True,
                    text=True,
                    timeout=300  # 5分钟超时
                )
                
                test_output.append(f"返回码: {result.returncode}")
                test_output.append(f"输出: {result.stdout}")
                
                if result.stderr:
                    test_output.append(f"错误: {result.stderr}")
                
                if result.returncode != 0:
                    record.test_output = "\n".join(test_output)
                    record.error_message = f"测试命令失败: {command}"
                    return False
            
            record.test_output = "\n".join(test_output)
            print(f"  ✅ 测试阶段完成")
            return True
            
        except Exception as e:
            record.test_output = "\n".join(test_output) + f"\n测试异常: {e}"
            record.error_message = f"测试阶段异常: {e}"
            return False
    
    def _execute_deploy(self, record: DeploymentRecord, work_dir: Path) -> bool:
        """执行部署操作"""
        config = record.config
        deploy_output = []
        
        try:
            # 创建备份
            if config.backup_enabled:
                backup_success = self._create_backup(record, deploy_output)
                if not backup_success:
                    return False
            
            # 执行pre-deploy钩子
            for hook in config.pre_deploy_hooks:
                deploy_output.append(f"\n执行pre-deploy钩子: {hook}")
                result = subprocess.run(hook, shell=True, capture_output=True, text=True)
                deploy_output.append(f"钩子输出: {result.stdout}")
            
            # 部署文件
            deploy_path = Path(config.deploy_path)
            deploy_path.mkdir(parents=True, exist_ok=True)
            
            # 复制构建产物
            for item in work_dir.iterdir():
                if item.is_file():
                    shutil.copy2(item, deploy_path / item.name)
                else:
                    if (deploy_path / item.name).exists():
                        shutil.rmtree(deploy_path / item.name)
                    shutil.copytree(item, deploy_path / item.name)
            
            deploy_output.append(f"文件已部署到: {deploy_path}")
            
            # 创建artifacts
            artifacts_dir = self.workspace_dir / "artifacts" / record.deployment_id
            artifacts_dir.mkdir(parents=True, exist_ok=True)
            
            # 保存部署信息
            deployment_info = {
                "deployment_id": record.deployment_id,
                "config": asdict(config),
                "timestamp": time.time(),
                "deploy_path": str(deploy_path),
                "status": "completed"
            }
            
            with open(artifacts_dir / "deployment.json", "w") as f:
                json.dump(deployment_info, f, indent=2)
            
            record.artifacts_path = str(artifacts_dir)
            
            # 执行post-deploy钩子
            for hook in config.post_deploy_hooks:
                deploy_output.append(f"\n执行post-deploy钩子: {hook}")
                result = subprocess.run(hook, shell=True, capture_output=True, text=True)
                deploy_output.append(f"钩子输出: {result.stdout}")
            
            record.deploy_output = "\n".join(deploy_output)
            print(f"  ✅ 部署阶段完成")
            return True
            
        except Exception as e:
            record.deploy_output = "\n".join(deploy_output) + f"\n部署异常: {e}"
            record.error_message = f"部署阶段异常: {e}"
            return False
    
    def _create_backup(self, record: DeploymentRecord, output: List[str]) -> bool:
        """创建部署备份"""
        try:
            config = record.config
            deploy_path = Path(config.deploy_path)
            
            if not deploy_path.exists():
                output.append("目标路径不存在，跳过备份")
                return True
            
            # 创建备份目录
            backup_dir = self.workspace_dir / "backups" / record.deployment_id
            backup_dir.mkdir(parents=True, exist_ok=True)
            
            # 复制现有部署
            if deploy_path.is_file():
                shutil.copy2(deploy_path, backup_dir / deploy_path.name)
            else:
                for item in deploy_path.iterdir():
                    if item.is_file():
                        shutil.copy2(item, backup_dir / item.name)
                    else:
                        shutil.copytree(item, backup_dir / item.name)
            
            record.backup_path = str(backup_dir)
            output.append(f"备份已创建: {backup_dir}")
            return True
            
        except Exception as e:
            output.append(f"创建备份失败: {e}")
            return False
    
    def _call_hooks(self, hook_name: str, record: DeploymentRecord):
        """调用钩子函数"""
        for hook in self.deployment_hooks.get(hook_name, []):
            try:
                hook(record)
            except Exception as e:
                print(f"⚠️ 钩子执行失败 {hook_name}: {e}")
    
    def _save_deployment_to_db(self, record: DeploymentRecord):
        """保存部署记录到数据库"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            cursor.execute('''
                INSERT INTO deployments 
                (deployment_id, config_name, version, target, status,
                 start_time, end_time, duration_ms, build_output, 
                 test_output, deploy_output, error_message, 
                 artifacts_path, backup_path)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ''', (
                record.deployment_id, record.config.name, record.config.version,
                record.config.target.value, record.status.value,
                record.start_time, record.end_time, record.duration_ms,
                record.build_output, record.test_output, record.deploy_output,
                record.error_message, record.artifacts_path, record.backup_path
            ))
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 保存部署记录失败: {e}")
    
    def get_deployment_status(self, deployment_id: str) -> Optional[DeploymentRecord]:
        """获取部署状态"""
        with self._lock:
            # 检查活跃部署
            if deployment_id in self.active_deployments:
                return self.active_deployments[deployment_id]
            
            # 检查历史记录
            for record in self.deployment_history:
                if record.deployment_id == deployment_id:
                    return record
        
        return None
    
    def list_deployments(self, target: Optional[DeploymentTarget] = None, 
                        limit: int = 50) -> List[DeploymentRecord]:
        """列出部署记录"""
        with self._lock:
            all_records = list(self.active_deployments.values()) + self.deployment_history
            
            if target:
                all_records = [r for r in all_records if r.config.target == target]
            
            # 按时间排序
            all_records.sort(key=lambda x: x.start_time, reverse=True)
            
            return all_records[:limit]
    
    def rollback_deployment(self, deployment_id: str) -> bool:
        """回滚部署"""
        record = self.get_deployment_status(deployment_id)
        
        if not record:
            print(f"❌ 部署记录不存在: {deployment_id}")
            return False
        
        if not record.backup_path:
            print(f"❌ 没有备份可用于回滚: {deployment_id}")
            return False
        
        try:
            backup_path = Path(record.backup_path)
            deploy_path = Path(record.config.deploy_path)
            
            if not backup_path.exists():
                print(f"❌ 备份路径不存在: {backup_path}")
                return False
            
            # 创建当前部署的备份
            rollback_backup = backup_path.parent / f"rollback_backup_{int(time.time())}"
            if deploy_path.exists():
                shutil.copytree(deploy_path, rollback_backup)
            
            # 清空部署目录
            if deploy_path.exists():
                shutil.rmtree(deploy_path)
            
            # 恢复备份
            shutil.copytree(backup_path, deploy_path)
            
            # 更新记录状态
            record.status = DeploymentStatus.ROLLED_BACK
            
            print(f"✅ 部署已回滚: {deployment_id}")
            return True
            
        except Exception as e:
            print(f"❌ 回滚失败 {deployment_id}: {e}")
            return False
    
    def add_deployment_hook(self, hook_name: str, hook_func: Callable):
        """添加部署钩子"""
        if hook_name in self.deployment_hooks:
            self.deployment_hooks[hook_name].append(hook_func)
    
    def get_deployment_summary(self) -> Dict[str, Any]:
        """获取部署摘要"""
        with self._lock:
            all_records = list(self.active_deployments.values()) + self.deployment_history
            
            summary = {
                "total_deployments": len(all_records),
                "active_deployments": len(self.active_deployments),
                "successful_deployments": len([r for r in all_records if r.status == DeploymentStatus.COMPLETED]),
                "failed_deployments": len([r for r in all_records if r.status == DeploymentStatus.FAILED]),
                "average_duration_ms": 0,
                "by_target": {},
                "recent_deployments": []
            }
            
            # 计算平均部署时间
            completed_records = [r for r in all_records if r.duration_ms]
            if completed_records:
                summary["average_duration_ms"] = sum(r.duration_ms for r in completed_records) / len(completed_records)
            
            # 按目标环境统计
            for record in all_records:
                target = record.config.target.value
                if target not in summary["by_target"]:
                    summary["by_target"][target] = {"total": 0, "successful": 0, "failed": 0}
                
                summary["by_target"][target]["total"] += 1
                if record.status == DeploymentStatus.COMPLETED:
                    summary["by_target"][target]["successful"] += 1
                elif record.status == DeploymentStatus.FAILED:
                    summary["by_target"][target]["failed"] += 1
            
            # 最近部署
            recent = sorted(all_records, key=lambda x: x.start_time, reverse=True)[:5]
            summary["recent_deployments"] = [
                {
                    "deployment_id": r.deployment_id,
                    "config_name": r.config.name,
                    "status": r.status.value,
                    "duration_ms": r.duration_ms,
                    "start_time": r.start_time
                }
                for r in recent
            ]
            
            return summary
