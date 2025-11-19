/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * Validation Bridge - 验证系统桥接
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🎯 职责:
 * - 调用Rust验证系统
 * - 显示验证结果
 * - 处理验证错误和警告
 * 
 * 🚫 不做:
 * - 实现验证逻辑
 * - 文件读取
 * - 参数计算
 */
(function(window) {
    'use strict';
    
    const log = window.pixlyLog;
    
    /**
     * 验证桥接类
     */
    class ValidationBridge {
        constructor() {
            this.kernelBridge = window.PIXLY.kernelBridge;
        }
        
        /**
         * 🔍 验证转换前的准备
         * 
         * @param {Array} files - 文件路径列表
         * @param {Object} config - 转换配置
         * @returns {Promise<Object>} 验证结果
         */
        async validatePreConversion(files, config) {
            log.info('ValidationBridge', '开始转换前验证', { fileCount: files.length });
            
            try {
                const result = await this.kernelBridge.validateConversion(files, config);
                
                if (result.valid) {
                    log.info('ValidationBridge', '✅ 验证通过', { level: result.level });
                    this.displayValidationSuccess(result);
                } else {
                    log.error('ValidationBridge', '❌ 验证失败', { 
                        level: result.level,
                        errors: result.errors 
                    });
                    this.displayValidationErrors(result);
                }
                
                // 显示警告
                if (result.warnings && result.warnings.length > 0) {
                    this.displayValidationWarnings(result.warnings);
                }
                
                return result;
            } catch (error) {
                log.error('ValidationBridge', '验证过程出错', { error: error.message });
                throw error;
            }
        }
        
        /**
         * 🔍 验证转换后的输出
         * 
         * @param {string} outputPath - 输出文件路径
         * @param {number} inputSize - 输入文件大小
         * @returns {Promise<Object>} 验证结果
         */
        async validatePostConversion(outputPath, inputSize) {
            log.info('ValidationBridge', '开始转换后验证', { output: outputPath });
            
            try {
                const result = await this.kernelBridge.executeCommand('validate_output', {
                    output_path: outputPath,
                    expected_size: inputSize
                });
                
                if (result.valid) {
                    log.info('ValidationBridge', '✅ 输出验证通过');
                } else {
                    log.error('ValidationBridge', '❌ 输出验证失败', { errors: result.errors });
                    this.displayValidationErrors(result);
                }
                
                return result;
            } catch (error) {
                log.error('ValidationBridge', '输出验证出错', { error: error.message });
                throw error;
            }
        }
        
        /**
         * 📊 显示验证成功
         */
        displayValidationSuccess(result) {
            if (window.addLog) {
                window.addLog(`✅ 验证通过 (Level ${result.level})`, 'success');
            }
        }
        
        /**
         * ❌ 显示验证错误
         */
        displayValidationErrors(result) {
            if (!window.addLog) return;
            
            window.addLog(`❌ 验证失败 (Level ${result.level})`, 'error');
            
            if (result.errors && result.errors.length > 0) {
                result.errors.forEach(error => {
                    window.addLog(error, 'error');
                });
            }
        }
        
        /**
         * ⚠️ 显示验证警告
         */
        displayValidationWarnings(warnings) {
            if (!window.addLog) return;
            
            warnings.forEach(warning => {
                window.addLog(warning, 'warning');
            });
        }
        
        /**
         * 📋 显示详细验证报告
         */
        displayDetailedReport(result) {
            if (!window.addLog) return;
            
            window.addLog('═'.repeat(60), 'info');
            window.addLog('  验证报告', 'info');
            window.addLog('═'.repeat(60), 'info');
            
            const status = result.valid ? '通过 ✅' : '失败 ❌';
            window.addLog(`状态: ${status}`, result.valid ? 'success' : 'error');
            window.addLog(`级别: Level ${result.level}`, 'info');
            
            if (result.errors && result.errors.length > 0) {
                window.addLog('\n错误列表:', 'error');
                result.errors.forEach((error, i) => {
                    window.addLog(`  ${i + 1}. ${error}`, 'error');
                });
            }
            
            if (result.warnings && result.warnings.length > 0) {
                window.addLog('\n警告列表:', 'warning');
                result.warnings.forEach((warning, i) => {
                    window.addLog(`  ${i + 1}. ${warning}`, 'warning');
                });
            }
            
            window.addLog('═'.repeat(60), 'info');
        }
        
        /**
         * 🎨 创建验证结果UI
         */
        createValidationUI(result) {
            const container = document.createElement('div');
            container.className = 'validation-result';
            container.style.cssText = `
                margin: 10px 0;
                padding: 15px;
                border-radius: 8px;
                background: ${result.valid ? '#e8f5e9' : '#ffebee'};
                border-left: 4px solid ${result.valid ? '#4caf50' : '#f44336'};
            `;
            
            // 标题
            const title = document.createElement('div');
            title.style.cssText = 'font-weight: bold; margin-bottom: 10px;';
            title.textContent = result.valid ? '✅ 验证通过' : '❌ 验证失败';
            container.appendChild(title);
            
            // 级别
            const level = document.createElement('div');
            level.style.cssText = 'font-size: 12px; color: #666; margin-bottom: 10px;';
            level.textContent = `验证级别: Level ${result.level}`;
            container.appendChild(level);
            
            // 错误
            if (result.errors && result.errors.length > 0) {
                const errorsDiv = document.createElement('div');
                errorsDiv.style.cssText = 'margin-top: 10px;';
                
                const errorsTitle = document.createElement('div');
                errorsTitle.style.cssText = 'font-weight: bold; color: #f44336; margin-bottom: 5px;';
                errorsTitle.textContent = '错误:';
                errorsDiv.appendChild(errorsTitle);
                
                const errorsList = document.createElement('ul');
                errorsList.style.cssText = 'margin: 0; padding-left: 20px;';
                result.errors.forEach(error => {
                    const li = document.createElement('li');
                    li.textContent = error;
                    errorsList.appendChild(li);
                });
                errorsDiv.appendChild(errorsList);
                container.appendChild(errorsDiv);
            }
            
            // 警告
            if (result.warnings && result.warnings.length > 0) {
                const warningsDiv = document.createElement('div');
                warningsDiv.style.cssText = 'margin-top: 10px;';
                
                const warningsTitle = document.createElement('div');
                warningsTitle.style.cssText = 'font-weight: bold; color: #ff9800; margin-bottom: 5px;';
                warningsTitle.textContent = '警告:';
                warningsDiv.appendChild(warningsTitle);
                
                const warningsList = document.createElement('ul');
                warningsList.style.cssText = 'margin: 0; padding-left: 20px;';
                result.warnings.forEach(warning => {
                    const li = document.createElement('li');
                    li.textContent = warning;
                    warningsList.appendChild(li);
                });
                warningsDiv.appendChild(warningsList);
                container.appendChild(warningsDiv);
            }
            
            return container;
        }
        
        /**
         * 📍 在页面中显示验证结果
         */
        showValidationInPage(result, containerId = 'validation-container') {
            const container = document.getElementById(containerId);
            if (!container) {
                log.warn('ValidationBridge', '验证结果容器不存在', { containerId });
                return;
            }
            
            // 清空现有内容
            container.innerHTML = '';
            
            // 添加验证结果UI
            const ui = this.createValidationUI(result);
            container.appendChild(ui);
        }
    }
    
    // 导出到全局
    window.PIXLY = window.PIXLY || {};
    window.PIXLY.ValidationBridge = ValidationBridge;
    
    // 创建单例
    window.PIXLY.validationBridge = new ValidationBridge();
    
    if (log) {
        log.info('ValidationBridge', '✅ Validation Bridge已加载');
    }
    
})(window);
