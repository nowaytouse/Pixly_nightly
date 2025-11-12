"""
📨 PIXLY v3.0 统一消息传递系统

基于Go废弃模块messaging.go的消息架构：
- 三端统一格式 (Go/Rust/Python)
- 本地化消息传递 (零网络依赖)  
- 进程内高速通信
- 消息路由与分发
- 状态同步机制

迁移自：Go messaging.go 的消息格式和逻辑
"""

import time
import json
import threading
import uuid
from enum import Enum
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Any, Callable, Set
from collections import defaultdict, deque
import queue


class MessageType(Enum):
    """消息类型 - 与Go模块完全一致"""
    INFO = "info"
    WARNING = "warning"
    ERROR = "error"
    SUCCESS = "success"
    PROGRESS = "progress"
    STATUS = "status"


class MessageLevel(Enum):
    """消息级别 - 与Go模块完全一致"""
    DEBUG = 0
    INFO = 1
    WARNING = 2
    ERROR = 3
    CRITICAL = 4


@dataclass
class UnifiedMessage:
    """统一消息结构 - 与Go模块格式一致"""
    id: str
    type: str                          # MessageType.value
    level: int                         # MessageLevel.value
    source: str                        # "python-core", "rust-core", etc.
    component: str                     # 组件名称
    message: str                       # 消息内容
    timestamp: int                     # Unix时间戳
    progress: Optional[int] = None     # 进度 0-100
    data: Optional[Dict[str, Any]] = None      # 附加数据
    error_code: Optional[str] = None           # 错误码
    trace_id: Optional[str] = None             # 追踪ID
    target: Optional[str] = None               # 目标组件
    reply_to: Optional[str] = None             # 回复消息ID
    
    def __post_init__(self):
        if self.data is None:
            self.data = {}


class MessageSubscriber:
    """消息订阅者"""
    
    def __init__(self, subscriber_id: str, callback: Callable,
                 message_types: Optional[List[MessageType]] = None,
                 components: Optional[List[str]] = None):
        
        self.subscriber_id = subscriber_id
        self.callback = callback
        self.message_types = set(message_types or [])
        self.components = set(components or [])
        self.created_at = time.time()
        self.message_count = 0
        self.last_message_time = None
        
    def should_receive(self, message: UnifiedMessage) -> bool:
        """判断是否应该接收消息"""
        # 检查消息类型
        if self.message_types and message.type not in [mt.value for mt in self.message_types]:
            return False
        
        # 检查组件
        if self.components and message.component not in self.components:
            return False
        
        # 检查目标 (如果指定了目标，只有匹配的订阅者才接收)
        if message.target and message.target != self.subscriber_id:
            return False
        
        return True
    
    def deliver_message(self, message: UnifiedMessage) -> bool:
        """投递消息"""
        try:
            self.callback(message)
            self.message_count += 1
            self.last_message_time = time.time()
            return True
        except Exception as e:
            print(f"⚠️ 消息投递失败 {self.subscriber_id}: {e}")
            return False


class MessageQueue:
    """本地化消息队列"""
    
    def __init__(self, max_size: int = 10000):
        self.max_size = max_size
        self.messages: deque = deque(maxlen=max_size)
        self.pending_queue: queue.Queue = queue.Queue(maxsize=max_size)
        self._lock = threading.RLock()
        
    def put(self, message: UnifiedMessage) -> bool:
        """添加消息到队列"""
        try:
            with self._lock:
                self.messages.append(message)
                self.pending_queue.put_nowait(message)
            return True
        except queue.Full:
            return False
    
    def get(self, timeout: Optional[float] = None) -> Optional[UnifiedMessage]:
        """从队列获取消息"""
        try:
            return self.pending_queue.get(timeout=timeout)
        except queue.Empty:
            return None
    
    def get_recent_messages(self, count: int = 100) -> List[UnifiedMessage]:
        """获取最近的消息"""
        with self._lock:
            return list(self.messages)[-count:]
    
    def clear(self):
        """清空队列"""
        with self._lock:
            self.messages.clear()
            
            # 清空pending队列
            try:
                while True:
                    self.pending_queue.get_nowait()
            except queue.Empty:
                pass


