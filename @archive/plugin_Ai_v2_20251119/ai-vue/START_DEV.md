# 🚀 PIXLY AI Vue Plugin - 开发指南

## 快速启动

### 1. 安装依赖

```bash
cd plugin/ai-vue-new
npm install
```

### 2. 启动开发服务器

```bash
# 方式1: 标准开发模式
npm run dev

# 方式2: 局域网测试模式（推荐）
npm run test:local
```

### 3. 访问应用

- **本机访问**: http://localhost:5173
- **局域网访问**: http://[你的IP]:5173
- **测试页面**: 直接打开 `test-local.html`

## 📱 局域网测试

### 查看本机 IP

```bash
# macOS/Linux
ifconfig | grep "inet " | grep -v 127.0.0.1

# Windows
ipconfig | findstr IPv4
```

### 手机/平板测试

1. 确保设备和电脑在同一 WiFi
2. 在手机浏览器访问: `http://[电脑IP]:5173`
3. 测试 Eagle API 集成和 UI 响应

## 🏗️ 项目结构

```
plugin/ai-vue-new/
├── src/
│   ├── App.vue                    # 主应用组件
│   ├── main.js                    # 入口文件
│   └── composables/
│       ├── useRustCLI.js         # 🔥 Rust CLI 集成
│       └── useEagleAPI.js        # 🦅 Eagle API 集成
├── public/
│   └── logo.png                   # Logo 图标
├── index.html                     # HTML 模板
├── test-local.html                # 本地测试页面
├── vite.config.js                 # Vite 配置
├── package.json                   # 依赖配置
├── manifest.json                  # Eagle 插件配置
└── README.md                      # 说明文档
```

## 🔧 开发模式特性

### Mock 数据

在本地开发时（检测不到 `window.eagle`），会自动使用 Mock 数据：

- 3 个示例图片文件
- 模拟的文件属性（尺寸、大小、格式）
- 模拟的缩略图（使用 picsum.photos）

### Rust CLI 检测

启动时会自动检测 Rust CLI：

```javascript
// 检测 bin/pixly-rust 是否存在
await detectRustCLI()

// 如果不存在，会显示错误提示
// ❌ Rust 转换内核未找到！请确保 pixly-rust 已编译。
```

### Eagle API 检测

```javascript
// 检测 window.eagle 是否可用
detectEagle()

// 开发模式会自动使用 Mock 数据
```

## 🎨 UI 组件库

使用 **Element Plus** - Vue 3 的企业级 UI 组件库

### 主要组件

- `el-button` - 按钮
- `el-table` - 表格
- `el-select` - 下拉选择
- `el-radio-group` - 单选按钮组
- `el-progress` - 进度条
- `el-card` - 卡片
- `el-message` - 消息提示
- `el-notification` - 通知

### 图标

使用 `@element-plus/icons-vue`：

```vue
<el-icon><Refresh /></el-icon>
<el-icon><Moon /></el-icon>
<el-icon><MagicStick /></el-icon>
```

## 🔥 架构原则（重要！）

### ✅ 正确的做法

```javascript
// Vue 层：收集参数，调用 Rust CLI
const startProcessing = async () => {
  const results = await batchConvert(
    selectedFiles.value,
    {
      mode: 'smart',
      optimizeTarget: 'balanced',
      outputFormat: 'avif'
    }
  )
}
```

### ❌ 禁止的做法

```javascript
// ❌ 不要在 JS 中实现转换逻辑
const convertImage = (input, output) => {
  // 200+ lines 转换代码
  exec(`cjxl ${input} ${output}`)  // ← 违反架构
}

// ❌ 不要在 JS 中计算参数
const calculateQuality = (image) => {
  if (image.complexity > 0.8) return 95  // ← 硬编码规则
  return 85
}
```

### 架构分层

```
┌─────────────────────────────────────┐
│  Vue UI Layer (plugin/ai-vue-new)  │
│  - 用户交互                          │
│  - 参数收集                          │
│  - 进度显示                          │
└──────────────┬──────────────────────┘
               │ useRustCLI.js
               ↓
┌─────────────────────────────────────┐
│  Rust CLI (bin/pixly-rust)         │
│  - AI 预测                           │
│  - 文件处理                          │
│  - 格式转换                          │
│  - 参数优化                          │
└─────────────────────────────────────┘
```

## 🧪 测试清单

### 本地开发测试

- [ ] `npm run dev` 启动成功
- [ ] 浏览器打开 http://localhost:5173
- [ ] 看到 Mock 数据（3 个示例文件）
- [ ] 可以选择文件
- [ ] 可以切换主题（深色/浅色）
- [ ] 可以切换语言（中文/英文）

### Rust CLI 集成测试

- [ ] 检测到 Rust CLI 版本
- [ ] AI 状态显示 "✅ 就绪"
- [ ] 点击"开始 AI 处理"调用 Rust CLI
- [ ] 进度条正常更新
- [ ] 转换完成显示通知

### Eagle 集成测试

- [ ] 在 Eagle 中选择文件
- [ ] 打开插件，文件列表正确显示
- [ ] 缩略图正确加载
- [ ] 文件信息（尺寸、大小）正确
- [ ] 转换后 Eagle 库自动刷新

## 📦 构建生产版本

```bash
# 构建
npm run build

# 输出目录
dist/
├── index.html
├── assets/
│   ├── js/
│   └── css/
└── ...
```

### 部署到 Eagle

1. 复制 `dist/` 内容到 Eagle 插件目录
2. 复制 `manifest.json` 和 `logo.png`
3. 复制 `bin/` 目录（Rust CLI）
4. 在 Eagle 中重新加载插件

## 🐛 常见问题

### Q: 开发服务器启动失败？

```bash
# 检查端口是否被占用
lsof -i :5173

# 更换端口（修改 vite.config.js）
server: { port: 3000 }
```

### Q: Rust CLI 检测失败？

```bash
# 确保 Rust CLI 已编译
cd ../../  # 回到项目根目录
cargo build --release

# 复制到插件 bin 目录
cp target/release/pixly-rust plugin/ai-vue-new/bin/
```

### Q: Element Plus 样式不显示？

```bash
# 确保已安装依赖
npm install element-plus

# 检查 main.js 是否导入样式
import 'element-plus/dist/index.css'
```

## 📚 参考文档

- [Vue 3 文档](https://vuejs.org/)
- [Element Plus 文档](https://element-plus.org/)
- [Vite 文档](https://vitejs.dev/)
- [Eagle Plugin API](https://developer.eagle.cool/plugin-api)
- [PROJECT_QUALITY_MANIFESTO.md](../../PROJECT_QUALITY_MANIFESTO.md)

## 🎯 下一步

1. ✅ 完成基础 UI 和 Mock 数据
2. ⏳ 集成真实 Rust CLI 调用
3. ⏳ 实现 AI 参数预测显示
4. ⏳ 添加更多格式选项
5. ⏳ 实现批量处理进度
6. ⏳ 添加国际化支持
7. ⏳ 性能优化和错误处理

---

**遵循 PROJECT_QUALITY_MANIFESTO.md 质量标准** 🔥
