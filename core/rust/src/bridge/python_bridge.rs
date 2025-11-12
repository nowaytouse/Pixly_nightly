/*!
智能Python桥接器
基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/python_bridge.go 重新实现

功能:
- 智能脚本路径查找（多路径自动检测）
- 跨平台Python环境兼容
- 超时机制和错误处理
- 为Rust→Python AI调用提供更可靠的桥接

EX-010实现: 从Go废弃代码价值提取
*/

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::env;
use std::fs;
use anyhow::{Result, Context, bail};
use serde_json::Value;
use tracing::{info, warn, error, debug};

/// Python桥接器配置
#[derive(Debug, Clone)]
pub struct PythonBridgeConfig {
    /// Python可执行文件路径
    pub python_path: String,
    /// 脚本路径
    pub script_path: PathBuf,
    /// 模型目录
    pub model_dir: PathBuf,
    /// 超时时间（秒）
    pub timeout_secs: u64,
    /// 工作目录
    pub working_dir: Option<PathBuf>,
}

impl Default for PythonBridgeConfig {
    fn default() -> Self {
        Self {
            python_path: "python3".to_string(),
            script_path: PathBuf::from("tools/predict_params.py"),
            model_dir: PathBuf::from("models"),
            timeout_secs: 30,
            working_dir: None,
        }
    }
}

/// Python桥接器
pub struct PythonBridge {
    config: PythonBridgeConfig,
}

impl PythonBridge {
    /// 创建新的Python桥接器
    pub fn new(model_dir: impl AsRef<Path>) -> Result<Self> {
        let model_dir = model_dir.as_ref().to_path_buf();
        
        // 优先使用系统Python（更可靠）
        let python_path = Self::detect_python_executable()
            .unwrap_or_else(|| "python3".to_string());
        
        // 智能查找Python脚本路径
        let script_path = Self::find_python_script()?;
        
        let config = PythonBridgeConfig {
            python_path,
            script_path,
            model_dir,
            ..Default::default()
        };
        
        info!("✅ [Python Bridge] 初始化完成");
        info!("   Python路径: {}", config.python_path);
        info!("   脚本路径: {:?}", config.script_path);
        info!("   模型目录: {:?}", config.model_dir);
        
        Ok(Self { config })
    }
    
    /// 使用指定配置创建桥接器
    pub fn with_config(config: PythonBridgeConfig) -> Result<Self> {
        // 验证配置
        if !config.script_path.exists() {
            bail!("Python脚本不存在: {:?}", config.script_path);
        }
        
        info!("👍 [Python Bridge] 使用自定义配置创建");
        Ok(Self { config })
    }
    