class UnifiedMessenger:
    """
    📨 企业级统一消息系统
    
    功能特性：
    - 本地化高速通信
    - 消息路由分发
    - 订阅发布模式
    - 状态同步机制
    """
    
    def __init__(self, source_id: str = "python-core"):
        self.source_id = source_id
        
        # 消息队列
        self.message_queue = MessageQueue()
        
        # 订阅者管理
        self.subscribers: Dict[str, MessageSubscriber] = {}
        
        # 消息路由
        self.routes: Dict[str, Set[str]] = defaultdict(set)
        
        # 消息统计
        self.message_stats: Dict[str, int] = defaultdict(int)
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 消息处理线程
        self._processor_thread = None
        self._stop_processing = False
        
        # 消息历史 (用于调试)
        self.message_history: deque = deque(maxlen=1000)
        
        # 启动处理
        self._start_processing()
    
    def _generate_message_id(self) -> str:
        """生成消息ID"""
        return f"msg_{int(time.time() * 1000000)}_{uuid.uuid4().hex[:8]}"
    
    def create_message(self, message_type: MessageType, level: MessageLevel,
                      component: str, message: str,
                      progress: Optional[int] = None,
                      data: Optional[Dict[str, Any]] = None,
                      error_code: Optional[str] = None,
                      trace_id: Optional[str] = None,
                      target: Optional[str] = None) -> UnifiedMessage:
        """创建统一消息"""
        
        return UnifiedMessage(
            id=self._generate_message_id(),
            type=message_type.value,
            level=level.value,
            source=self.source_id,
            component=component,
            message=message,
            timestamp=int(time.time()),
            progress=progress,
            data=data,
            error_code=error_code,
            trace_id=trace_id,
            target=target
        )
    
    def send_message(self, message: UnifiedMessage) -> bool:
        """发送消息"""
        # 记录到历史
        self.message_history.append(message)
        
        # 更新统计
        with self._lock:
            self.message_stats[f"sent_{message.type}"] += 1
            self.message_stats["total_sent"] += 1
        
        # 放入队列处理
        return self.message_queue.put(message)
    
    def send_info(self, component: str, message: str, **kwargs) -> bool:
        """发送信息消息"""
        msg = self.create_message(MessageType.INFO, MessageLevel.INFO, 
                                component, message, **kwargs)
        return self.send_message(msg)
    
    def send_warning(self, component: str, message: str, **kwargs) -> bool:
        """发送警告消息"""
        msg = self.create_message(MessageType.WARNING, MessageLevel.WARNING,
                                component, message, **kwargs)
        return self.send_message(msg)
    
    def send_error(self, component: str, message: str, 
                  error_code: Optional[str] = None, **kwargs) -> bool:
        """发送错误消息"""
        msg = self.create_message(MessageType.ERROR, MessageLevel.ERROR,
                                component, message, error_code=error_code, **kwargs)
        return self.send_message(msg)
    
    def send_success(self, component: str, message: str, **kwargs) -> bool:
        """发送成功消息"""
        msg = self.create_message(MessageType.SUCCESS, MessageLevel.INFO,
                                component, message, **kwargs)
        return self.send_message(msg)
    
    def send_progress(self, component: str, message: str, progress: int, **kwargs) -> bool:
        """发送进度消息"""
        progress = max(0, min(100, progress))  # 限制在0-100
        msg = self.create_message(MessageType.PROGRESS, MessageLevel.INFO,
                                component, message, progress=progress, **kwargs)
        return self.send_message(msg)
    
    def send_status(self, component: str, message: str, 
                   status_data: Dict[str, Any], **kwargs) -> bool:
        """发送状态消息"""
        msg = self.create_message(MessageType.STATUS, MessageLevel.INFO,
                                component, message, data=status_data, **kwargs)
        return self.send_message(msg)
    
    def subscribe(self, subscriber_id: str, callback: Callable,
                 message_types: Optional[List[MessageType]] = None,
                 components: Optional[List[str]] = None) -> bool:
        """订阅消息"""
        
        subscriber = MessageSubscriber(
            subscriber_id=subscriber_id,
            callback=callback,
            message_types=message_types,
            components=components
        )
        
        with self._lock:
            self.subscribers[subscriber_id] = subscriber
        
        print(f"📨 订阅者注册: {subscriber_id}")
        return True
    
    def unsubscribe(self, subscriber_id: str) -> bool:
        """取消订阅"""
        with self._lock:
            if subscriber_id in self.subscribers:
                del self.subscribers[subscriber_id]
                print(f"📨 订阅者注销: {subscriber_id}")
                return True
        
        return False
    
    def add_route(self, source_component: str, target_subscriber: str):
        """添加消息路由"""
        with self._lock:
            self.routes[source_component].add(target_subscriber)
    
    def remove_route(self, source_component: str, target_subscriber: str):
        """移除消息路由"""
        with self._lock:
            if source_component in self.routes:
                self.routes[source_component].discard(target_subscriber)
    
    def _start_processing(self):
        """启动消息处理线程"""
        if self._processor_thread is not None:
            return
        
        self._stop_processing = False
        self._processor_thread = threading.Thread(target=self._process_messages)
        self._processor_thread.daemon = True
        self._processor_thread.start()
    
    def _process_messages(self):
        """处理消息循环"""
        while not self._stop_processing:
            try:
                # 获取消息
                message = self.message_queue.get(timeout=1.0)
                if message is None:
                    continue
                
                # 分发消息
                self._distribute_message(message)
                
            except Exception as e:
                print(f"⚠️ 消息处理异常: {e}")
                time.sleep(0.1)
    
    def _distribute_message(self, message: UnifiedMessage):
        """分发消息给订阅者"""
        delivered_count = 0
        
        with self._lock:
            # 获取所有可能的接收者
            potential_subscribers = list(self.subscribers.values())
            
            # 检查路由规则
            routed_subscribers = self.routes.get(message.component, set())
            
        for subscriber in potential_subscribers:
            try:
                # 检查是否应该接收
                should_receive = subscriber.should_receive(message)
                
                # 检查路由规则 (如果有路由规则，只发给指定的订阅者)
                if routed_subscribers and subscriber.subscriber_id not in routed_subscribers:
                    should_receive = False
                
                if should_receive:
                    if subscriber.deliver_message(message):
                        delivered_count += 1
                        
            except Exception as e:
                print(f"⚠️ 消息投递异常 {subscriber.subscriber_id}: {e}")
        
        # 更新统计
        with self._lock:
            self.message_stats[f"delivered_{message.type}"] += delivered_count
            self.message_stats["total_delivered"] += delivered_count
    
    def get_message_stats(self) -> Dict[str, Any]:
        """获取消息统计"""
        with self._lock:
            stats = dict(self.message_stats)
        
        # 添加订阅者统计
        subscriber_stats = {}
        for sub_id, subscriber in self.subscribers.items():
            subscriber_stats[sub_id] = {
                "message_count": subscriber.message_count,
                "last_message_time": subscriber.last_message_time,
                "uptime_seconds": time.time() - subscriber.created_at
            }
        
        return {
            "message_stats": stats,
            "subscriber_count": len(self.subscribers),
            "subscriber_stats": subscriber_stats,
            "queue_size": len(self.message_queue.messages),
            "route_count": len(self.routes)
        }
    
    def get_recent_messages(self, count: int = 50) -> List[Dict[str, Any]]:
        """获取最近的消息"""
        recent = list(self.message_history)[-count:]
        return [asdict(msg) for msg in recent]
    
    def broadcast_message(self, message_type: MessageType, level: MessageLevel,
                         component: str, message: str, **kwargs) -> int:
        """广播消息给所有订阅者"""
        msg = self.create_message(message_type, level, component, message, **kwargs)
        
        if self.send_message(msg):
            # 返回预期的接收者数量
            with self._lock:
                return len(self.subscribers)
        
        return 0
    
    def send_targeted_message(self, target_subscriber: str, 
                            message_type: MessageType, level: MessageLevel,
                            component: str, message: str, **kwargs) -> bool:
        """发送定向消息"""
        msg = self.create_message(message_type, level, component, message, 
                                target=target_subscriber, **kwargs)
        return self.send_message(msg)
    
    def create_message_channel(self, channel_name: str) -> 'MessageChannel':
        """创建消息通道"""
        return MessageChannel(self, channel_name)
    
    def flush_messages(self) -> int:
        """强制处理所有待处理消息"""
        processed = 0
        
        while True:
            message = self.message_queue.get(timeout=0.1)
            if message is None:
                break
            
            self._distribute_message(message)
            processed += 1
        
        return processed
    
    def shutdown(self):
        """关闭消息系统"""
        self._stop_processing = True
        
        if self._processor_thread:
            self._processor_thread.join(timeout=5.0)
        
        # 处理剩余消息
        remaining = self.flush_messages()
        if remaining > 0:
            print(f"📨 处理了 {remaining} 条剩余消息")
        
        print("📨 统一消息系统已关闭")


