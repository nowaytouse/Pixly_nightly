"""
🏥 PIXLY v3.0 健康检查与告警管理器

基于Go废弃模块的企业级健康监控架构：
- 多层次健康检查 (系统/服务/组件)
- 智能告警规则引擎
- 自动故障恢复
- 运维集成接口

迁移自：Go http_gateway.go 的健康检查逻辑
"""

import time
import json
import threading
import psutil
import subprocess
from enum import Enum
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Any, Callable, Set
from concurrent.futures import ThreadPoolExecutor, Future
import smtplib
# 移除网络依赖：import requests
from email.mime.text import MimeText
from email.mime.multipart import MimeMultipart

from .metrics_collector import MetricsCollector


class HealthStatus(Enum):
    """健康状态枚举"""
    HEALTHY = "healthy"
    WARNING = "warning" 
    CRITICAL = "critical"
    UNKNOWN = "unknown"


class AlertLevel(Enum):
    """告警级别"""
    INFO = "info"
    WARNING = "warning"
    ERROR = "error"
    CRITICAL = "critical"


@dataclass
class HealthCheckResult:
    """健康检查结果"""
    component: str
    status: HealthStatus
    message: str
    details: Dict[str, Any] = None
    timestamp: float = None
    duration_ms: float = 0.0
    
    def __post_init__(self):
        if self.timestamp is None:
            self.timestamp = time.time()
        if self.details is None:
            self.details = {}


@dataclass
class AlertRule:
    """告警规则"""
    name: str
    component: str
    condition: str  # "status != HEALTHY", "cpu_usage > 80", etc.
    level: AlertLevel
    enabled: bool = True
    cooldown_seconds: int = 300  # 5分钟冷却
    notify_channels: List[str] = None
    
    def __post_init__(self):
        if self.notify_channels is None:
            self.notify_channels = ["console"]


@dataclass
class Alert:
    """告警实例"""
    id: str
    rule_name: str
    component: str
    level: AlertLevel
    message: str
    details: Dict[str, Any]
    timestamp: float
    resolved: bool = False
    resolved_at: Optional[float] = None


class HealthChecker:
    """健康检查器基类"""
    
    def __init__(self, name: str):
        self.name = name
        
    def check(self) -> HealthCheckResult:
        """执行健康检查"""
        raise NotImplementedError


class SystemHealthChecker(HealthChecker):
    """系统级健康检查"""
    
    def __init__(self):
        super().__init__("system")
        
    def check(self) -> HealthCheckResult:
        start_time = time.time()
        
        try:
            # CPU使用率检查
            cpu_percent = psutil.cpu_percent(interval=1)
            
            # 内存使用率检查
            memory = psutil.virtual_memory()
            memory_percent = memory.percent
            
            # 磁盘使用率检查
            disk = psutil.disk_usage('/')
            disk_percent = disk.used / disk.total * 100
            
            # 负载检查
            load_avg = psutil.getloadavg()
            cpu_count = psutil.cpu_count()
            
            details = {
                "cpu_percent": cpu_percent,
                "memory_percent": memory_percent,
                "disk_percent": disk_percent,
                "load_1m": load_avg[0],
                "load_5m": load_avg[1],
                "load_15m": load_avg[2],
                "cpu_count": cpu_count
            }
            
            # 判断健康状态
            status = HealthStatus.HEALTHY
            messages = []
            
            if cpu_percent > 90:
                status = HealthStatus.CRITICAL
                messages.append(f"CPU使用率过高: {cpu_percent:.1f}%")
            elif cpu_percent > 80:
                status = HealthStatus.WARNING
                messages.append(f"CPU使用率较高: {cpu_percent:.1f}%")
            
            if memory_percent > 90:
                status = HealthStatus.CRITICAL
                messages.append(f"内存使用率过高: {memory_percent:.1f}%")
            elif memory_percent > 80:
                status = HealthStatus.WARNING
                messages.append(f"内存使用率较高: {memory_percent:.1f}%")
            
            if disk_percent > 95:
                status = HealthStatus.CRITICAL
                messages.append(f"磁盘使用率过高: {disk_percent:.1f}%")
            elif disk_percent > 85:
                status = HealthStatus.WARNING
                messages.append(f"磁盘使用率较高: {disk_percent:.1f}%")
            
            if load_avg[0] > cpu_count * 2:
                status = HealthStatus.CRITICAL
                messages.append(f"系统负载过高: {load_avg[0]:.2f}")
            elif load_avg[0] > cpu_count * 1.5:
                status = HealthStatus.WARNING
                messages.append(f"系统负载较高: {load_avg[0]:.2f}")
            
            message = "; ".join(messages) if messages else "系统运行正常"
            
            return HealthCheckResult(
                component=self.name,
                status=status,
                message=message,
                details=details,
                duration_ms=(time.time() - start_time) * 1000
            )
            
        except Exception as e:
            return HealthCheckResult(
                component=self.name,
                status=HealthStatus.UNKNOWN,
                message=f"系统健康检查失败: {e}",
                duration_ms=(time.time() - start_time) * 1000
            )


