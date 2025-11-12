# 🧪 UI控件完整性测试清单

## 测试目的
验证所有UI控件在修复`log is not defined`错误后是否正常工作。

---

## ✅ 测试环境
- **Eagle版本**: 4.x (Electron)
- **插件版本**: 3.0.0
- **测试文件**: 
  - 图像：JPEG, PNG, HEIC
  - 视频：MP4, MOV
  - 动图：GIF, APNG

---

## 📋 测试项目

### 1. 基础UI加载
- [ ] 插件正常加载，无JavaScript错误
- [ ] 所有模板正确注入（header, file-selection, panels, modals）
- [ ] 日志系统初始化（`pixlyLog`可用）
- [ ] 国际化系统加载（i18n）
- [ ] 主题系统正常（light/dark切换）

**验证方法**：
```javascript
// 在Eagle控制台运行
console.log('pixlyLog:', typeof window.pixlyLog);
console.log('i18n:', typeof window.i18n);
console.log('PIXLY:', window.PIXLY);
```

---

### 2. 面板切换
#### 2.1 主面板切换
- [ ] **Image Conversion** ↔ **Video Processing** 标签切换
- [ ] 切换时正确显示/隐藏对应面板
- [ ] 切换时自动刷新文件列表
- [ ] 切换时更新UI状态

**验证方法**：
1. 点击"Image Conversion"标签
2. 检查`#image-panel`是否显示
3. 点击"Video Processing"标签
4. 检查`#video-panel`是否显示

#### 2.2 模式切换（Image）
- [ ] **Smart Mode** ↔ **Manual Mode** 切换
- [ ] Smart模式显示AI选项
- [ ] Manual模式显示手动参数
- [ ] 模式切换时保持文件选择

**验证方法**：
1. 在Image面板切换Smart/Manual
2. 检查对应选项是否显示/隐藏
3. 验证日志：`[PIXLY UI] Image mode: Smart` / `Manual`

#### 2.3 模式切换（Video）
- [ ] **Smart Mode** ↔ **Manual Mode** 切换
- [ ] Smart模式显示视频AI选项
- [ ] Manual模式显示编码器选项
- [ ] 模式切换时更新状态指示器

---

### 3. 文件选择
- [ ] 点击"Select Files"按钮响应
- [ ] 文件选择对话框打开
- [ ] 选择文件后正确显示文件卡片
- [ ] 文件卡片显示正确信息（名称、大小、图标）
- [ ] 多文件选择正常
- [ ] 文件列表可滚动

**验证方法**：
1. 点击"Select Files"
2. 选择1-5个文件
3. 检查文件卡片是否正确显示
4. 验证日志：`[PIXLY File] ✅ Selected N files`

---

### 4. 格式选择
#### 4.1 图像格式
- [ ] JXL单选按钮可选
- [ ] AVIF单选按钮可选
- [ ] WebP单选按钮可选
- [ ] JPEG单选按钮可选
- [ ] PNG单选按钮可选
- [ ] 选择后触发格式专属参数更新

**验证方法**：
1. 逐个点击格式单选按钮
2. 验证日志：`[PIXLY] 🔍 Current selected format: xxx`

#### 4.2 视频格式
- [ ] H.264编码器可选
- [ ] H.265编码器可选
- [ ] VP9编码器可选
- [ ] AV1编码器可选
- [ ] 容器格式（MP4/MOV/MKV/WebM）可选

---

### 5. 优化模式（Image Smart）
- [ ] **Fast预设**：点击响应
- [ ] **Balanced预设**：点击响应
- [ ] **Quality预设**：点击响应
- [ ] 切换预设时更新说明卡片
- [ ] 说明卡片显示正确图标和文本

**验证方法**：
1. 在Image Smart模式点击各预设
2. 检查说明卡片是否更新
3. 验证日志：`[PIXLY UI] ✨ Updated to xxx mode info`

---

### 6. 视频AI预设（Video Smart）
- [ ] **Fast预设**：点击响应
- [ ] **Balanced预设**：点击响应
- [ ] **Quality预设**：点击响应
- [ ] 切换预设时更新视频说明卡片

**验证方法**：
1. 切换到Video Processing
2. 在Smart模式点击各预设
3. 检查视频说明卡片是否更新

---

### 7. 高级选项
#### 7.1 图像高级AI选项
- [ ] 展开/折叠按钮响应
- [ ] 展开时显示完整选项
- [ ] 折叠时隐藏选项
- [ ] 动画流畅

**验证方法**：
1. 点击"Advanced AI Options"
2. 检查`#advancedAIOptions`显示状态
3. 验证日志：`[PIXLY AI] Image advanced AI options expanded/collapsed`

#### 7.2 AI功能复选框
- [ ] **Auto-Optimize**：可勾选/取消
- [ ] **Bayesian Optimization**：依赖Auto-Optimize
- [ ] **Format Recommendation**：可勾选/取消
- [ ] **Video-for-Animation**：依赖Format Recommendation
- [ ] 依赖关系正确（父选项禁用时子选项自动禁用）

