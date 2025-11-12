# 项目重组计划（Phase 41）

## 🎯 用户要求

1. **编译二进制文件** → 统一到 `bin/` 文件夹
2. **核心代码** (Go + Rust) → `core/` 文件夹
3. **模块化代码** → `pkg/` 文件夹（Go已有）
4. **未使用文件** → `_archive/` 垃圾文件夹

---

## 📁 当前结构

```
Pixly_Nightly/
├── cmd/                  # Go命令行工具
│   └── ai-service/
├── pkg/                  # Go模块化代码
│   └── ai/
├── pixly-rust/           # Rust核心
│   ├── src/
│   └── target/release/   # 二进制在这里！
├── plugin/               # Eagle插件
│   └── js/
│       └── plugin-modules/
└── newdocs/              # 新文档
```

## 🎨 目标结构

```
Pixly_Nightly/
├── bin/                  # ✅ 统一二进制文件夹
│   ├── pixly-rust        # Rust CLI
│   ├── pixly-ai-service  # Go AI服务
│   └── ...
├── core/                 # ✅ 核心代码
│   ├── rust/             # Rust转换核心
│   │   ├── src/
│   │   └── Cargo.toml
│   └── go/               # Go AI核心
│       ├── ai-service/
│       └── pkg/
├── plugin/               # Eagle插件（保持不变）
│   └── js/
│       └── plugin-modules/
├── newdocs/              # 文档（保持不变）
├── _archive/             # ✅ 未使用文件
│   ├── old-scripts/
│   └── deprecated/
└── README.md
```

---

## 🚀 重组步骤

### Step 1: 创建新目录结构
```bash
mkdir -p bin core/rust core/go _archive
```

### Step 2: 移动核心代码
```bash
# Rust核心
mv pixly-rust/* core/rust/

# Go核心
mv cmd core/go/cmd
mv pkg core/go/pkg

# 保留Go配置文件
cp go.mod go.sum core/go/
```

### Step 3: 统一二进制文件
```bash
# 编译Rust
cd core/rust
cargo build --release
cp target/release/pixly-rust ../../bin/

# 编译Go AI服务
cd ../go/cmd/ai-service
go build -o ../../../../bin/pixly-ai-service
```

### Step 4: 归档未使用文件
```bash
# 查找并移动未使用的脚本
find . -name "*.sh" -not -path "./plugin/*" -exec mv {} _archive/ \;

# 移动旧文档（如果存在）
find . -name "*.md" -not -path "./newdocs/*" -not -name "README.md" -exec mv {} _archive/ \;
```

### Step 5: 更新路径引用
```bash
# plugin/js/plugin-modules/path-resolver.js
# 更新 Rust core 路径: 
# FROM: /pixly-rust/target/release/pixly-rust
# TO:   /bin/pixly-rust

# 更新 AI service 路径:
# FROM: /cmd/ai-service
# TO:   /core/go/cmd/ai-service
```

### Step 6: 更新构建脚本
```bash
# 创建统一构建脚本
cat > build.sh << 'EOF'
#!/bin/bash
echo "🔨 Building Pixly..."

# Build Rust core
echo "📦 Building Rust core..."
cd core/rust
cargo build --release
cp target/release/pixly-rust ../../bin/
cd ../..

# Build Go AI service
echo "🧠 Building Go AI service..."
cd core/go/cmd/ai-service
go build -o ../../../../bin/pixly-ai-service
cd ../../../..

echo "✅ Build complete! Binaries in bin/"
EOF
chmod +x build.sh
```

---

## ⚠️ 风险与注意事项

1. **路径破坏**：
   - Eagle插件的path-resolver需要更新
   - 所有硬编码路径需要修改
   
2. **Git历史**：
   - 大量文件移动会影响Git blame
   - 建议先打tag保存当前状态

3. **依赖关系**：
   - Go的`go.mod`需要更新module路径
   - Rust的Cargo.toml可能需要调整

---

## ✅ 验证清单

- [ ] Rust CLI可以从`bin/`执行
- [ ] Go AI服务可以从`bin/`执行
- [ ] Eagle插件能找到Rust CLI
- [ ] AI服务路径正确
- [ ] 所有测试通过
- [ ] 文档路径更新

---

**执行时机**: Phase 40.6测试成功后
**预计时间**: 1-2小时
**回滚方案**: Git reset --hard