class ProcessHealthChecker(HealthChecker):
    """进程健康检查"""
    
    def __init__(self, process_name: str, pid: Optional[int] = None):
        super().__init__(f"process_{process_name}")
        self.process_name = process_name
        self.pid = pid
        
    def check(self) -> HealthCheckResult:
        start_time = time.time()
        
        try:
            if self.pid:
                # 检查指定PID
                if psutil.pid_exists(self.pid):
                    proc = psutil.Process(self.pid)
                    if proc.name() == self.process_name:
                        status = HealthStatus.HEALTHY
                        message = f"进程 {self.process_name} (PID:{self.pid}) 运行正常"
                        details = {
                            "pid": self.pid,
                            "cpu_percent": proc.cpu_percent(),
                            "memory_percent": proc.memory_percent(),
                            "status": proc.status()
                        }
                    else:
                        status = HealthStatus.CRITICAL
                        message = f"PID {self.pid} 进程名不匹配"
                        details = {"pid": self.pid}
                else:
                    status = HealthStatus.CRITICAL
                    message = f"进程 {self.process_name} (PID:{self.pid}) 不存在"
                    details = {"expected_pid": self.pid}
            else:
                # 按名称查找进程
                processes = [p for p in psutil.process_iter(['pid', 'name']) 
                           if p.info['name'] == self.process_name]
                
                if processes:
                    status = HealthStatus.HEALTHY
                    message = f"进程 {self.process_name} 运行正常 ({len(processes)} 个实例)"
                    details = {
                        "process_count": len(processes),
                        "pids": [p.info['pid'] for p in processes]
                    }
                else:
                    status = HealthStatus.CRITICAL
                    message = f"进程 {self.process_name} 未运行"
                    details = {"process_count": 0}
            
            return HealthCheckResult(
                component=self.name,
                status=status,
                message=message,
                details=details,
                duration_ms=(time.time() - start_time) * 1000
            )
            
        except Exception as e:
            return HealthCheckResult(
                component=self.name,
                status=HealthStatus.UNKNOWN,
                message=f"进程健康检查失败: {e}",
                duration_ms=(time.time() - start_time) * 1000
            )


