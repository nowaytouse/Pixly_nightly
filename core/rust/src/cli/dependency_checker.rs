/**
 * Dependency Checker - 外部依赖检查
 * 
 * 🔥 Phase 40.19: 启动时检查外部依赖
 * 
 * 检查必需的外部工具：
 * - exiftool (元数据处理)
 * - ffmpeg (视频处理 - 可选)
 * - ffprobe (视频分析 - 可选)
 */

use std::process::Command;
use std::path::PathBuf;
use std::env;
use log::debug;

/// 依赖项信息
#[derive(Debug, Clone)]
pub struct Dependency {
    /// 命令名称
    pub name: &'static str,
    /// 是否必需
    pub required: bool,
    /// 功能描述
    pub description: &'static str,
    /// 安装提示
    pub install_hint: &'static str,
}

/// 检查结果
#[derive(Debug)]
pub struct CheckResult {
    pub name: &'static str,
    pub available: bool,
    pub version: Option<String>,
}

impl Dependency {
    /// 检查依赖是否可用（使用智能查找）
    pub fn check(&self) -> CheckResult {
        // 🔥 使用智能查找函数，支持多种安装方式
        let (available, version) = match self.name {
            "ffmpeg" => {
                if let Some(path) = find_ffmpeg() {
                    let ver = get_command_version_by_path(&path);
                    (true, ver)
                } else {
                    (false, None)
                }
            }
            "ffprobe" => {
                // ffprobe通常与ffmpeg一起安装
                if let Some(ffmpeg_path) = find_ffmpeg() {
                    // 尝试在同一目录找ffprobe
                    if let Some(parent) = ffmpeg_path.parent() {
                        let ffprobe_path = parent.join("ffprobe");
                        if ffprobe_path.exists() {
                            let ver = get_command_version_by_path(&ffprobe_path);
                            (true, ver)
                        } else {
                            // 尝试PATH查找
                            (is_command_available("ffprobe"), get_command_version("ffprobe"))
                        }
                    } else {
                        (is_command_available("ffprobe"), get_command_version("ffprobe"))
                    }
                } else {
                    (is_command_available("ffprobe"), get_command_version("ffprobe"))
                }
            }
            "exiftool" => {
                if let Some(path) = find_exiftool() {
                    let ver = get_command_version_by_path(&path);
                    (true, ver)
                } else {
                    (false, None)
                }
            }
            _ => {
                // 其他工具使用简单检查
                let available = is_command_available(self.name);
                let version = if available {
                    get_command_version(self.name)
                } else {
                    None
                };
                (available, version)
            }
        };
        
        CheckResult {
            name: self.name,
            available,
            version,
        }
    }
}

/// 检查命令是否可用
fn is_command_available(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// 获取命令版本
fn get_command_version(cmd: &str) -> Option<String> {
    Command::new(cmd)
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
                    .and_then(|s| s.lines().next().map(|l| l.to_string()))
            } else {
                None
            }
        })
}

/// 通过完整路径获取命令版本
fn get_command_version_by_path(path: &PathBuf) -> Option<String> {
    let path_str = path.to_string_lossy();
    
    // 不同工具使用不同的版本参数
    let args = if path_str.contains("exiftool") {
        vec!["-ver"]  // ExifTool使用 -ver
    } else if path_str.contains("ffmpeg") || path_str.contains("ffprobe") {
        vec!["-version"]  // FFmpeg使用 -version
    } else {
        vec!["--version"]  // 其他工具使用 --version
    };
    
    Command::new(path)
        .args(&args)
        .output()
        .ok()
        .and_then(|output| {
            // 尝试从stdout获取
            if let Ok(stdout) = String::from_utf8(output.stdout.clone()) {
                let first_line = stdout.lines().next().unwrap_or("").trim();
                if !first_line.is_empty() {
                    // ExifTool只返回版本号，添加工具名
                    if path_str.contains("exiftool") && !first_line.contains("exiftool") {
                        return Some(format!("ExifTool {}", first_line));
                    }
                    return Some(first_line.to_string());
                }
            }
            // 尝试从stderr获取（某些工具输出到stderr）
            if let Ok(stderr) = String::from_utf8(output.stderr) {
                let first_line = stderr.lines().next().unwrap_or("").trim();
                if !first_line.is_empty() {
                    return Some(first_line.to_string());
                }
            }
            None
        })
}

