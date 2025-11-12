"""
🏪 PIXLY v3.0 插件市场与生态平台

本地化插件生态系统：
- 本地插件仓库管理  
- 插件发现与安装
- 版本依赖解析
- 插件评级系统
- 开发者SDK

完全本地化实现，零网络依赖
"""

import json
import time
import shutil
import sqlite3
from pathlib import Path
from typing import Dict, List, Optional, Any, Set
from dataclasses import dataclass, asdict
from enum import Enum
import hashlib
import zipfile
import importlib.util


class PluginStatus(Enum):
    """插件状态"""
    AVAILABLE = "available"     # 可用
    INSTALLED = "installed"     # 已安装
    ENABLED = "enabled"         # 已启用
    DISABLED = "disabled"       # 已禁用
    DEPRECATED = "deprecated"   # 已废弃
    INCOMPATIBLE = "incompatible"  # 不兼容


class PluginCategory(Enum):
    """插件分类"""
    AI_MODEL = "ai_model"           # AI模型插件
    IMAGE_PROCESSOR = "image_processor"  # 图像处理
    FORMAT_CONVERTER = "format_converter"  # 格式转换
    OPTIMIZATION = "optimization"    # 性能优化
    MONITORING = "monitoring"       # 监控工具
    VALIDATION = "validation"       # 验证工具
    UTILITY = "utility"            # 实用工具
    INTEGRATION = "integration"     # 集成工具


@dataclass
class PluginMetadata:
    """插件元数据"""
    plugin_id: str
    name: str
    version: str
    description: str
    category: PluginCategory
    author: str
    homepage: Optional[str] = None
    dependencies: List[str] = None
    pixly_version_min: str = "3.0.0"
    pixly_version_max: Optional[str] = None
    python_version_min: str = "3.8"
    file_size_bytes: int = 0
    install_size_bytes: int = 0
    checksum_sha256: Optional[str] = None
    created_at: float = None
    updated_at: float = None
    
    def __post_init__(self):
        if self.dependencies is None:
            self.dependencies = []
        if self.created_at is None:
            self.created_at = time.time()
        if self.updated_at is None:
            self.updated_at = self.created_at


@dataclass 
class PluginRating:
    """插件评级"""
    plugin_id: str
    rating: float  # 1.0-5.0
    review_count: int
    download_count: int
    compatibility_score: float  # 0.0-1.0
    performance_score: float    # 0.0-1.0
    security_score: float       # 0.0-1.0
    last_updated: float = None
    
    def __post_init__(self):
        if self.last_updated is None:
            self.last_updated = time.time()


