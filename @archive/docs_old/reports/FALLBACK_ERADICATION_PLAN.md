# 🔥 Fallback根除计划 - 质量宣言执行

## 问题严重性：⚠️ **CRITICAL**

**违反质量宣言原则：**
- ❌ Fallback代码（让AI成为摆设）
- ❌ Mock数据（假装功能正常）
- ❌ 静默降级（静默失败）
- ✅ 应该：响亮报错 > 静默降级

---

## 📋 发现的Fallback代码

### JS端（严重违规）

#### 1. **ai-client.js** - 最严重
```javascript
// ❌ 第153行：GO不可用时fallback
if (!this.healthy) {
  return this.getFallbackParams(tool, optimizeMode);
}

// ❌ 第159行：参数验证失败fallback
if (!this.validateParams(...)) {
  return this.getFallbackParams(tool, optimizeMode);
}

// ❌ 第245行：AI预测失败fallback
if (result.error) {
  return this.getFallbackParams(tool, optimizeMode);
}

// ❌ 第260行：异常时fallback
catch (error) {
  return this.getFallbackParams(tool, optimizeMode);
}

// ❌ 第331-387行：getFallbackParams整个方法
getFallbackParams(tool, optimizeMode) {
  // 硬编码的质量参数
  const baseConfigs = {
    quality: { quality: 100, distance: 0.0 }
  };
  return { mode: 'fallback', source: 'static_rules' };
}
```

**问题：**
- AI不可用时静默降级
- 用户不知道AI没有工作
- 违反"响亮报错"原则

---

### Go端（中等违规）

#### 2. **ai/video_handlers.go**
```go
// ❌ 第97-103行：视频预测失败fallback
if err != nil {
  fallback := gw.getVideoFallbackParams(req.OptimizeMode)
  fallback.Error = "using fallback"
  json.NewEncoder(w).Encode(fallback)
  return
}

// ❌ 第165-180行：getVideoFallbackParams方法
func (gw *HTTPGateway) getVideoFallbackParams(mode string) {
  params := &VideoParams{
    Encoder: "h265",
    CRF: 23,  // 硬编码！
  }
}
```

#### 3. **ai/http_gateway_models.go**
```go
// ❌ 第275-277行：TODO mock数据
// TODO: 实际调用模型进行预测
// 这里暂时返回mock数据
response := &PredictResponse{
  Quality: 85,  // 硬编码！
}
```

#### 4. **predictor/predictor.go**
```go
// ❌ 第75-113行：getDefaultPrediction fallback
func (p *PredictorImpl) getDefaultPrediction(features) {
  return &Prediction{
    Method:    "fallback_feature_based",
    RuleName:  "FEATURE_BASED_FALLBACK",
  }
}
```

#### 5. **predictor/feature_extractor.go**
```go
// ❌ 第52-56行：FFprobe失败fallback
if err := fe.extractFFprobeInfo(...); err != nil {
  fe.applyFallback(features)
}

// ❌ 第309-311行：applyFallback方法
func (fe *FeatureExtractor) applyFallback(features) {
  features.Width = 1920  // 硬编码！
}
```

---

## ✅ 根除方案

### 原则

1. **响亮报错 > 静默降级**
   - AI不可用 → 直接报错，不转换
   - 参数预测失败 → 直接报错，不转换

2. **真实调用 > Mock数据**
   - 删除所有mock返回值
   - 删除所有TODO预测

3. **验证 ≠ 篡改**
   - JS只验证AI返回值
   - 不修改AI返回值
   - 不提供fallback值

---

### JS端修复

#### 修复 ai-client.js

```javascript
// ✅ 修复方案
async predictParams(imagePath, tool, optimizeMode, options = {}) {
  const Logger = window.pixlyLog || console;
  
  // ❌ 删除：快速失败fallback
  // if (!this.healthy) {
  //   return this.getFallbackParams(tool, optimizeMode);
  // }
  
  // ✅ 改为：响亮报错
  if (!this.healthy) {
    Logger.error('[AI Client] ❌ GO核心不可用');
    throw new Error('🚨 GO AI核心不可用！无法进行智能预测。\n\n请检查：\n1. GO核心是否启动（端口50052）\n2. Python环境是否配置\n3. AI模型文件是否存在');
  }
  
  // ❌ 删除：参数验证失败fallback
  // if (!this.validateParams(...)) {
  //   return this.getFallbackParams(tool, optimizeMode);
  // }
  
  // ✅ 改为：响亮报错
  if (!this.validateParams(imagePath, tool, optimizeMode)) {
    Logger.error('[AI Client] ❌ 参数验证失败');
    throw new Error(`🚨 参数验证失败！\n\nimage=${imagePath}\ntool=${tool}\nmode=${optimizeMode}`);
  }
  
  try {
    const response = await this.fetchWithTimeout(...);
    const result = await response.json();
    
    // ❌ 删除：AI失败fallback
    // if (!result.success) {
    //   return this.getFallbackParams(tool, optimizeMode);
    // }
    
    // ✅ 改为：响亮报错
    if (!result.success) {
      Logger.error('[AI Client] ❌ AI预测失败:', result.error);
      throw new Error(`🚨 AI预测失败！\n\n${result.error || 'Unknown error'}\n\n请检查GO核心日志`);
    }
    
    // ✅ 只验证，不篡改
    this.validatePredictionResult(result);
    
    return {
      success: true,
      params: result.params,
      source: 'go_ai',  // ✅ 永远是AI
    };
    
  } catch (error) {
    // ❌ 删除：异常时fallback
    // return this.getFallbackParams(tool, optimizeMode);
    
    // ✅ 改为：响亮报错
    Logger.error('[AI Client] ❌ AI调用异常:', error);
    throw new Error(`🚨 AI调用失败！\n\n${error.message}\n\n可能原因：\n1. GO核心未启动\n2. 网络超时\n3. 模型加载失败`);
  }
}

// ❌ 完全删除：getFallbackParams方法
// getFallbackParams(tool, optimizeMode) { ... }

// ✅ 新增：验证方法（只验证，不篡改）
validatePredictionResult(result) {
  const Logger = window.pixlyLog || console;
  
  // 验证必需字段
  if (!result.params) {
    throw new Error('AI返回结果缺少params字段');
  }
  
  if (typeof result.params.quality !== 'number') {
    throw new Error('AI返回的quality不是数字');
  }
  
  // 验证范围（警告，但不修改）
  if (result.params.quality < 1 || result.params.quality > 100) {
    Logger.warn(`[AI Client] ⚠️ AI返回的quality超出范围: ${result.params.quality}`);
    // ❌ 不要修改值！
    // ✅ 只记录警告
  }
  
  Logger.debug('[AI Client] ✅ AI返回值验证通过');
}
```

