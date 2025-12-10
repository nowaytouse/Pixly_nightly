# Changelog

All notable changes to the JPEG to JXL Eagle Plugin will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-11-27

### Added
- Initial release of JPEG to JXL Converter Eagle Plugin
- One-click lossless JPEG to JXL conversion
- Batch processing support for multiple files
- Cross-platform compatibility (macOS, Windows, Linux)
- Platform-specific binary detection and error handling
- Automatic platform detection (darwin, win32, linux)
- Complete setup guide for missing binaries
- Interactive setup script (`setup.sh`) for automated binary download
- Validation script (`validate.sh`) for plugin integrity checks
- Comprehensive README with installation and usage instructions
- Logo image for plugin branding
- Progress display during conversion
- Error handling with detailed user feedback

### Fixed
- Eagle import issues due to missing `logo.png`
- Eagle import issues due to incomplete `manifest.json`
- Cross-platform binary path resolution
- Permission issues for binary execution on Unix systems

### Technical Details
- Built on Eagle Plugin API (Chromium 107 + Node 16)
- Uses libjxl's `cjxl` for conversion with `-d 0` (lossless) parameter
- Supports JPEG file extensions: `.jpg`, `.jpeg`, `.jpe`, `.jfif`
- Output format: `.jxl` in same directory as source files

### Platform Support
- ✅ macOS (darwin): Fully supported with included binary
- ✅ Windows (win32): Fully supported with included binary  
- ⚠️ Linux: Requires manual binary installation via `setup.sh`

### Known Issues
- Linux binary not included by default (must be downloaded via setup script)
- macOS binary may show warnings on some systems (but should work correctly)

## [Unreleased]

### Planned
- Quality/compression settings customization
- Support for other image formats (PNG, WebP, etc.)
- Batch conversion with configurable output directory
- Conversion history and statistics
- Automatic binary updates
- ARM architecture support for macOS and Linux
