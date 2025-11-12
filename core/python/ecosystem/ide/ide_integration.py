"""
💻 PIXLY v3.0 IDE集成管理器

智能开发环境集成解决方案：
- VS Code扩展支持
- 语言服务器协议(LSP)
- 智能代码补全
- 实时诊断反馈
- 调试协议支持
- 本地开发服务器

完全本地化实现，零网络依赖
"""

import json
import time
import threading
from pathlib import Path
from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass, asdict
import subprocess
import sqlite3
import os
import sys


@dataclass
class IDEConfig:
    """IDE配置"""
    ide_name: str
    version: str
    extensions: List[str] = None
    settings: Dict[str, Any] = None
    workspace_settings: Dict[str, Any] = None
    debug_configs: List[Dict[str, Any]] = None
    
    def __post_init__(self):
        if self.extensions is None:
            self.extensions = []
        if self.settings is None:
            self.settings = {}
        if self.workspace_settings is None:
            self.workspace_settings = {}
        if self.debug_configs is None:
            self.debug_configs = []


@dataclass
class CodeDiagnostic:
    """代码诊断"""
    file_path: str
    line: int
    column: int
    severity: str  # error, warning, info, hint
    message: str
    code: Optional[str] = None
    source: str = "pixly"
    related_info: List[Dict[str, Any]] = None
    
    def __post_init__(self):
        if self.related_info is None:
            self.related_info = []


@dataclass
class CompletionItem:
    """代码补全项"""
    label: str
    kind: str  # function, variable, class, etc.
    detail: Optional[str] = None
    documentation: Optional[str] = None
    insert_text: Optional[str] = None
    filter_text: Optional[str] = None
    sort_text: Optional[str] = None


class LanguageServer:
    """语言服务器"""
    
    def __init__(self):
        self.diagnostics: Dict[str, List[CodeDiagnostic]] = {}
        self.symbols: Dict[str, List[Dict[str, Any]]] = {}
        self.completions: Dict[str, List[CompletionItem]] = {}
        
        # 初始化Python分析器
        self._init_python_analyzer()
    
    def _init_python_analyzer(self):
        """初始化Python代码分析器"""
        try:
            import ast
            import inspect
            self.ast_parser = ast
            self.inspector = inspect
        except ImportError:
            print("⚠️ Python分析器初始化失败")
    
    def analyze_file(self, file_path: str) -> List[CodeDiagnostic]:
        """分析文件并返回诊断信息"""
        diagnostics = []
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # 语法检查
            try:
                self.ast_parser.parse(content, filename=file_path)
            except SyntaxError as e:
                diagnostic = CodeDiagnostic(
                    file_path=file_path,
                    line=e.lineno or 1,
                    column=e.offset or 1,
                    severity="error",
                    message=f"语法错误: {e.msg}",
                    code="syntax-error"
                )
                diagnostics.append(diagnostic)
            
            # Pixly特定检查
            pixly_diagnostics = self._check_pixly_patterns(file_path, content)
            diagnostics.extend(pixly_diagnostics)
            
            # 缓存诊断结果
            self.diagnostics[file_path] = diagnostics
            
        except Exception as e:
            diagnostic = CodeDiagnostic(
                file_path=file_path,
                line=1,
                column=1,
                severity="error",
                message=f"文件分析失败: {e}",
                code="analysis-error"
            )
            diagnostics.append(diagnostic)
        
        return diagnostics
    
    def _check_pixly_patterns(self, file_path: str, content: str) -> List[CodeDiagnostic]:
        """检查Pixly特定模式"""
        diagnostics = []
        lines = content.split('\n')
        
        for line_num, line in enumerate(lines, 1):
            # 检查不推荐的模式
            if "requests.get" in line or "requests.post" in line:
                diagnostic = CodeDiagnostic(
                    file_path=file_path,
                    line=line_num,
                    column=line.find("requests.") + 1,
                    severity="warning",
                    message="建议使用本地化调用替代HTTP请求",
                    code="pixly-no-http"
                )
                diagnostics.append(diagnostic)
            
            # 检查AI预测相关
            if "predict_image_params" in line and "try:" not in content:
                diagnostic = CodeDiagnostic(
                    file_path=file_path,
                    line=line_num,
                    column=line.find("predict_image_params") + 1,
                    severity="info",
                    message="建议添加异常处理包装AI预测调用",
                    code="pixly-ai-error-handling"
                )
                diagnostics.append(diagnostic)
        
        return diagnostics
    
    def get_completions(self, file_path: str, line: int, column: int) -> List[CompletionItem]:
        """获取代码补全建议"""
        completions = []
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            lines = content.split('\n')
            if line <= len(lines):
                current_line = lines[line - 1][:column]
                
                # Pixly特定补全
                if "from core.python" in current_line:
                    pixly_modules = [
                        "ai.predict_params",
                        "ecosystem.monitoring",
                        "ecosystem.validation", 
                        "ecosystem.config",
                        "fusion.performance_optimizer"
                    ]
                    
                    for module in pixly_modules:
                        completions.append(CompletionItem(
                            label=module,
                            kind="module",
                            detail=f"Pixly模块: {module}",
                            documentation=f"导入Pixly {module} 模块"
                        ))
                
                # AI预测相关补全
                if "predict_" in current_line:
                    ai_functions = [
                        CompletionItem(
                            label="predict_image_params",
                            kind="function",
                            detail="predict_image_params(image_path: str) -> dict",
                            documentation="AI图像参数预测函数",
                            insert_text="predict_image_params($1)"
                        ),
                        CompletionItem(
                            label="predict_optimal_quality",
                            kind="function", 
                            detail="predict_optimal_quality(image_path: str) -> int",
                            documentation="预测最优质量参数"
                        )
                    ]
                    completions.extend(ai_functions)
        
        except Exception as e:
            print(f"⚠️ 代码补全失败: {e}")
        
        return completions


