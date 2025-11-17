// 🔍 透明日志演示
// 展示详细的操作日志

use pixly_kernel::transparent_logger::{TransparentLogger, OperationTracker, LogLevel};
use std::thread;
use std::time::Duration;

fn main() {
    println!("\n🔍 Transparent Logging System Demo");
    println!("{}", "=".repeat(80));
    
    // 创建日志记录器
    let logger = TransparentLogger::new();
    
    // 演示1: 基本日志
    logger.log_header("基本日志功能");
    logger.log(LogLevel::Info, "这是一条信息日志");
    logger.log(LogLevel::Detail, "这是一条详细日志");
    logger.log(LogLevel::Warning, "这是一条警告日志");
    logger.log(LogLevel::Error, "这是一条错误日志");
    logger.log(LogLevel::Debug, "这是一条调试日志");
    
    // 演示2: 带详细信息的日志
    logger.log_separator();
    logger.log_header("详细信息日志");
    logger.log_with_details(
        LogLevel::Info,
        "文件转换配置",
        &[
            ("输入文件", "image.jpg".to_string()),
            ("输出文件", "image.webp".to_string()),
            ("目标格式", "WebP".to_string()),
            ("质量", "85".to_string()),
            ("使用PPO", "是".to_string()),
        ]
    );
    
    // 演示3: 操作追踪
    logger.log_separator();
    logger.log_header("操作追踪");
    {
        let tracker = OperationTracker::start(logger.clone(), "图像转换");
        
        tracker.log_step("检查输入文件");
        thread::sleep(Duration::from_millis(100));
        
        tracker.log_step("加载PPO模型");
        tracker.log_details(&[
            ("模型文件", "ppo_model.json".to_string()),
            ("样本数", "1479".to_string()),
        ]);
        thread::sleep(Duration::from_millis(100));
        
        tracker.log_step("预测最优参数");
        tracker.log_details(&[
            ("预测质量", "85".to_string()),
            ("预测速度", "4".to_string()),
        ]);
        thread::sleep(Duration::from_millis(100));
        
        tracker.log_step("执行转换");
        thread::sleep(Duration::from_millis(200));
        
        tracker.log_step("评估质量");
        tracker.log_details(&[
            ("SSIM", "0.9520".to_string()),
            ("PSNR", "38.20 dB".to_string()),
            ("综合评分", "92.50/100".to_string()),
        ]);
        thread::sleep(Duration::from_millis(100));
        
        // tracker会在drop时自动记录完成时间
    }
    
    // 演示4: 嵌套操作
    logger.log_separator();
    logger.log_header("嵌套操作追踪");
    {
        let tracker = OperationTracker::start(logger.clone(), "批量转换");
        
        tracker.log_step("扫描输入目录");
        tracker.log_details(&[
            ("找到文件", "10".to_string()),
            ("总大小", "25.6 MB".to_string()),
        ]);
        
        // 模拟处理每个文件
        for i in 1..=3 {
            let file_tracker = OperationTracker::start(
                logger.clone(),
                &format!("Processing file {}/10", i)
            );
            
            file_tracker.log_step("分析图像特征");
            thread::sleep(Duration::from_millis(50));
            
            file_tracker.log_step("PPO参数预测");
            thread::sleep(Duration::from_millis(50));
            
            file_tracker.log_step("格式转换");
            thread::sleep(Duration::from_millis(100));
            
            file_tracker.log_step("质量验证");
            thread::sleep(Duration::from_millis(50));
        }
        
        tracker.log_step("生成转换报告");
    }
    
    // 演示5: 警告和错误处理
    logger.log_separator();
    logger.log_header("警告和错误处理");
    {
        let tracker = OperationTracker::start(logger.clone(), "转换带问题的文件");
        
        tracker.log_step("检查文件格式");
        tracker.log_warning("文件格式可能不受支持");
        
        tracker.log_step("尝试转换");
        tracker.log_error("转换失败: 编码器不可用");
        
        tracker.log_step("回退到默认格式");
        tracker.log_details(&[
            ("原格式", "HEIC".to_string()),
            ("回退格式", "JPEG".to_string()),
        ]);
    }
    
    // 演示6: 完整的转换流程
    logger.log_separator();
    logger.log_header("完整转换流程示例");
    {
        let tracker = OperationTracker::start(logger.clone(), "完整转换流程");
        
        // 1. 初始化
        tracker.log_step("初始化转换引擎");
        tracker.log_details(&[
            ("PPO模型", "已加载".to_string()),
            ("格式支持", "AVIF, JXL, WebP".to_string()),
            ("质量评估", "VMAF已启用".to_string()),
        ]);
        thread::sleep(Duration::from_millis(100));
        
        // 2. 文件分析
        tracker.log_step("分析输入文件");
        tracker.log_details(&[
            ("文件名", "photo.jpg".to_string()),
            ("大小", "2.5 MB".to_string()),
            ("分辨率", "1920x1080".to_string()),
            ("复杂度", "0.65".to_string()),
        ]);
        thread::sleep(Duration::from_millis(100));
        
        // 3. PPO预测
        tracker.log_step("PPO参数预测");
        tracker.log_details(&[
            ("推荐格式", "AVIF".to_string()),
            ("推荐质量", "85".to_string()),
            ("预期压缩率", "45%".to_string()),
            ("置信度", "0.92".to_string()),
        ]);
        thread::sleep(Duration::from_millis(100));
        
        // 4. 格式转换
        tracker.log_step("执行格式转换");
        tracker.log_details(&[
            ("编码器", "libaom-av1".to_string()),
            ("CRF", "30".to_string()),
            ("速度预设", "4".to_string()),
        ]);
        thread::sleep(Duration::from_millis(300));
        
        // 5. 质量评估
        tracker.log_step("评估转换质量");
        tracker.log_details(&[
            ("VMAF", "88.5/100".to_string()),
            ("SSIM", "0.9520".to_string()),
            ("PSNR", "38.20 dB".to_string()),
            ("质量等级", "良好".to_string()),
        ]);
        thread::sleep(Duration::from_millis(100));
        
        // 6. 结果验证
        tracker.log_step("验证转换结果");
        tracker.log_details(&[
            ("输入大小", "2.5 MB".to_string()),
            ("输出大小", "1.1 MB".to_string()),
            ("压缩率", "44%".to_string()),
            ("空间节省", "1.4 MB".to_string()),
        ]);
        
        tracker.logger().log(LogLevel::Info, "✅ 转换成功完成！");
    }
    
    println!("\n{}", "=".repeat(80));
    println!("🎉 Demo completed!");
    println!("{}", "=".repeat(80));
}
