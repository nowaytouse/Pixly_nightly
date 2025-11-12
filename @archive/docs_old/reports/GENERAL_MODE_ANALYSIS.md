# 🚨 通用模式空壳功能分析报告

## 问题发现

**通用模式（general）在UI层有定义，但在整个转换链路中没有实现！**

---

## 证据链

### 1. UI层定义 ✅
- **位置**: `core/plugin/templates/image-panel.html:45`
- **HTML**: `<input type="radio" name="optimizeMode" value="general">`
- **翻译**: zh_CN.json 定义了 generalDesc

### 2. JS层传递 ❌ 
- **检查**: `image-conversion.js` - 没有读取optimizeMode
- **检查**: `rust-cli-executor.js` - 没有传递optimizeMode参数
- **结果**: **断裂点1 - JS未传递给Rust**

### 3. Rust CLI接收 ❌
- **检查**: `core/rust/src/cli/commands.rs`
- **发现**: 只有 "balanced" 硬编码
- **结果**: **断裂点2 - Rust CLI不接受optimize_mode参数**

### 4. AI Client传递 ❌
- **检查**: `core/rust/src/converter/ai_client.rs`
- **发现**: optimize_mode字段传递给GO Service
- **但**: GO Service只使用 "size", "balanced", "quality" 三个值
- **结果**: **断裂点3 - GO不认识general模式**

---

## 用户期望 vs 实际情况

| 组件 | 用户期望 | 实际情况 |
|------|---------|---------|
| **UI** | 选择"通用"模式 | ✅ 可以选择 |
| **描述** | "硬编码规则：JPEG/PNG无损转JXL" | ✅ 有描述 |
| **状态显示** | "✅ 规则模式（无需AI）" | ✅ Phase 46.5.11实现 |
| **实际转换** | JPEG→JXL无损 + Q95规则 | ❌ **未实现** |
| **AI调用** | 不调用AI（规则模式） | ❌ **实际仍调用AI** |

---

## 核心问题

**通用模式的所有UI/UX都已完成，但核心转换逻辑完全缺失！**

### 缺失的实现

1. **JS层**:
   ```javascript
   // 需要在 image-conversion.js 中:
   const optimizeMode = document.querySelector('input[name="optimizeMode"]:checked')?.value || 'balanced';
   ```

2. **Rust CLI**:
   ```bash
   # 需要支持参数:
   pixly-rust convert input.jpg output.jxl --optimize-mode general
   ```

3. **Rust逻辑**:
   ```rust
   // 需要在 convert_image() 中:
   if optimize_mode == "general" {
       // JPEG → JXL 无损转码
       // PNG → JXL 无损
       // 其他 → JXL Q95
       // 不调用AI
   }
   ```

---

## 解决方案

### 方案1: 删除通用模式（简单）
- 删除UI中的general选项
- 删除翻译文件中的相关文本
- 删除Phase 46.5.11的特殊处理
- **优点**: 快速，不留空壳
- **缺点**: 失去规则模式功能

### 方案2: 完整实现通用模式（推荐）
- JS层: 读取optimizeMode并传递
- Rust CLI: 添加 `--optimize-mode` 参数
- Rust逻辑: 实现规则路由
  - JPEG输入 → JXL无损转码 (--lossless_jpeg=1)
  - PNG输入 → JXL无损 (-d 0 -q 100)
  - 其他格式 → JXL Q95
  - **不调用AI Service**
- **优点**: 真正的规则模式，快速转换
- **缺点**: 需要约200行代码

---

## 推荐行动

**立即实现方案2 - 完整实现通用模式**

原因：
1. 用户明确描述了通用模式的规则逻辑
2. UI/UX已完整，只差核心逻辑
3. 符合"反对空壳代码"原则
4. 提供真正的"无需AI快速转换"选项

---

## 实施步骤

### Step 1: JS层传递 (30行)
- `image-conversion.js`: 读取optimizeMode
- `rust-cli-executor.js`: 添加 `--optimize-mode` 参数

### Step 2: Rust CLI接收 (50行)
- `commands.rs`: 解析 `--optimize-mode` 参数
- 传递给 `convert_image(optimize_mode)`

### Step 3: Rust逻辑实现 (120行)
- `conversion.rs`: 新增 `apply_general_mode_rules()` 函数
- 规则路由:
  - JPEG → JXL无损转码
  - PNG → JXL无损 (Q100, d0)
  - 动图 → AVIF有损
  - 其他 → JXL Q95
- **跳过AI预测调用**

### Step 4: 修改提示文本
- `image-panel.html`: 动态显示规则说明
- 翻译文件: 更新generalDesc

---

**估计时间**: 1.5-2小时  
**预期效果**: 真正的规则模式，10-20x速度提升（无AI延迟）
