# PIXLY Format Vue - 安装和运行

## 快速开始

### 1. 安装依赖
```bash
cd plugin/format-vue
npm install
```

### 2. 开发模式（实时预览）
```bash
npm run dev
```
浏览器打开 http://localhost:3000

### 3. 构建生产版本
```bash
npm run build
```
输出到 `dist/` 目录

### 4. 在Eagle中使用
1. 构建完成后，将整个 `format-vue` 文件夹复制到Eagle插件目录
2. 在Eagle中刷新插件列表
3. 启动 PIXLY Format Vue

## 项目结构
```
format-vue/
├── src/
│   ├── App.vue              # 主应用
│   ├── main.js              # 入口
│   ├── components/          # Vue组件
│   │   ├── Header.vue
│   │   ├── FormatSelector.vue
│   │   ├── QualityPanel.vue
│   │   ├── AdvancedParams.vue
│   │   ├── FileList.vue
│   │   └── ConvertButton.vue
│   └── styles/
│       └── global.css       # 全局样式
├── dist/                    # 构建输出
├── package.json
├── vite.config.js
└── manifest.json
```

## 当前状态
✅ 基础架构完成
✅ 核心组件完成
✅ 响应式布局
✅ 暗色/亮色主题
⏳ Rust CLI集成（待完成）
⏳ 高级参数面板（待完成）
⏳ 国际化（待完成）