**验证方法**：
1. 取消勾选Auto-Optimize
2. 验证Bayesian自动禁用
3. 验证日志：`[PIXLY AI] Bayesian dependency initialized`

---

### 8. 范围收束（Scope Filter）
- [ ] **Enable Scope Filter**：主开关可切换
- [ ] 启用后显示过滤选项
- [ ] **File Size Filter**：可启用，输入框可用
- [ ] **Resolution Filter**：可启用，输入框可用
- [ ] 输入框验证正确（只允许数字）

**验证方法**：
1. 勾选"Enable Scope Filter"
2. 验证过滤选项显示
3. 验证日志：`[PIXLY Scope Filter] Scope filter initialized`

---

### 9. 手动参数（Manual Mode）
#### 9.1 质量滑块
- [ ] Quality滑块可拖动（0-100）
- [ ] 显示当前值
- [ ] 实时更新

#### 9.2 CRF滑块
- [ ] CRF滑块可拖动（0-51）
- [ ] 显示当前值
- [ ] 提示信息正确

#### 9.3 JPEG专属
- [ ] **JPEG Lossless**复选框响应
- [ ] 勾选后禁用质量参数
- [ ] 取消后恢复质量参数

**验证方法**：
1. 切换到Manual模式
2. 拖动各滑块
3. 勾选/取消JPEG Lossless
4. 验证日志：`[PIXLY UI] ✅ CRF slider bound`

---

### 10. 快速工具
- [ ] **AI Validation**按钮可点击
- [ ] **Format Correction**按钮可点击
- [ ] 点击后有反馈（loading状态或结果）

**验证方法**：
1. 选择文件后点击快速工具
2. 验证日志：`[PIXLY UI] ✅ Quick Tools bound`

---

### 11. 日志级别切换
- [ ] 日志级别下拉菜单可点击
- [ ] 展开显示所有级别（ERROR, WARN, INFO, DEBUG, TRACE）
- [ ] 选择后正确切换
- [ ] 控制台日志级别相应变化

**验证方法**：
1. 点击日志级别选择器
2. 切换到DEBUG
3. 验证日志：`[📑 Log Level] Switched to DEBUG`
4. 检查控制台是否显示更详细日志

---

### 12. 语言切换
- [ ] 语言下拉菜单可点击
- [ ] 展开显示所有语言（中文简体、中文繁体、日语、英语）
- [ ] 选择后正确切换
- [ ] UI文本自动翻译

**验证方法**：
1. 点击语言选择器
2. 切换到不同语言
3. 验证日志：`[PIXLY UI] 🌐 Switch language: xxx`
4. 检查UI是否更新为对应语言

---

### 13. 转换功能
#### 13.1 开始转换
- [ ] **Start Conversion**按钮可点击
- [ ] 点击后按钮禁用
- [ ] 显示进度条
- [ ] 显示取消按钮
- [ ] 进度实时更新

#### 13.2 取消转换
- [ ] **Cancel**按钮可点击
- [ ] 点击后停止转换
- [ ] 恢复UI状态

#### 13.3 转换完成
- [ ] 显示成功通知
- [ ] 显示文件锁定覆盖层
- [ ] "Select New Files"按钮可用
- [ ] 点击后解锁并刷新

**验证方法**：
1. 选择文件并点击"Start Conversion"
2. 观察进度条和日志
3. 等待完成或点击取消
4. 验证最终状态

---

### 14. 核心状态指示器
#### 14.1 图像面板
- [ ] **Smart模式**：显示GO核心状态（🤖 在线✅ / ❌离线）
- [ ] **Manual模式**：显示本地工具状态（🔧 Local）
- [ ] 状态正确反映实际连接

#### 14.2 视频面板
- [ ] **Smart模式**：显示GO核心状态
- [ ] **Manual模式**：显示Rust核心状态（⚙️ Rust v0.3.0）
- [ ] 状态指示器颜色正确（绿色=在线，红色=离线，橙色=本地）

**验证方法**：
1. 检查各面板状态指示器
2. 验证日志：
   - `[PIXLY Image Smart] GO core ready: xxx`
   - `[PIXLY Video Manual] Rust core ready: xxx`

---

### 15. 模态框（Modals）
- [ ] **Help模态框**：可打开/关闭
- [ ] **Settings模态框**：可打开/关闭
- [ ] 关闭按钮响应
- [ ] 点击遮罩层关闭
- [ ] ESC键关闭

---

### 16. 下拉菜单
- [ ] JXL/Normal模式下拉可点击
- [ ] 展开显示选项
- [ ] 选择后正确切换
- [ ] 点击外部自动关闭

**验证方法**：
1. 点击下拉菜单
2. 验证日志：`[PIXLY UI] 🔽 Dropdown toggled`

---