class PluginMarketplace:
    """
    🏪 企业级插件市场
    
    功能特性：
    - 本地插件仓库
    - 智能依赖解析
    - 插件生命周期管理
    - 社区评级系统
    """
    
    def __init__(self, marketplace_dir: str = "marketplace",
                 db_path: str = "data/marketplace.db"):
        
        self.marketplace_dir = Path(marketplace_dir)
        self.db_path = db_path
        
        # 插件数据
        self.available_plugins: Dict[str, PluginMetadata] = {}
        self.installed_plugins: Dict[str, PluginMetadata] = {}
        self.plugin_ratings: Dict[str, PluginRating] = {}
        
        # 本地仓库路径
        self.repo_dir = self.marketplace_dir / "repository"
        self.install_dir = self.marketplace_dir / "installed"
        self.cache_dir = self.marketplace_dir / "cache"
        
        # 初始化
        self._init_directories()
        self._init_database()
        self._load_local_repository()
        self._init_sample_plugins()
    
    def _init_directories(self):
        """初始化目录结构"""
        dirs = [
            self.marketplace_dir,
            self.repo_dir,
            self.install_dir,
            self.cache_dir,
            self.marketplace_dir / "manifests",
            self.marketplace_dir / "downloads"
        ]
        
        for directory in dirs:
            directory.mkdir(parents=True, exist_ok=True)
    
    def _init_database(self):
        """初始化市场数据库"""
        try:
            Path(self.db_path).parent.mkdir(parents=True, exist_ok=True)
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 插件元数据表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS plugins (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    plugin_id TEXT UNIQUE NOT NULL,
                    name TEXT NOT NULL,
                    version TEXT NOT NULL,
                    description TEXT,
                    category TEXT NOT NULL,
                    author TEXT NOT NULL,
                    homepage TEXT,
                    dependencies TEXT,
                    pixly_version_min TEXT,
                    pixly_version_max TEXT,
                    python_version_min TEXT,
                    file_size_bytes INTEGER,
                    install_size_bytes INTEGER,
                    checksum_sha256 TEXT,
                    status TEXT NOT NULL,
                    created_at REAL NOT NULL,
                    updated_at REAL NOT NULL
                )
            ''')
            
            # 插件评级表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS plugin_ratings (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    plugin_id TEXT UNIQUE NOT NULL,
                    rating REAL NOT NULL,
                    review_count INTEGER NOT NULL,
                    download_count INTEGER NOT NULL,
                    compatibility_score REAL,
                    performance_score REAL,
                    security_score REAL,
                    last_updated REAL NOT NULL,
                    FOREIGN KEY (plugin_id) REFERENCES plugins (plugin_id)
                )
            ''')
            
            # 安装历史表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS installation_history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    plugin_id TEXT NOT NULL,
                    action TEXT NOT NULL,
                    version TEXT NOT NULL,
                    timestamp REAL NOT NULL,
                    success BOOLEAN NOT NULL,
                    error_message TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_plugins_id ON plugins(plugin_id)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_plugins_category ON plugins(category)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_ratings_plugin ON plugin_ratings(plugin_id)')
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 初始化市场数据库失败: {e}")
    
    def _load_local_repository(self):
        """加载本地仓库"""
        manifest_dir = self.marketplace_dir / "manifests"
        
        for manifest_file in manifest_dir.glob("*.json"):
            try:
                with open(manifest_file, 'r', encoding='utf-8') as f:
                    plugin_data = json.load(f)
                
                # 转换为插件元数据
                metadata = PluginMetadata(
                    plugin_id=plugin_data["plugin_id"],
                    name=plugin_data["name"],
                    version=plugin_data["version"],
                    description=plugin_data["description"],
                    category=PluginCategory(plugin_data["category"]),
                    author=plugin_data["author"],
                    homepage=plugin_data.get("homepage"),
                    dependencies=plugin_data.get("dependencies", []),
                    pixly_version_min=plugin_data.get("pixly_version_min", "3.0.0"),
                    pixly_version_max=plugin_data.get("pixly_version_max"),
                    python_version_min=plugin_data.get("python_version_min", "3.8"),
                    file_size_bytes=plugin_data.get("file_size_bytes", 0),
                    checksum_sha256=plugin_data.get("checksum_sha256")
                )
                
                self.available_plugins[metadata.plugin_id] = metadata
                
            except Exception as e:
                print(f"⚠️ 加载插件清单失败 {manifest_file}: {e}")
    
    def _init_sample_plugins(self):
        """初始化示例插件"""
        sample_plugins = [
            {
                "plugin_id": "pixly_webp_optimizer",
                "name": "WebP智能优化器",
                "version": "1.0.0",
                "description": "专为WebP格式优化的AI驱动压缩插件",
                "category": "optimization",
                "author": "Pixly Team",
                "dependencies": ["pillow>=9.0.0"],
                "features": ["ai_optimization", "lossless_mode", "progressive_encoding"]
            },
            {
                "plugin_id": "pixly_batch_processor",
                "name": "批量处理引擎",
                "version": "1.2.0", 
                "description": "高性能批量图像处理工具",
                "category": "image_processor",
                "author": "Community",
                "dependencies": ["numpy>=1.21.0"],
                "features": ["parallel_processing", "progress_tracking", "error_recovery"]
            },
            {
                "plugin_id": "pixly_ai_upscaler",
                "name": "AI超分辨率放大",
                "version": "2.0.0",
                "description": "基于深度学习的图像超分辨率放大",
                "category": "ai_model",
                "author": "AI Labs",
                "dependencies": ["torch>=1.12.0", "torchvision>=0.13.0"],
                "features": ["4x_upscaling", "real_time_preview", "gpu_acceleration"]
            }
        ]
        
        for plugin_data in sample_plugins:
            if plugin_data["plugin_id"] not in self.available_plugins:
                metadata = PluginMetadata(
                    plugin_id=plugin_data["plugin_id"],
                    name=plugin_data["name"],
                    version=plugin_data["version"],
                    description=plugin_data["description"],
                    category=PluginCategory(plugin_data["category"]),
                    author=plugin_data["author"],
                    dependencies=plugin_data["dependencies"]
                )
                
                self.available_plugins[metadata.plugin_id] = metadata
                
                # 创建示例评级
                rating = PluginRating(
                    plugin_id=metadata.plugin_id,
                    rating=4.2 + (hash(metadata.plugin_id) % 8) / 10,  # 模拟评分
                    review_count=15 + (hash(metadata.plugin_id) % 50),
                    download_count=100 + (hash(metadata.plugin_id) % 500),
                    compatibility_score=0.9 + (hash(metadata.plugin_id) % 10) / 100,
                    performance_score=0.8 + (hash(metadata.plugin_id) % 20) / 100,
                    security_score=0.95
                )
                
                self.plugin_ratings[metadata.plugin_id] = rating
    
    def search_plugins(self, query: str = "", category: Optional[PluginCategory] = None,
                      sort_by: str = "rating") -> List[Dict[str, Any]]:
        """搜索插件"""
        results = []
        
        for plugin_id, metadata in self.available_plugins.items():
            # 分类过滤
            if category and metadata.category != category:
                continue
            
            # 关键词搜索
            if query:
                query_lower = query.lower()
                if not (query_lower in metadata.name.lower() or 
                       query_lower in metadata.description.lower() or
                       query_lower in metadata.author.lower()):
                    continue
            
            # 构建结果
            rating = self.plugin_ratings.get(plugin_id)
            
            result = {
                "plugin_id": plugin_id,
                "name": metadata.name,
                "version": metadata.version,
                "description": metadata.description,
                "category": metadata.category.value,
                "author": metadata.author,
                "status": self._get_plugin_status(plugin_id),
                "rating": rating.rating if rating else 0.0,
                "review_count": rating.review_count if rating else 0,
                "download_count": rating.download_count if rating else 0,
                "compatibility_score": rating.compatibility_score if rating else 1.0,
                "dependencies_count": len(metadata.dependencies),
                "file_size_mb": round(metadata.file_size_bytes / 1024 / 1024, 2)
            }
            
            results.append(result)
        
        # 排序
        if sort_by == "rating":
            results.sort(key=lambda x: x["rating"], reverse=True)
        elif sort_by == "downloads":
            results.sort(key=lambda x: x["download_count"], reverse=True)
        elif sort_by == "name":
            results.sort(key=lambda x: x["name"])
        elif sort_by == "updated":
            # 按更新时间排序
            results.sort(key=lambda x: self.available_plugins[x["plugin_id"]].updated_at, reverse=True)
        
        return results
    
    def get_plugin_details(self, plugin_id: str) -> Optional[Dict[str, Any]]:
        """获取插件详细信息"""
        if plugin_id not in self.available_plugins:
            return None
        
        metadata = self.available_plugins[plugin_id]
        rating = self.plugin_ratings.get(plugin_id)
        
        # 检查依赖状态
        dependency_status = []
        for dep in metadata.dependencies:
            dep_available = dep in self.available_plugins
            dependency_status.append({
                "name": dep,
                "available": dep_available,
                "installed": dep in self.installed_plugins if dep_available else False
            })
        
        details = {
            "metadata": asdict(metadata),
            "rating": asdict(rating) if rating else None,
            "status": self._get_plugin_status(plugin_id),
            "dependency_status": dependency_status,
            "installation_path": str(self.install_dir / plugin_id) if self._is_installed(plugin_id) else None,
            "compatibility": self._check_compatibility(metadata),
            "changelog": self._get_plugin_changelog(plugin_id),
            "screenshots": self._get_plugin_screenshots(plugin_id)
        }
        
        return details
    
    def install_plugin(self, plugin_id: str, force: bool = False) -> bool:
        """安装插件"""
        if plugin_id not in self.available_plugins:
            print(f"❌ 插件不存在: {plugin_id}")
            return False
        
        metadata = self.available_plugins[plugin_id]
        
        # 检查兼容性
        if not force and not self._check_compatibility(metadata)["compatible"]:
            print(f"❌ 插件不兼容: {plugin_id}")
            return False
        
        # 检查是否已安装
        if not force and self._is_installed(plugin_id):
            print(f"⚠️ 插件已安装: {plugin_id}")
            return True
        
        try:
            print(f"📦 安装插件: {metadata.name} v{metadata.version}")
            
            # 安装依赖
            for dep_id in metadata.dependencies:
                if dep_id in self.available_plugins and not self._is_installed(dep_id):
                    print(f"  📦 安装依赖: {dep_id}")
                    if not self.install_plugin(dep_id, force):
                        print(f"❌ 依赖安装失败: {dep_id}")
                        return False
            
            # 创建安装目录
            install_path = self.install_dir / plugin_id
            install_path.mkdir(parents=True, exist_ok=True)
            
            # 模拟插件文件创建 (实际应该从仓库下载)
            plugin_file = install_path / "__init__.py"
            
            plugin_code = f'''"""
{metadata.name} v{metadata.version}
{metadata.description}

Author: {metadata.author}
Category: {metadata.category.value}
"""

__version__ = "{metadata.version}"
__author__ = "{metadata.author}"
__category__ = "{metadata.category.value}"

def activate():
    """激活插件"""
    print(f"🔌 激活插件: {metadata.name}")
    return True

def deactivate():
    """停用插件"""
    print(f"🔌 停用插件: {metadata.name}")
    return True

def get_info():
    """获取插件信息"""
    return {{
        "name": "{metadata.name}",
        "version": "{metadata.version}",
        "description": "{metadata.description}",
        "author": "{metadata.author}",
        "category": "{metadata.category.value}"
    }}
'''
            
            with open(plugin_file, 'w', encoding='utf-8') as f:
                f.write(plugin_code)
            
            # 创建配置文件
            config_file = install_path / "plugin.json"
            with open(config_file, 'w', encoding='utf-8') as f:
                json.dump(asdict(metadata), f, indent=2, default=str)
            
            # 记录安装
            self.installed_plugins[plugin_id] = metadata
            self._log_installation(plugin_id, "install", metadata.version, True)
            
            # 更新下载统计
            if plugin_id in self.plugin_ratings:
                self.plugin_ratings[plugin_id].download_count += 1
            
            print(f"✅ 插件安装成功: {metadata.name}")
            return True
            
        except Exception as e:
            print(f"❌ 插件安装失败 {plugin_id}: {e}")
            self._log_installation(plugin_id, "install", metadata.version, False, str(e))
            return False
    
    def uninstall_plugin(self, plugin_id: str, force: bool = False) -> bool:
        """卸载插件"""
        if not self._is_installed(plugin_id):
            print(f"⚠️ 插件未安装: {plugin_id}")
            return True
        
        try:
            metadata = self.installed_plugins[plugin_id]
            
            # 检查依赖关系
            if not force:
                dependents = self._find_dependents(plugin_id)
                if dependents:
                    print(f"❌ 无法卸载，以下插件依赖它: {', '.join(dependents)}")
                    return False
            
            print(f"🗑️ 卸载插件: {metadata.name}")
            
            # 停用插件
            self._deactivate_plugin(plugin_id)
            
            # 删除安装目录
            install_path = self.install_dir / plugin_id
            if install_path.exists():
                shutil.rmtree(install_path)
            
            # 从已安装列表移除
            del self.installed_plugins[plugin_id]
            
            # 记录卸载
            self._log_installation(plugin_id, "uninstall", metadata.version, True)
            
            print(f"✅ 插件卸载成功: {metadata.name}")
            return True
            
        except Exception as e:
            print(f"❌ 插件卸载失败 {plugin_id}: {e}")
            return False
    
    def enable_plugin(self, plugin_id: str) -> bool:
        """启用插件"""
        if not self._is_installed(plugin_id):
            print(f"❌ 插件未安装: {plugin_id}")
            return False
        
        try:
            # 加载插件模块
            install_path = self.install_dir / plugin_id
            spec = importlib.util.spec_from_file_location(
                plugin_id, install_path / "__init__.py"
            )
            
            if spec and spec.loader:
                module = importlib.util.module_from_spec(spec)
                spec.loader.exec_module(module)
                
                # 调用激活函数
                if hasattr(module, 'activate'):
                    result = module.activate()
                    if result:
                        print(f"✅ 插件已启用: {plugin_id}")
                        return True
            
            print(f"❌ 插件启用失败: {plugin_id}")
            return False
            
        except Exception as e:
            print(f"❌ 插件启用异常 {plugin_id}: {e}")
            return False
    
    def disable_plugin(self, plugin_id: str) -> bool:
        """禁用插件"""
        return self._deactivate_plugin(plugin_id)
    
    def _deactivate_plugin(self, plugin_id: str) -> bool:
        """停用插件"""
        if not self._is_installed(plugin_id):
            return True
        
        try:
            install_path = self.install_dir / plugin_id
            spec = importlib.util.spec_from_file_location(
                plugin_id, install_path / "__init__.py"
            )
            
            if spec and spec.loader:
                module = importlib.util.module_from_spec(spec)
                spec.loader.exec_module(module)
                
                if hasattr(module, 'deactivate'):
                    module.deactivate()
            
            print(f"✅ 插件已停用: {plugin_id}")
            return True
            
        except Exception as e:
            print(f"⚠️ 插件停用异常 {plugin_id}: {e}")
            return False
    
    def _get_plugin_status(self, plugin_id: str) -> PluginStatus:
        """获取插件状态"""
        if plugin_id in self.installed_plugins:
            return PluginStatus.INSTALLED
        elif plugin_id in self.available_plugins:
            return PluginStatus.AVAILABLE
        else:
            return PluginStatus.INCOMPATIBLE
    
    def _is_installed(self, plugin_id: str) -> bool:
        """检查插件是否已安装"""
        return plugin_id in self.installed_plugins
    
    def _check_compatibility(self, metadata: PluginMetadata) -> Dict[str, Any]:
        """检查插件兼容性"""
        compatibility = {
            "compatible": True,
            "issues": []
        }
        
        # 检查Pixly版本
        current_pixly_version = "3.0.0"  # 当前版本
        if metadata.pixly_version_min and current_pixly_version < metadata.pixly_version_min:
            compatibility["compatible"] = False
            compatibility["issues"].append(f"需要Pixly版本 >= {metadata.pixly_version_min}")
        
        # 检查Python版本
        import sys
        python_version = f"{sys.version_info.major}.{sys.version_info.minor}"
        if metadata.python_version_min and python_version < metadata.python_version_min:
            compatibility["compatible"] = False
            compatibility["issues"].append(f"需要Python版本 >= {metadata.python_version_min}")
        
        return compatibility
    
    def _find_dependents(self, plugin_id: str) -> List[str]:
        """查找依赖此插件的其他插件"""
        dependents = []
        
        for other_id, other_metadata in self.installed_plugins.items():
            if plugin_id in other_metadata.dependencies:
                dependents.append(other_id)
        
        return dependents
    
    def _log_installation(self, plugin_id: str, action: str, version: str, 
                         success: bool, error_message: Optional[str] = None):
        """记录安装日志"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            cursor.execute('''
                INSERT INTO installation_history 
                (plugin_id, action, version, timestamp, success, error_message)
                VALUES (?, ?, ?, ?, ?, ?)
            ''', (plugin_id, action, version, time.time(), success, error_message))
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 记录安装日志失败: {e}")
    
    def _get_plugin_changelog(self, plugin_id: str) -> List[Dict[str, Any]]:
        """获取插件更新日志"""
        # 模拟更新日志
        return [
            {
                "version": "1.0.0",
                "date": "2024-01-15",
                "changes": ["初始版本发布", "基础功能实现"]
            }
        ]
    
    def _get_plugin_screenshots(self, plugin_id: str) -> List[str]:
        """获取插件截图"""
        # 模拟截图列表
        return []
    
    def list_installed_plugins(self) -> List[Dict[str, Any]]:
        """列出已安装插件"""
        results = []
        
        for plugin_id, metadata in self.installed_plugins.items():
            rating = self.plugin_ratings.get(plugin_id)
            
            result = {
                "plugin_id": plugin_id,
                "name": metadata.name,
                "version": metadata.version,
                "category": metadata.category.value,
                "author": metadata.author,
                "status": "installed",
                "rating": rating.rating if rating else 0.0,
                "install_path": str(self.install_dir / plugin_id)
            }
            
            results.append(result)
        
        return results
    
    def get_marketplace_stats(self) -> Dict[str, Any]:
        """获取市场统计"""
        stats = {
            "total_plugins": len(self.available_plugins),
            "installed_plugins": len(self.installed_plugins),
            "categories": {},
            "top_rated": [],
            "most_downloaded": [],
            "installation_success_rate": 0.0
        }
        
        # 分类统计
        for metadata in self.available_plugins.values():
            category = metadata.category.value
            if category not in stats["categories"]:
                stats["categories"][category] = 0
            stats["categories"][category] += 1
        
        # 热门插件
        plugins_with_ratings = [
            (plugin_id, rating) for plugin_id, rating in self.plugin_ratings.items()
        ]
        
        # 最高评分
        plugins_with_ratings.sort(key=lambda x: x[1].rating, reverse=True)
        stats["top_rated"] = [
            {
                "plugin_id": plugin_id,
                "name": self.available_plugins[plugin_id].name,
                "rating": rating.rating
            }
            for plugin_id, rating in plugins_with_ratings[:5]
        ]
        
        # 最多下载
        plugins_with_ratings.sort(key=lambda x: x[1].download_count, reverse=True)
        stats["most_downloaded"] = [
            {
                "plugin_id": plugin_id,
                "name": self.available_plugins[plugin_id].name,
                "downloads": rating.download_count
            }
            for plugin_id, rating in plugins_with_ratings[:5]
        ]
        
        return stats
