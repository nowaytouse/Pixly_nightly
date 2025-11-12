# 归档和废弃文件清理计划

**日期**: 2025-11-12  
**预计节省空间**: ~2.0GB

## 🎯 清理目标

1. 删除个人资料和无关项目（1.8GB+）
2. 整合分散的废弃代码目录
3. 保留有价值的技术参考
4. 创建完整备份后安全删除

## 📊 空间分析

| 目录 | 当前大小 | 保留/删除 | 理由 |
|------|---------|----------|------|
| `@reference/learn.library/` | 1.8G | ❌ 删除 | 个人Eagle图片库，无技术价值 |
| `@reference/sharp-main/` | 48M | ❌ 删除 | Node.js项目，与Rust架构无关 |
| `@reference/apps.apple.com-main/` | 14M | ❌ 删除 | 网页参考，无技术价值 |
| `@reference/squoosh-dev/` | 43M | ⚠️ 部分保留 | 仅保留codecs目录 |
| `@reference/rimage-main/` | 552K | ✅ 保留 | Rust图像处理参考 |
| `@reference/data/` | 87M | ✅ 保留 | 测试数据有价值 |
| `@archive/docs_old/` | ~50M | ❌ 删除 | 已整合到核心文档 |
| `@archive/deprecated_phase47_archived/` | ~50M | ❌ 删除 | 废弃代码无复用价值 |
| `core/@deprecated/` | 1.9M | ❌ 删除 | Go代码已废弃 |
| `core/go/_deprecated_orphan_files/` | 104K | ❌ 删除 | 孤儿文件 |

**预计节省**: ~2.0GB

## 🛠️ 执行步骤

### Step 1: 创建完整备份

```bash
# 创建归档备份（以防万一）
cd /Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly
tar -czf ~/Desktop/pixly_archive_backup_20251112.tar.gz \
    @archive \
    @reference \
    core/@deprecated \
    core/go/_deprecated_orphan_files

echo "备份已创建: ~/Desktop/pixly_archive_backup_20251112.tar.gz"
```

### Step 2: 清理@reference目录

```bash
# 删除大容量低价值内容
rm -rf @reference/learn.library          # 1.8G
rm -rf @reference/sharp-main             # 48M
rm -rf @reference/apps.apple.com-main    # 14M

# 整理squoosh，仅保留codecs
mkdir -p @reference/squoosh_codecs
cp -r @reference/squoosh-dev/codecs @reference/squoosh_codecs/
rm -rf @reference/squoosh-dev

# 重命名保留的测试数据
mv @reference/data @reference/test_samples

echo "✅ @reference目录清理完成"
```

### Step 3: 整合废弃代码

```bash
# 合并所有废弃Go代码到archive
mkdir -p @archive/deprecated_code_2025_11
mv core/@deprecated/go_ai_service_2025_11_11 @archive/deprecated_code_2025_11/
mv core/go/_deprecated_orphan_files @archive/deprecated_code_2025_11/

# 删除空目录
rmdir core/@deprecated/Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/core/@deprecated 2>/dev/null || true
rmdir core/@deprecated 2>/dev/null || true

echo "✅ 废弃代码整合完成"
```

### Step 4: 压缩历史文档

```bash
cd @archive

# 压缩旧文档为单一文件
tar -czf historical_docs_phase46-47.tar.gz \
    phase_46_docs \
    docs_old \
    deprecated_phase47_archived

# 删除原目录
rm -rf phase_46_docs
rm -rf docs_old  
rm -rf deprecated_phase47_archived

echo "✅ 历史文档已压缩"
```

### Step 5: 创建新的README

```bash
cat > @archive/README.md << 'EOF'
# Pixly 归档目录

## 目录说明

### 保留的归档
- `historical_docs_phase46-47.tar.gz` - Phase 46-47所有历史文档
- `deprecated_code_2025_11/` - 废弃的Go AI服务代码（2025年11月）
- `QUICK_VALIDATION_TEST.md` - 快速验证测试文档

### 完整备份
如需完整历史备份，请查看：
`~/Desktop/pixly_archive_backup_20251112.tar.gz`

## 参考项目

已移至 `@reference/`:
- `rimage-main/` - Rust图像处理参考实现
- `test_samples/` - 测试数据样本
- `squoosh_codecs/` - 编解码器参考
- `xl-converter-unstable/` - XL转换器参考
- `pio-master/` - PIO参考

## 清理记录

**日期**: 2025-11-12  
**清理空间**: ~2.0GB  
**删除内容**:
- learn.library/ (1.8G个人图片库)
- sharp-main/ (48M Node.js项目)
- apps.apple.com-main/ (14M)
- 重复的Phase文档 (~50M)
- 废弃的standalone工具 (~50M)

**保留原则**: 仅保留技术参考价值的内容
EOF

echo "✅ 新README已创建"
```

### Step 6: Git提交

```bash
git add -A
git commit -m "清理: 归档整合与大容量文件删除

🗑️ 删除内容:
- learn.library/ (1.8G个人图片库)
- sharp-main/, apps.apple.com-main/ (62M无关项目)
- 重复历史文档和废弃代码 (~100M)

📦 整合内容:
- 所有废弃Go代码合并到 @archive/deprecated_code_2025_11/
- Phase 46-47文档压缩为 historical_docs_phase46-47.tar.gz
- @reference仅保留技术参考项目

✅ 保留内容:
- rimage-main/ (Rust参考)
- test_samples/ (测试数据)
- squoosh_codecs/ (编解码器参考)

💾 节省空间: ~2.0GB
🔒 完整备份: ~/Desktop/pixly_archive_backup_20251112.tar.gz
"
```

## ⚠️ 注意事项

1. **备份优先**: 执行Step 1创建备份后，才能继续后续步骤
2. **确认备份**: 验证备份文件完整性
3. **逐步执行**: 建议分步执行，每步后检查结果
4. **可回滚**: 如有问题，可从备份恢复

## 📋 检查清单

- [ ] Step 1: 创建完整备份
- [ ] 验证备份文件大小合理（预计~2GB）
- [ ] Step 2: 清理@reference目录
- [ ] Step 3: 整合废弃代码
- [ ] Step 4: 压缩历史文档
- [ ] Step 5: 创建新README
- [ ] Step 6: Git提交
- [ ] 验证项目功能正常
- [ ] （可选）7天后删除桌面备份

## 🎯 预期结果

**清理前**:
- @archive: 109M
- @reference: 2.0G
- core废弃文件: 2M

**清理后**:
- @archive: ~5M (压缩tar.gz)
- @reference: ~90M (仅技术参考)
- core废弃文件: 0

**总节省**: ~2.0GB ✨
