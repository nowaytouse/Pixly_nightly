# Bug修复 - Phase 45.8

## 修复日期
2025-11-07

## 问题汇总

### 1. ✅ 视频面板Rust在线时无法使用（已修复）

**问题**：日志显示`[PIXLY Video Manual] Rust core not connected`，即使Rust CLI可用。

**根本原因**：`detectRustCore()`函数试图检测Rust HTTP服务（端口8080），而不是检查Rust CLI的可用性。

**修复方案**：
```javascript
async function detectRustCore() {
    // �� Phase 45.7: 直接检查Rust CLI可用性（不是HTTP服务）
    const rustCLI = window.rustCLI;
    if (rustCLI && rustCLI.available) {
        return {
            available: true,
            version: rustCLI.version || 'Unknown',
            source: 'CLI'
        };
    }
    
    // 🔥 向下兼容：检测Rust HTTP服务（如果存在）
    // ... 原有的端口检测逻辑
}
```

---

### 2. ✅ JPEG无损转码选项不显示BUG（已修复）

**问题**：选择JPEG文件后切换到JXL格式，JPEG无损转码选项不会立即显示，需要来回切换才能触发。

**根本原因**：
1. `selectedFiles`变量未使用`window.selectedFiles`
2. 扩展名检测不支持带/不带点号的两种情况
3. 格式切换时未触发更新

**修复方案**：
```javascript
function updateJPEGNotice() {
    // 🔥 Phase 45.7: 支持带/不带点号的扩展名
    const jpegExtensions = ['jpg', 'jpeg', 'jpe', 'jfif', 'jfi', '.jpg', '.jpeg', '.jpe', '.jfif', '.jfi'];
    const hasJPEG = (window.selectedFiles || []).some(f => {
        if (!f || !f.ext) return false;
        const ext = f.ext.toLowerCase();
        return jpegExtensions.includes(ext);
    });
    
    console.log('[PIXLY UI] 🔍 JPEG Notice Update - Format:', selectedFormat?.value, 'isJXL:', isJXL, 'hasJPEG:', hasJPEG);
    
    if (isJXL && hasJPEG) {
        jpegNotice.style.display = 'block';
        // ...
    }
}
```

---

### 3. ✅ 文件选择区域布局错乱（已修复）

**问题**：文件卡片显示异常，分辨率和大小信息错位。

**根本原因**：CSS类名与JavaScript生成的HTML类名不匹配
- CSS定义：`.file-card`, `.file-card-icon`, `.file-card-info`, `.file-card-name`, `.file-card-meta`
- JavaScript生成：`.file-item`, `.file-icon`, `.file-info`, `.file-name`, `.file-meta`

**修复方案**：
修改`file-handler.js`中的HTML生成代码，使用正确的类名：

```javascript
filesList.innerHTML = files.map((file, index) => {
    const fileSizeKB = file.size ? (file.size / 1024).toFixed(1) : '?';
    const dimensions = (file.width && file.height) ? `${file.width}×${file.height}` : '';
    
    return `
        <div class="file-card" data-index="${index}" title="${file.name}">
            <div class="file-card-icon">${this.getFileIcon(file.ext)}</div>
            <div class="file-card-info">
                <div class="file-card-name">${file.name}</div>
                <div class="file-card-meta">
                    ${file.ext.toUpperCase()} · ${fileSizeKB} KB${dimensions ? ' · ' + dimensions : ''}
                </div>
            </div>
            <div class="file-card-remove" onclick="window.PIXLY.FileHandler.removeFile(${index})" title="移除">×</div>
        </div>
    `;
}).join('');
```

**改进**：
- ✅ 使用正确的CSS类名
- ✅ 简化元数据显示（使用 `·` 分隔）
- ✅ 添加文件移除按钮
- ✅ 移除不必要的日期显示
- ✅ 添加`removeFile()`方法支持单文件移除

---

## 测试建议

### 视频面板测试
1. [ ] 确认「✅ Rust在线」
2. [ ] 进入视频处理面板 → 手动模式
3. [ ] 确认所有编码器可选择（H.265/H.266/AV1/ProRes）
4. [ ] 确认质量滑块、速度、容器格式都可用
5. [ ] 选择一个视频文件，点击「开始转换」

### JPEG无损转码测试
1. [ ] 选择JPEG文件
2. [ ] 切换到手动模式
3. [ ] 选择JXL格式
4. [ ] 确认「✅ JPEG 无损转码」选项立即显示
5. [ ] 切换到其他格式，确认选项立即隐藏

### 文件卡片布局测试
1. [ ] 选择多个文件（JPEG、PNG、GIF等）
2. [ ] 确认每个文件卡片显示：
   - 文件图标（左侧）
   - 文件名（主要信息）
   - 格式 · 大小 · 分辨率（元数据，单行，用 `·` 分隔）
   - 移除按钮×（右侧）
3. [ ] hover文件卡片，确认有平滑动画
4. [ ] 点击×按钮，确认可以移除单个文件

---

## 文件修改清单

1. ✅ `core/plugin/js/plugin-modules/ui-handlers.js`
   - 修复`detectRustCore()`优先检查Rust CLI
   - 改进`updateJPEGNotice()`支持带/不带点号的扩展名
   - 添加日志输出便于调试

2. ✅ `core/plugin/js/plugin-modules/file-handler.js`
   - 修复HTML类名：`.file-item` → `.file-card`
   - 简化元数据显示格式
   - 添加`removeFile()`方法
   - 添加文件移除按钮

---

## 技术要点

### 1. Rust检测优先级
```javascript
// 优先级：Rust CLI > Rust HTTP > 失败
async function detectRustCore() {
    // 1. 检查Rust CLI（主要方式）
    if (window.rustCLI?.available) {
        return { available: true, source: 'CLI' };
    }
    
    // 2. 检查Rust HTTP服务（向下兼容）
    // ... 端口检测逻辑
    
    // 3. 返回不可用
    return { available: false };
}
```

### 2. CSS类名规范
遵循BEM命名规范：
- **Block**: `.file-card`
- **Element**: `.file-card-icon`, `.file-card-info`, `.file-card-name`
- **Modifier**: `.file-card--active`, `.file-card--error`

### 3. 扩展名标准化
```javascript
// Eagle API返回的ext可能带/不带点号
const jpegExtensions = [
    'jpg', 'jpeg', 'jpe', 'jfif', 'jfi',  // 无点号
    '.jpg', '.jpeg', '.jpe', '.jfif', '.jfi'  // 带点号
];
```

---

## 下一步

1. **测试所有修复**：刷新插件，逐项测试
2. **监控日志**：观察控制台是否有新的错误
3. **收集反馈**：如有问题，提供详细日志和截图

---

**修复完成时间**：2025-11-07 03:45 AM  
**修复负责人**：AI Assistant (Claude Sonnet 4.5)  
**版本**：Phase 45.8

**累计修复**：
- Phase 45.6: 下拉菜单 + 视频控件启用
- Phase 45.7: Tooltip + Badge + 视频控件禁用
- Phase 45.8: Rust检测 + JPEG通知 + 文件卡片布局
