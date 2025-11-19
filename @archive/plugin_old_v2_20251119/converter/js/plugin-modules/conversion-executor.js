/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * Conversion Executor
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🎯 职责:
 * - 执行转换流程
 * - 管理转换状态
 * - 处理转换结果
 * 
 * 🚫 不做:
 * - 文件读写（由kernel完成）
 * - 参数收集（由collector完成）
 * - AI预测（由kernel完成）
 */
(function(window) {
    'use strict';
    
    const log = window.pixlyLog;
    const LOG = window.LOG_CONSTANTS || {};
    
    // 转换状态
    let conversionCancelled = false;
    
    /**
     * 启动转换流程
     */
    async function startConversion() {
        // 🔥 Phase 46.2: 防止重复点击 - 转换锁
        if (window.PIXLY_CONVERTING === true) {
            if (log) {
                log.warn('PIXLY', LOG.IMAGE_CONV_DUPLICATE_CALL || 'Duplicate conversion call blocked');
            }
            return;
        }
        
        // 设置转换锁
        window.PIXLY_CONVERTING = true;
        conversionCancelled = false;
        
        try {
            // 获取选中的文件
            const selectedFiles = window.selectedFiles || [];
            if (!selectedFiles || selectedFiles.length === 0) {
                throw new Error('No files selected');
            }
            
            // 收集配置
            const config = window.ConversionConfigCollector.getConversionConfig();
            
            // 收集完整参数
            const fullParams = window.FormatParamsCollector.collectAllConversionParams(config);
            
            if (log) {
                log.info('Conversion', 'Starting conversion', {
                    fileCount: selectedFiles.length,
                    format: config.format,
                    mode: config.mode
                });
            }
            
            // 调用kernel bridge执行转换
            const result = await window.kernelBridge.convert(selectedFiles, fullParams);
            
            if (log) {
                log.info('Conversion', 'Conversion completed', {
                    successful: result.successful,
                    failed: result.failed
                });
            }
            
            // 显示结果
            if (window.addLog) {
                window.addLog(`✅ Conversion completed: ${result.successful} successful, ${result.failed} failed`, 'success');
            }
            
            return result;
            
        } catch (error) {
            if (log) {
                log.error('Conversion', 'Conversion failed', { error: error.message });
            }
            
            if (window.addLog) {
                window.addLog(`❌ Conversion failed: ${error.message}`, 'error');
            }
            
            throw error;
            
        } finally {
            // 释放转换锁
            window.PIXLY_CONVERTING = false;
        }
    }
    
    /**
     * 取消转换
     */
    function cancelConversion() {
        conversionCancelled = true;
        
        if (log) {
            log.warn('Conversion', LOG.IMAGE_CONV_CANCEL_REQUESTED || 'Conversion cancel requested');
        }
        
        if (window.addLog) {
            window.addLog('Cancelling conversion...', 'warning');
        }
        
        // 通知kernel取消
        if (window.kernelBridge && window.kernelBridge.cancel) {
            window.kernelBridge.cancel();
        }
    }
    
    /**
     * 检查是否已取消
     */
    function isCancelled() {
        return conversionCancelled;
    }
    
    /**
     * 打开输出文件夹
     */
    async function openOutputFolder() {
        const selectedFiles = window.selectedFiles || [];
        
        if (!selectedFiles || selectedFiles.length === 0) {
            if (log) {
                log.warn('Conversion', 'No files to locate');
            }
            if (window.addLog) {
                window.addLog('No files to locate', 'warning');
            }
            return;
        }
        
        try {
            // 获取第一个文件的路径
            const firstFile = selectedFiles[0];
            const folderPath = firstFile.filePath.substring(0, firstFile.filePath.lastIndexOf('/'));
            
            if (log) {
                log.info('Conversion', 'Opening folder', { path: folderPath });
            }
            
            // 调用Eagle API打开文件夹
            if (window.eagle && window.eagle.app) {
                await window.eagle.app.openPath(folderPath);
                if (log) {
                    log.info('Conversion', 'Folder opened successfully');
                }
            } else {
                if (log) {
                    log.warn('Conversion', 'Eagle API not available');
                }
            }
        } catch (error) {
            if (log) {
                log.error('Conversion', 'Failed to open folder', { error: error.message });
            }
        }
    }
    
    // 导出到全局
    window.ConversionExecutor = {
        startConversion,
        cancelConversion,
        isCancelled,
        openOutputFolder
    };
    
})(window);