class MessageChannel:
    """消息通道 - 简化的消息收发接口"""
    
    def __init__(self, messenger: UnifiedMessenger, channel_name: str):
        self.messenger = messenger
        self.channel_name = channel_name
        
    def send(self, message: str, message_type: MessageType = MessageType.INFO,
            level: MessageLevel = MessageLevel.INFO, **kwargs) -> bool:
        """发送消息"""
        return self.messenger.send_message(
            self.messenger.create_message(
                message_type, level, self.channel_name, message, **kwargs
            )
        )
    
    def info(self, message: str, **kwargs) -> bool:
        """发送信息"""
        return self.send(message, MessageType.INFO, MessageLevel.INFO, **kwargs)
    
    def warning(self, message: str, **kwargs) -> bool:
        """发送警告"""
        return self.send(message, MessageType.WARNING, MessageLevel.WARNING, **kwargs)
    
    def error(self, message: str, **kwargs) -> bool:
        """发送错误"""
        return self.send(message, MessageType.ERROR, MessageLevel.ERROR, **kwargs)
    
    def success(self, message: str, **kwargs) -> bool:
        """发送成功"""
        return self.send(message, MessageType.SUCCESS, MessageLevel.INFO, **kwargs)
    
    def progress(self, message: str, progress: int, **kwargs) -> bool:
        """发送进度"""
        return self.send(message, MessageType.PROGRESS, MessageLevel.INFO, 
                        progress=progress, **kwargs)