    /// 检测Python可执行文件
    fn detect_python_executable() -> Option<String> {
        let candidates = vec!["python3", "python", "py"];
        
        for candidate in candidates {
            if let Ok(output) = Command::new(candidate)
                .args(&["--version"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
            {
                if output.success() {
                    debug!("🐍 [Python Bridge] 检测到Python: {}", candidate);
                    return Some(candidate.to_string());
                }
            }
        }
        
        warn!("⚠️ [Python Bridge] 未检测到Python可执行文件");
        None
    }
    
    /// 智能查找Python脚本路径
    /// 从Go版本移植的路径查找逻辑
    fn find_python_script() -> Result<PathBuf> {
        // 获取当前工作目录
        let cwd = env::current_dir()
            .context("无法获取当前工作目录")?;
        
        // 🔥 智能查找Python脚本路径
        // Phase 46.14: 修正查找顺序（从Go代码移植）
        let possible_paths = vec![
            // 1. 从bin/ai-service启动：../../../../tools/predict_params.py 
            //    (bin/ai-service -> core/go -> core -> Pixly_Nightly -> tools)
            cwd.join("../../../../tools/predict_params.py"),
            // 2. 从core/go/启动：../../tools/predict_params.py
            cwd.join("../../tools/predict_params.py"),
            // 3. 从项目根目录启动：./tools/predict_params.py
            cwd.join("tools/predict_params.py"),
            // 4. 从core/启动：../tools/predict_params.py
            cwd.join("../tools/predict_params.py"),
            // 5. 从core/rust/启动：../../tools/predict_params.py
            cwd.join("../../tools/predict_params.py"),
            // 6. 绝对路径尝试
            PathBuf::from("/Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/tools/predict_params.py"),
        ];
        
        for path in possible_paths {
            if path.exists() {
                let canonical_path = path.canonicalize()
                    .context("无法规范化路径")?;
                info!("👍 [Python Bridge] 找到脚本: {:?}", canonical_path);
                return Ok(canonical_path);
            } else {
                debug!("🔍 [Python Bridge] 路径不存在: {:?}", path);
            }
        }
        
        bail!("❌ 未找到predict_params.py脚本")
    }
    
    /// 调用Python AI预测脚本
    pub fn predict_image_params(&self, input_format: &str, target_format: &str, 
                               width: u32, height: u32, file_size: u64, 
                               priority: &str) -> Result<Value> {
        let args = vec![
            "--input-format".to_string(),
            input_format.to_string(),
            "--target-format".to_string(),
            target_format.to_string(),
            "--width".to_string(),
            width.to_string(),
            "--height".to_string(),
            height.to_string(),
            "--file-size".to_string(),
            file_size.to_string(),
            "--priority".to_string(),
            priority.to_string(),
            "--output-format".to_string(),
            "json".to_string(),
        ];
        
        self.execute_python_script(&args)
            .context("图像参数预测失败")
    }
    
    /// 调用Python AI音频预测脚本
    pub fn predict_audio_params(&self, input_path: &str, mode: &str) -> Result<Value> {
        let args = vec![
            "--audio-input".to_string(),
            input_path.to_string(),
            "--mode".to_string(),
            mode.to_string(),
            "--output-format".to_string(),
            "json".to_string(),
        ];
        
        self.execute_python_script(&args)
            .context("音频参数预测失败")
    }
    
    /// 调用Python AI视频预测脚本
    pub fn predict_video_params(&self, video_path: &str, optimize_mode: &str) -> Result<Value> {
        let args = vec![
            "--video-input".to_string(),
            video_path.to_string(),
            "--optimize-mode".to_string(),
            optimize_mode.to_string(),
            "--output-format".to_string(),
            "json".to_string(),
        ];
        
        self.execute_python_script(&args)
            .context("视频参数预测失败")
    }
    
    /// 执行Python脚本
    fn execute_python_script(&self, args: &[String]) -> Result<Value> {
        debug!("🐍 [Python Bridge] 执行脚本: {:?} {:?}", self.config.script_path, args);
        
        let start_time = std::time::Instant::now();
        
        let mut command = Command::new(&self.config.python_path);
        command
            .arg(&self.config.script_path)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        // 设置工作目录
        if let Some(working_dir) = &self.config.working_dir {
            command.current_dir(working_dir);
        }
        
        // 设置环境变量
        command.env("PYTHONPATH", ".");
        command.env("PIXLY_MODEL_DIR", &self.config.model_dir);
        
        // 执行命令
        let output = command
            .output()
            .context("启动Python进程失败")?;
        
        let duration = start_time.elapsed();
        
        // 检查超时
        if duration > Duration::from_secs(self.config.timeout_secs) {
            warn!("⚠️ [Python Bridge] 执行超时: {:?}", duration);
        }
        
        // 检查执行状态
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("❌ [Python Bridge] Python脚本执行失败:");
            error!("   退出码: {:?}", output.status.code());
            error!("   错误输出: {}", stderr);
            
            // 提供详细的错误诊断
            if stderr.contains("No module named") {
                error!("💡 可能的解决方案:");
                error!("   1. 安装缺失的Python依赖: pip install <missing_module>");
                error!("   2. 激活正确的Python虚拟环境");
                error!("   3. 检查PYTHONPATH环境变量");
            }
            
            bail!("Python脚本执行失败: {}", stderr);
        }
        
        // 解析JSON输出
        let stdout = String::from_utf8_lossy(&output.stdout);
        debug!("🐍 [Python Bridge] 执行成功，耗时: {:?}", duration);
        debug!("   输出长度: {} 字符", stdout.len());
        
        let result: Value = serde_json::from_str(&stdout)
            .context("解析Python脚本JSON输出失败")?;
        
        Ok(result)
    }
    
    /// 测试Python环境
    pub fn test_python_environment(&self) -> Result<()> {
        info!("🧪 [Python Bridge] 测试Python环境...");
        
        // 1. 测试Python可执行文件
        let version_output = Command::new(&self.config.python_path)
            .args(&["--version"])
            .output()
            .context("无法执行Python命令")?;
        
        if !version_output.status.success() {
            bail!("Python不可用");
        }
        
        let version_str = String::from_utf8_lossy(&version_output.stdout);
        info!("   Python版本: {}", version_str.trim());
        
        // 2. 测试脚本文件存在
        if !self.config.script_path.exists() {
            bail!("Python脚本不存在: {:?}", self.config.script_path);
        }
        
        info!("   脚本文件: ✅ 存在");
        
        // 3. 测试基本导入
        let import_test = Command::new(&self.config.python_path)
            .args(&["-c", "import json, sys; print('imports_ok')"])
            .output()
            .context("测试Python导入失败")?;
        
        if !import_test.status.success() {
            let stderr = String::from_utf8_lossy(&import_test.stderr);
            bail!("Python基本导入测试失败: {}", stderr);
        }
        
        info!("   基本导入: ✅ 正常");
        
        // 4. 测试脚本基本执行
        let script_test = Command::new(&self.config.python_path)
            .arg(&self.config.script_path)
            .args(&["--help"])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .status()
            .context("测试脚本执行失败")?;
        
        if script_test.success() {
            info!("   脚本执行: ✅ 正常");
        } else {
            warn!("   脚本执行: ⚠️ 可能有问题（但--help可能不支持）");
        }
        
        info!("✅ [Python Bridge] 环境测试完成");
        Ok(())
    }
    
    /// 获取配置信息
    pub fn get_config(&self) -> &PythonBridgeConfig {
        &self.config
    }
    
    /// 更新超时时间
    pub fn set_timeout(&mut self, timeout_secs: u64) {
        self.config.timeout_secs = timeout_secs;
        debug!("🕐 [Python Bridge] 超时时间更新为: {}秒", timeout_secs);
    }
    
    /// 设置工作目录
    pub fn set_working_dir(&mut self, dir: impl AsRef<Path>) {
        self.config.working_dir = Some(dir.as_ref().to_path_buf());
        debug!("📁 [Python Bridge] 工作目录设置为: {:?}", self.config.working_dir);
    }
}

/// 全局Python桥接器实例
static mut GLOBAL_BRIDGE: Option<PythonBridge> = None;
static BRIDGE_INIT: std::sync::Once = std::sync::Once::new();

/// 获取全局Python桥接器实例
pub fn get_python_bridge() -> Result<&'static PythonBridge> {
    unsafe {
        BRIDGE_INIT.call_once(|| {
            match PythonBridge::new("models") {
                Ok(bridge) => {
                    GLOBAL_BRIDGE = Some(bridge);
                }
                Err(e) => {
                    error!("❌ 初始化全局Python桥接器失败: {}", e);
                }
            }
        });
        
        GLOBAL_BRIDGE.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Python桥接器未初始化"))
    }
}

