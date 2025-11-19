# cli_convert.rs 集成指南

## 🎯 目标
将cli_convert.rs (1,172行完整实现) 集成到pixly_converter_cli.rs

## 📊 当前状态
- **pixly_converter_cli.rs**: 1,498行，包含Convert命令的内联实现
- **cli_convert.rs**: 1,172行，完整的转换命令模块
- **问题**: 功能重复，cli_convert未被使用

## 🔍 集成步骤

### Step 1: 分析cli_convert.rs的API
```bash
grep "pub fn\|pub struct" src/cli_convert.rs | head -20
```

### Step 2: 在pixly_converter_cli.rs中导入
```rust
use pixly_kernel::cli_convert::{handle_convert_command, ConvertOptions};
```

### Step 3: 重构Convert命令处理
将Commands::Convert的内联实现替换为cli_convert调用

### Step 4: 测试验证
```bash
cargo build --release
./target/release/pixly-converter convert test.png test.webp
```

## ⚠️ 注意事项
1. 保持向后兼容
2. 保留所有CLI参数
3. 确保错误处理一致
4. 测试所有转换场景

## 📈 预期收益
- 删除重复代码 ~500行
- 统一转换逻辑
- 更易维护

---
**状态**: 待执行
**优先级**: 🔴 最高
**预计时间**: 2-3小时