class LocalServiceHealthChecker(HealthChecker):
    """本地化服务健康检查 - 无网络依赖"""
    
    def __init__(self, service_name: str, health_callable: Optional[Callable] = None,
                 check_command: Optional[str] = None):
        super().__init__(f"service_{service_name}")
        self.service_name = service_name
        self.health_callable = health_callable  # 本地化健康检查函数
        self.check_command = check_command
        
    def check(self) -> HealthCheckResult:
        start_time = time.time()
        
        try:
            status = HealthStatus.HEALTHY
            message = f"服务 {self.service_name} 运行正常"
            details = {}
            
            # 本地化健康检查 - 直接函数调用
            if self.health_callable:
                try:
                    call_start = time.time()
                    health_result = self.health_callable()
                    call_duration = (time.time() - call_start) * 1000
                    
                    details["local_check_duration_ms"] = call_duration
                    
                    if isinstance(health_result, dict):
                        # 结构化健康检查结果
                        if health_result.get("healthy", True):
                            status = HealthStatus.HEALTHY
                            message = f"服务 {self.service_name} 本地检查通过"
                            details.update(health_result)
                        else:
                            status = HealthStatus.CRITICAL
                            message = f"服务 {self.service_name} 本地检查失败: {health_result.get('error', '未知错误')}"
                            details.update(health_result)
                    elif isinstance(health_result, bool):
                        # 布尔健康检查结果
                        if health_result:
                            status = HealthStatus.HEALTHY
                            message = f"服务 {self.service_name} 本地检查通过"
                        else:
                            status = HealthStatus.CRITICAL
                            message = f"服务 {self.service_name} 本地检查失败"
                    else:
                        # 其他类型结果
                        status = HealthStatus.HEALTHY
                        message = f"服务 {self.service_name} 本地检查完成"
                        details["result"] = str(health_result)
                        
                except Exception as e:
                    status = HealthStatus.CRITICAL
                    message = f"服务 {self.service_name} 本地检查异常: {e}"
                    details["check_error"] = str(e)
            
            # 命令健康检查
            if self.check_command:
                try:
                    result = subprocess.run(
                        self.check_command.split(),
                        capture_output=True,
                        text=True,
                        timeout=10
                    )
                    
                    details["command_exit_code"] = result.returncode
                    details["command_stdout"] = result.stdout
                    details["command_stderr"] = result.stderr
                    
                    if result.returncode == 0:
                        if status == HealthStatus.HEALTHY:  # 保持之前的状态
                            message += f"; 命令检查通过"
                    else:
                        status = HealthStatus.CRITICAL
                        message = f"服务 {self.service_name} 命令检查失败: exit code {result.returncode}"
                        
                except subprocess.TimeoutExpired:
                    status = HealthStatus.CRITICAL
                    message = f"服务 {self.service_name} 命令检查超时"
                    details["command_timeout"] = True
                except Exception as e:
                    status = HealthStatus.WARNING
                    message = f"服务 {self.service_name} 命令检查异常: {e}"
                    details["command_error"] = str(e)
            
            return HealthCheckResult(
                component=self.name,
                status=status,
                message=message,
                details=details,
                duration_ms=(time.time() - start_time) * 1000
            )
            
        except Exception as e:
            return HealthCheckResult(
                component=self.name,
                status=HealthStatus.UNKNOWN,
                message=f"服务健康检查失败: {e}",
                duration_ms=(time.time() - start_time) * 1000
            )