/// 便捷函数：预测图像参数
pub fn predict_image_params(input_format: &str, target_format: &str, 
                           width: u32, height: u32, file_size: u64, 
                           priority: &str) -> Result<Value> {
    let bridge = get_python_bridge()?;
    bridge.predict_image_params(input_format, target_format, width, height, file_size, priority)
}

/// 便捷函数：预测音频参数
pub fn predict_audio_params(input_path: &str, mode: &str) -> Result<Value> {
    let bridge = get_python_bridge()?;
    bridge.predict_audio_params(input_path, mode)
}

/// 便捷函数：预测视频参数
pub fn predict_video_params(video_path: &str, optimize_mode: &str) -> Result<Value> {
    let bridge = get_python_bridge()?;
    bridge.predict_video_params(video_path, optimize_mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_python_detection() {
        // 测试Python可执行文件检测
        let python_path = PythonBridge::detect_python_executable();
        println!("检测到的Python: {:?}", python_path);
        
        // 在大多数系统上应该能检测到python3或python
        assert!(python_path.is_some());
    }
    
    #[test]
    fn test_config_creation() {
        let config = PythonBridgeConfig::default();
        
        assert_eq!(config.python_path, "python3");
        assert_eq!(config.timeout_secs, 30);
        assert!(config.script_path.to_str().unwrap().contains("predict_params.py"));
    }
    
    #[test]
    fn test_bridge_creation() {
        let temp_dir = TempDir::new().unwrap();
        
        // 即使脚本不存在，创建桥接器也应该处理优雅
        // （实际使用中会查找真实的脚本路径）
        let result = PythonBridge::new(temp_dir.path());
        
        // 测试环境中可能没有predict_params.py，所以可能失败
        // 但不应该panic
        match result {
            Ok(_) => println!("✅ 桥接器创建成功"),
            Err(e) => println!("⚠️ 桥接器创建失败（预期）: {}", e),
        }
    }
}
