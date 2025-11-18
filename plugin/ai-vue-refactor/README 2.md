# PIXLY AI Vue 重构版

## 快速开始

### 1. 安装依赖
```bash
cd plugin/ai-vue-refactor
npm install
```

### 2. 开发预览（浏览器）
```bash
npm run dev
```
然后在 Brave 浏览器打开 http://localhost:5173

### 3. 构建用于 Eagle
```bash
npm run build
```

构建产物在 `dist/` 目录，将以下文件复制到 Eagle 插件目录：
- dist/ 下的所有文件
- manifest.json
- logo.png（如果需要）

## 目录结构
```
plugin/ai-vue-refactor/
├── src/
│   ├── App.vue          # 主应用
│   └── main.js          # 入口
├── public/
│   └── logo.png         # Logo
├── index.html           # HTML 模板
├── vite.config.js       # Vite 配置
├── package.json         # 依赖
└── manifest.json        # Eagle 配置
```

## 架构原则
- Vue 层：仅 UI 交互
- Rust CLI：所有核心功能
- 遵循 PROJECT_QUALITY_MANIFESTO.md