---

### Go端修复

#### 1. 删除 video_handlers.go 的fallback

```go
func (gw *HTTPGateway) handleVideoPredict(w http.ResponseWriter, r *http.Request) {
  result, err := gw.callVideoPredictScript(req)
  if err != nil {
    // ❌ 删除fallback
    // fallback := gw.getVideoFallbackParams(...)
    
    // ✅ 改为响亮报错
    gw.sendError(w, fmt.Sprintf("🚨 Video AI prediction failed: %v", err), 
                 http.StatusInternalServerError, startTime)
    return
  }
  
  // ✅ 只返回真实AI结果
  w.Header().Set("Content-Type", "application/json")
  json.NewEncoder(w).Encode(result)
}

// ❌ 完全删除：getVideoFallbackParams方法
```

#### 2. 修复 http_gateway_models.go 的mock

```go
func (gw *HTTPGateway) handlePredictWithModel(w http.ResponseWriter, r *http.Request) {
  // ❌ 删除TODO mock
  // response := &PredictResponse{
  //   Quality: 85,
  // }
  
  // ✅ 改为真实调用
  result, err := gw.modelRouter.PredictWithModel(model.Name, req.Request)
  if err != nil {
    gw.sendError(w, fmt.Sprintf("🚨 Model prediction failed: %v", err), 
                 http.StatusInternalServerError, startTime)
    return
  }
  
  w.Header().Set("Content-Type", "application/json")
  json.NewEncoder(w).Encode(result)
}
```

#### 3. 删除 predictor 的fallback

```go
// ❌ 完全删除：getDefaultPrediction方法
// func (p *PredictorImpl) getDefaultPrediction(features) { ... }

// ❌ 完全删除：applyFallback方法
// func (fe *FeatureExtractor) applyFallback(features) { ... }

// ✅ FFprobe失败时响亮报错
func (fe *FeatureExtractor) extractFFprobeInfo(...) error {
  if err := fe.runFFprobe(); err != nil {
    // ❌ 不要fallback
    // ✅ 直接返回错误
    return fmt.Errorf("🚨 FFprobe extraction failed: %w", err)
  }
  return nil
}
```

---

## 📊 修复效果

### 修复前（❌ 静默降级）

```
[转换开始]
↓
AI不可用？→ 使用fallback（用户不知道）
↓
转换完成（用硬编码参数）
↓
用户以为AI工作了 ❌
```

### 修复后（✅ 响亮报错）

```
[转换开始]
↓
AI不可用？→ 🚨 响亮报错
↓
显示详细错误信息
↓
提供修复步骤
↓
转换中止 ✅
```

---

## 🎯 用户体验改进

### 修复前
- ❌ AI失败，用户不知道
- ❌ 用fallback，质量不保证
- ❌ 以为AI在工作

### 修复后
- ✅ AI失败，立即报错
- ✅ 显示详细错误信息
- ✅ 提供修复步骤
- ✅ 不转换 > 低质量转换

---

## 📝 需要删除的文件/方法

### JS端
- `ai-client.js::getFallbackParams()` - 完全删除
- 所有 `return this.getFallbackParams(...)` 调用

### Go端
- `video_handlers.go::getVideoFallbackParams()` - 完全删除
- `http_gateway_models.go` 的 TODO mock代码
- `predictor.go::getDefaultPrediction()` - 完全删除
- `feature_extractor.go::applyFallback()` - 完全删除

---

## ✅ 成功标准

1. **零Fallback**
   - `grep -r "fallback" core/` → 只有文档
   - `grep -r "getFallback" core/` → 0结果

2. **响亮报错**
   - AI不可用 → 抛出异常
   - 用户看到详细错误信息

3. **真实调用**
   - `grep -r "mock" core/` → 0结果
   - `grep -r "TODO.*predict" core/` → 0结果

4. **验证不篡改**
   - AI返回值直接使用
   - 只验证格式，不修改值

---

## 🚀 执行顺序

1. ✅ JS端 ai-client.js（最重要）
2. ✅ Go端 video_handlers.go
3. ✅ Go端 http_gateway_models.go
4. ✅ Go端 predictor相关
5. ✅ 验证：搜索残留fallback

---

## 📅 时间表

- **Phase 1:** JS端根除（30分钟）
- **Phase 2:** Go端根除（30分钟）
- **Phase 3:** 测试验证（15分钟）
- **Phase 4:** 文档更新（15分钟）

**总计：** 90分钟完成彻底根除

---

**执行人：** Cascade AI  
**时间：** 2025-01-09  
**优先级：** CRITICAL  
**原则：** 质量 > 速度，响亮报错 > 静默降级
