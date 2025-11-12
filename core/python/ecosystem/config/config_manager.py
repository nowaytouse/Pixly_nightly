"""
⚙️ PIXLY v3.0 企业级配置管理器

统一配置管理解决方案：
- 动态配置热更新
- 多环境配置隔离
- 配置变更追踪
- 本地化存储架构
- 配置验证框架

完全本地化实现，零网络依赖
"""

import json
import time
import threading
from pathlib import Path
from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass
import sqlite3
import hashlib


@dataclass
class ConfigChange:
    """配置变更记录"""
    key: str
    old_value: Any
    new_value: Any
    timestamp: float
    source: str = "system"


class ConfigManager:
    """
    ⚙️ 企业级配置管理器
    
    功能特性：
    - 分层配置加载
    - 动态配置更新
    - 变更历史追踪
    - 环境隔离支持
    """
    
    def __init__(self, config_dir: str = "config", 
                 db_path: str = "data/config.db",
                 environment: str = "production"):
        
        self.config_dir = Path(config_dir)
        self.db_path = db_path
        self.environment = environment
        
        # 配置数据
        self.config_data: Dict[str, Any] = {}
        self.config_metadata: Dict[str, Dict[str, Any]] = {}
        
        # 变更追踪
        self.change_history: List[ConfigChange] = []
        self.change_listeners: List[Callable] = []
        
        # 文件监控
        self.file_watchers: Dict[str, float] = {}
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 监控线程
        self._watch_thread = None
        self._stop_watching = False
        
        # 初始化
        self._init_directories()
        self._init_database()
        self._load_all_configs()
        self._start_file_watching()
    
    def _init_directories(self):
        """初始化配置目录"""
        self.config_dir.mkdir(parents=True, exist_ok=True)
        
        # 环境特定目录
        env_dirs = ["default", self.environment, "local"]
        for env_dir in env_dirs:
            (self.config_dir / env_dir).mkdir(exist_ok=True)
    
    def _init_database(self):
        """初始化配置数据库"""
        try:
            Path(self.db_path).parent.mkdir(parents=True, exist_ok=True)
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 配置变更历史表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS config_changes (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    config_key TEXT NOT NULL,
                    old_value TEXT,
                    new_value TEXT,
                    timestamp REAL NOT NULL,
                    source TEXT NOT NULL,
                    environment TEXT NOT NULL,
                    checksum TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 配置快照表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS config_snapshots (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    snapshot_name TEXT UNIQUE NOT NULL,
                    config_data TEXT NOT NULL,
                    timestamp REAL NOT NULL,
                    description TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_changes_key_time ON config_changes(config_key, timestamp)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_snapshots_name ON config_snapshots(snapshot_name)')
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 初始化配置数据库失败: {e}")
    
    def _load_all_configs(self):
        """加载所有配置文件"""
        print("⚙️ 加载配置文件...")
        
        # 配置加载优先级: default < environment < local
        config_layers = ["default", self.environment, "local"]
        
        for layer in config_layers:
            layer_path = self.config_dir / layer
            if layer_path.exists():
                self._load_config_layer(layer, layer_path)
    
    def _load_config_layer(self, layer_name: str, layer_path: Path):
        """加载配置层"""
        for config_file in layer_path.glob("*.json"):
            try:
                with open(config_file, 'r', encoding='utf-8') as f:
                    layer_config = json.load(f)
                
                # 更新配置数据 (后加载的覆盖先加载的)
                self._merge_config(layer_config, source=f"{layer_name}:{config_file.name}")
                
                # 记录文件修改时间
                self.file_watchers[str(config_file)] = config_file.stat().st_mtime
                
                print(f"  ✅ 加载配置: {layer_name}/{config_file.name}")
                
            except Exception as e:
                print(f"  ❌ 加载配置失败 {config_file}: {e}")
    
    def _merge_config(self, new_config: Dict[str, Any], source: str = "unknown"):
        """合并配置数据"""
        with self._lock:
            for key, value in new_config.items():
                old_value = self.config_data.get(key)
                
                if old_value != value:
                    # 记录变更
                    change = ConfigChange(
                        key=key,
                        old_value=old_value,
                        new_value=value,
                        timestamp=time.time(),
                        source=source
                    )
                    self.change_history.append(change)
                    
                    # 更新配置
                    self.config_data[key] = value
                    
                    # 更新元数据
                    self.config_metadata[key] = {
                        "source": source,
                        "last_updated": time.time(),
                        "type": type(value).__name__
                    }
                    
                    # 通知监听器
                    self._notify_change_listeners(key, old_value, value)
    
    def get(self, key: str, default: Any = None) -> Any:
        """获取配置值"""
        with self._lock:
            return self.config_data.get(key, default)
    
    def set(self, key: str, value: Any, source: str = "api") -> bool:
        """设置配置值"""
        try:
            with self._lock:
                old_value = self.config_data.get(key)
                
                if old_value != value:
                    # 记录变更
                    change = ConfigChange(
                        key=key,
                        old_value=old_value,
                        new_value=value,
                        timestamp=time.time(),
                        source=source
                    )
                    self.change_history.append(change)
                    
                    # 更新配置
                    self.config_data[key] = value
                    
                    # 更新元数据
                    self.config_metadata[key] = {
                        "source": source,
                        "last_updated": time.time(),
                        "type": type(value).__name__
                    }
                    
                    # 保存变更到数据库
                    self._save_change_to_db(change)
                    
                    # 通知监听器
                    self._notify_change_listeners(key, old_value, value)
                    
                    return True
            
            return False
            
        except Exception as e:
            print(f"⚠️ 设置配置失败 {key}: {e}")
            return False
    
    def get_all(self) -> Dict[str, Any]:
        """获取所有配置"""
        with self._lock:
            return self.config_data.copy()
    
    def get_metadata(self, key: str) -> Optional[Dict[str, Any]]:
        """获取配置元数据"""
        with self._lock:
            return self.config_metadata.get(key)
    
    def delete(self, key: str, source: str = "api") -> bool:
        """删除配置"""
        try:
            with self._lock:
                if key in self.config_data:
                    old_value = self.config_data[key]
                    
                    # 记录变更
                    change = ConfigChange(
                        key=key,
                        old_value=old_value,
                        new_value=None,
                        timestamp=time.time(),
                        source=source
                    )
                    self.change_history.append(change)
                    
                    # 删除配置
                    del self.config_data[key]
                    del self.config_metadata[key]
                    
                    # 保存变更
                    self._save_change_to_db(change)
                    
                    # 通知监听器
                    self._notify_change_listeners(key, old_value, None)
                    
                    return True
            
            return False
            
        except Exception as e:
            print(f"⚠️ 删除配置失败 {key}: {e}")
            return False
    
    def add_change_listener(self, listener: Callable[[str, Any, Any], None]):
        """添加配置变更监听器"""
        self.change_listeners.append(listener)
    
    def remove_change_listener(self, listener: Callable):
        """移除配置变更监听器"""
        if listener in self.change_listeners:
            self.change_listeners.remove(listener)
    
    def _notify_change_listeners(self, key: str, old_value: Any, new_value: Any):
        """通知配置变更监听器"""
        for listener in self.change_listeners:
            try:
                listener(key, old_value, new_value)
            except Exception as e:
                print(f"⚠️ 配置变更监听器异常: {e}")
    
    def _save_change_to_db(self, change: ConfigChange):
        """保存变更到数据库"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 计算值的校验和
            value_str = json.dumps(change.new_value, sort_keys=True)
            checksum = hashlib.md5(value_str.encode()).hexdigest()
            
            cursor.execute('''
                INSERT INTO config_changes 
                (config_key, old_value, new_value, timestamp, source, environment, checksum)
                VALUES (?, ?, ?, ?, ?, ?, ?)
            ''', (
                change.key,
                json.dumps(change.old_value) if change.old_value is not None else None,
                json.dumps(change.new_value) if change.new_value is not None else None,
                change.timestamp,
                change.source,
                self.environment,
                checksum
            ))
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 保存配置变更失败: {e}")
    
    def create_snapshot(self, name: str, description: str = "") -> bool:
        """创建配置快照"""
        try:
            with self._lock:
                snapshot_data = json.dumps(self.config_data, indent=2)
            
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            cursor.execute('''
                INSERT OR REPLACE INTO config_snapshots 
                (snapshot_name, config_data, timestamp, description)
                VALUES (?, ?, ?, ?)
            ''', (name, snapshot_data, time.time(), description))
            
            conn.commit()
            conn.close()
            
            print(f"✅ 创建配置快照: {name}")
            return True
            
        except Exception as e:
            print(f"⚠️ 创建配置快照失败: {e}")
            return False
    
    def restore_snapshot(self, name: str) -> bool:
        """恢复配置快照"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            cursor.execute('SELECT config_data FROM config_snapshots WHERE snapshot_name = ?', (name,))
            row = cursor.fetchone()
            conn.close()
            
            if not row:
                print(f"❌ 快照不存在: {name}")
                return False
            
            snapshot_config = json.loads(row[0])
            
            # 备份当前配置
            backup_name = f"before_restore_{int(time.time())}"
            self.create_snapshot(backup_name, f"恢复快照 {name} 前的备份")
            
            # 恢复配置
            with self._lock:
                # 清空当前配置
                old_config = self.config_data.copy()
                self.config_data.clear()
                self.config_metadata.clear()
                
                # 加载快照配置
                for key, value in snapshot_config.items():
                    self.config_data[key] = value
                    self.config_metadata[key] = {
                        "source": f"snapshot:{name}",
                        "last_updated": time.time(),
                        "type": type(value).__name__
                    }
                    
                    # 通知变更
                    old_value = old_config.get(key)
                    if old_value != value:
                        self._notify_change_listeners(key, old_value, value)
            
            print(f"✅ 恢复配置快照: {name}")
            return True
            
        except Exception as e:
            print(f"⚠️ 恢复配置快照失败: {e}")
            return False
    
    def list_snapshots(self) -> List[Dict[str, Any]]:
        """列出所有快照"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            cursor.execute('''
                SELECT snapshot_name, timestamp, description 
                FROM config_snapshots 
                ORDER BY timestamp DESC
            ''')
            rows = cursor.fetchall()
            conn.close()
            
            snapshots = []
            for row in rows:
                snapshots.append({
                    "name": row[0],
                    "timestamp": row[1],
                    "description": row[2],
                    "created_at": time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(row[1]))
                })
            
            return snapshots
            
        except Exception as e:
            print(f"⚠️ 获取快照列表失败: {e}")
            return []
    
    def get_change_history(self, key: Optional[str] = None, 
                          limit: int = 100) -> List[Dict[str, Any]]:
        """获取配置变更历史"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            if key:
                cursor.execute('''
                    SELECT config_key, old_value, new_value, timestamp, source 
                    FROM config_changes 
                    WHERE config_key = ? 
                    ORDER BY timestamp DESC 
                    LIMIT ?
                ''', (key, limit))
            else:
                cursor.execute('''
                    SELECT config_key, old_value, new_value, timestamp, source 
                    FROM config_changes 
                    ORDER BY timestamp DESC 
                    LIMIT ?
                ''', (limit,))
            
            rows = cursor.fetchall()
            conn.close()
            
            history = []
            for row in rows:
                history.append({
                    "key": row[0],
                    "old_value": json.loads(row[1]) if row[1] else None,
                    "new_value": json.loads(row[2]) if row[2] else None,
                    "timestamp": row[3],
                    "source": row[4],
                    "changed_at": time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(row[3]))
                })
            
            return history
            
        except Exception as e:
            print(f"⚠️ 获取变更历史失败: {e}")
            return []
    
    def _start_file_watching(self):
        """启动文件监控"""
        if self._watch_thread is not None:
            return
        
        self._stop_watching = False
        self._watch_thread = threading.Thread(target=self._file_watch_loop)
        self._watch_thread.daemon = True
        self._watch_thread.start()
    
    def _file_watch_loop(self):
        """文件监控循环"""
        while not self._stop_watching:
            try:
                self._check_file_changes()
                time.sleep(5)  # 每5秒检查一次
                
            except Exception as e:
                print(f"⚠️ 文件监控异常: {e}")
                time.sleep(10)
    
    def _check_file_changes(self):
        """检查文件变更"""
        for file_path, last_mtime in list(self.file_watchers.items()):
            try:
                path = Path(file_path)
                if path.exists():
                    current_mtime = path.stat().st_mtime
                    if current_mtime > last_mtime:
                        print(f"🔄 检测到配置文件变更: {file_path}")
                        self._reload_config_file(path)
                        self.file_watchers[file_path] = current_mtime
                        
            except Exception as e:
                print(f"⚠️ 检查文件变更失败 {file_path}: {e}")
    
    def _reload_config_file(self, config_file: Path):
        """重新加载配置文件"""
        try:
            with open(config_file, 'r', encoding='utf-8') as f:
                new_config = json.load(f)
            
            source = f"file_reload:{config_file.name}"
            self._merge_config(new_config, source=source)
            
            print(f"✅ 重新加载配置文件: {config_file}")
            
        except Exception as e:
            print(f"❌ 重新加载配置文件失败 {config_file}: {e}")
    
    def validate_config(self) -> List[str]:
        """验证配置"""
        issues = []
        
        # 基本验证
        required_keys = ["app_name", "version"]
        for key in required_keys:
            if key not in self.config_data:
                issues.append(f"缺少必需配置: {key}")
        
        return issues
    
    def export_config(self, format: str = "json") -> str:
        """导出配置"""
        with self._lock:
            if format == "json":
                return json.dumps(self.config_data, indent=2, ensure_ascii=False)
            else:
                raise ValueError(f"不支持的导出格式: {format}")
    
    def shutdown(self):
        """关闭配置管理器"""
        self._stop_watching = True
        if self._watch_thread:
            self._watch_thread.join(timeout=5.0)
        
        print("⚙️ 配置管理器已关闭")
