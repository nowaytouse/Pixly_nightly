/// 测试批量转换进度回调API
use pixly_kernel::{BatchConverter, BatchConverterConfig};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn main() -> anyhow::Result<()> {
    println!("🧪 测试批量转换进度回调API\n");
    
    // 创建测试文件列表
    let test_files = vec![
        PathBuf::from("data/test_images/test.jpg"),
    ];
    
    // 进度计数器
    let progress_count = Arc::new(Mutex::new(0));
    let progress_count_clone = progress_count.clone();
    
    // 配置批量转换器，带进度回调
    let mut config = BatchConverterConfig::default();
    config.conversion_config.quality = 85;
    config.conversion_config.speed = 4;
    config.max_parallel = 2;
    config.overwrite = true;
    config.show_progress = false; // 关闭默认进度显示
    
    let config = config.with_progress_callback(move |completed, total, filename| {
        let mut count = progress_count_clone.lock().unwrap();
        *count += 1;
        println!("📊 进度回调 #{}: [{}/{}] 正在处理: {}", 
            count, completed, total, filename);
    });
    
    // 创建转换器
    let converter = BatchConverter::new(config);
    
    // 执行批量转换
    println!("🔄 开始批量转换...\n");
    let result = converter.convert_batch(
        test_files,
        &PathBuf::from("test_output"),
        "webp",
    )?;
    
    // 验证回调被调用
    let callback_count = *progress_count.lock().unwrap();
    println!("\n✅ 转换完成!");
    println!("   成功: {}/{}", result.success, result.total);
    println!("   回调次数: {}", callback_count);
    
    assert!(callback_count > 0, "进度回调应该被调用");
    
    Ok(())
}
