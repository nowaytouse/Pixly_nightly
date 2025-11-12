# 8层验证器增强方案

## 📊 现状分析

### 已有实现
- **JS版本**: `archive/experimental/plugin_modules/validator.js` (8层框架，简化实现)
- **Go版本**: `deprecated/archive/standalone_tools/` (完整的8层验证器)
- **Rust版本**: `core/rust/src/converter/validation.rs` (6层验证系统)

###现有Rust验证层级
1. **Level 1: Basic** - 文件存在性
2. **Level 2: Format** - 格式验证
3. **Level 3: Integrity** - 文件完整性
4. **Level 4: Deep** - 深度解码
5. **Level 5: Security** - 安全检查
6. **Level 6: AntiCheat** - 防作弊

### 缺失功能
- ❌ **Level 7**: 尺寸验证（输入输出尺寸一致性）
- ❌ **Level 8**: 质量验证（SSIM/PSNR）
- ❌ 元数据保留验证
- ❌ 转换前后对比
- ❌ 批量验证优化

---

## 🎯 增强目标

### 8层验证体系

```
Level 1: 文件存在性检查
  ✓ 输入文件存在
  ✓ 输出文件生成
  ✓ 文件可读写

Level 2: 格式验证
  ✓ 输入格式检测（magika）
  ✓ 输出格式确认
  ✓ 扩展名一致性

Level 3: 文件完整性
  ✓ 文件头验证
  ✓ 文件尾验证
  ✓ 大小合理性

Level 4: 深度解码
  ✓ 输入文件可解码
  ✓ 输出文件可解码
  ✓ 帧数验证（动画）

Level 5: 安全检查
  ✓ 文件大小限制
  ✓ 恶意内容检测
  ✓ 资源消耗检查

Level 6: 防作弊
  ✓ 真实转换验证
  ✓ 非复制检测
  ✓ 时间戳验证

Level 7: 尺寸验证 (新增)
  ✓ 宽度一致性
  ✓ 高度一致性
  ✓ 纵横比保持

Level 8: 质量验证 (新增)
  ✓ SSIM相似度 (可选)
  ✓ 视觉质量检查
  ✓ 元数据保留
```

---

## 🔧 实现方案

### 1. 增强验证级别枚举

```rust
pub enum ValidationLevel {
    Basic = 1,        // 存在性
    Format = 2,       // 格式
    Integrity = 3,    // 完整性
    Deep = 4,         // 解码
    Security = 5,     // 安全
    AntiCheat = 6,    // 防作弊
    Dimensions = 7,   // 尺寸 (新增)
    Quality = 8,      // 质量 (新增)
}
```

### 2. 增强验证结果

```rust
pub struct ValidationResult {
    pub passed: bool,
    pub level: u8,
    pub detected_format: Option<String>,
    pub file_size: u64,
    pub dimensions: Option<(u32, u32)>,
    pub is_animated: Option<bool>,
    
    // 新增字段
    pub input_dimensions: Option<(u32, u32)>,
    pub output_dimensions: Option<(u32, u32)>,
    pub dimensions_match: Option<bool>,
    pub ssim_score: Option<f64>,
    pub quality_acceptable: Option<bool>,
    pub metadata_preserved: Option<bool>,
    
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
```

### 3. 实现Level 7: 尺寸验证

```rust
impl FileValidator {
    pub fn validate_dimensions(
        &self,
        input_path: &Path,
        output_path: &Path
    ) -> Result<bool> {
        let input_dims = self.get_image_dimensions(input_path)?;
        let output_dims = self.get_image_dimensions(output_path)?;
        
        // 宽高必须一致
        let match_ok = input_dims.0 == output_dims.0 
                    && input_dims.1 == output_dims.1;
        
        if !match_ok {
            bail!(
                "Dimension mismatch: {}x{} -> {}x{}",
                input_dims.0, input_dims.1,
                output_dims.0, output_dims.1
            );
        }
        
        Ok(true)
    }
}
```

