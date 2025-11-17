/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * Kernel Bridge - Rust Kernel通信桥接
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🎯 职责:
 * - 统一与Rust Kernel的通信
 * - 标准化请求/响应格式
 * - 错误处理和重试
 * - 不实现任何业务逻辑
 * 
 * 🚫 不做:
 * - 文件读写
 * - 格式转换
 * - AI推荐
 * - 参数计算
 */
(function(window) {
    'use strict';
    
    const log = window.pixlyLog;
    
    /**
     * Kernel Bridge类
     */
    class KernelBridge {
        constructor() {
            this.rustCLI = window.rustCLI;
            this.requestId = 0;
        }
        
        /**
         * 生成请求ID
         */
        generateRequestId() {
            return `req_${Date.now()}_${++this.requestId}`;
        }
        
        /**
         * 执行Rust命令
         */
        async executeCommand(command, params) {
            if (!this.rustCLI || !this.rustCLI.available) {
                throw new Error('Rust CLI不可用');
            }
            
            const requestId = this.generateRequestId();
            log.info('KernelBridge', `执行命令: ${command}`, { requestId, params });
            
            try {
                const result = await this.rustCLI.execute(command, params);
                log.info('KernelBridge', `命令成功: ${command}`, { requestId });
                return result;
            } catch (error) {
                log.error('KernelBridge', `命令失败: ${command}`, { requestId, error: error.message });
                throw error;
            }
        }
        
        /**
         * 🔍 验证转换参数
         * 
         * @param {Array} files - 文件路径列表
         * @param {Object} config - 转换配置
         * @returns {Promise<Object>} 验证结果
         */
        async validateConversion(files, config) {
            log.info('KernelBridge', '调用验证系统', { fileCount: files.length });
            
            const params = {
                input_files: files,
                target_format: config.format,
                quality: config.quality,
                speed: config.speed,
                lossless: config.lossless || false,
                mode: config.mode || 'manual'
            };
            
            return await this.executeCommand('validate', params);
        }
        
        /**
         * 🤖 获取AI参数预测
         * 
         * @param {Object} imageInfo - 图像信息
         * @param {string} targetFormat - 目标格式
         * @param {string} qualityMode - 质量模式
         * @returns {Promise<Object>} AI预测结果
         */
        async predictParameters(imageInfo, targetFormat, qualityMode) {
            log.info('KernelBridge', '调用AI预测', { format: targetFormat, mode: qualityMode });
            
            const params = {
                features: {
                    width: imageInfo.width,
                    height: imageInfo.height,
                    file_size: imageInfo.size,
                    format: imageInfo.format,
                    has_alpha: imageInfo.hasAlpha || false,
                    is_animated: imageInfo.isAnimated || false,
                    complexity: imageInfo.complexity || 0.5
                },
                target_format: targetFormat,
                quality_mode: qualityMode
            };
            
            return await this.executeCommand('predict', params);
        }
        
        /**
         * 🎨 执行图像转换
         * 
         * @param {Array} files - 文件路径列表
         * @param {Object} config - 完整转换配置（包含format_params）
         * @returns {Promise<Object>} 转换结果
         */
        async convertImages(files, config) {
            log.info('KernelBridge', 'Starting image conversion', { 
                fileCount: files.length, 
                format: config.format,
                mode: config.mode 
            });
            
            const params = {
                // 基础参数
                input_files: files.map(f => f.filePath || f.path),
                target_format: config.format,
                quality: config.quality,
                speed: config.speed,
                lossless: config.lossless || false,
                mode: config.mode || 'manual',
                
                // 🔥 格式专属参数 - 直接传递
                format_params: config.format_params || {},
                
                // 元数据选项
                preserve_metadata: config.preserveMetadata !== false,
                keep_animated: config.keepAnimated !== false,
                merge_xmp_sidecar: config.mergeXmpSidecar || false,
                
                // 预处理选项
                resize: config.resize || null,
                quantize: config.quantize || null,
                sharpen: config.sharpen || null,
                resize_filter: config.resizeFilter || 'lanczos3',
                
                // 高级选项
                chroma_subsampling: config.chromaSubsampling || 'auto',
                alpha_quality: config.alphaQuality || null,
                effort: config.effort || null,
                
                // 输出选项
                output_dir: config.outputDir || null,
                normalize_filenames: config.normalizeFilenames || false,
                
                // AI选项
                enable_ai: config.enableAI !== false,
                enable_validation: config.enableValidation !== false,
                
                // 模式标识
                mode: config.mode || 'manual',
                
                // 🎛️ 功能开关传递到Rust Kernel (完整版)
                feature_toggles: {
                    // ═══════════════════════════════════════════════════
                    // 基础功能开关 (HTML界面可见)
                    // ═══════════════════════════════════════════════════
                    enable_ai_prediction: featureToggles.enableAI !== false,
                    enable_validation: featureToggles.enableValidation !== false,
                    enable_file_validation: featureToggles.enableFileValidation === true,
                    preserve_metadata: featureToggles.preserveMetadata !== false,
                    keep_animated: featureToggles.keepAnimated !== false,
                    normalize_filenames: featureToggles.normalizeFilenames === true,
                    
                    // ═══════════════════════════════════════════════════
                    // AI高级选项 (HTML AI标签页)
                    // ═══════════════════════════════════════════════════
                    enable_smart_quality: aiOptions.enable_smart_quality !== false,
                    enable_auto_optimize: aiOptions.enable_auto_optimize !== false,
                    enable_ssim_validation: aiOptions.enable_ssim_validation !== false,
                    enable_video_for_animation: aiOptions.enable_video_for_animation === true,
                    enable_ppo: aiOptions.enable_ppo !== false,
                    expected_format: aiOptions.expected_format || null,
                    
                    // ═══════════════════════════════════════════════════
                    // 内部开关 (bridge内部控制，不暴露到HTML)
                    // ═══════════════════════════════════════════════════
                    
                    // 质量验证 - 内部强制启用
                    enable_quality_validation: true,
                    
                    // 性能优化 - 内部强制启用
                    enable_simd: true,           // SIMD加速
                    enable_cache: true,          // 缓存系统
                    enable_parallel: true,       // 并行处理
                    
                    // 参数控制 - 根据模式自动设置
                    allow_manual_override: config.mode === 'manual' || config.mode === 'ai',
                    enable_advanced_params: config.mode === 'manual' || formatParams !== null,
                    
                    // 超时和重试 - 内部配置
                    ai_prediction_timeout_secs: 30,
                    ai_min_confidence: 0.7,
                    ai_fallback_to_defaults: true,
                    
                    // 并发控制 - 根据系统自动调整
                    max_concurrent_conversions: this.getOptimalConcurrency(),
                    
                    // 验证失败处理 - 内部策略
                    abort_on_validation_failure: false,
                    
                    // 实验性功能 - 内部控制
                    enable_experimental: false,
                },
                
                // 📦 格式专属参数传递到Rust Kernel
                format_specific_params: formatParams
            };
            
            // 🔍 智能清理参数 (移除null/undefined，保持payload精简)
            const cleanedParams = this.cleanParams(params);
            
            // 📊 日志记录
            log.info('KernelBridge', '转换参数摘要', {
                fileCount: files.length,
                format: config.format,
                mode: config.mode,
                quality: config.quality,
                hasFormatParams: Object.keys(formatParams).length > 0,
                aiEnabled: featureToggles.enableAI,
                validationEnabled: featureToggles.enableValidation,
            });
            
            log.debug('KernelBridge', '功能开关详情', featureToggles);
            log.debug('KernelBridge', 'AI选项详情', aiOptions);
            log.debug('KernelBridge', '格式参数详情', formatParams);
            
            return await this.executeCommand('convert', cleanedParams);
        }
        
        /**
         * 🎛️ 获取功能开关状态
         * 从HTML界面读取用户设置
         */
        getFeatureToggles() {
            // 尝试从全局函数获取
            if (typeof window.getFeatureToggles === 'function') {
                return window.getFeatureToggles();
            }
            
            // 回退：从localStorage读取
            const toggles = {
                enableValidation: true,
                enableAI: true,
                enableFileValidation: false,
                preserveMetadata: true,
                keepAnimated: true,
                normalizeFilenames: false
            };
            
            Object.keys(toggles).forEach(key => {
                const saved = localStorage.getItem(`pixly_${key}`);
                if (saved !== null) {
                    toggles[key] = saved === 'true';
                }
            });
            
            return toggles;
        }
        
        /**
         * 🤖 收集AI选项 (从HTML界面)
         * 带健壮的默认值处理
         */
        collectAIOptions() {
            const getCheckboxValue = (id, defaultValue = true) => {
                const el = document.getElementById(id);
                return el ? el.checked : defaultValue;
            };
            
            const getSelectValue = (id, defaultValue = '') => {
                const el = document.getElementById(id);
                return el ? el.value : defaultValue;
            };
            
            return {
                // 智能质量预测 - 默认启用
                enable_smart_quality: getCheckboxValue('enableSmartQuality', true),
                
                // 自动参数优化 - 默认启用
                enable_auto_optimize: getCheckboxValue('enableAutoOptimize', true),
                
                // SSIM质量验证 - 默认启用
                enable_ssim_validation: getCheckboxValue('enableSSIMValidation', true),
                
                // 动画转视频推荐 - 默认禁用
                enable_video_for_animation: getCheckboxValue('enableVideoForAnimation', false),
                
                // PPO强化学习 - 默认启用 (隐藏checkbox)
                enable_ppo: getCheckboxValue('enablePPO', true),
                
                // 自定义期望格式 - 默认空 (自动推断)
                expected_format: getSelectValue('expectedFormatSelect', ''),
            };
        }
        
        /**
         * 📦 收集格式专属参数 (从HTML界面)
         * 带健壮的默认值处理和类型转换
         */
        collectFormatParams(format) {
            const getValue = (id, defaultValue = null) => {
                const el = document.getElementById(id);
                if (!el) return defaultValue;
                
                const value = el.value;
                // 尝试转换为数字
                if (value && !isNaN(value)) {
                    return parseFloat(value);
                }
                return value || defaultValue;
            };
            
            const getCheckbox = (id, defaultValue = false) => {
                const el = document.getElementById(id);
                return el ? el.checked : defaultValue;
            };
            
            const params = {};
            
            // JXL参数 (JPEG XL)
            if (format === 'jxl') {
                const jxlParams = {
                    effort: getValue('jxlEffort', 7),
                    distance: getValue('jxlDistance', 1.0),
                    bit_depth: getValue('jxlBitDepth', 8),
                    color_space: getValue('jxlColorSpace', 'sRGB'),
                    patches: getValue('jxlPatches', 1),
                };
                
                // 只添加非默认值
                if (Object.values(jxlParams).some(v => v !== null)) {
                    params.jxl = jxlParams;
                }
            }
            
            // WebP参数
            if (format === 'webp') {
                const webpParams = {
                    method: getValue('webpMethod', 4),
                    filter_strength: getValue('webpFilterStrength', 60),
                    sharpness: getValue('webpSharpness', 0),
                    segments: getValue('webpSegments', 4),
                    sns_strength: getValue('webpSnsStrength', 50),
                    auto_filter: getCheckbox('webpAutoFilter', true),
                    exact_mode: getCheckbox('webpExactMode', false),
                    no_alpha: getCheckbox('webpNoAlpha', false),
                    low_memory: getCheckbox('webpLowMemory', false),
                    pass: getValue('webpPass', 1),
                };
                
                if (Object.values(webpParams).some(v => v !== null)) {
                    params.webp = webpParams;
                }
            }
            
            // AVIF参数
            if (format === 'avif') {
                const avifParams = {
                    speed: getValue('avifSpeed', 6),
                    min_quantizer: getValue('avifMinQuantizer', 0),
                    max_quantizer: getValue('avifMaxQuantizer', 63),
                    chroma_subsampling: getValue('avifChromaSubsampling', '420'),
                    bit_depth: getValue('avifBitDepth', 8),
                    tiles_rows: getValue('avifTilesRows', 1),
                    tiles_cols: getValue('avifTilesCols', 1),
                    premultiply_alpha: getCheckbox('avifPremultiply', false),
                };
                
                if (Object.values(avifParams).some(v => v !== null)) {
                    params.avif = avifParams;
                }
            }
            
            // HEIC参数
            if (format === 'heic') {
                const heicParams = {
                    quality: getValue('heicQuality', 85),
                    encoder: getValue('heicEncoder', 'x265'),
                    chroma_subsampling: getValue('heicChromaSubsampling', '444'),
                    lossless: getCheckbox('heicLossless', false),
                    embed_thumbnail: getCheckbox('heicThumbEmbed', true),
                };
                
                if (Object.values(heicParams).some(v => v !== null)) {
                    params.heic = heicParams;
                }
            }
            
            return params;
        }
        
        /**
         * 🔧 获取最优并发数 (内部方法)
         * 根据系统CPU核心数自动调整
         */
        getOptimalConcurrency() {
            // 尝试获取CPU核心数
            const cores = navigator.hardwareConcurrency || 4;
            
            // 策略：使用核心数的75%，最少2个，最多8个
            const optimal = Math.max(2, Math.min(8, Math.floor(cores * 0.75)));
            
            log.debug('KernelBridge', `最优并发数: ${optimal} (CPU核心: ${cores})`);
            return optimal;
        }
        
        /**
         * 🔍 智能参数清理 (内部方法)
         * 移除null/undefined值，保持payload精简
         */
        cleanParams(params) {
            const cleaned = {};
            
            for (const [key, value] of Object.entries(params)) {
                if (value === null || value === undefined) {
                    continue;
                }
                
                // 递归清理嵌套对象
                if (typeof value === 'object' && !Array.isArray(value)) {
                    const cleanedNested = this.cleanParams(value);
                    if (Object.keys(cleanedNested).length > 0) {
                        cleaned[key] = cleanedNested;
                    }
                } else {
                    cleaned[key] = value;
                }
            }
            
            return cleaned;
        }
        
        /**
         * 🎬 执行视频转换
         * 
         * @param {Array} files - 文件路径列表
         * @param {Object} config - 转换配置
         * @returns {Promise<Object>} 转换结果
         */
        async convertVideos(files, config) {
            log.info('KernelBridge', '开始视频转换', { fileCount: files.length, format: config.format });
            
            const params = {
                input_files: files,
                target_format: config.format,
                quality: config.quality,
                speed: config.speed,
                codec: config.codec,
                output_dir: config.outputDir
            };
            
            return await this.executeCommand('convert_video', params);
        }
        
        /**
         * 🎵 执行音频转换
         * 
         * @param {Array} files - 文件路径列表
         * @param {Object} config - 转换配置
         * @returns {Promise<Object>} 转换结果
         */
        async convertAudio(files, config) {
            log.info('KernelBridge', '开始音频转换', { fileCount: files.length, format: config.format });
            
            const params = {
                input_files: files,
                target_format: config.format,
                quality: config.quality,
                bitrate: config.bitrate,
                output_dir: config.outputDir
            };
            
            return await this.executeCommand('convert_audio', params);
        }
        
        /**
         * 📊 获取文件信息
         * 
         * @param {string} filePath - 文件路径
         * @returns {Promise<Object>} 文件信息
         */
        async getFileInfo(filePath) {
            log.info('KernelBridge', '获取文件信息', { file: filePath });
            
            return await this.executeCommand('analyze', {
                file_path: filePath
            });
        }
        
        /**
         * 🔍 批量获取文件信息
         * 
         * @param {Array} files - 文件路径列表
         * @returns {Promise<Array>} 文件信息列表
         */
        async batchGetFileInfo(files) {
            log.info('KernelBridge', '批量获取文件信息', { fileCount: files.length });
            
            return await this.executeCommand('batch_analyze', {
                file_paths: files
            });
        }
        
        /**
         * 📈 获取转换进度
         * 
         * @param {string} taskId - 任务ID
         * @returns {Promise<Object>} 进度信息
         */
        async getProgress(taskId) {
            return await this.executeCommand('get_progress', {
                task_id: taskId
            });
        }
        
        /**
         * ❌ 取消转换
         * 
         * @param {string} taskId - 任务ID
         * @returns {Promise<Object>} 取消结果
         */
        async cancelConversion(taskId) {
            log.info('KernelBridge', '取消转换', { taskId });
            
            return await this.executeCommand('cancel', {
                task_id: taskId
            });
        }
        
        /**
         * 🔧 检查依赖
         * 
         * @returns {Promise<Object>} 依赖检查结果
         */
        async checkDependencies() {
            log.info('KernelBridge', '检查依赖');
            
            return await this.executeCommand('check_deps', {});
        }
        
        /**
         * 📝 获取支持的格式
         * 
         * @returns {Promise<Object>} 支持的格式列表
         */
        async getSupportedFormats() {
            return await this.executeCommand('list_formats', {});
        }
        
        /**
         * 🚀 统一转换入口（主方法）
         * 
         * @param {Array} files - 文件列表
         * @param {Object} config - 完整配置（包含format_params）
         * @returns {Promise<Object>} 转换结果
         */
        async convert(files, config) {
            return await this.convertImages(files, config);
        }
    }
    
    // 导出到全局
    window.PIXLY = window.PIXLY || {};
    window.PIXLY.KernelBridge = KernelBridge;
    
    // 创建单例
    window.kernelBridge = new KernelBridge();
    window.PIXLY.kernelBridge = window.kernelBridge;
    
    if (log) {
        log.info('KernelBridge', '✅ Kernel Bridge已加载');
    }
    
})(window);