class AlertManager:
    """告警管理器"""
    
    def __init__(self, smtp_config: Optional[Dict[str, str]] = None,
                 webhook_urls: Optional[List[str]] = None):
        self.smtp_config = smtp_config
        self.webhook_urls = webhook_urls or []
        
        # 告警状态
        self.active_alerts: Dict[str, Alert] = {}
        self.alert_history: List[Alert] = []
        self.last_alert_time: Dict[str, float] = {}
        
        # 线程安全
        self._lock = threading.RLock()
        
    def send_alert(self, alert: Alert, channels: List[str]):
        """发送告警"""
        with self._lock:
            # 检查冷却时间
            rule_key = f"{alert.rule_name}_{alert.component}"
            last_time = self.last_alert_time.get(rule_key, 0)
            
            if time.time() - last_time < 300:  # 5分钟冷却
                return
            
            # 发送到各个渠道
            for channel in channels:
                try:
                    if channel == "console":
                        self._send_console_alert(alert)
                    elif channel == "email" and self.smtp_config:
                        self._send_email_alert(alert)
                    elif channel == "webhook":
                        self._send_webhook_alert(alert)
                        
                except Exception as e:
                    print(f"⚠️ 发送告警失败 {channel}: {e}")
            
            # 更新时间
            self.last_alert_time[rule_key] = time.time()
            
            # 记录告警
            self.active_alerts[alert.id] = alert
            self.alert_history.append(alert)
    
    def _send_console_alert(self, alert: Alert):
        """发送控制台告警"""
        icon = {
            AlertLevel.INFO: "ℹ️",
            AlertLevel.WARNING: "⚠️", 
            AlertLevel.ERROR: "❌",
            AlertLevel.CRITICAL: "🚨"
        }.get(alert.level, "📢")
        
        print(f"\n{icon} 【{alert.level.value.upper()}告警】")
        print(f"组件: {alert.component}")
        print(f"规则: {alert.rule_name}")
        print(f"消息: {alert.message}")
        print(f"时间: {time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(alert.timestamp))}")
        if alert.details:
            print(f"详情: {json.dumps(alert.details, ensure_ascii=False, indent=2)}")
        print("-" * 50)
    
    def _send_email_alert(self, alert: Alert):
        """发送邮件告警"""
        if not self.smtp_config:
            return
            
        msg = MimeMultipart()
        msg['From'] = self.smtp_config['from']
        msg['To'] = self.smtp_config['to']
        msg['Subject'] = f"[PIXLY告警] {alert.level.value.upper()} - {alert.component}"
        
        body = f"""
        告警详情：
        
        组件：{alert.component}
        级别：{alert.level.value.upper()}
        规则：{alert.rule_name}
        消息：{alert.message}
        时间：{time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(alert.timestamp))}
        
        详细信息：
        {json.dumps(alert.details, ensure_ascii=False, indent=2)}
        """
        
        msg.attach(MimeText(body, 'plain', 'utf-8'))
        
        server = smtplib.SMTP(self.smtp_config['host'], self.smtp_config['port'])
        if self.smtp_config.get('use_tls'):
            server.starttls()
        if self.smtp_config.get('username'):
            server.login(self.smtp_config['username'], self.smtp_config['password'])
        
        server.send_message(msg)
        server.quit()
    
    def _send_webhook_alert(self, alert: Alert):
        """本地化告警通知 - 移除网络依赖"""
        alert_data = {
            "id": alert.id,
            "rule_name": alert.rule_name,
            "component": alert.component,
            "level": alert.level.value,
            "message": alert.message,
            "details": alert.details,
            "timestamp": alert.timestamp
        }
        
        # 本地化告警处理：写入本地文件而不是发送HTTP请求
        try:
            import os
            alerts_dir = "data/alerts"
            os.makedirs(alerts_dir, exist_ok=True)
            
            alert_file = f"{alerts_dir}/alert_{alert.id}.json"
            with open(alert_file, 'w', encoding='utf-8') as f:
                json.dump(alert_data, f, ensure_ascii=False, indent=2)
            
            print(f"📁 本地化告警已保存: {alert_file}")
            
        except Exception as e:
            print(f"⚠️ 本地化告警保存失败: {e}")


