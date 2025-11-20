#!/bin/bash
# 🔥 Phase 40.8: 文件夹结构重构脚本
# 根据PROJECT_QUALITY_MANIFESTO.md原则，整理项目结构

set -e

echo "🔄 Phase 40.8: Project Structure Refactoring"
echo "============================================="
echo ""

# 1. 移动Rust内核到core/rust
echo "📦 Step 1: Moving Rust kernel to core/rust..."
if [ -d "pixly-rust" ] && [ ! -L "pixly-rust" ]; then
    if [ ! -d "core/rust" ]; then
        mkdir -p core/rust
    fi
    
    # 复制而不是移动（保持兼容）
    cp -r pixly-rust/* core/rust/ 2>/dev/null || true
    
    # 创建符号链接保持向后兼容
    if [ ! -L "pixly-rust" ]; then
        mv pixly-rust pixly-rust.backup
        ln -s core/rust pixly-rust
        echo "  ✅ Created symlink: pixly-rust -> core/rust"
    fi
fi

# 2. 整理GO AI核心（保留AI相关）
echo ""
echo "🤖 Step 2: Organizing GO AI core..."
mkdir -p core/go/ai

# 保留AI相关模块
AI_MODULES=(
    "pkg/ai"
    "pkg/predictor" 
    "pkg/quality"
    "pkg/knowledge"
)

for module in "${AI_MODULES[@]}"; do
    if [ -d "$module" ]; then
        target="core/go/$(basename $module)"
        if [ ! -d "$target" ]; then
            cp -r "$module" "$target"
            echo "  ✅ Preserved: $module -> $target"
        fi
    fi
done

# 3. 整理Plugin到core/plugin
echo ""
echo "🔌 Step 3: Organizing Plugin to core/plugin..."
if [ -d "plugin" ] && [ ! -L "plugin" ]; then
    if [ ! -d "core/plugin" ]; then
        mkdir -p core/plugin
    fi
    
    # 复制plugin内容
    cp -r plugin/* core/plugin/ 2>/dev/null || true
    
    # 创建符号链接保持向后兼容
    if [ ! -L "plugin" ]; then
        mv plugin plugin.backup
        ln -s core/plugin plugin
        echo "  ✅ Created symlink: plugin -> core/plugin"
    fi
fi

# 4. 标记待删除的废弃模块
echo ""
echo "🗑️  Step 4: Identifying deprecated modules..."

DEPRECATED_MODULES=(
    "pkg/cache"
    "pkg/checkpoint"
    "pkg/concurrency"
    "pkg/config"
    "pkg/converter"
    "pkg/deduplicator"
    "pkg/errorhandling"
    "pkg/i18n"
    "pkg/metadata"
    "pkg/utils"
    "pkg/validation"
    "pkg/xmp"
)

echo "  Creating deprecated/ folder for review..."
mkdir -p deprecated

for module in "${DEPRECATED_MODULES[@]}"; do
    if [ -d "$module" ]; then
        target="deprecated/$(basename $module)"
        if [ ! -d "$target" ]; then
            mv "$module" "$target"
            echo "  📦 Moved to deprecated: $module"
        fi
    fi
done

# 5. 创建新的cmd结构
echo ""
echo "📁 Step 5: Organizing cmd structure..."
if [ -d "cmd" ]; then
    mkdir -p core/go/cmd
    cp -r cmd/* core/go/cmd/ 2>/dev/null || true
    echo "  ✅ Organized cmd to core/go/cmd"
fi

# 6. 创建README说明新结构
echo ""
echo "📝 Step 6: Creating structure documentation..."

cat > core/README.md << 'EOF'
# Pixly Core Structure

**Date**: 2025-11-06  
**Phase**: 40.8 Structure Refactoring

## 📁 New Structure

```
core/
├── rust/           # 🦀 Rust转换内核
│   ├── src/
│   │   ├── converter/   # 转换策略
│   │   ├── cli/         # CLI接口
│   │   └── ...
│   └── Cargo.toml
│
├── go/             # 🤖 GO AI服务
│   ├── ai/         # AI核心
│   ├── predictor/  # 参数预测
│   ├── quality/    # 质量评估
│   └── cmd/        # GO命令入口
│
└── plugin/         # 🔌 Eagle插件
    ├── js/         # JavaScript模块
    ├── css/        # 样式文件
    └── index.html  # 插件UI
```

## 🎯 Architecture Principles

### Rust Kernel (core/rust)
- **职责**: 图像/视频转换、文件处理、元数据管理
- **禁止**: 参数决策（由GO AI负责）
- **接口**: CLI、FFI、HTTP（可选）

### GO AI Service (core/go)
- **职责**: AI参数预测、格式推荐、质量评估
- **禁止**: 文件处理、转换逻辑
- **接口**: gRPC (port 50051)、HTTP (port 50052)

### Plugin (core/plugin)
- **职责**: UI交互、Eagle集成
- **禁止**: 转换逻辑、参数计算
- **接口**: Eagle Plugin API

## 🔄 Migration Status

- ✅ Rust kernel organized
- ✅ GO AI core preserved
- ✅ Plugin structure maintained
- ✅ Deprecated modules moved
- ⏳ Symlinks created for compatibility

## 🗑️ Deprecated Modules

Moved to `deprecated/` for review:
- cache, checkpoint, concurrency
- config, converter, deduplicator
- errorhandling, i18n, metadata
- utils, validation, xmp

**Reason**: Functionality duplicated in Rust kernel or no longer needed.

## 📚 Related Documents

- [PROJECT_QUALITY_MANIFESTO.md](../newdocs/PROJECT_QUALITY_MANIFESTO.md)
- [PHASE_40.8_SUMMARY.md](../newdocs/PHASE_40.8_SUMMARY.md)
EOF

echo "  ✅ Created core/README.md"

echo ""
echo "============================================="
echo "✅ Structure refactoring complete!"
echo ""
echo "📊 Summary:"
echo "  - Rust kernel: core/rust"
echo "  - GO AI core: core/go"  
echo "  - Plugin: core/plugin"
echo "  - Deprecated: deprecated/"
echo ""
echo "⚠️  Note: Original folders are preserved as .backup"
echo "   You can safely delete them after verification."
echo ""