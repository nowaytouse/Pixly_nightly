/*
 * PIXLY Converter - C FFI Header
 * 
 * For use with Go CGO
 * 
 * Build: cargo build --release
 * Library: target/release/libpixly_converter.dylib (macOS)
 *          target/release/libpixly_converter.so (Linux)
 */

#ifndef PIXLY_FFI_H
#define PIXLY_FFI_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================================
 * Types
 * ========================================================================== */

/**
 * 图像信息结构
 */
typedef struct {
    uint32_t width;           // 图像宽度
    uint32_t height;          // 图像高度
    const char* format;       // 图像格式 (需要调用 pixly_free_string 释放)
    bool is_animated;         // 是否为动画
    uint32_t frame_count;     // 帧数
    float fps;                // FPS
    uint64_t file_size;       // 文件大小（字节）
} CImageInfo;

/**
 * 转换配置结构
 */
typedef struct {
    uint8_t quality;          // 质量 (0-100)
    uint8_t speed;            // 速度/effort
    bool preserve_metadata;   // 保留元数据
    bool keep_animated;       // 保留动画
} CConversionConfig;

/**
 * 转换结果结构
 */
typedef struct {
    bool success;             // 是否成功
    const char* error_message; // 错误信息 (需要调用 pixly_free_string 释放)
    const char* output_path;  // 输出路径 (需要调用 pixly_free_string 释放)
    uint64_t output_size;     // 输出大小（字节）
    uint64_t processing_time_ms; // 处理时间（毫秒）
} CConversionResult;

/* ============================================================================
 * Core Functions
 * ========================================================================== */

/**
 * 获取库版本
 * 
 * @return 版本字符串 (不需要释放)
 */
const char* pixly_version(void);

/**
 * 读取图像信息
 * 
 * @param path 图像文件路径 (UTF-8 编码的 C 字符串)
 * @return CImageInfo 指针，失败返回 NULL
 * 
 * @note 调用者需要使用 pixly_free_image_info 释放返回的指针
 */
CImageInfo* pixly_read_image_info(const char* path);

/**
 * 检测图像格式
 * 
 * @param path 图像文件路径
 * @return 格式字符串 (jpeg/png/gif/webp/avif/jxl/heic/unknown)
 * 
 * @note 调用者需要使用 pixly_free_string 释放返回的字符串
 */
const char* pixly_detect_format(const char* path);

/**
 * 检测是否为动画
 * 
 * @param path 图像文件路径
 * @return 1=动画, 0=静态, -1=错误
 */
int32_t pixly_is_animated(const char* path);

/**
 * 转换图像
 * 
 * @param input 输入文件路径
 * @param output 输出文件路径
 * @param format 目标格式 (jpeg/png/webp/avif/jxl)
 * @param config 转换配置 (可为 NULL 使用默认配置)
 * @return CConversionResult 指针，失败返回 NULL
 * 
 * @note 调用者需要使用 pixly_free_conversion_result 释放返回的指针
 */
CConversionResult* pixly_convert_image(
    const char* input,
    const char* output,
    const char* format,
    const CConversionConfig* config
);

/**
 * Eagle 原地替换
 * 
 * 删除原文件，重命名新文件，更新 metadata.json
 * 
 * @param original 原文件路径
 * @param converted 转换后文件路径
 * @param width 新图像宽度
 * @param height 新图像高度
 * @return 0=成功, -1=失败
 */
int32_t pixly_eagle_replace_in_place(
    const char* original,
    const char* converted,
    uint32_t width,
    uint32_t height
);

/* ============================================================================
 * Memory Management
 * ========================================================================== */

/**
 * 释放字符串内存
 * 
 * @param ptr 由 Rust 分配的字符串指针
 */
void pixly_free_string(char* ptr);

/**
 * 释放 CImageInfo 内存
 * 
 * @param ptr CImageInfo 指针
 */
void pixly_free_image_info(CImageInfo* ptr);

/**
 * 释放 CConversionResult 内存
 * 
 * @param ptr CConversionResult 指针
 */
void pixly_free_conversion_result(CConversionResult* ptr);

#ifdef __cplusplus
}
#endif

#endif /* PIXLY_FFI_H */