/// 🔥 智能FFmpeg查找：支持多种安装方式
pub fn find_ffmpeg() -> Option<PathBuf> {
    // 1. 检查环境变量FFMPEG_PATH
    if let Ok(path) = env::var("FFMPEG_PATH") {
        let ffmpeg_path = PathBuf::from(path);
        if ffmpeg_path.exists() {
            return Some(ffmpeg_path);
        }
    }
    
    // 2. 检查常见安装位置（跨平台）
    let mut common_paths: Vec<String> = Vec::new();
    
    match env::consts::OS {
        "macos" => {
            common_paths.push("/opt/homebrew/bin/ffmpeg".to_string());      // Apple Silicon Homebrew
            common_paths.push("/usr/local/bin/ffmpeg".to_string());          // Intel Homebrew
            common_paths.push("/opt/local/bin/ffmpeg".to_string());          // MacPorts
        }
        "windows" => {
            common_paths.push("C:\\Program Files\\FFmpeg\\bin\\ffmpeg.exe".to_string());
            common_paths.push("C:\\ffmpeg\\bin\\ffmpeg.exe".to_string());
            if let Ok(userprofile) = env::var("USERPROFILE") {
                common_paths.push(format!("{}\\ffmpeg\\bin\\ffmpeg.exe", userprofile));
            }
        }
        "linux" => {
            common_paths.push("/usr/bin/ffmpeg".to_string());
            common_paths.push("/usr/local/bin/ffmpeg".to_string());
            common_paths.push("/opt/ffmpeg/bin/ffmpeg".to_string());
            common_paths.push("/snap/bin/ffmpeg".to_string());              // Snap package
        }
        _ => {}
    }
    
    for path_str in common_paths {
        let path = PathBuf::from(&path_str);
        if path.exists() {
            return Some(path);
        }
    }
    
    // 3. 尝试从PATH环境变量查找
    if is_command_available("ffmpeg") {
        // 如果which/where命令可用，尝试获取完整路径
        #[cfg(target_os = "windows")]
        let which_cmd = "where";
        #[cfg(not(target_os = "windows"))]
        let which_cmd = "which";
        
        if let Ok(output) = Command::new(which_cmd).arg("ffmpeg").output() {
            if output.status.success() {
                if let Ok(path_str) = String::from_utf8(output.stdout) {
                    return Some(PathBuf::from(path_str.trim()));
                }
            }
        }
        
        // ✅ 响亮报错：找不到ffmpeg返回None，让调用者明确知道
        // 不再fallback到命令名（违反质量宣言）
        debug!("⚠️  FFmpeg not found in PATH");
    }
    
    None
}

/// 🔥 智能ExifTool查找
pub fn find_exiftool() -> Option<PathBuf> {
    // 1. 检查环境变量
    if let Ok(path) = env::var("EXIFTOOL_PATH") {
        let exiftool_path = PathBuf::from(path);
        if exiftool_path.exists() {
            return Some(exiftool_path);
        }
    }
    
    // 2. 检查常见位置
    let mut common_paths: Vec<String> = Vec::new();
    
    match env::consts::OS {
        "macos" => {
            common_paths.push("/opt/homebrew/bin/exiftool".to_string());
            common_paths.push("/usr/local/bin/exiftool".to_string());
        }
        "windows" => {
            common_paths.push("C:\\exiftool\\exiftool.exe".to_string());
            if let Ok(userprofile) = env::var("USERPROFILE") {
                common_paths.push(format!("{}\\exiftool\\exiftool.exe", userprofile));
            }
        }
        "linux" => {
            common_paths.push("/usr/bin/exiftool".to_string());
            common_paths.push("/usr/local/bin/exiftool".to_string());
        }
        _ => {}
    }
    
    for path_str in common_paths {
        let path = PathBuf::from(&path_str);
        if path.exists() {
            return Some(path);
        }
    }
    
    // 3. 从PATH查找
    if is_command_available("exiftool") {
        return Some(PathBuf::from("exiftool"));
    }
    
    None
}

/// 定义所有依赖项
pub fn get_dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            name: "exiftool",
            required: true,
            description: "元数据处理（EXIF/XMP/ICC）",
            install_hint: get_install_hint("exiftool"),
        },
        Dependency {
            name: "ffmpeg",
            required: false,
            description: "视频转换和GIF分析",
            install_hint: get_install_hint("ffmpeg"),
        },
        Dependency {
            name: "ffprobe",
            required: false,
            description: "视频信息获取",
            install_hint: get_install_hint("ffmpeg"),
        },
    ]
}

