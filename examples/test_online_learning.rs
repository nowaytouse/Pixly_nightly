// examples/test_online_learning.rs
//! 测试在线学习功能

use pixly_kernel::online_learning::OnlineLearner;
use pixly_kernel::reward_calculator::ConversionResult;
use std::path::PathBuf;

fn main() {
    println!("🎓 Testing Online Learning Module");
    println!("{}", "=".repeat(60));
    
    // 创建在线学习器
    let learner = OnlineLearner::new(
        PathBuf::from("models/ppo/ppo_best_webp.pth"),
        5  // 每5个经验更新一次
    );
    
    println!("✅ Created online learner");
    println!("   Model: models/ppo/ppo_best_webp.pth");
    println!("   Update interval: 5 experiences");
    
    // 模拟记录几次转换经验
    for i in 1..=3 {
        println!("\n📝 Recording experience {}", i);
        
        let features = vec![0.5; 128];
        let result = ConversionResult {
            original_size: 1000000,
            output_size: 500000 + i * 10000,
            ssim: 0.95 + (i as f64 * 0.01),
            processing_time: 2.0,
        };
        
        learner.record_conversion(features, 80, 6, result).unwrap();
        println!("   Buffer size: {}", learner.buffer_size());
    }
    
    println!("\n📊 Final buffer size: {}", learner.buffer_size());
    println!("\n✅ Online learning test complete!");
}
