# 🔧 Eagle插件重构计划

## 🎯 核心原则

**插件职责**: 仅UI和交互，不做文件操作
**Kernel职责**: 所有文件处理和转换逻辑

---

## 📊 当前问题分析

### 1. 架构违规模块

| 模块 | 问题 | 解决方案 |
|------|------|---------|
| `format-recommender.js` | JS本地推荐，未调用AI | ❌ 废弃，使用Rust AI |
| `deduplicator.js` | JS实现去重逻辑 | ❌ 废弃，使用Rust |
| `file-handler.js` | 部分文件操作 | ⚠️ 重构，仅UI部分 |
| `validation.js` | 重复验证逻辑 | ✅ 已有Rust验证系统 |

### 2. 功能缺失

- ❌ 未集成新的验证系统
- ❌ 未使用Rust的格式推荐
- ❌ 智能模式失效
- ❌ XMP元数据丢失

---

## 🚀 重构步骤

### Phase 1: 清理废弃模块 ✅

**目标**: 移除违反架构的模块

```bash
# 移动到deprecated
mv plugin/converter/js/plugin-modules/format-recommender.js \
   plugin/converter/js/deprecated/format-recommender.js.DEPRECATED

mv plugin/converter/js/plugin-modules/deduplicator.js \
   plugin/converter/js/deprecated/deduplicator.js.DEPRECATED
```

### Phase 2: 集成验证系统 🔄

**目标**: 使用Rust的多级验证系统

**新模块**: `validation-bridge.js`
```javascript
// 调用Rust验证API
async function validateConversion(files, config) {
    const response = await rustCLI.execute('validate', {
        files: files,
        format: config.format,
        quality: config.quality,
        speed: config.speed,
        lossless: config.lossless
    });
    return response;
}
```

### Phase 3: 集成AI推荐 🔄

**目标**: 使用Rust的AI预测

**新模块**: `ai-bridge.js`
```javascript
// 调用Rust AI API
async function getAIRecommendation(imageInfo) {
    const response = await rustCLI.execute('predict', {
        width: imageInfo.width,
        height: imageInfo.height,
        format: imageInfo.format,
        size: imageInfo.size,
        has_alpha: imageInfo.hasAlpha,
        is_animated: imageInfo.isAnimated
    });
    return response;
}
```

### Phase 4: 重构文件处理 🔄

**目标**: 仅保留UI相关的文件选择

**保留**:
- 文件选择UI
- 文件列表显示
- 拖放处理

**移除**:
- 文件读取逻辑
- 文件信息提取
- 文件验证逻辑

### Phase 5: 统一参数传递 🔄

**目标**: 标准化与Rust的通信

**新模块**: `kernel-bridge.js`
```javascript
class KernelBridge {
    // 转换请求
    async convert(files, config) {
        return await rustCLI.execute('convert', {
            input_files: files.map(f => f.path),
            target_format: config.format,
            quality: config.quality,
            speed: config.speed,
            lossless: config.lossless,
            preserve_metadata: config.preserveMetadata,
            keep_animated: config.keepAnimated
        });
    }
    
    // 验证请求
    async validate(files, config) {
        return await rustCLI.execute('validate', {...});
    }
    
    // AI预测请求
    async predict(imageInfo) {
        return await rustCLI.execute('predict', {...});
    }
}
```

---

## 📋 重构清单

### 立即执行 (Priority 0)

- [ ] 废弃 `format-recommender.js`
- [ ] 废弃 `deduplicator.js`
- [ ] 创建 `validation-bridge.js`
- [ ] 创建 `ai-bridge.js`
- [ ] 创建 `kernel-bridge.js`

### 短期 (1周内)

- [ ] 重构 `file-handler.js` - 仅UI部分
- [ ] 重构 `image-conversion.js` - 使用新bridge
- [ ] 重构 `video-conversion.js` - 使用新bridge
- [ ] 更新 `ui-handlers.js` - 集成验证显示

### 中期 (1个月内)

- [ ] 添加验证结果UI显示
- [ ] 添加AI推荐UI显示
- [ ] 添加格式特定警告UI
- [ ] 优化错误处理和用户反馈

---

## 🎯 目标架构

```
┌─────────────────────────────────────┐
│         Eagle Plugin (JS)           │
│  ┌───────────────────────────────┐  │
│  │  UI Layer                     │  │
│  │  - 文件选择                    │  │
│  │  - 参数设置                    │  │
│  │  - 结果显示                    │  │
│  └───────────────────────────────┘  │
│  ┌───────────────────────────────┐  │
│  │  Bridge Layer                 │  │
│  │  - validation-bridge.js       │  │
│  │  - ai-bridge.js               │  │
│  │  - kernel-bridge.js           │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
              ↓ ↑
        (JSON RPC / CLI)
              ↓ ↑
┌─────────────────────────────────────┐
│      Pixly Kernel (Rust)            │
│  ┌───────────────────────────────┐  │
│  │  Validation System            │  │
│  │  - 多级验证                    │  │
│  │  - 格式检查                    │  │
│  └───────────────────────────────┘  │
│  ┌───────────────────────────────┐  │
│  │  AI System                    │  │
│  │  - 参数预测                    │  │
│  │  - 格式推荐                    │  │
│  └───────────────────────────────┘  │
│  ┌───────────────────────────────┐  │
│  │  Conversion Engine            │  │
│  │  - 文件处理                    │  │
│  │  - 格式转换                    │  │
│  │  - 元数据处理                  │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

---

## ✅ 成功标准

1. **零文件操作**: JS不读写文件
2. **零转换逻辑**: JS不实现转换算法
3. **零AI逻辑**: JS不实现AI推荐
4. **完整验证**: 使用Rust验证系统
5. **完整AI**: 使用Rust AI系统
6. **清晰职责**: UI和Kernel完全分离

---

**开始重构！**
