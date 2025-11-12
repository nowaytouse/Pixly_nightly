/**
 * Detect Command - Magika AI 文件类型检测
 * 从 cli/commands.rs 拆分 (~173行)
 * 
 * 负责:
 * - AI 驱动的文件类型检测
 * - 伪装文件识别
 * - 安全验证
 * - JSON/人类可读输出
 */

use pixly_converter::converter::magika_detector::MagikaDetector;
use std::path::Path;

/// 处理 detect 命令
pub fn handle(args: &[String]) {
    if args.is_empty() {
        eprintln!("❌ Error: Missing file path");
        eprintln!("   Usage: pixly-rust detect <file> [--json] [--security]");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --json      Output in JSON format");
        eprintln!("  --security  Perform security validation");
        eprintln!("  --verbose   Show detailed information");
        return;
    }
    
    let file_path = &args[0];
    let path = Path::new(file_path);
    
    if !path.exists() {
        eprintln!("❌ Error: File not found: {}", file_path);
        return;
    }
    
    // 解析参数
    let mut json_output = false;
    let mut security_check = false;
    let mut verbose = false;
    
    for arg in &args[1..] {
        match arg.as_str() {
            "--json" => json_output = true,
            "--security" => security_check = true,
            "--verbose" | "-v" => verbose = true,
            _ => {
                eprintln!("⚠️  Unknown option: {}", arg);
            }
        }
    }
    
    // 创建检测器
    let detector = MagikaDetector::with_defaults();
    
    // 执行检测
    match detector.detect_file_type(path) {
        Ok(detection) => {
            if json_output {
                output_json(&detection, file_path);
            } else {
                output_human_readable(&detection, file_path, path, &detector, verbose);
            }
            
            // 安全验证
            if security_check {
                perform_security_check(path, &detector, json_output);
            }
            
            if !json_output {
                println!();
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            }
        }
        Err(e) => {
            eprintln!("❌ Detection failed: {}", e);
            std::process::exit(1);
        }
    }
}

/// JSON 格式输出
fn output_json(detection: &pixly_converter::converter::magika_detector::FileTypeDetection, file_path: &str) {
    let json = serde_json::json!({
        "file": file_path,
        "detected_type": detection.detected_type,
        "confidence": detection.confidence,
        "is_high_confidence": detection.is_high_confidence,
        "mime_type": detection.mime_type,
        "description": detection.description,
        "is_binary": detection.is_binary,
    });
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

/// 人类可读格式输出
fn output_human_readable(
    detection: &pixly_converter::converter::magika_detector::FileTypeDetection,
    file_path: &str,
    path: &Path,
    detector: &MagikaDetector,
    verbose: bool,
) {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔍 Magika AI File Type Detection");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("📁 File: {}", file_path);
    println!("🎯 Detected Type: {}", detection.detected_type);
    println!("📊 Confidence: {:.1}%", detection.confidence * 100.0);
    
    if detection.is_high_confidence {
        println!("✅ High Confidence Detection");
    } else {
        println!("⚠️  Low Confidence Detection");
    }
    
    if let Some(mime) = &detection.mime_type {
        println!("🏷️  MIME Type: {}", mime);
    }
    
    if let Some(desc) = &detection.description {
        println!("📝 Description: {}", desc);
    }
    
    println!("📦 Binary File: {}", if detection.is_binary { "Yes" } else { "No" });
    
    if verbose {
        output_verbose_info(path, detector, detection);
    }
}

/// 详细信息输出
fn output_verbose_info(
    path: &Path,
    detector: &MagikaDetector,
    detection: &pixly_converter::converter::magika_detector::FileTypeDetection,
) {
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Detailed Information");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // 文件信息
    if let Ok(metadata) = std::fs::metadata(path) {
        println!("📏 File Size: {} bytes", metadata.len());
        println!("🕐 Modified: {:?}", metadata.modified().ok());
    }
    
    // 扩展名信息
    if let Some(ext) = path.extension() {
        println!("🔤 Extension: .{}", ext.to_string_lossy());
        
        let ext_str = ext.to_str().unwrap_or("");
        if detector.extension_matches_type(ext_str, &detection.detected_type) {
            println!("✅ Extension matches detected type");
        } else {
            println!("⚠️  Extension does NOT match detected type");
        }
    } else {
        println!("🔤 Extension: None");
    }
}

/// 执行安全检查
fn perform_security_check(path: &Path, detector: &MagikaDetector, json_output: bool) {
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔒 Security Validation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let expected_ext = path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());
    
    match detector.validate_security(path, expected_ext.as_deref()) {
        Ok(validation) => {
            if json_output {
                let json = serde_json::json!({
                    "is_safe": validation.is_safe,
                    "type_match": validation.type_match,
                    "detected_type": validation.detected_type,
                    "is_suspicious": validation.is_suspicious,
                    "warnings": validation.warnings,
                });
                println!("{}", serde_json::to_string_pretty(&json).unwrap());
            } else {
                if validation.is_safe {
                    println!("✅ File appears safe");
                } else {
                    println!("⚠️  File may be suspicious");
                }
                
                if validation.type_match {
                    println!("✅ Type matches extension");
                } else {
                    println!("⚠️  Type does NOT match extension");
                }
                
                if validation.is_suspicious {
                    println!("🚨 SUSPICIOUS FILE DETECTED");
                }
                
                if !validation.warnings.is_empty() {
                    println!();
                    println!("⚠️  Warnings:");
                    for warning in &validation.warnings {
                        println!("   • {}", warning);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Security validation failed: {}", e);
        }
    }
}
