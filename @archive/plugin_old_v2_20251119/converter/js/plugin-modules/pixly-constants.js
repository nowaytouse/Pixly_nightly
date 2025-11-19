/**
 * 🎯 Pixly统一常量配置 (JavaScript)
 * 
 * Phase 46.8: 三端共享的常量定义
 * 确保Go/Rust/JS使用相同的参数范围和阈值
 */

/**
 * 参数范围常量
 */
const ParamRanges = {
    // Quality参数范围 (1-100)
    QUALITY_MIN: 1,
    QUALITY_MAX: 100,
    QUALITY_DEFAULT: 85,

    // Speed/Effort参数范围 (0-10)
    SPEED_MIN: 0,
    SPEED_MAX: 10,
    SPEED_DEFAULT: 6,

    // AVIF Quantizer范围 (0-63)
    AVIF_QUANTIZER_MIN: 0,
    AVIF_QUANTIZER_MAX: 63,

    // JXL Distance范围 (0.0-15.0)
    JXL_DISTANCE_MIN: 0.0,
    JXL_DISTANCE_MAX: 15.0,

    // WebP Method范围 (0-6)
    WEBP_METHOD_MIN: 0,
    WEBP_METHOD_MAX: 6,

    // FFmpeg CRF范围 (0-51)
    FFMPEG_CRF_MIN: 0,
    FFMPEG_CRF_MAX: 51,

    // 图像尺寸范围
    IMAGE_DIM_MIN: 1,
    IMAGE_DIM_MAX: 65535,

    // 图像总像素数限制（10亿像素）
    IMAGE_MAX_PIXELS: 1000000000,

    // 文件大小限制 (100MB)
    FILE_SIZE_MAX: 100 * 1024 * 1024,
};

/**
 * AI相关常量
 */
const AIConstants = {
    // AI置信度阈值
    CONFIDENCE_REJECT_THRESHOLD: 0.5,  // 低于此值拒绝
    CONFIDENCE_WARN_THRESHOLD: 0.7,    // 低于此值警告

    // AI服务超时（毫秒）
    AI_TIMEOUT_MS: 30000,

    // AI最大重试次数
    AI_MAX_RETRIES: 3,
};

/**
 * 支持的格式列表
 */
const Formats = {
    // 输入格式
    SUPPORTED_INPUT: [
        'jpg', 'jpeg', 'png', 'webp', 'gif', 'bmp',
        'tiff', 'tif', 'avif', 'jxl', 'heic', 'heif',
    ],

    // 输出格式
    SUPPORTED_OUTPUT: [
        'jpg', 'jpeg', 'png', 'webp', 'avif', 'jxl', 'gif', 'heic',
    ],

    // 支持无损模式的格式
    LOSSLESS_FORMATS: [
        'png', 'webp', 'avif', 'jxl',
    ],
};

/**
 * 工具名称常量
 */
const Tools = {
    CJXL: 'cjxl',
    DJXL: 'djxl',
    AVIFENC: 'avifenc',
    AVIFDEC: 'avifdec',
    CWEBP: 'cwebp',
    DWEBP: 'dwebp',
    FFMPEG: 'ffmpeg',
    MAGICK: 'magick',

    // 所有支持的工具
    ALL: [
        'cjxl', 'djxl', 'avifenc', 'avifdec',
        'cwebp', 'dwebp', 'ffmpeg', 'magick',
    ],
};

/**
 * 验证消息常量
 */
const ValidationMessages = {
    QUALITY_OUT_OF_RANGE: 'Quality参数超出范围',
    SPEED_OUT_OF_RANGE: 'Speed参数超出范围',
    IMAGE_TOO_LARGE: '图像尺寸超限',
    FILE_NOT_FOUND: '文件不存在',
    FORMAT_UNSUPPORTED: '格式不支持',
    AI_CONFIDENCE_LOW: 'AI置信度过低',
};

/**
 * 性能相关常量
 */
const Performance = {
    // 批量处理默认并发数
    DEFAULT_CONCURRENCY: 4,

    // 批量处理最大并发数
    MAX_CONCURRENCY: 16,

    // 单个图像处理超时（毫秒）
    PROCESSING_TIMEOUT_MS: 300000,
};

/**
 * 验证辅助函数
 */
const Validators = {
    /**
     * 验证quality参数
     */
    validateQuality(value) {
        if (value < ParamRanges.QUALITY_MIN || value > ParamRanges.QUALITY_MAX) {
            return {
                valid: false,
                error: `${ValidationMessages.QUALITY_OUT_OF_RANGE} (应为${ParamRanges.QUALITY_MIN}-${ParamRanges.QUALITY_MAX})`,
            };
        }
        return { valid: true };
    },

    /**
     * 验证speed参数
     */
    validateSpeed(value) {
        if (value < ParamRanges.SPEED_MIN || value > ParamRanges.SPEED_MAX) {
            return {
                valid: false,
                error: `${ValidationMessages.SPEED_OUT_OF_RANGE} (应为${ParamRanges.SPEED_MIN}-${ParamRanges.SPEED_MAX})`,
            };
        }
        return { valid: true };
    },

    /**
     * 验证图像尺寸
     */
    validateImageDimensions(width, height) {
        if (width < ParamRanges.IMAGE_DIM_MIN || width > ParamRanges.IMAGE_DIM_MAX ||
            height < ParamRanges.IMAGE_DIM_MIN || height > ParamRanges.IMAGE_DIM_MAX) {
            return {
                valid: false,
                error: `${ValidationMessages.IMAGE_TOO_LARGE} (应为${ParamRanges.IMAGE_DIM_MIN}x${ParamRanges.IMAGE_DIM_MIN} - ${ParamRanges.IMAGE_DIM_MAX}x${ParamRanges.IMAGE_DIM_MAX})`,
            };
        }

        const totalPixels = width * height;
        if (totalPixels > ParamRanges.IMAGE_MAX_PIXELS) {
            return {
                valid: false,
                error: `${ValidationMessages.IMAGE_TOO_LARGE} (总像素: ${totalPixels}, 最大: ${ParamRanges.IMAGE_MAX_PIXELS})`,
            };
        }

        return { valid: true };
    },

    /**
     * 检查输入格式是否支持
     */
    isInputFormatSupported(format) {
        return Formats.SUPPORTED_INPUT.includes(format.toLowerCase());
    },

    /**
     * 检查输出格式是否支持
     */
    isOutputFormatSupported(format) {
        return Formats.SUPPORTED_OUTPUT.includes(format.toLowerCase());
    },

    /**
     * 检查格式是否支持无损模式
     */
    supportsLossless(format) {
        return Formats.LOSSLESS_FORMATS.includes(format.toLowerCase());
    },

    /**
     * 检查工具名称是否有效
     */
    isValidTool(tool) {
        return Tools.ALL.includes(tool.toLowerCase());
    },
};

// 导出
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        ParamRanges,
        AIConstants,
        Formats,
        Tools,
        ValidationMessages,
        Performance,
        Validators,
    };
}

// 全局导出（用于Eagle插件）
if (typeof window !== 'undefined') {
    window.PIXLY = window.PIXLY || {};
    window.PIXLY.ParamRanges = ParamRanges;
    window.PIXLY.AIConstants = AIConstants;
    window.PIXLY.Formats = Formats;
    window.PIXLY.Tools = Tools;
    window.PIXLY.ValidationMessages = ValidationMessages;
    window.PIXLY.Performance = Performance;
    window.PIXLY.Validators = Validators;
}