### 17. 兼容性提示
- [ ] 选择H.265 + MP4时显示兼容性提示
- [ ] 提示内容正确（"✅ H.265 + MP4 = 广泛兼容"）
- [ ] 其他组合更新提示

**验证方法**：
1. 在视频面板选择不同编码器和容器
2. 验证日志：`[PIXLY UI] 🔄 Updated compatibility: xxx`

---

## ❌ 已知问题

### 1. GO Log API 404
**现象**：
```
localhost:50052/api/v1/logs?limit=50: Failed to load resource: 404
```

**原因**：
- GO服务的日志API端点可能未实现
- 或路径不正确

**影响**：
- 不影响核心功能
- 仅影响跨平台日志收集

**计划修复**：
- 检查GO服务的实际API端点
- 更新`cross-platform-log-collector.js`的轮询URL

---

### 2. Eagle Logger TypeError
**现象**：
```
TypeError: Cannot read properties of undefined (reading 'name')
at Object.info (/Applications/Eagle.app/Contents/Resources/app.asar/app/js/plugin/logger.js:7:116)
```

**原因**：
- Eagle内部logger期望特定的参数格式
- 我们的日志调用格式不匹配

**影响**：
- 不影响功能
- 会产生额外的错误日志

**解决方案**：
- 已在`logger.js`中添加try-catch包装
- 错误被静默处理

---

### 3. 文件选择器卡死（macOS）
**现象**：
- macOS最新版本Electron应用文件选择器可能无响应
- 需要等待较长时间

**原因**：
- macOS系统对Electron应用的安全限制
- 符号链接可能加剧问题

**解决方案**：
- 使用rsync同步代替符号链接
- 或等待系统响应完成（用户已验证可用）

---

## 📊 测试结果模板

```markdown
测试日期：2025-11-10
测试人员：[你的名字]
Eagle版本：4.x
插件版本：3.0.0

| 测试项 | 状态 | 备注 |
|--------|------|------|
| 基础UI加载 | ✅ / ❌ | |
| 面板切换 | ✅ / ❌ | |
| 文件选择 | ✅ / ❌ | |
| 格式选择 | ✅ / ❌ | |
| 优化模式 | ✅ / ❌ | |
| 视频AI预设 | ✅ / ❌ | |
| 高级选项 | ✅ / ❌ | |
| 范围收束 | ✅ / ❌ | |
| 手动参数 | ✅ / ❌ | |
| 快速工具 | ✅ / ❌ | |
| 日志级别 | ✅ / ❌ | |
| 语言切换 | ✅ / ❌ | |
| 转换功能 | ✅ / ❌ | |
| 核心状态 | ✅ / ❌ | |
| 模态框 | ✅ / ❌ | |
| 下拉菜单 | ✅ / ❌ | |
| 兼容性提示 | ✅ / ❌ | |

**总体评价**：
- 通过项：___ / 17
- 失败项：___ / 17
- 通过率：____%

**重大问题**：
1. 
2. 

**次要问题**：
1. 
2. 
```

---

## 🎯 快速验证命令

在Eagle控制台运行以下命令快速验证关键功能：

```javascript
// 1. 检查核心对象
console.log('✅ Core objects:', {
    pixlyLog: typeof window.pixlyLog,
    i18n: typeof window.i18n,
    PIXLY: typeof window.PIXLY,
    rustCLI: typeof window.rustCLI
});

// 2. 检查日志级别
window.pixlyLog.enableDebug();
console.log('✅ Debug logging enabled');

// 3. 检查GO核心
(async () => {
    const goStatus = await window.PIXLY.UIHandlers.detectGoCore();
    console.log('✅ GO Core:', goStatus);
})();

// 4. 检查Rust核心
console.log('✅ Rust CLI:', {
    available: window.rustCLI.isAvailable(),
    version: window.rustCLI.getVersion(),
    path: window.rustCLI.getCliPath()
});

// 5. 检查文件选择状态
console.log('✅ File selection:', {
    count: eagle?.item?.get?.()?.length || 0,
    locked: window.PIXLY_SELECTION_LOCKED
});
```

---

## 🚀 自动化测试（未来）

考虑使用Playwright或Puppeteer实现自动化测试：

```javascript
// test/ui-controls.spec.js
describe('PIXLY UI Controls', () => {
    test('should switch between image and video panels', async () => {
        // 自动化测试代码
    });
    
    test('should change optimization modes', async () => {
        // 自动化测试代码
    });
    
    // ...更多测试
});
```

---

## 📝 注意事项

1. **测试环境清洁**：每次测试前清除缓存和重启Eagle
2. **日志记录**：保持控制台打开，记录所有错误
3. **截图证据**：对于UI问题，截图保存
4. **版本标记**：记录测试的确切版本号
5. **重现步骤**：详细记录问题重现步骤

---

**最后更新**：2025-11-10 11:37
**下次复查**：每次重大更新后