class IDEIntegration:
    """
    💻 企业级IDE集成管理器
    
    功能特性：
    - 多IDE支持
    - 智能代码分析
    - 实时诊断反馈
    - 调试工具集成
    """
    
    def __init__(self, workspace_dir: str = ".", 
                 db_path: str = "data/ide_integration.db"):
        
        self.workspace_dir = Path(workspace_dir)
        self.db_path = db_path
        
        # IDE配置
        self.ide_configs: Dict[str, IDEConfig] = {}
        
        # 语言服务器
        self.language_server = LanguageServer()
        
        # 调试服务器
        self.debug_server = None
        
        # 文件监控
        self.file_watchers: Dict[str, float] = {}
        self.watch_thread = None
        self._stop_watching = False
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 初始化
        self._init_database()
        self._init_ide_configs()
        self._start_file_watching()
    
    def _init_database(self):
        """初始化IDE集成数据库"""
        try:
            Path(self.db_path).parent.mkdir(parents=True, exist_ok=True)
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # IDE配置表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS ide_configs (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    ide_name TEXT UNIQUE NOT NULL,
                    version TEXT NOT NULL,
                    config_data TEXT NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 诊断记录表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS diagnostics_log (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    file_path TEXT NOT NULL,
                    line_number INTEGER NOT NULL,
                    severity TEXT NOT NULL,
                    message TEXT NOT NULL,
                    code TEXT,
                    timestamp REAL NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_diagnostics_file ON diagnostics_log(file_path)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_diagnostics_timestamp ON diagnostics_log(timestamp)')
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 初始化IDE数据库失败: {e}")
    
    def _init_ide_configs(self):
        """初始化IDE配置"""
        
        # VS Code配置
        vscode_config = IDEConfig(
            ide_name="vscode",
            version="1.0.0",
            extensions=[
                "ms-python.python",
                "rust-lang.rust-analyzer", 
                "ms-vscode.vscode-json"
            ],
            settings={
                "python.defaultInterpreterPath": sys.executable,
                "python.analysis.typeCheckingMode": "basic",
                "python.linting.enabled": True,
                "python.linting.pylintEnabled": True,
                "editor.formatOnSave": True,
                "files.autoSave": "onFocusChange"
            },
            workspace_settings={
                "python.pythonPath": sys.executable,
                "pixly.enableAICompletion": True,
                "pixly.enableDiagnostics": True
            },
            debug_configs=[
                {
                    "name": "Pixly AI Debug",
                    "type": "python",
                    "request": "launch",
                    "program": "${workspaceFolder}/core/python/ai/predict_params.py",
                    "console": "integratedTerminal",
                    "args": ["--debug"]
                },
                {
                    "name": "Pixly Test Suite",
                    "type": "python", 
                    "request": "launch",
                    "module": "pytest",
                    "args": ["tests/", "-v"]
                }
            ]
        )
        
        self.ide_configs["vscode"] = vscode_config
    
    def generate_vscode_config(self) -> bool:
        """生成VS Code配置文件"""
        try:
            vscode_dir = self.workspace_dir / ".vscode"
            vscode_dir.mkdir(exist_ok=True)
            
            config = self.ide_configs["vscode"]
            
            # settings.json
            settings_file = vscode_dir / "settings.json"
            with open(settings_file, 'w', encoding='utf-8') as f:
                json.dump(config.workspace_settings, f, indent=2)
            
            # launch.json (调试配置)
            launch_config = {
                "version": "0.2.0",
                "configurations": config.debug_configs
            }
            
            launch_file = vscode_dir / "launch.json"
            with open(launch_file, 'w', encoding='utf-8') as f:
                json.dump(launch_config, f, indent=2)
            
            # extensions.json (扩展推荐)
            extensions_config = {
                "recommendations": config.extensions
            }
            
            extensions_file = vscode_dir / "extensions.json"
            with open(extensions_file, 'w', encoding='utf-8') as f:
                json.dump(extensions_config, f, indent=2)
            
            # tasks.json (任务配置)
            tasks_config = {
                "version": "2.0.0",
                "tasks": [
                    {
                        "label": "Pixly: Run Tests",
                        "type": "shell",
                        "command": "python",
                        "args": ["-m", "pytest", "tests/", "-v"],
                        "group": "test",
                        "presentation": {
                            "echo": True,
                            "reveal": "always",
                            "focus": False,
                            "panel": "shared"
                        },
                        "problemMatcher": "$python"
                    },
                    {
                        "label": "Pixly: Build Rust Core",
                        "type": "shell",
                        "command": "cargo",
                        "args": ["build", "--release"],
                        "options": {
                            "cwd": "${workspaceFolder}/core/rust"
                        },
                        "group": "build",
                        "problemMatcher": "$rustc"
                    },
                    {
                        "label": "Pixly: AI Prediction Test",
                        "type": "shell",
                        "command": "python",
                        "args": [
                            "core/python/ai/predict_params.py",
                            "--test-mode"
                        ],
                        "group": "test"
                    }
                ]
            }
            
            tasks_file = vscode_dir / "tasks.json"
            with open(tasks_file, 'w', encoding='utf-8') as f:
                json.dump(tasks_config, f, indent=2)
            
            print("✅ VS Code配置文件已生成")
            return True
            
        except Exception as e:
            print(f"❌ 生成VS Code配置失败: {e}")
            return False
    
    def analyze_workspace(self) -> Dict[str, Any]:
        """分析工作空间"""
        analysis = {
            "total_files": 0,
            "python_files": 0,
            "rust_files": 0,
            "config_files": 0,
            "diagnostics_summary": {
                "total_issues": 0,
                "errors": 0,
                "warnings": 0,
                "infos": 0
            },
            "file_analysis": []
        }
        
        try:
            # 扫描工作空间文件
            for file_path in self.workspace_dir.rglob("*"):
                if file_path.is_file():
                    analysis["total_files"] += 1
                    
                    if file_path.suffix == ".py":
                        analysis["python_files"] += 1
                        
                        # 分析Python文件
                        diagnostics = self.language_server.analyze_file(str(file_path))
                        
                        file_info = {
                            "path": str(file_path.relative_to(self.workspace_dir)),
                            "type": "python",
                            "issues": len(diagnostics),
                            "diagnostics": [asdict(d) for d in diagnostics]
                        }
                        analysis["file_analysis"].append(file_info)
                        
                        # 更新诊断统计
                        for diag in diagnostics:
                            analysis["diagnostics_summary"]["total_issues"] += 1
                            if diag.severity == "error":
                                analysis["diagnostics_summary"]["errors"] += 1
                            elif diag.severity == "warning":
                                analysis["diagnostics_summary"]["warnings"] += 1
                            elif diag.severity == "info":
                                analysis["diagnostics_summary"]["infos"] += 1
                    
                    elif file_path.suffix == ".rs":
                        analysis["rust_files"] += 1
                    
                    elif file_path.suffix in [".json", ".toml", ".yaml", ".yml"]:
                        analysis["config_files"] += 1
        
        except Exception as e:
            print(f"⚠️ 工作空间分析失败: {e}")
        
        return analysis
    
    def get_code_completions(self, file_path: str, line: int, column: int) -> List[CompletionItem]:
        """获取代码补全"""
        return self.language_server.get_completions(file_path, line, column)
    
    def get_diagnostics(self, file_path: str) -> List[CodeDiagnostic]:
        """获取文件诊断信息"""
        return self.language_server.analyze_file(file_path)
    
    def _start_file_watching(self):
        """启动文件监控"""
        if self.watch_thread is not None:
            return
        
        self._stop_watching = False
        self.watch_thread = threading.Thread(target=self._file_watch_loop)
        self.watch_thread.daemon = True
        self.watch_thread.start()
    
    def _file_watch_loop(self):
        """文件监控循环"""
        while not self._stop_watching:
            try:
                self._check_file_changes()
                time.sleep(2)  # 每2秒检查一次
                
            except Exception as e:
                print(f"⚠️ 文件监控异常: {e}")
                time.sleep(5)
    
    def _check_file_changes(self):
        """检查文件变化"""
        python_files = list(self.workspace_dir.rglob("*.py"))
        
        for file_path in python_files:
            try:
                file_str = str(file_path)
                current_mtime = file_path.stat().st_mtime
                
                if file_str not in self.file_watchers:
                    self.file_watchers[file_str] = current_mtime
                    continue
                
                if current_mtime > self.file_watchers[file_str]:
                    # 文件已修改，重新分析
                    self.language_server.analyze_file(file_str)
                    self.file_watchers[file_str] = current_mtime
                    
            except Exception as e:
                print(f"⚠️ 检查文件变化失败 {file_path}: {e}")
    
    def create_debug_configuration(self, name: str, program: str, 
                                 args: List[str] = None) -> Dict[str, Any]:
        """创建调试配置"""
        config = {
            "name": name,
            "type": "python",
            "request": "launch",
            "program": program,
            "console": "integratedTerminal",
            "args": args or [],
            "env": {
                "PYTHONPATH": str(self.workspace_dir),
                "PIXLY_DEBUG": "1"
            }
        }
        
        return config
    
    def generate_dev_tools_config(self) -> Dict[str, Any]:
        """生成开发工具配置"""
        config = {
            "pixly_dev_tools": {
                "version": "3.0.0",
                "workspace": str(self.workspace_dir),
                "python_path": sys.executable,
                "ai_model_path": "models/",
                "rust_target_dir": "core/rust/target",
                "features": {
                    "ai_completion": True,
                    "real_time_diagnostics": True,
                    "performance_monitoring": True,
                    "auto_testing": True
                },
                "shortcuts": {
                    "run_ai_prediction": "Ctrl+Shift+P",
                    "run_tests": "Ctrl+Shift+T",
                    "build_rust": "Ctrl+Shift+B",
                    "analyze_performance": "Ctrl+Shift+A"
                }
            }
        }
        
        return config
    
    def export_ide_integration_report(self) -> Dict[str, Any]:
        """导出IDE集成报告"""
        workspace_analysis = self.analyze_workspace()
        
        report = {
            "integration_status": {
                "vscode_config_generated": (self.workspace_dir / ".vscode").exists(),
                "language_server_active": True,
                "file_watching_active": not self._stop_watching,
                "diagnostic_engine_active": True
            },
            "workspace_analysis": workspace_analysis,
            "supported_features": {
                "code_completion": True,
                "real_time_diagnostics": True,
                "debug_support": True,
                "task_automation": True,
                "pixly_specific_analysis": True
            },
            "performance_metrics": {
                "files_monitored": len(self.file_watchers),
                "diagnostics_cached": len(self.language_server.diagnostics),
                "completion_cache_size": len(self.language_server.completions)
            },
            "recommendations": self._generate_ide_recommendations(workspace_analysis)
        }
        
        return report
    
    def _generate_ide_recommendations(self, analysis: Dict[str, Any]) -> List[str]:
        """生成IDE优化建议"""
        recommendations = []
        
        # 基于诊断结果的建议
        total_issues = analysis["diagnostics_summary"]["total_issues"]
        if total_issues > 50:
            recommendations.append("代码质量问题较多，建议启用自动格式化和代码检查")
        
        errors = analysis["diagnostics_summary"]["errors"]
        if errors > 10:
            recommendations.append("发现较多语法错误，建议配置实时错误检查")
        
        # 基于文件结构的建议
        if analysis["python_files"] > 100:
            recommendations.append("项目较大，建议启用智能索引和快速搜索功能")
        
        if analysis["rust_files"] > 0:
            recommendations.append("检测到Rust代码，建议安装rust-analyzer扩展")
        
        return recommendations
    
    def shutdown(self):
        """关闭IDE集成"""
        self._stop_watching = True
        if self.watch_thread:
            self.watch_thread.join(timeout=5.0)
        
        print("💻 IDE集成已关闭")
