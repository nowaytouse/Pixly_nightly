/**
 * PIXLY - 视频AI客户端模块 v3.0
 * 通过GO核心API调用Python v3.0脚本进行视频AI预测
 * 架构：插件 → GO核心API → GO调用Python v3.0 → 返回增强结果
 * 
 * 🔥 Phase 45.9 新特性:
 * - 视频类型识别（动画/真人/游戏/电影/纪录片/屏幕录制）
 * - 智能码率预测
 * - 两遍编码推荐
 * - 分辨率缩放建议
 */

(function() {
    'use strict';
    
    // 视频AI客户端（封装GO核心调用）
    const VideoAIClient = {
        /**
         * 预测视频编码参数
         * @param {string} videoPath - 视频文件路径
         * @param {string} optimizeMode - 优化模式 ('balanced', 'size', 'quality')
         * @param {object} options - 可选参数
         * @returns {Promise<object>} 预测结果
         */
        async predictVideoParams(videoPath, optimizeMode = 'balanced', options = {}) {
            const Logger = window.PIXLY?.Logger || console;
            
            try {
                // 🔥 Phase 45.9: 读取UI开关状态（如果未显式指定）
                const enableAdvancedFeatures = document.getElementById('enableAdvancedFeatures');
                const forceTransformer = document.getElementById('forceTransformer');
                const enableVMAF = document.getElementById('enableVMAF');
                
                // 读取优化模式预设（如果UI没有传入）
                if (!options.optimizeMode && !optimizeMode) {
                    const selectedPreset = document.querySelector('input[name="videoAIPreset"]:checked');
                    if (selectedPreset) {
                        optimizeMode = selectedPreset.value; // fast, balanced, quality
                        Logger.info(`[Video AI] 📋 UI预设模式: ${optimizeMode}`);
                    }
                }
                
                // 合并UI状态和options参数
                const finalOptions = {
                    use_advanced_ai: enableAdvancedFeatures?.checked !== false,  // 默认true（场景检测+特征分析）
                    enable_transformer: forceTransformer?.checked || false,      // 默认false（强制Transformer精细处理）
                    enable_vmaf: enableVMAF?.checked || false,                   // 默认false（VMAF质量验证）
                    ...options  // options可以覆盖UI设置
                };
                
                Logger.info(`[Video AI] 🎬 预测视频参数: ${videoPath}`);
                Logger.info(`[Video AI] 🎯 优化模式: ${optimizeMode}`);
                Logger.info(`[Video AI] 🔧 高级AI: ${finalOptions.use_advanced_ai ? 'ON' : 'OFF'}`);
                Logger.info(`[Video AI] 🔮 Transformer: ${finalOptions.enable_transformer ? 'ON' : 'OFF'}`);
                
                if (finalOptions.enable_vmaf) {
                    Logger.info(`[Video AI] 📊 VMAF validation: ON`);
                }
                
                // 🔥 响亮报错：AIClient未初始化
                if (!window.PIXLY?.AIClient || !window.aiClient) {
                    Logger.error('[Video AI] ❌ AIClient未初始化');
                    throw new Error(
                        '🚨 AI客户端未初始化！\n\n' +
                        '无法进行视频AI预测，转换已中止。\n\n' +
                        '【质量宣言】响亮报错 > 静默降级'
                    );
                }
                
                const result = await window.aiClient.predictVideoParams(
                    videoPath,
                    optimizeMode,
                    finalOptions
                );
                
                if (result.success) {
                    Logger.info(`[Video AI] ✅ AI预测成功`);
                    Logger.info(`[Video AI] 📊 编码器: ${result.params.encoder}`);
                    Logger.info(`[Video AI] 📊 CRF: ${result.params.crf}`);
                    Logger.info(`[Video AI] 📊 Preset: ${result.params.preset}`);
                    
                    // 🔥 Phase 45.9: 新增特性输出
                    if (result.video_type) {
                        Logger.info(`[Video AI] 🎭 视频类型: ${result.video_type.type} (置信度: ${(result.video_type.confidence * 100).toFixed(1)}%)`);
                    }
                    
                    if (result.params.target_bitrate) {
                        Logger.info(`[Video AI] 📊 目标码率: ${result.params.target_bitrate}kbps`);
                    }
                    
                    if (result.params.max_bitrate) {
                        Logger.info(`[Video AI] 📊 最大码率: ${result.params.max_bitrate}kbps`);
                    }
                    
                    if (result.params.scale) {
                        Logger.info(`[Video AI] 💡 分辨率建议: ${result.params.scale}`);
                    }
                    
                    if (result.params.two_pass) {
                        Logger.info(`[Video AI] 💡 推荐两遍编码（更好的质量/码率平衡）`);
                    }
                    
                    Logger.info(`[Video AI] 📊 置信度: ${(result.confidence * 100).toFixed(1)}%`);
                } else {
                    // 🔥 响亮报错：AI预测失败
                    Logger.error('[Video AI] ❌ AI预测返回失败');
                    throw new Error(
                        '🚨 视频AI预测失败！\n\n' +
                        '请检查GO核心日志查看详细原因\n\n' +
                        '【质量宣言】响亮报错 > 静默降级'
                    );
                }
                
                return result;
                
            } catch (error) {
                // 🔥 响亮报错：AI调用异常
                Logger.error(`[Video AI] ❌ 预测异常: ${error.message}`);
                // 重新抛出错误，不要fallback
                throw new Error(
                    `🚨 视频AI调用失败！\n\n` +
                    `${error.message}\n\n` +
                    `请检查GO核心日志\n\n` +
                    '【质量宣言】响亮报错 > 静默降级'
                );
            }
        },
        
        /**
         * 验证视频质量（VMAF）
         * @param {string} originalPath - 原始视频路径
         * @param {string} convertedPath - 转换后视频路径
         * @param {object} options - 验证选项
         * @returns {Promise<object>} VMAF验证结果
         */
        async validateVMAF(originalPath, convertedPath, options = {}) {
            const Logger = window.PIXLY?.Logger || console;
            
            try {
                Logger.info(`[Video AI] 📊 VMAF质量验证...`);
                
                // 调用GO核心API（GO核心会调度Python/ffmpeg）
                if (!window.PIXLY?.AIClient || !window.aiClient) {
                    Logger.warn('[Video AI] ⚠️ AIClient未初始化，跳过VMAF验证');
                    return {
                        success: false,
                        error: 'AIClient not initialized',
                        score: null,
                        passed: null
                    };
                }
                
                const result = await window.aiClient.validateVMAF(
                    originalPath,
                    convertedPath,
                    options
                );
                
                if (result.success) {
                    Logger.info(`[Video AI] ✅ VMAF验证完成: ${result.score.toFixed(2)}`);
                    Logger.info(`[Video AI] 📊 评级: ${result.details?.rating || 'N/A'}`);
                    Logger.info(`[Video AI] 📊 通过: ${result.passed ? '✅' : '❌'}`);
                } else {
                    Logger.warn(`[Video AI] ⚠️ VMAF验证失败: ${result.error}`);
                }
                
                return result;
                
            } catch (error) {
                Logger.error(`[Video AI] ❌ VMAF验证异常: ${error.message}`);
                return {
                    success: false,
                    error: error.message,
                    score: null,
                    passed: null
                };
            }
        },
        
        /**
         * 获取当前AI设置
         */
        getCurrentSettings() {
            return {
                enabled: document.getElementById('enableVideoAI')?.checked || false,
                advancedFeatures: document.getElementById('enableAdvancedFeatures')?.checked || false,
                forceTransformer: document.getElementById('forceTransformer')?.checked || false,
                enableVMAF: document.getElementById('enableVMAF')?.checked || false
            };
        },
        
        /**
         * 应用预设配置
         */
        applyPreset(preset) {
            const advancedFeatures = document.getElementById('enableAdvancedFeatures');
            const forceTransformer = document.getElementById('forceTransformer');
            const enableVMAF = document.getElementById('enableVMAF');
            
            switch(preset) {
                case 'fast':
                    if (advancedFeatures) advancedFeatures.checked = false;
                    if (forceTransformer) forceTransformer.checked = false;
                    if (enableVMAF) enableVMAF.checked = false;
                    break;
                case 'balanced':
                    if (advancedFeatures) advancedFeatures.checked = true;
                    if (forceTransformer) forceTransformer.checked = false;
                    if (enableVMAF) enableVMAF.checked = false;
                    break;
                case 'quality':
                    if (advancedFeatures) advancedFeatures.checked = true;
                    if (forceTransformer) forceTransformer.checked = true;
                    if (enableVMAF) enableVMAF.checked = false;
                    break;
            }
            const log = window.pixlyLog;
            if (log) {
                log.info('Video AI', formatLog(LOG.VIDEO_AI_CLIENT_PRESET_APPLIED, { preset }));
            }
        },
        
        /**
         * ❌ DELETED: getFallbackParams
         *
         * 【质量宣言执行】
         * - ❌ Fallback代码（让AI成为摆设）
         * - ✅ 响亮报错 > 静默降级
         *
         * AI不可用时，应该直接抛出错误，而不是静默降级到hardcode参数。
         */
    };
    
    // 暴露到全局
    if (!window.PIXLY) {
        window.PIXLY = {};
    }
    
    window.PIXLY.VideoAIClient = VideoAIClient;
    
    const log = window.pixlyLog;
    if (log) {
        log.info('PIXLY Video AI', formatLog(LOG.VIDEO_AI_CLIENT_MODULE_LOADED, {}));
    }                                                              
    
})();
