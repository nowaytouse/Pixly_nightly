# GO日志系统设置指南

## 依赖安装

在`core/go`目录下运行：

```bash
cd core/go

# 初始化go.mod（如果还没有）
go mod init github.com/pixly/go-service

# 安装zerolog依赖
go get github.com/rs/zerolog

# 更新依赖
go mod tidy
```

## 在main.go中集成

```go
package main

import (
    "os"
    
    "github.com/pixly/go-service/pkg/logging"
)

func main() {
    // 初始化日志系统
    // 从环境变量读取配置
    logLevel := os.Getenv("PIXLY_LOG_LEVEL")
    if logLevel == "" {
        logLevel = "INFO"
    }
    
    jsonOutput := os.Getenv("PIXLY_LOG_JSON") != ""
    
    logging.InitLogging(logLevel, jsonOutput)
    logging.Info("PIXLY GO Service started", map[string]interface{}{
        "version": "4.2.0",
        "port":    50052,
    })
    
    // ... 启动gRPC服务 ...
}
```

## 使用示例

### 基础日志

```go
import "github.com/pixly/go-service/pkg/logging"

// 简单消息
logging.Info("AI prediction started")

// 带字段
logging.Info("AI prediction completed", map[string]interface{}{
    "file":       "photo.jpg",
    "quality":    85,
    "confidence": 0.92,
})

// 错误日志
logging.Error("Prediction failed", map[string]interface{}{
    "file":  "photo.jpg",
    "error": err.Error(),
})
```

### 性能追踪

```go
import "github.com/pixly/go-service/pkg/logging"

func PredictQuality(file string) (int, error) {
    // 开始计时
    timer := logging.NewTimedLogger("ai_prediction", map[string]interface{}{
        "file": file,
    })
    
    // ... 执行AI推理 ...
    
    if err != nil {
        timer.EndWithError("Prediction failed", err)
        return 0, err
    }
    
    timer.End("Prediction completed")
    return quality, nil
}
```

### 结构化日志

```go
// 使用zerolog直接API
import (
    "github.com/rs/zerolog/log"
)

log.Info().
    Str("file", "photo.jpg").
    Int("quality", 85).
    Float64("confidence", 0.92).
    Dur("duration", duration).
    Msg("AI prediction completed")
```

## 输出格式

### 人类可读（开发）

```
2025-11-10T10:02:00+08:00 INF PIXLY GO Service started version=4.2.0 port=50052
2025-11-10T10:02:01+08:00 INF AI prediction started file=photo.jpg
2025-11-10T10:02:01+08:00 INF Prediction completed duration_ms=123 quality=85
```

### JSON格式（生产）

```json
{"level":"info","time":"2025-11-10T10:02:00+08:00","message":"PIXLY GO Service started","version":"4.2.0","port":50052}
{"level":"info","time":"2025-11-10T10:02:01+08:00","message":"AI prediction started","file":"photo.jpg"}
{"level":"info","time":"2025-11-10T10:02:01+08:00","message":"Prediction completed","duration_ms":123,"quality":85}
```

## 环境变量控制

```bash
# 设置日志级别
export PIXLY_LOG_LEVEL=DEBUG

# 启用JSON输出
export PIXLY_LOG_JSON=1

# 运行GO服务
./pixly-go-service
```

## JS插件集成

JS插件可以通过HTTP API或WebSocket接收GO日志：

```javascript
// HTTP轮询
async function fetchGoLogs() {
    const response = await fetch('http://localhost:50052/api/v1/logs?limit=100');
    const logs = await response.json();
    
    logs.forEach(log => {
        if (log.level === 'error') {
            pixlyLog.error('GO', log.message, log);
        } else if (log.level === 'info') {
            pixlyLog.info('GO', log.message, log);
        }
    });
}

// WebSocket实时流（可选）
const ws = new WebSocket('ws://localhost:50052/api/v1/logs/stream');
ws.onmessage = (event) => {
    const log = JSON.parse(event.data);
    pixlyLog.info('GO', log.message, log);
};
```

## 日志级别映射

| 级别 | GO常量 | Zerolog | JS | Rust |
|------|--------|---------|-----|------|
| ERROR | 0 | ErrorLevel | ERROR | error!() |
| WARN | 1 | WarnLevel | WARN | warn!() |
| INFO | 2 | InfoLevel | INFO | info!() |
| DEBUG | 3 | DebugLevel | DEBUG | debug!() |
| TRACE | 4 | TraceLevel | TRACE | trace!() |

## 最佳实践

1. **结构化字段**：使用map传递字段而非字符串拼接
2. **性能追踪**：使用TimedLogger自动记录duration
3. **错误上下文**：Error日志必须包含error字段
4. **避免敏感信息**：不要记录密码、token等
5. **适当级别**：INFO用于用户操作，DEBUG用于开发调试

## 集成checklist

- [ ] 安装zerolog依赖
- [ ] 创建go.mod（如果需要）
- [ ] 在main.go中InitLogging
- [ ] 替换现有log.Println为logging.Info
- [ ] 添加结构化字段
- [ ] 使用TimedLogger追踪性能
- [ ] 配置环境变量
- [ ] 测试JSON输出
- [ ] JS插件接收日志