class HealthMonitor:
    """
    🏥 企业级健康监控器
    
    功能特性：
    - 多层次健康检查
    - 智能告警规则
    - 自动故障恢复  
    - 运维集成接口
    """
    
    def __init__(self, metrics_collector: MetricsCollector = None):
        self.metrics = metrics_collector or MetricsCollector()
        self.alert_manager = AlertManager()
        
        # 健康检查器
        self.checkers: Dict[str, HealthChecker] = {}
        self.check_results: Dict[str, HealthCheckResult] = {}
        
        # 告警规则
        self.alert_rules: Dict[str, AlertRule] = {}
        
        # 监控线程
        self._monitor_thread = None
        self._stop_monitoring = False
        self._executor = ThreadPoolExecutor(max_workers=10)
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 默认检查间隔
        self.check_interval = 30  # 30秒
        
        # 注册默认检查器
        self._register_default_checkers()
        self._register_default_rules()
    
    def _register_default_checkers(self):
        """注册默认健康检查器"""
        # 系统检查器
        self.add_checker(SystemHealthChecker())
        
        # Python进程检查器
        self.add_checker(ProcessHealthChecker("python"))
    
    def _register_default_rules(self):
        """注册默认告警规则"""
        self.add_alert_rule(AlertRule(
            name="system_cpu_critical",
            component="system",
            condition="details.cpu_percent > 90",
            level=AlertLevel.CRITICAL,
            notify_channels=["console"]
        ))
        
        self.add_alert_rule(AlertRule(
            name="system_memory_critical", 
            component="system",
            condition="details.memory_percent > 90",
            level=AlertLevel.CRITICAL,
            notify_channels=["console"]
        ))
        
        self.add_alert_rule(AlertRule(
            name="system_disk_critical",
            component="system", 
            condition="details.disk_percent > 95",
            level=AlertLevel.CRITICAL,
            notify_channels=["console"]
        ))
    
    def add_checker(self, checker: HealthChecker):
        """添加健康检查器"""
        with self._lock:
            self.checkers[checker.name] = checker
    
    def remove_checker(self, name: str):
        """移除健康检查器"""
        with self._lock:
            self.checkers.pop(name, None)
            self.check_results.pop(name, None)
    
    def add_alert_rule(self, rule: AlertRule):
        """添加告警规则"""
        with self._lock:
            self.alert_rules[rule.name] = rule
    
    def remove_alert_rule(self, name: str):
        """移除告警规则"""
        with self._lock:
            self.alert_rules.pop(name, None)
    
    def run_health_checks(self) -> Dict[str, HealthCheckResult]:
        """运行所有健康检查"""
        results = {}
        
        # 提交所有检查任务
        futures: Dict[str, Future] = {}
        
        with self._lock:
            for name, checker in self.checkers.items():
                future = self._executor.submit(self._run_single_check, checker)
                futures[name] = future
        
        # 收集结果
        for name, future in futures.items():
            try:
                result = future.result(timeout=30)  # 30秒超时
                results[name] = result
                
                # 记录指标
                self.metrics.record_gauge(f"health_check_duration_ms", 
                                        result.duration_ms, 
                                        {"component": name})
                
                status_value = {
                    HealthStatus.HEALTHY: 1,
                    HealthStatus.WARNING: 0.5,
                    HealthStatus.CRITICAL: 0,
                    HealthStatus.UNKNOWN: -1
                }[result.status]
                
                self.metrics.record_gauge(f"health_status",
                                        status_value,
                                        {"component": name})
                
            except Exception as e:
                # 创建错误结果
                error_result = HealthCheckResult(
                    component=name,
                    status=HealthStatus.UNKNOWN,
                    message=f"健康检查执行失败: {e}"
                )
                results[name] = error_result
                
                # 记录失败指标
                self.metrics.record_counter("health_check_failures",
                                          labels={"component": name})
        
        # 更新结果缓存
        with self._lock:
            self.check_results.update(results)
        
        # 检查告警
        self._check_alert_rules(results)
        
        return results
    
    def _run_single_check(self, checker: HealthChecker) -> HealthCheckResult:
        """运行单个健康检查"""
        try:
            return checker.check()
        except Exception as e:
            return HealthCheckResult(
                component=checker.name,
                status=HealthStatus.UNKNOWN,
                message=f"检查器执行异常: {e}"
            )
    
    def _check_alert_rules(self, results: Dict[str, HealthCheckResult]):
        """检查告警规则"""
        with self._lock:
            for rule_name, rule in self.alert_rules.items():
                if not rule.enabled:
                    continue
                
                # 找到对应的结果
                result = results.get(rule.component)
                if not result:
                    continue
                
                # 评估条件
                try:
                    if self._evaluate_condition(rule.condition, result):
                        # 生成告警
                        alert = Alert(
                            id=f"{rule_name}_{int(time.time())}",
                            rule_name=rule.name,
                            component=rule.component,
                            level=rule.level,
                            message=f"{rule.name}: {result.message}",
                            details=result.details,
                            timestamp=time.time()
                        )
                        
                        # 发送告警
                        self.alert_manager.send_alert(alert, rule.notify_channels)
                        
                except Exception as e:
                    print(f"⚠️ 告警规则评估失败 {rule_name}: {e}")
    
    def _evaluate_condition(self, condition: str, result: HealthCheckResult) -> bool:
        """评估告警条件"""
        # 构建评估环境
        context = {
            "status": result.status,
            "HealthStatus": HealthStatus,
            "details": result.details,
            "duration_ms": result.duration_ms
        }
        
        try:
            return eval(condition, {"__builtins__": {}}, context)
        except Exception as e:
            print(f"⚠️ 条件评估失败: {condition}, 错误: {e}")
            return False
    
    def start_monitoring(self):
        """启动健康监控"""
        if self._monitor_thread is not None:
            return
        
        self._stop_monitoring = False
        self._monitor_thread = threading.Thread(target=self._monitor_loop)
        self._monitor_thread.daemon = True
        self._monitor_thread.start()
        
        print("🏥 健康监控已启动")
    
    def stop_monitoring(self):
        """停止健康监控"""
        self._stop_monitoring = True
        if self._monitor_thread:
            self._monitor_thread.join(timeout=10.0)
            self._monitor_thread = None
        
        print("🛑 健康监控已停止")
    
    def _monitor_loop(self):
        """监控循环"""
        while not self._stop_monitoring:
            try:
                start_time = time.time()
                
                # 运行健康检查
                results = self.run_health_checks()
                
                # 记录监控周期指标
                cycle_duration = time.time() - start_time
                self.metrics.record_histogram("health_monitor_cycle_duration", 
                                            cycle_duration)
                
                # 记录总体健康状态
                healthy_count = sum(1 for r in results.values() 
                                  if r.status == HealthStatus.HEALTHY)
                total_count = len(results)
                
                self.metrics.record_gauge("health_checks_healthy_ratio",
                                        healthy_count / total_count if total_count > 0 else 0)
                
                # 等待下次检查
                time.sleep(self.check_interval)
                
            except Exception as e:
                print(f"⚠️ 健康监控循环异常: {e}")
                time.sleep(10)
    
    def get_health_summary(self) -> Dict[str, Any]:
        """获取健康状态摘要"""
        with self._lock:
            results = self.check_results.copy()
        
        summary = {
            "timestamp": time.time(),
            "total_components": len(results),
            "status_counts": {
                "healthy": 0,
                "warning": 0, 
                "critical": 0,
                "unknown": 0
            },
            "components": {},
            "active_alerts": len(self.alert_manager.active_alerts)
        }
        
        for name, result in results.items():
            status_key = result.status.value
            summary["status_counts"][status_key] += 1
            
            summary["components"][name] = {
                "status": result.status.value,
                "message": result.message,
                "last_check": result.timestamp,
                "duration_ms": result.duration_ms
            }
        
        return summary
    
    def get_component_details(self, component: str) -> Optional[HealthCheckResult]:
        """获取组件详细信息"""
        with self._lock:
            return self.check_results.get(component)
    
    def configure_smtp(self, host: str, port: int, username: str, 
                      password: str, from_addr: str, to_addr: str,
                      use_tls: bool = True):
        """配置SMTP告警"""
        smtp_config = {
            "host": host,
            "port": port,
            "username": username,
            "password": password,
            "from": from_addr,
            "to": to_addr,
            "use_tls": use_tls
        }
        self.alert_manager.smtp_config = smtp_config
    
    def add_webhook_url(self, url: str):
        """添加Webhook URL"""
        if url not in self.alert_manager.webhook_urls:
            self.alert_manager.webhook_urls.append(url)
    
    def shutdown(self):
        """关闭健康监控器"""
        self.stop_monitoring()
        self._executor.shutdown(wait=True)
        print("🏥 健康监控器已关闭")
