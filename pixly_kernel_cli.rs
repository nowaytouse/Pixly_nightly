use std::io::{self, Read};
use std::process;
use std::env;

use anyhow::Result;
use pixly_kernel::{PredictionRequest, PredictionWithConfidence, UnifiedAIPredictor};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // 检查命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-v" => {
                println!("pixly-kernel v{}", VERSION);
                return;
            }
            "--help" | "-h" => {
                println!("PIXLY Kernel - AI Prediction Engine");
                println!("Version: {}", VERSION);
                println!("\nUsage:");
                println!("  pixly-kernel [OPTIONS]");
                println!("\nOptions:");
                println!("  --version, -v    Show version");
                println!("  --help, -h       Show this help");
                println!("\nInput: JSON PredictionRequest via stdin");
                println!("Output: JSON PredictionWithConfidence via stdout");
                return;
            }
            _ => {
                eprintln!("Unknown option: {}", args[1]);
                eprintln!("Use --help for usage information");
                process::exit(1);
            }
        }
    }

    if let Err(err) = run() {
        eprintln!("pixly-kernel error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    // 读取标准输入中的 JSON 请求
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let input = buf.trim();
    if input.is_empty() {
        anyhow::bail!("expected JSON PredictionRequest on stdin, but input was empty");
    }

    // 反序列化为 PredictionRequest
    let request: PredictionRequest = serde_json::from_str(input)?;

    // 调用统一 AI 预测内核
    let predictor = UnifiedAIPredictor::new();
    let prediction: PredictionWithConfidence = predictor.predict_from_request(&request);

    // 序列化为 JSON 输出
    let json = serde_json::to_string(&prediction)?;
    println!("{}", json);

    Ok(())
}
