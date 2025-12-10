# PIXLY 一键环境配置脚本 (Windows)
# 自动检测并安装所需依赖

$ErrorActionPreference = "Stop"

Write-Host "🚀 PIXLY 环境配置脚本 (Windows)" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

# 检查管理员权限
function Test-Administrator {
    $currentUser = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    return $currentUser.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

if (-not (Test-Administrator)) {
    Write-Host "⚠️  需要管理员权限" -ForegroundColor Yellow
    Write-Host "   请右键点击PowerShell，选择'以管理员身份运行'" -ForegroundColor Yellow
    Write-Host ""
    Read-Host "按任意键退出"
    exit 1
}

# 创建安装目录
$InstallRoot = "C:\pixly-tools"
if (-not (Test-Path $InstallRoot)) {
    New-Item -ItemType Directory -Path $InstallRoot -Force | Out-Null
}

# 安装ExifTool
function Install-ExifTool {
    Write-Host "📥 正在安装 ExifTool..." -ForegroundColor Blue
    
    $exiftoolPath = "C:\exiftool"
    $exiftoolExe = "$exiftoolPath\exiftool.exe"
    
    if (Test-Path $exiftoolExe) {
        Write-Host "✅ ExifTool 已安装" -ForegroundColor Green
        return
    }
    
    $downloadUrl = "https://exiftool.org/exiftool-12.70.zip"
    $zipPath = "$InstallRoot\exiftool.zip"
    
    try {
        Write-Host "   下载中..." -ForegroundColor Gray
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
        
        Write-Host "   解压中..." -ForegroundColor Gray
        Expand-Archive -Path $zipPath -DestinationPath $InstallRoot -Force
        
        # 创建exiftool目录并移动文件
        if (-not (Test-Path $exiftoolPath)) {
            New-Item -ItemType Directory -Path $exiftoolPath -Force | Out-Null
        }
        
        $extractedExe = Get-ChildItem -Path $InstallRoot -Filter "exiftool(-k).exe" -Recurse | Select-Object -First 1
        if ($extractedExe) {
            Move-Item -Path $extractedExe.FullName -Destination $exiftoolExe -Force
        }
        
        # 清理
        Remove-Item $zipPath -Force
        
        # 添加到PATH
        Add-ToPath $exiftoolPath
        
        Write-Host "✅ ExifTool 安装完成" -ForegroundColor Green
    }
    catch {
        Write-Host "❌ ExifTool 安装失败: $_" -ForegroundColor Red
        Write-Host "   请手动下载: https://exiftool.org/" -ForegroundColor Yellow
    }
}

# 安装FFmpeg
function Install-FFmpeg {
    Write-Host "📥 正在安装 FFmpeg..." -ForegroundColor Blue
    
    $ffmpegPath = "C:\ffmpeg\bin"
    $ffmpegExe = "$ffmpegPath\ffmpeg.exe"
    
    if (Test-Path $ffmpegExe) {
        Write-Host "✅ FFmpeg 已安装" -ForegroundColor Green
        return
    }
    
    # FFmpeg下载需要从GitHub releases获取
    $downloadUrl = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip"
    $zipPath = "$InstallRoot\ffmpeg.zip"
    
    try {
        Write-Host "   下载中 (可能需要几分钟)..." -ForegroundColor Gray
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
        
        Write-Host "   解压中..." -ForegroundColor Gray
        Expand-Archive -Path $zipPath -DestinationPath $InstallRoot -Force
        
        # 查找并移动ffmpeg
        $extractedBin = Get-ChildItem -Path $InstallRoot -Filter "bin" -Directory -Recurse | Select-Object -First 1
        if ($extractedBin) {
            if (-not (Test-Path "C:\ffmpeg")) {
                New-Item -ItemType Directory -Path "C:\ffmpeg" -Force | Out-Null
            }
            Move-Item -Path $extractedBin.FullName -Destination "C:\ffmpeg\bin" -Force
        }
        
        # 清理
        Remove-Item $zipPath -Force
        Get-ChildItem -Path $InstallRoot -Directory | Where-Object { $_.Name -like "ffmpeg-*" } | Remove-Item -Recurse -Force
        
        # 添加到PATH
        Add-ToPath $ffmpegPath
        
        Write-Host "✅ FFmpeg 安装完成" -ForegroundColor Green
    }
    catch {
        Write-Host "❌ FFmpeg 安装失败: $_" -ForegroundColor Red
        Write-Host "   请手动下载: https://ffmpeg.org/download.html#build-windows" -ForegroundColor Yellow
    }
}

# 添加到PATH环境变量
function Add-ToPath {
    param([string]$PathToAdd)
    
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    
    if ($currentPath -notlike "*$PathToAdd*") {
        Write-Host "   添加到系统PATH: $PathToAdd" -ForegroundColor Gray
        $newPath = "$currentPath;$PathToAdd"
        [Environment]::SetEnvironmentVariable("Path", $newPath, "Machine")
        
        # 更新当前会话的PATH
        $env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
    }
}

# 验证安装
function Test-Installation {
    Write-Host ""
    Write-Host "🔍 验证安装..." -ForegroundColor Cyan
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
    Write-Host ""
    
    # 刷新环境变量
    $env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
    
    $allOk = $true
    
    # 检查ExifTool
    try {
        $exiftoolVersion = & exiftool -ver 2>$null
        Write-Host "✅ ExifTool: $exiftoolVersion" -ForegroundColor Green
    }
    catch {
        Write-Host "❌ ExifTool: 未找到" -ForegroundColor Red
        $allOk = $false
    }
    
    # 检查FFmpeg
    try {
        $ffmpegOutput = & ffmpeg -version 2>$null | Select-Object -First 1
        if ($ffmpegOutput -match "ffmpeg version ([^ ]+)") {
            Write-Host "✅ FFmpeg: $($matches[1])" -ForegroundColor Green
        }
    }
    catch {
        Write-Host "❌ FFmpeg: 未找到" -ForegroundColor Red
        $allOk = $false
    }
    
    # 检查FFprobe
    try {
        $ffprobeOutput = & ffprobe -version 2>$null | Select-Object -First 1
        if ($ffprobeOutput -match "ffprobe version ([^ ]+)") {
            Write-Host "✅ FFprobe: $($matches[1])" -ForegroundColor Green
        }
    }
    catch {
        Write-Host "⚠️  FFprobe: 未找到 (通常随FFmpeg一起安装)" -ForegroundColor Yellow
    }
    
    Write-Host ""
    
    if ($allOk) {
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
        Write-Host "🎉 所有依赖安装成功！" -ForegroundColor Green
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
        Write-Host ""
        Write-Host "📖 下一步：" -ForegroundColor Cyan
        Write-Host "   1. 重启 Eagle 应用" -ForegroundColor White
        Write-Host "   2. 启用 PIXLY 插件" -ForegroundColor White
        Write-Host "   3. 开始使用！" -ForegroundColor White
        Write-Host ""
        Write-Host "💡 提示：" -ForegroundColor Cyan
        Write-Host "   - 可能需要重启PowerShell或系统才能生效" -ForegroundColor White
        Write-Host "   - 运行 'pixly-rust --check-deps' 验证依赖" -ForegroundColor White
        Write-Host "   - 查看文档: https://pixly.app/docs" -ForegroundColor White
        Write-Host ""
    }
    else {
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Red
        Write-Host "❌ 部分依赖安装失败" -ForegroundColor Red
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Red
        Write-Host ""
        Write-Host "请检查上方错误信息，或访问：" -ForegroundColor Yellow
        Write-Host "https://pixly.app/docs/troubleshooting" -ForegroundColor Yellow
    }
}

# 主流程
try {
    Install-ExifTool
    Write-Host ""
    Install-FFmpeg
    Test-Installation
}
catch {
    Write-Host ""
    Write-Host "❌ 安装过程出错: $_" -ForegroundColor Red
    Write-Host ""
    Read-Host "按任意键退出"
    exit 1
}

Write-Host ""
Read-Host "按任意键退出"
