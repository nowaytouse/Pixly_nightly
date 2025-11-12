package converter

/*
#cgo LDFLAGS: -L../../pixly-rust/target/release -lpixly_converter
#include "../../pixly-rust/pixly_ffi.h"
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"fmt"
	"unsafe"
)

// ImageInfo Rust 返回的图像信息
type ImageInfo struct {
	Width      uint32
	Height     uint32
	Format     string
	IsAnimated bool
	FrameCount uint32
	FPS        float32
	FileSize   uint64
}

// ConversionConfig 转换配置
type ConversionConfig struct {
	Quality          uint8
	Speed            uint8
	PreserveMetadata bool
	KeepAnimated     bool
}

// ConversionResult 转换结果
type ConversionResult struct {
	Success          bool
	ErrorMessage     string
	OutputPath       string
	OutputSize       uint64
	ProcessingTimeMs uint64
}

// RustConverter Rust 转换器
type RustConverter struct {
	available bool
}

// NewRustConverter 创建 Rust 转换器
func NewRustConverter() *RustConverter {
	// 尝试获取版本以检测可用性
	version := C.pixly_version()
	available := version != nil
	
	return &RustConverter{
		available: available,
	}
}

// IsAvailable 检查 Rust 是否可用
func (rc *RustConverter) IsAvailable() bool {
	return rc.available
}

// Version 获取版本
func (rc *RustConverter) Version() string {
	version := C.pixly_version()
	if version == nil {
		return "unknown"
	}
	return C.GoString(version)
}

// ReadImageInfo 读取图像信息
func (rc *RustConverter) ReadImageInfo(path string) (*ImageInfo, error) {
	if !rc.available {
		return nil, errors.New("Rust converter not available")
	}
	
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))
	
	cInfo := C.pixly_read_image_info(cPath)
	if cInfo == nil {
		return nil, fmt.Errorf("failed to read image info: %s", path)
	}
	defer C.pixly_free_image_info(cInfo)
	
	info := &ImageInfo{
		Width:      uint32(cInfo.width),
		Height:     uint32(cInfo.height),
		Format:     C.GoString(cInfo.format),
		IsAnimated: bool(cInfo.is_animated),
		FrameCount: uint32(cInfo.frame_count),
		FPS:        float32(cInfo.fps),
		FileSize:   uint64(cInfo.file_size),
	}
	
	return info, nil
}

// DetectFormat 检测图像格式
func (rc *RustConverter) DetectFormat(path string) (string, error) {
	if !rc.available {
		return "", errors.New("Rust converter not available")
	}
	
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))
	
	cFormat := C.pixly_detect_format(cPath)
	if cFormat == nil {
		return "unknown", nil
	}
	defer C.pixly_free_string((*C.char)(unsafe.Pointer(cFormat)))
	
	return C.GoString(cFormat), nil
}

// IsAnimated 检测是否为动画
func (rc *RustConverter) IsAnimated(path string) (bool, error) {
	if !rc.available {
		return false, errors.New("Rust converter not available")
	}
	
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))
	
	result := C.pixly_is_animated(cPath)
	switch result {
	case 1:
		return true, nil
	case 0:
		return false, nil
	default:
		return false, fmt.Errorf("failed to detect animation: %s", path)
	}
}

// ConvertImage 转换图像
func (rc *RustConverter) ConvertImage(input, output, format string, config *ConversionConfig) (*ConversionResult, error) {
	if !rc.available {
		return nil, errors.New("Rust converter not available")
	}
	
	cInput := C.CString(input)
	cOutput := C.CString(output)
	cFormat := C.CString(format)
	defer func() {
		C.free(unsafe.Pointer(cInput))
		C.free(unsafe.Pointer(cOutput))
		C.free(unsafe.Pointer(cFormat))
	}()
	
	// 创建 C 配置
	var cConfig *C.CConversionConfig
	if config != nil {
		cConfig = &C.CConversionConfig{
			quality:            C.uint8_t(config.Quality),
			speed:              C.uint8_t(config.Speed),
			preserve_metadata:  C.bool(config.PreserveMetadata),
			keep_animated:      C.bool(config.KeepAnimated),
		}
	}
	
	// 调用转换
	cResult := C.pixly_convert_image(cInput, cOutput, cFormat, cConfig)
	if cResult == nil {
		return nil, fmt.Errorf("conversion failed: %s", input)
	}
	defer C.pixly_free_conversion_result(cResult)
	
	result := &ConversionResult{
		Success:          bool(cResult.success),
		OutputPath:       C.GoString(cResult.output_path),
		OutputSize:       uint64(cResult.output_size),
		ProcessingTimeMs: uint64(cResult.processing_time_ms),
	}
	
	if cResult.error_message != nil {
		result.ErrorMessage = C.GoString(cResult.error_message)
	}
	
	return result, nil
}

// EagleReplaceInPlace Eagle 原地替换
func (rc *RustConverter) EagleReplaceInPlace(original, converted string, width, height uint32) error {
	if !rc.available {
		return errors.New("Rust converter not available")
	}
	
	cOriginal := C.CString(original)
	cConverted := C.CString(converted)
	defer func() {
		C.free(unsafe.Pointer(cOriginal))
		C.free(unsafe.Pointer(cConverted))
	}()
	
	result := C.pixly_eagle_replace_in_place(cOriginal, cConverted, C.uint32_t(width), C.uint32_t(height))
	if result != 0 {
		return fmt.Errorf("Eagle replace in place failed: %s -> %s", original, converted)
	}
	
	return nil
}
