/**
 * Dependency Checker - 外部依赖检查
 * 
 * 检查必需的外部工具：
 * - exiftool (元数据处理)
 * - ffmpeg (视频处理)
 * - ffprobe (视频分析)
 * - cjxl/djxl (JPEG XL)
 * - avifenc/avifdec (AVIF)
 */
use std::process::Command;
use std::path::PathBuf;
use std::env;
use log::{debug, info};

/// 依赖项信息
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: &'static str,
    pub required: bool,
    pub description: &'static str,
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
    /// 检查依赖是否可用
    pub fn check(&self) -> CheckResult {
        let (available, version) = match self.name {
            "ffmpeg" => {
                if let Some(path) = find_ffmpeg() {
                    (true, get_command_version_by_path(&path))
                } else {
                    (false, None)
                }
            }
            "ffprobe" => {
                if let Some(ffmpeg_path) = find_ffmpeg() {
                    if let Some(parent) = ffmpeg_path.parent() {
                        let ffprobe_path = parent.join("ffprobe");
                        if ffprobe_path.exists() {
                            (true, get_command_version_by_path(&ffprobe_path))
                        } else {
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
                    (true, get_command_version_by_path(&path))
                } else {
                    (false, None)
                }
            }
            _ => {
                let available = is_command_available(self.name);
                let version = if available { get_command_version(self.name) } else { None };
                (available, version)
            }
        };
        
        CheckResult { name: self.name, available, version }
    }
}

fn is_command_available(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

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

fn get_command_version_by_path(path: &PathBuf) -> Option<String> {
    let path_str = path.to_string_lossy();
    let args = if path_str.contains("exiftool") {
        vec!["-ver"]
    } else if path_str.contains("ffmpeg") || path_str.contains("ffprobe") {
        vec!["-version"]
    } else {
        vec!["--version"]
    };
    
    Command::new(path)
        .args(&args)
        .output()
        .ok()
        .and_then(|output| {
            if let Ok(stdout) = String::from_utf8(output.stdout.clone()) {
                let first_line = stdout.lines().next().unwrap_or("").trim();
                if !first_line.is_empty() {
                    if path_str.contains("exiftool") && !first_line.contains("exiftool") {
                        return Some(format!("ExifTool {}", first_line));
                    }
                    return Some(first_line.to_string());
                }
            }
            if let Ok(stderr) = String::from_utf8(output.stderr) {
                let first_line = stderr.lines().next().unwrap_or("").trim();
                if !first_line.is_empty() {
                    return Some(first_line.to_string());
                }
            }
            None
        })
}

pub fn find_ffmpeg() -> Option<PathBuf> {
    if let Ok(path) = env::var("FFMPEG_PATH") {
        let ffmpeg_path = PathBuf::from(path);
        if ffmpeg_path.exists() {
            return Some(ffmpeg_path);
        }
    }
    
    let mut common_paths: Vec<String> = Vec::new();
    match env::consts::OS {
        "macos" => {
            common_paths.push("/opt/homebrew/bin/ffmpeg".to_string());
            common_paths.push("/usr/local/bin/ffmpeg".to_string());
            common_paths.push("/opt/local/bin/ffmpeg".to_string());
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
            common_paths.push("/snap/bin/ffmpeg".to_string());
        }
        _ => {}
    }
    
    for path_str in common_paths {
        let path = PathBuf::from(&path_str);
        if path.exists() {
            return Some(path);
        }
    }
    
    if is_command_available("ffmpeg") {
        #[cfg(target_os = "windows")]
        let which_cmd = "where";
        #[cfg(not(target_os = "windows"))]
        let which_cmd = "which";
        
        if let Ok(output) = Command::new(which_cmd).arg("ffmpeg").output()
            && output.status.success()
            && let Ok(path_str) = String::from_utf8(output.stdout)
        {
            return Some(PathBuf::from(path_str.trim()));
        }
        debug!("⚠️  FFmpeg not found in PATH");
    }
    None
}

pub fn find_exiftool() -> Option<PathBuf> {
    if let Ok(path) = env::var("EXIFTOOL_PATH") {
        let exiftool_path = PathBuf::from(path);
        if exiftool_path.exists() {
            return Some(exiftool_path);
        }
    }
    
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
    
    if is_command_available("exiftool") {
        return Some(PathBuf::from("exiftool"));
    }
    None
}

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
        Dependency {
            name: "cjxl",
            required: false,
            description: "JPEG XL编码",
            install_hint: get_install_hint("cjxl"),
        },
        Dependency {
            name: "djxl",
            required: false,
            description: "JPEG XL解码",
            install_hint: get_install_hint("cjxl"),
        },
        Dependency {
            name: "avifenc",
            required: false,
            description: "AVIF编码",
            install_hint: get_install_hint("avifenc"),
        },
        Dependency {
            name: "avifdec",
            required: false,
            description: "AVIF解码",
            install_hint: get_install_hint("avifenc"),
        },
    ]
}