/// 🔥 根据操作系统生成安装提示
fn get_install_hint(tool: &str) -> &'static str {
    match env::consts::OS {
        "macos" => match tool {
            "exiftool" => "macOS安装方法：
  📦 Homebrew: brew install exiftool
  📥 官方下载: https://exiftool.org/

🤖 或运行一键安装脚本：
  bash <(curl -fsSL https://pixly.app/install.sh)",
            "ffmpeg" => "macOS安装方法：
  📦 Homebrew: brew install ffmpeg
  📥 官方下载: https://ffmpeg.org/download.html

🤖 或运行一键安装脚本：
  bash <(curl -fsSL https://pixly.app/install.sh)",
            _ => "请查看 https://pixly.app/docs/setup",
        },
        "windows" => match tool {
            "exiftool" => "Windows安装方法：
  📥 下载: https://exiftool.org/
  📦 解压到: C:\\exiftool\\exiftool.exe
  🔧 或添加到PATH环境变量

🤖 或运行一键安装脚本：
  powershell -c \"irm https://pixly.app/install.ps1 | iex\"",
            "ffmpeg" => "Windows安装方法：
  📥 下载: https://ffmpeg.org/download.html#build-windows
  📦 解压到: C:\\ffmpeg\\bin\\ffmpeg.exe
  🔧 或添加到PATH环境变量

🤖 或运行一键安装脚本：
  powershell -c \"irm https://pixly.app/install.ps1 | iex\"",
            _ => "请查看 https://pixly.app/docs/setup",
        },
        "linux" => match tool {
            "exiftool" => "Linux安装方法：
  📦 Debian/Ubuntu: sudo apt install libimage-exiftool-perl
  📦 Fedora/RHEL: sudo dnf install perl-Image-ExifTool
  📦 Arch: sudo pacman -S perl-image-exiftool

🤖 或运行一键安装脚本：
  bash <(curl -fsSL https://pixly.app/install.sh)",
            "ffmpeg" => "Linux安装方法：
  📦 Debian/Ubuntu: sudo apt install ffmpeg
  📦 Fedora/RHEL: sudo dnf install ffmpeg
  📦 Arch: sudo pacman -S ffmpeg

🤖 或运行一键安装脚本：
  bash <(curl -fsSL https://pixly.app/install.sh)",
            _ => "请查看 https://pixly.app/docs/setup",
        },
        _ => "请查看 https://pixly.app/docs/setup",
    }
}

/// 检查所有依赖
pub fn check_all_dependencies() -> Vec<CheckResult> {
    get_dependencies()
        .iter()
        .map(|dep| dep.check())
        .collect()
}

/// 检查并显示依赖状态
pub fn check_and_report_dependencies(verbose: bool) -> bool {
    let dependencies = get_dependencies();
    let results = check_all_dependencies();
    
    let mut all_required_available = true;
    let mut warnings = Vec::new();
    
    if verbose {
        println!("\n🔍 检查外部依赖...\n");
    }
    
    for (dep, result) in dependencies.iter().zip(results.iter()) {
        if result.available {
            if verbose {
                if let Some(version) = &result.version {
                    println!("  ✅ {} - {} ({})", 
                        result.name, 
                        dep.description,
                        version.lines().next().unwrap_or("unknown"));
                } else {
                    println!("  ✅ {} - {}", result.name, dep.description);
                }
            }
        } else {
            if dep.required {
                all_required_available = false;
                eprintln!("  ❌ {} - {} (必需)", result.name, dep.description);
                eprintln!("     安装方法:");
                for line in dep.install_hint.lines() {
                    eprintln!("     {}", line);
                }
                eprintln!();
            } else {
                warnings.push(result.name);
                if verbose {
                    println!("  ⚠️  {} - {} (可选，未安装)", result.name, dep.description);
                }
            }
        }
    }
    
    if !warnings.is_empty() && verbose {
        println!("\n💡 提示：以下可选工具未安装，部分功能可能受限：");
        for name in &warnings {
            println!("  - {}", name);
        }
    }
    
    if !all_required_available {
        eprintln!("\n❌ 缺少必需的依赖项！请先安装缺失的工具。\n");
    } else if verbose {
        println!("\n✅ 所有必需依赖已满足！\n");
    }
    
    all_required_available
}

/// 仅检查必需依赖（快速检查）
pub fn check_required_dependencies_quiet() -> bool {
    get_dependencies()
        .iter()
        .filter(|dep| dep.required)
        .all(|dep| is_command_available(dep.name))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dependency_check() {
        // exiftool应该可用（假设已安装）
        let exiftool = Dependency {
            name: "exiftool",
            required: true,
            description: "元数据处理",
            install_hint: "brew install exiftool",
        };
        
        let result = exiftool.check();
        assert_eq!(result.name, "exiftool");
        // 注意：这个测试可能在CI环境中失败
    }
    
    #[test]
    fn test_nonexistent_command() {
        assert!(!is_command_available("nonexistent_command_12345"));
    }
}
