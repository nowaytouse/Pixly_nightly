// PIXLY File Validator - Last Modified: 2025-11-10 14:15:24
// Version: 3.0.0-fixed

/*
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 🔒 Pixly File Validator - AI-Powered File Type Validation
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🎯 功能：
 * - AI文件类型检测（Magika集成）
 * - 伪装文件检测
 * - 批量验证
 * - UI警告提示
 * 
 * 🔗 技术栈：
 * - Rust CLI (pixly-rust detect)
 * - Eagle Plugin API
 * - Magika AI (后端)
 * 
 * 📊 Phase 45.4: Eagle Plugin UI 集成
 * 
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 */

(function() {
'use strict';

// Log instance for the module
const log = window.pixlyLog;

/**
 * 文件验证器类
 */
class FileValidator {
    constructor() {
        this.enabled = true;  // 是否启用AI检测
        this.strictMode = false;  // 严格模式（检测到不匹配时拒绝）
        this.cache = new Map();  // 检测结果缓存
        this.formatCorrections = new Map();  // 记录需要修正的文件 {filePath: {from: 'jpg', to: 'png'}}
    }
    
    /**
     * 验证单个文件
     * 
     * @param {Object} file - Eagle 文件对象
     * @returns {Promise<Object>} 验证结果
     */
    async validateFile(file) {
        // 🔥 Phase 46.3: 从UI读取启用状态
        const enableFileValidation = document.getElementById('enableFileValidation')?.checked ?? true;
        if (!this.enabled && !enableFileValidation) {
            return {
                success: true,
                file: file,
                skipped: true,
                message: 'AI detection disabled'
            };
        }
        
        // 检查缓存
        const cacheKey = file.filePath;
        if (this.cache.has(cacheKey)) {
            const cached = this.cache.get(cacheKey);
            if (log) {
                log.debug('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_CACHED, { file: file.name }));
            }
            return cached;
        }
        
        try {
            if (log) {
                log.debug('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_VALIDATING, { file: file.name }));
            }
            
            // 调用 Rust CLI 进行检测
            const detection = await this.detectFileType(file.filePath);
            
            // 分析结果
            const result = this.analyzeDetection(file, detection);
            
            // 缓存结果
            this.cache.set(cacheKey, result);
            
            return result;
        } catch (error) {
            if (log) {
                log.error('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_FAILED, { error: error.message || error }));
            }
            return {
                success: false,
                file: file,
                error: error.message,
                warnings: [`检测失败: ${error.message}`]
            };
        }
    }
    
    /**
     * 批量验证文件
     * 
     * @param {Array} files - Eagle 文件数组
     * @param {Function} onProgress - 进度回调
     * @returns {Promise<Object>} 验证结果汇总
     */
    async validateBatch(files, onProgress) {
        const results = {
            total: files.length,
            validated: 0,
            passed: 0,
            warned: 0,
            failed: 0,
            suspicious: 0,
            details: []
        };
        
        if (log) {
            log.info('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_BATCH_START, { count: files.length }));
        }
        
        for (let i = 0; i < files.length; i++) {
            const file = files[i];
            const result = await this.validateFile(file);
            
            results.validated++;
            results.details.push(result);
            
            if (result.success) {
                if (result.warnings && result.warnings.length > 0) {
                    results.warned++;
                } else {
                    results.passed++;
                }
                
                if (result.suspicious) {
                    results.suspicious++;
                }
            } else {
                results.failed++;
            }
            
            // 进度回调
            if (onProgress) {
                onProgress({
                    current: i + 1,
                    total: files.length,
                    percent: ((i + 1) / files.length * 100).toFixed(1),
                    file: file.name
                });
            }
        }
        
        if (log) {
            log.info('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_BATCH_COMPLETE, { results: JSON.stringify(results.slice(0, 3)) }));
        }
        return results;
    }
    
    /**
     * 调用 Rust CLI 检测文件类型
     * 
     * @param {string} filePath - 文件路径
     * @returns {Promise<Object>} 检测结果
     */
    async detectFileType(filePath) {
        try {
            // 使用 pixly-rust detect 命令（JSON输出）
            // 🔥 Phase 46.1: 修复方法名 → exec（正确方法）
            // 🔥 Phase 46.2: exec()直接返回stdout字符串，不是{success, stdout}对象
            const stdout = await window.rustCLI.exec('detect', [
                filePath,
                '--json',
                '--security'
            ]);
            
            // 解析 JSON 输出
            const lines = stdout.split('\n');
            let detectionJson = null;
            let securityJson = null;
            
            // 查找 JSON 对象
            let currentJson = '';
            let braceCount = 0;
            
            for (const line of lines) {
                if (line.includes('{')) {
                    braceCount += (line.match(/{/g) || []).length;
                }
                if (line.includes('}')) {
                    braceCount -= (line.match(/}/g) || []).length;
                }
                
                if (braceCount > 0 || line.includes('{') || line.includes('}')) {
                    currentJson += line;
                    
                    if (braceCount === 0 && currentJson.includes('}')) {
                        try {
                            const parsed = JSON.parse(currentJson);
                            if (!detectionJson) {
                                detectionJson = parsed;
                            } else if (!securityJson) {
                                securityJson = parsed;
                            }
                            currentJson = '';
                        } catch (e) {
                            // JSON 未完整，继续累积
                        }
                    }
                }
            }
            
            return {
                detection: detectionJson || {},
                security: securityJson || {}
            };
        } catch (error) {
            if (log) {
                log.error('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_RUST_CLI_FAILED, { error: error.message || error }));
            }
            throw error;
        }
    }
    
    /**
     * 分析检测结果
     * 
     * @param {Object} file - Eagle 文件对象
     * @param {Object} detection - 检测结果
     * @returns {Object} 分析结果
     */
    analyzeDetection(file, detection) {
        const warnings = [];
        let suspicious = false;
        let success = true;
        
        const det = detection.detection || {};
        const sec = detection.security || {};
        
        // 检查置信度
        if (det.confidence && det.confidence < 0.9) {
            warnings.push(`⚠️ 低置信度检测: ${(det.confidence * 100).toFixed(1)}%`);
        }
        
        // 检查类型匹配
        if (sec.type_match === false) {
            warnings.push(`⚠️ 文件类型不匹配: 扩展名为 .${file.ext}，但检测为 ${det.detected_type}`);
            suspicious = true;
            
            // 🔥 Phase 46.3: 记录需要修正的文件
            const currentExt = (file.ext || '').toLowerCase().replace(/^\./, '');
            const detectedExt = (det.detected_type || '').toLowerCase();
            if (currentExt && detectedExt && currentExt !== detectedExt) {
                this.formatCorrections.set(file.filePath, {
                    from: currentExt,
                    to: detectedExt,
                    fileName: file.name
                });
                if (log) {
                    log.info('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_FORMAT_MISMATCH, { file: file.name, from: currentExt, to: detectedExt }));
                }
            }
        }
        
        // 检查可疑标记
        if (sec.is_suspicious) {
            warnings.push('🚨 检测到可疑文件');
            suspicious = true;
        }
        
        // 检查安全性
        if (sec.is_safe === false) {
            warnings.push('❌ 文件可能不安全');
            if (this.strictMode) {
                success = false;
            }
        }
        
        // 添加安全验证的警告
        if (sec.warnings && Array.isArray(sec.warnings)) {
            warnings.push(...sec.warnings);
        }
        
        return {
            success: success,
            file: file,
            detectedType: det.detected_type,
            confidence: det.confidence,
            mimeType: det.mime_type,
            description: det.description,
            isBinary: det.is_binary,
            typeMatch: sec.type_match,
            isSafe: sec.is_safe,
            suspicious: suspicious,
            warnings: warnings
        };
    }
    
    /**
     * 🔥 Phase 46.3: 执行格式修正
     * 
     * @param {string} convertedFilePath - 转换后的文件路径
     * @param {boolean} enabled - 是否启用格式修正
     * @returns {Promise<Object>} 修正结果
     */
    async applyFormatCorrection(convertedFilePath, enabled = true) {
        if (!enabled) {
            if (log) {
                log.info('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_CORRECTION_DISABLED, {}));
            }
            return { corrected: false, reason: 'disabled' };
        }
        
        // 查找原始文件路径（去掉新扩展名，找到对应的原始路径）
        const path = require('path');
        const dir = path.dirname(convertedFilePath);
        const baseName = path.basename(convertedFilePath, path.extname(convertedFilePath));
        
        // 在记录中查找匹配的原始文件
        let matchedEntry = null;
        for (const [originalPath, correction] of this.formatCorrections.entries()) {
            const originalBase = path.basename(originalPath, path.extname(originalPath));
            if (originalBase === baseName && path.dirname(originalPath) === dir) {
                matchedEntry = { originalPath, correction };
                break;
            }
        }
        
        if (!matchedEntry) {
            if (log) {
                log.info('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_NO_CORRECTION, { file: convertedFilePath }));
            }
            return { corrected: false, reason: 'no_mismatch' };
        }
        
        const { originalPath, correction } = matchedEntry;
        const { from, to, fileName } = correction;
        
        if (log) {
            log.info('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_APPLYING_CORRECTION, { file: fileName, from, to }));
        }
        
        // 检查转换后的文件扩展名
        const convertedExt = path.extname(convertedFilePath).toLowerCase().replace(/^\./, '');
        
        // 如果转换后的扩展名与检测到的类型不符，需要修正
        if (convertedExt !== to) {
            const fs = require('fs');
            const correctedPath = path.join(dir, `${baseName}.${to}`);
            
            try {
                // 重命名文件
                fs.renameSync(convertedFilePath, correctedPath);
                if (log) {
                    log.info('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_RENAME_SUCCESS, { from: path.basename(convertedFilePath), to: path.basename(correctedPath) }));
                }
                
                // 🔥 更新 Eagle metadata.json
                const eagleUpdateResult = await this.updateEagleMetadata(correctedPath, to);
                
                // 清除记录
                this.formatCorrections.delete(originalPath);
                
                return {
                    corrected: true,
                    from: convertedExt,
                    to: to,
                    oldPath: convertedFilePath,
                    newPath: correctedPath,
                    eagleUpdated: eagleUpdateResult
                };
            } catch (error) {
                if (log) {
                    log.error('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_RENAME_FAILED, { error: error.message || error }));
                }
                return {
                    corrected: false,
                    reason: 'error',
                    error: error.message
                };
            }
        } else {
            if (log) {
                log.info('PIXLY Validator', `Format already correct: .${convertedExt}`);
            }
            this.formatCorrections.delete(originalPath);
            return { corrected: false, reason: 'already_correct' };
        }
    }
    
    /**
     * 🔥 Phase 46.3: 更新 Eagle metadata.json
     * 
     * @param {string} filePath - 文件路径
     * @param {string} newExt - 新扩展名
     * @returns {Promise<boolean>} 是否成功
     */
    async updateEagleMetadata(filePath, newExt) {
        const path = require('path');
        const fs = require('fs');
        const { promisify } = require('util');
        const readFileAsync = promisify(fs.readFile);
        const writeFileAsync = promisify(fs.writeFile);
        
        // 查找 .info 目录
        const fileName = path.basename(filePath);
        const parentDir = path.dirname(filePath);
        
        if (!parentDir.endsWith('.info')) {
            if (log) {
                log.info('PIXLY Validator', 'Not in Eagle .info directory, skipping metadata update');
            }
            return false;
        }
        
        const metadataPath = path.join(parentDir, 'metadata.json');
        if (!fs.existsSync(metadataPath)) {
            if (log) {
                log.warn('PIXLY Validator', `metadata.json not found: ${metadataPath}`);
            }
            return false;
        }
        
        // 🔥 Phase 46.4: 重试机制，防止文件锁定冲突
        const maxRetries = 3;
        const retryDelay = 300; // ms
        
        for (let attempt = 1; attempt <= maxRetries; attempt++) {
            try {
                // 异步读取 metadata.json
                const content = await readFileAsync(metadataPath, 'utf8');
                const metadata = JSON.parse(content);
                
                // 更新扩展名和文件名
                metadata.ext = `.${newExt}`;
                metadata.name = path.basename(fileName, path.extname(fileName));
                
                // 异步写回，使用临时文件+原子性rename
                const tempPath = metadataPath + '.tmp';
                await writeFileAsync(tempPath, JSON.stringify(metadata, null, 2), 'utf8');
                
                // 原子性替换（减少锁定风险）
                fs.renameSync(tempPath, metadataPath);
                
                if (log) {
                    log.info('PIXLY Validator', `Eagle metadata.json updated (attempt ${attempt}): ${metadataPath}`);
                }
                return true;
            } catch (error) {
                if (log) {
                    log.warn('PIXLY Validator', `Attempt ${attempt}/${maxRetries} failed: ${error.message}`);
                }
                
                if (attempt < maxRetries) {
                    // 等待后重试
                    await new Promise(resolve => setTimeout(resolve, retryDelay * attempt));
                } else {
                    // 最后一次失败
                    if (log) {
                        log.error('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_METADATA_ERROR, { error: `Failed after all retries: ${error.message || error}` }));
                    }
                    return false;
                }
            }
        }
        
        return false;
    }
    
    /**
     * 显示验证警告对话框
     * 
     * @param {Array} warnings - 警告列表
     * @param {string} fileName - 文件名
     * @returns {Promise<boolean>} 用户是否确认继续
     */
    async showWarningDialog(warnings, fileName) {
        const message = `文件 "${fileName}" 检测到以下问题：\n\n` +
            warnings.map(w => `• ${w}`).join('\n') +
            '\n\n是否继续转换？';
        
        return await eagle.dialog.confirm({
            title: '⚠️ 文件验证警告',
            message: message,
            okText: '继续转换',
            cancelText: '取消'
        });
    }
    
    /**
     * 显示批量验证结果
     * 
     * @param {Object} results - 批量验证结果
     */
    showBatchResults(results) {
        let message = `批量验证完成：\n\n`;
        message += `✅ 通过: ${results.passed}\n`;
        
        if (results.warned > 0) {
            message += `⚠️ 警告: ${results.warned}\n`;
        }
        if (results.suspicious > 0) {
            message += `🚨 可疑: ${results.suspicious}\n`;
        }
        if (results.failed > 0) {
            message += `❌ 失败: ${results.failed}\n`;
        }
        
        if (results.suspicious > 0 || results.failed > 0) {
            message += `\n检测到可疑或失败的文件，建议检查后再转换。`;
        }
        
        eagle.notification.show({
            title: '🔒 文件验证结果',
            description: message,
            duration: 5000
        });
    }
    
    /**
     * 清除缓存
     */
    clearCache() {
        this.cache.clear();
        if (log) {
            log.info('PIXLY Validator', formatLog(LOG.FILE_VALIDATOR_CACHE_CLEARED, {}));
        }
    }
    
    /**
     * 设置是否启用
     * 
     * @param {boolean} enabled - 是否启用
     */
    setEnabled(enabled) {
        this.enabled = enabled;
        if (log) {
            log.info('PIXLY Validator', `Validator ${enabled ? 'Enabled' : 'Disabled'}`);
        }
    }
    
    /**
     * 设置严格模式
     * 
     * @param {boolean} strict - 是否严格模式
     */
    setStrictMode(strict) {
        this.strictMode = strict;
        if (log) {
            log.info('PIXLY Validator', `Strict mode: ${strict ? 'ON' : 'OFF'}`);
        }
    }
}

// 🔥 Phase 45.4: 全局文件验证器实例
window.fileValidator = new FileValidator();

if (log) {
    log.info('PIXLY', formatLog(LOG.FILE_VALIDATOR_MODULE_LOADED, {}));
}

})(); // End of IIFE
