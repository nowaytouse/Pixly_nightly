# Pixly Eagle Plugins - 开发者快速启动指南

## 🎯 概览

本项目包含两个 Eagle 插件，共享统一的 Rust 转换引擎：

- **format-vue**: 基础媒体转换（图像/视频/动图 + 辅助功能）
- **ai-vue-refactor**: 智能媒体转换（AI 参数预测 + 全部功能）

**核心特性**：
- ✅ 共享 Rust 二进制（`pixly-eagle-core`），避免代码重复
- ✅ 仅在 Eagle 环境运行，开发模式支持浏览器调试
- ✅ Feature flags 功能隔离，统一维护

---

## 🚀 快速开始

### 1. 一次性设置

```bash
# 进入项目根目录
cd /path/to/Pixly_Nightly

# 构建共享 Rust 二进制
cd plugin/shared
bash build.sh
# 预期: ✅ Built: plugin/shared/bin/pixly-eagle-core

# 设置 format-vue
cd ../format-vue
npm install
npm run setup
# 预期: ✅ Symlinked: format-vue/bin/pixly-eagle-core

# 设置 ai-vue-refactor
cd ../ai-vue-refactor
npm install
npm run setup
# 预期: ✅ Symlinked: ai-vue-refactor/bin/pixly-eagle-core
```

### 2. 启动开发服务器

**format-vue**:
```bash
cd plugin/format-vue
npm run dev
# 访问: http://localhost:5173
```

**ai-vue-refactor**:
```bash
cd plugin/ai-vue-refactor
npm run dev
# 访问: http://localhost:5174
```

---

## 🔧 开发工作流

### 修改 Vue 代码

直接编辑 `src/` 目录下的文件，Vite 会自动热重载：

```bash
# 示例：修改 format-vue 的主界面
vim plugin/format-vue/src/App.vue
# 保存后浏览器自动刷新
```

### 修改 Rust 代码

修改 Rust 代码后需要重新编译：

```bash
# 1. 修改主库代码（src/*）或共享二进制（plugin/shared/src/main.rs）
vim src/core/conversion_core.rs  # 或其他 Rust 文件

# 2. 重新构建共享二进制
cd plugin/shared
bash build.sh

# 3. Vue 插件会自动使用新的二进制（符号链接）
```

### 环境模式

**开发模式** (`NODE_ENV=development`):
- ✅ 允许在浏览器中运行（跳过 Eagle 检测）
- ✅ Rust CLI 自动接收 `--dev` 参数
- ⚠️ 日志提示 "Development mode: Eagle environment not detected"

**生产模式** (`NODE_ENV=production`):
- ❌ 强制 Eagle 环境检测
- ❌ 非 Eagle 环境会抛出错误

---

## 🧪 测试

### 测试共享二进制

```bash
cd plugin/shared/bin

# 测试版本号（开发模式）
./pixly-eagle-core --dev --version
# 预期: pixly-eagle-core 3.0.0

# 测试 Analyze 命令
./pixly-eagle-core --dev analyze --help
# 预期: 显示完整的参数说明

# 测试 Eagle 环境检测（应该失败）
./pixly-eagle-core --version
# 预期: ❌ This binary can ONLY run inside Eagle.
```

### 测试插件环境检测

**浏览器测试（开发模式）**:
```bash
cd plugin/format-vue
npm run dev
# 在浏览器打开 http://localhost:5173
# 预期: 插件正常加载，控制台显示 "Development mode" 警告
```

**Eagle 测试**:
1. 在 Eagle 中安装插件（将 `dist/` 目录安装为插件）
2. 打开插件
3. 预期: 正常运行，不显示环境警告

---

## 📦 打包分发

### 构建 Release 版本

```bash
cd plugin/format-vue
npm run release
# 自动执行:
# 1. 构建共享 Rust 二进制 (plugin/shared/build.sh)
# 2. 复制二进制到 bin/
# 3. 构建 Vue 应用 (npm run build)
# 预期: ✅ Release package ready in dist/
```

### 分发目录结构

```
plugin/format-vue/dist/
├── index.html
├── assets/
│   ├── index-xxxxx.js
│   └── index-xxxxx.css
├── manifest.json
└── bin/
    └── pixly-eagle-core  # 共享二进制（已复制）
```

将整个 `dist/` 目录打包为 `.zip` 即可分发。

---

## 🐛 常见问题

### Q: 二进制文件未找到

**错误**: `pixly-eagle-core not found`

**解决**:
```bash
# 检查符号链接
ls -la plugin/format-vue/bin/pixly-eagle-core

# 如果不存在，重新运行 setup
cd plugin/format-vue
npm run setup
```

### Q: Eagle 环境检测失败

**错误**: `This plugin can ONLY run inside Eagle.`

**解决**:
- **开发环境**: 确保 `NODE_ENV=development`（Vite 默认设置）
- **生产环境**: 必须在 Eagle 中运行插件

### Q: Rust 代码修改未生效

**解决**:
```bash
# 重新编译共享二进制
cd plugin/shared
bash build.sh

# 刷新浏览器或重启 Eagle 插件
```

### Q: 符号链接在 Windows 下不工作

**解决**:
Windows 用户直接复制二进制文件：
```bash
# 替代 npm run setup
cd plugin/format-vue
mkdir -p bin
cp ../shared/bin/pixly-eagle-core.exe bin/
```

---

## 📚 项目结构

```
Pixly_Nightly/
├── Cargo.toml              # 主项目配置（含 feature flags）
├── src/                    # Rust 主库代码
│   ├── core/               # 核心转换逻辑
│   ├── codecs/             # 编解码器
│   ├── ai/                 # AI 模块
│   └── utils/              # 辅助工具
└── plugin/
    ├── shared/             # 共享 Rust 二进制
    │   ├── Cargo.toml      # 依赖主库
    │   ├── src/main.rs     # Eagle 专用 CLI
    │   ├── build.sh        # 构建脚本
    │   └── bin/
    │       └── pixly-eagle-core
    ├── format-vue/         # 基础转换插件
    │   ├── src/
    │   │   ├── App.vue
    │   │   └── composables/
    │   │       └── useRustCLI.js  # CLI 调用逻辑
    │   ├── bin/ -> ../shared/bin/ # 符号链接
    │   ├── setup-bin.sh    # 开发环境设置
    │   └── package.json
    └── ai-vue-refactor/    # AI 智能转换插件
        └── (相同结构)
```

---

## 🔄 下一步

1. **完善 main.rs**: 实现完整的 `Convert` 和 `Video` 命令
2. **Eagle 测试**: 在 Eagle 中测试插件功能
3. **文档更新**: 更新插件 README 和用户手册

---

## 💡 开发提示

- **日志调试**: 检查浏览器控制台和 Rust CLI 输出
- **Feature Flags**: 修改 `Cargo.toml` features 控制编译内容
- **环境变量**: JavaScript 通过 `process.env.EAGLE_PLUGIN` 与 Rust 通信
- **共享代码**: 编解码器和辅助功能 100% 共享，仅 AI 模块独立