### 4. 实现Level 8: 质量验证

```rust
impl FileValidator {
    pub fn validate_quality(
        &self,
        input_path: &Path,
        output_path: &Path,
        min_ssim: f64  // 默认0.95
    ) -> Result<(bool, f64)> {
        // 可选：使用image-compare crate
        // 或调用外部工具(ffmpeg, imagemagick)
        
        // 简化版：检查元数据保留
        let metadata_ok = self.check_metadata_preserved(
            input_path,
            output_path
        )?;
        
        // 简化版：文件大小合理性
        let size_ok = self.check_size_reasonable(
            input_path,
            output_path
        )?;
        
        let score = if metadata_ok && size_ok {
            0.98  // 假设质量优秀
        } else {
            0.80  // 质量一般
        };
        
        Ok((score >= min_ssim, score))
    }
}
```

---

## 📋 实现步骤

### Step 1: 增强枚举和结构
- [ ] 添加Level 7和Level 8到ValidationLevel
- [ ] 扩展ValidationResult结构
- [ ] 更新构造函数和默认值

### Step 2: 实现Level 7 (尺寸验证)
- [ ] get_image_dimensions() 方法
- [ ] validate_dimensions() 完整实现
- [ ] 动画帧数验证
- [ ] 测试用例

### Step 3: 实现Level 8 (质量验证)
- [ ] check_metadata_preserved() 方法
- [ ] validate_quality() 完整实现
- [ ] SSIM计算（可选，使用image-compare）
- [ ] 测试用例

### Step 4: 整合到转换流程
- [ ] 更新batch_processor.rs
- [ ] 更新file_manager.rs
- [ ] CLI参数支持（--validation-level）
- [ ] 文档更新

### Step 5: 清理旧代码
- [ ] 标记JS validator为已废弃
- [ ] 标记Go validator为已废弃
- [ ] 文档说明迁移完成
- [ ] 删除旧代码

---

## 🧪 测试计划

### 单元测试
```rust
#[test]
fn test_level7_dimensions() {
    let validator = FileValidator::new();
    validator.set_level(ValidationLevel::Dimensions);
    
    let result = validator.validate_conversion(
        "tests/input.jpg",
        "tests/output.jxl"
    );
    
    assert!(result.is_ok());
    assert!(result.unwrap().dimensions_match.unwrap());
}

#[test]
fn test_level8_quality() {
    let validator = FileValidator::new();
    validator.set_level(ValidationLevel::Quality);
    
    let result = validator.validate_conversion(
        "tests/input.jpg",
        "tests/output.jxl"
    );
    
    assert!(result.is_ok());
    assert!(result.unwrap().quality_acceptable.unwrap());
}
```

### 集成测试
- 批量转换场景
- 动画文件场景
- 错误处理场景
- 性能测试

---

## 📊 性能优化

### 缓存机制
```rust
pub struct CachedValidator {
    cache: HashMap<PathBuf, ValidationResult>,
    validator: FileValidator,
}
```

### 并行验证
```rust
pub fn validate_batch(
    &self,
    pairs: Vec<(PathBuf, PathBuf)>
) -> Vec<ValidationResult> {
    pairs.par_iter()
        .map(|(input, output)| {
            self.validate_conversion(input, output)
        })
        .collect()
}
```

---

## 🎯 成功标准

- ✅ 8层验证全部实现
- ✅ 单元测试覆盖率 > 90%
- ✅ 性能损失 < 5%
- ✅ 旧代码安全删除
- ✅ 文档完整更新

---

## 📚 参考资料

- JS版本: `archive/experimental/plugin_modules/validator.js`
- Go版本: `deprecated/archive/standalone_tools/`
- 现有Rust: `core/rust/src/converter/validation.rs`
- image-compare: https://crates.io/crates/image-compare
- SSIM算法: https://en.wikipedia.org/wiki/Structural_similarity

---

**预计工作量**: 2-3小时
**优先级**: 高（输入输出可靠性关键）
