# PIXLY AI Vue Plugin

Vue 3 + Element Plus 版本的 PIXLY AI 智能媒体处理插件

## 🚀 快速开始

### 本地开发测试

```bash
cd plugin/ai-vue-new

# 安装依赖
npm install

# 启动开发服务器（带本地网络访问）
npm run test:local

# 或者标准开发模式
npm run dev
```

开发服务器会在 `http://localhost:5173` 启动，并且可以通过局域网 IP 访问（如 `http://192.168.x.x:5173`）

### 构建生产版本

```bash
npm run build
```

构建产物会输出到 `dist/` 目录

### 部署到 Eagle

1. 构建生产版本
2. 将 `dist/` 目录内容复制到 Eagle 插件目录
3. 复制 `manifest.json` 和 `logo.png` 到插件根目录
4. 在 Eagle 中重新加载插件

## 📁 项目结构

```
plugin/ai-vue-new/
├── src/
│   ├── App.vue           # 主应用组件
│   └── main.js           # 入口文件
├── index.html            # HTML 模板
├── vite.config.js        # Vite 配置
├── package.json          # 依赖配置
├── manifest.json         # Eagle 插件配置
└── README.md            # 说明文档
```

## 🎨 技术栈

- **Vue 3** - 渐进式 JavaScript 框架
- **Element Plus** - Vue 3 UI 组件库
- **Vite** - 下一代前端构建工具

## 🔧 功能特性

- ✅ 智能/手动双模式
- ✅ AI 驱动的格式推荐
- ✅ 批量文件处理
- ✅ 实时进度显示
- ✅ 深色模式支持
- ✅ 多语言支持（中文/英文）
- ✅ 本地开发模拟数据

## 🌐 本地网络测试

运行 `npm run test:local` 后，可以通过以下方式访问：

1. **本机访问**: http://localhost:5173
2. **局域网访问**: http://[你的IP]:5173
3. **手机测试**: 确保手机和电脑在同一网络，访问电脑 IP

查看本机 IP：
```bash
# macOS/Linux
ifconfig | grep "inet "

# Windows
ipconfig
```

## 📝 开发说明

### Mock 数据模式

在本地开发时，如果检测不到 `window.eagle` API，会自动使用 Mock 数据进行测试。

### Eagle API 集成

生产环境会使用真实的 Eagle API：
- `window.eagle.item.getSelected()` - 获取选中的文件
- 更多 API 参考 Eagle 官方文档

## 🔗 相关链接

- [Eagle Plugin API 文档](https://developer.eagle.cool/plugin-api)
- [Vue 3 文档](https://vuejs.org/)
- [Element Plus 文档](https://element-plus.org/)
- [Vite 文档](https://vitejs.dev/)