# 全局默认消息系统
_default_messenger = None

def get_messenger(source_id: str = "python-core") -> UnifiedMessenger:
    """获取默认消息系统"""
    global _default_messenger
    
    if _default_messenger is None:
        _default_messenger = UnifiedMessenger(source_id=source_id)
    
    return _default_messenger

def setup_messaging(source_id: str = "python-core") -> UnifiedMessenger:
    """设置全局消息系统"""
    global _default_messenger
    _default_messenger = UnifiedMessenger(source_id=source_id)
    return _default_messenger

# 便捷函数
def send_info(component: str, message: str, **kwargs) -> bool:
    """发送信息消息"""
    return get_messenger().send_info(component, message, **kwargs)

def send_warning(component: str, message: str, **kwargs) -> bool:
    """发送警告消息"""
    return get_messenger().send_warning(component, message, **kwargs)

def send_error(component: str, message: str, **kwargs) -> bool:
    """发送错误消息"""
    return get_messenger().send_error(component, message, **kwargs)

def send_success(component: str, message: str, **kwargs) -> bool:
    """发送成功消息"""
    return get_messenger().send_success(component, message, **kwargs)

def send_progress(component: str, message: str, progress: int, **kwargs) -> bool:
    """发送进度消息"""
    return get_messenger().send_progress(component, message, progress, **kwargs)

def subscribe(subscriber_id: str, callback: Callable, **kwargs) -> bool:
    """订阅消息"""
    return get_messenger().subscribe(subscriber_id, callback, **kwargs)