fn get_install_hint(tool: &str) -> &'static str {
    match env::consts::OS {
        "macos" => match tool {
            "exiftool" => "macOS: brew install exiftool",
            "ffmpeg" => "macOS: brew install ffmpeg",
            "cjxl" => "macOS: brew install jpeg-xl",
            "avifenc" => "macOS: brew install libavif",
            _ => "请查看官方文档",
        },
        "windows" => match tool {
            "exiftool" => "Windows: 下载 https://exiftool.org/",
            "ffmpeg" => "Windows: 下载 https://ffmpeg.org/",
            "cjxl" => "Windows: 下载 https://github.com/libjxl/libjxl/releases",
            "avifenc" => "Windows: 下载 https://github.com/AOMediaCodec/libavif/releases",
            _ => "请查看官方文档",
        },
        "linux" => match tool {
            "exiftool" => "Linux: sudo apt install libimage-exiftool-perl",
            "ffmpeg" => "Linux: sudo apt install ffmpeg",
            "cjxl" => "Linux: sudo apt install libjxl-tools",
            "avifenc" => "Linux: sudo apt install libavif-bin",
            _ => "请查看官方文档",
        },
        _ => "请查看官方文档",
    }
}

pub fn check_all_dependencies() -> Vec<CheckResult> {
    get_dependencies().iter().map(|dep| dep.check()).collect()
}

pub fn check_and_report_dependencies(verbose: bool) -> bool {
    let dependencies = get_dependencies();
    let results = check_all_dependencies();
    
    let mut all_required_available = true;
    let mut warnings = Vec::new();
    
    if verbose {
        info!("🔍 Checking external dependencies...");
    }
    
    for (dep, result) in dependencies.iter().zip(results.iter()) {
        if result.available {
            if verbose {
                if let Some(version) = &result.version {
                    info!("  ✅ {} - {} ({})", 
                        result.name, dep.description,
                        version.lines().next().unwrap_or("unknown"));
                } else {
                    info!("  ✅ {} - {}", result.name, dep.description);
                }
            }
        } else if dep.required {
            all_required_available = false;
            eprintln!("  ❌ {} - {} (required)", result.name, dep.description);
            eprintln!("     {}", dep.install_hint);
        } else {
            warnings.push(result.name);
            if verbose {
                info!("  ⚠️  {} - {} (optional)", result.name, dep.description);
            }
        }
    }
    
    if !warnings.is_empty() && verbose {
        info!("💡 Optional tools not installed: {}", warnings.join(", "));
    }
    
    if !all_required_available {
        eprintln!("❌ Missing required dependencies!");
    } else if verbose {
        info!("✅ All required dependencies are satisfied!");
    }
    
    all_required_available
}

pub fn check_required_dependencies_quiet() -> bool {
    get_dependencies()
        .iter()
        .filter(|dep| dep.required)
        .all(|dep| is_command_available(dep.name))
}
