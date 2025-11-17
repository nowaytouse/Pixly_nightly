/**
 * @file conversion-guard.js
 * @description Conversion safety and reliability guard
 * 
 * 🔥 Phase 40.24: 转换功能保障系统
 * 
 * 功能:
 * - 启动自检（Self-check）
 * - 错误恢复（Error recovery）
 * - 详细日志（Detailed logging）
 * - 重试机制（Retry mechanism）
 */

(function(PIXLY) {
    'use strict';

    class ConversionGuard {
        constructor() {
            this.healthStatus = {
                rustCLI: false,
                selectedFiles: false,
                eagleAPI: false,
                uiBindings: false,
                pathResolver: false
            };
            
            this.errors = [];
            this.warnings = [];
        }

        /**
         * 🔍 执行完整的健康检查
         */
        async performHealthCheck() {
            const log = window.pixlyLog;
            if (!log) {
                throw new Error('pixlyLog is required for ConversionGuard');
            }
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_HEALTH_CHECK_DIVIDER, {}));
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_HEALTH_CHECK_START, {}));
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_HEALTH_CHECK_DIVIDER, {}));
            
            // 1. 检查 Rust CLI
            await this._checkRustCLI();
            
            // 2. 检查 Path Resolver
            this._checkPathResolver();
            
            // 3. 检查 Eagle API
            this._checkEagleAPI();
            
            // 4. 检查 UI 绑定
            this._checkUIBindings();
            
            // 5. 检查 selectedFiles
            this._checkSelectedFiles();
            
            // 生成报告
            this._generateReport();
            
            return this.healthStatus;
        }

        async _checkRustCLI() {
            const log = window.pixlyLog;
            if (!log) return;
            
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_CHECKING_RUST_CLI, {}));
            
            if (!window.rustCLI) {
                this.healthStatus.rustCLI = false;
                this.errors.push('❌ Rust CLI not found (window.rustCLI is undefined)');
                log.error('Conversion Guard', formatLog(LOG.CONV_GUARD_RUST_CLI_NOT_FOUND, {}));
                return;
            }
            
            if (!window.rustCLI.available) {
                this.healthStatus.rustCLI = false;
                this.errors.push(`❌ Rust CLI not available: ${window.rustCLI.error || 'Unknown error'}`);
                log.error('Conversion Guard', formatLog(LOG.CONV_GUARD_RUST_CLI_NOT_AVAILABLE, { error: window.rustCLI.error || 'Unknown error' }));
                return;
            }
            
            // 测试基本功能
            try {
                const testResult = await window.rustCLI.getVersion();
                this.healthStatus.rustCLI = true;
                log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_RUST_CLI_OK, { version: testResult || window.rustCLI.version }));
            } catch (error) {
                this.healthStatus.rustCLI = false;
                this.errors.push(`❌ Rust CLI test failed: ${error.message}`);
                log.error('Conversion Guard', formatLog(LOG.CONV_GUARD_RUST_CLI_TEST_FAILED, { error: error.message }));
            }
        }

        _checkPathResolver() {
            const log = window.pixlyLog;
            if (!log) return;
            
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_CHECKING_PATH_RESOLVER, {}));
            
            if (!window.PIXLY_PATH_RESOLVER) {
                this.healthStatus.pathResolver = false;
                this.warnings.push('⚠️ Path Resolver not found (may cause path resolution issues)');
                log.warn('Conversion Guard', formatLog(LOG.CONV_GUARD_PATH_RESOLVER_NOT_FOUND, {}));
                return;
            }
            
            if (typeof window.PIXLY_PATH_RESOLVER.getRustCLIPath !== 'function') {
                this.healthStatus.pathResolver = false;
                this.warnings.push('⚠️ Path Resolver incomplete (getRustCLIPath missing)');
                log.warn('Conversion Guard', formatLog(LOG.CONV_GUARD_PATH_RESOLVER_INCOMPLETE, {}));
                return;
            }
            
            try {
                const cliPath = window.PIXLY_PATH_RESOLVER.getRustCLIPath();
                this.healthStatus.pathResolver = true;
                log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_PATH_RESOLVER_OK, { path: cliPath }));
            } catch (error) {
                this.healthStatus.pathResolver = false;
                this.errors.push(`❌ Path Resolver failed: ${error.message}`);
                log.error('Conversion Guard', formatLog(LOG.CONV_GUARD_PATH_RESOLVER_FAILED, { error: error.message }));
            }
        }

        _checkEagleAPI() {
            const log = window.pixlyLog;
            if (!log) return;
            
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_CHECKING_EAGLE_API, {}));
            
            if (typeof eagle === 'undefined') {
                this.healthStatus.eagleAPI = false;
                this.warnings.push('⚠️ Eagle API not found (notifications may not work)');
                log.warn('Conversion Guard', formatLog(LOG.CONV_GUARD_EAGLE_API_NOT_FOUND, {}));
                return;
            }
            
            const requiredAPIs = ['item', 'notification', 'app'];
            const missingAPIs = requiredAPIs.filter(api => !eagle[api]);
            
            if (missingAPIs.length > 0) {
                this.healthStatus.eagleAPI = false;
                this.warnings.push(`⚠️ Eagle API incomplete: missing ${missingAPIs.join(', ')}`);
                log.warn('Conversion Guard', formatLog(LOG.CONV_GUARD_EAGLE_API_INCOMPLETE, { apis: missingAPIs.join(', ') }));
                return;
            }
            
            this.healthStatus.eagleAPI = true;
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_EAGLE_API_OK, {}));
        }

        _checkUIBindings() {
            const log = window.pixlyLog;
            if (!log) return;
            
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_CHECKING_UI_BINDINGS, {}));
            
            const requiredElements = [
                'convertBtn',
                'formatSelect',
                'selectedFiles',
                'filesList'
            ];
            
            const missingElements = requiredElements.filter(id => !document.getElementById(id));
            
            if (missingElements.length > 0) {
                this.healthStatus.uiBindings = false;
                this.errors.push(`❌ UI elements missing: ${missingElements.join(', ')}`);
                log.error('Conversion Guard', formatLog(LOG.CONV_GUARD_UI_ELEMENTS_MISSING, { elements: missingElements.join(', ') }));
                return;
            }
            
            // 检查事件监听器
            const convertBtn = document.getElementById('convertBtn');
            if (convertBtn && !convertBtn.onclick && convertBtn.getAttribute('data-listener') !== 'attached') {
                this.warnings.push('⚠️ Convert button may not have event listener attached');
                log.warn('Conversion Guard', formatLog(LOG.CONV_GUARD_CONVERT_BTN_NO_LISTENER, {}));
            }
            
            this.healthStatus.uiBindings = true;
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_UI_BINDINGS_OK, {}));
        }

        _checkSelectedFiles() {
            const log = window.pixlyLog;
            if (!log) return;
            
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_CHECKING_SELECTED_FILES, {}));
            
            if (!window.selectedFiles) {
                this.healthStatus.selectedFiles = false;
                this.warnings.push('⚠️ selectedFiles array not found');
                log.warn('Conversion Guard', formatLog(LOG.CONV_GUARD_SELECTED_FILES_NOT_FOUND, {}));
                return;
            }
            
            if (!Array.isArray(window.selectedFiles)) {
                this.healthStatus.selectedFiles = false;
                this.errors.push('❌ selectedFiles is not an array');
                log.error('Conversion Guard', formatLog(LOG.CONV_GUARD_SELECTED_FILES_NOT_ARRAY, {}));
                return;
            }
            
            if (window.selectedFiles.length === 0) {
                this.healthStatus.selectedFiles = false;
                log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_NO_FILES_SELECTED, {}));
                return;
            }
            
            // 验证文件对象结构
            const firstFile = window.selectedFiles[0];
            const requiredProps = ['filePath', 'name'];
            const missingProps = requiredProps.filter(prop => !firstFile[prop]);
            
            if (missingProps.length > 0) {
                this.healthStatus.selectedFiles = false;
                this.errors.push(`❌ File objects incomplete: missing ${missingProps.join(', ')}`);
                log.error('Conversion Guard', formatLog(LOG.CONV_GUARD_FILE_OBJECTS_INCOMPLETE, { props: missingProps.join(', ') }));
                return;
            }
            
            this.healthStatus.selectedFiles = true;
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_FILES_SELECTED, { count: window.selectedFiles.length }));
        }

        _generateReport() {
            const log = window.pixlyLog;
            if (!log) return;
            
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_HEALTH_CHECK_DIVIDER, {}));
            log.info('Conversion Guard', 'Health Check Report:');
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_HEALTH_CHECK_DIVIDER, {}));
            
            Object.entries(this.healthStatus).forEach(([component, status]) => {
                const icon = status ? '✅' : '❌';
                log.info('Conversion Guard', `  ${icon} ${component}: ${status ? 'OK' : 'FAILED'}`);
            });
            
            if (this.errors.length > 0) {
                log.error('Conversion Guard', 'Errors:');
                this.errors.forEach(err => log.error('Conversion Guard', '  ' + err));
            }
            
            if (this.warnings.length > 0) {
                log.warn('Conversion Guard', 'Warnings:');
                this.warnings.forEach(warn => log.warn('Conversion Guard', '  ' + warn));
            }
            
            const criticalComponents = ['rustCLI', 'uiBindings'];
            const allCriticalOK = criticalComponents.every(comp => this.healthStatus[comp]);
            
            const status = allCriticalOK ? 'All critical components healthy' : 'Critical components have issues';
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_HEALTH_CHECK_COMPLETE, { status }));
            
            log.info('Conversion Guard', formatLog(LOG.CONV_GUARD_HEALTH_CHECK_DIVIDER, {}));
        }

        async attemptAutoFix() {
            const log = window.pixlyLog;
            if (!log) return;
            
            log.info('Conversion Guard', 'Attempting auto-fix...');
            
            // 1. 尝试重新初始化 Rust CLI
            if (!this.healthStatus.rustCLI && window.rustCLI) {
                log.info('Conversion Guard', 'Re-initializing Rust CLI...');
                try {
                    window.rustCLI.init();
                    await this._checkRustCLI();
                } catch (error) {
                    log.error('Conversion Guard', `Auto-fix failed for Rust CLI: ${error.message}`);
                }
            }
            
            // 2. 尝试重新绑定UI事件
            if (!this.healthStatus.uiBindings) {
                log.info('Conversion Guard', 'Re-binding UI events...');
                try {
                    const convertBtn = document.getElementById('convertBtn');
                    if (convertBtn && window.startConversion) {
                        convertBtn.addEventListener('click', window.startConversion);
                        convertBtn.setAttribute('data-listener', 'attached');
                        this.healthStatus.uiBindings = true;
                        log.info('Conversion Guard', 'UI events re-bound successfully');
                    }
                } catch (error) {
                    log.error('Conversion Guard', `Auto-fix failed for UI bindings: ${error.message}`);
                }
            }
            
            log.info('Conversion Guard', 'Auto-fix complete');
        }
    }

    // 导出到全局
    PIXLY.ConversionGuard = ConversionGuard;
    
    // 启动时自动执行健康检查
    document.addEventListener('DOMContentLoaded', async () => {
        const log = window.pixlyLog;
        if (log) {
            log.info('Conversion Guard', 'Initializing at DOMContentLoaded...');
        }
        const guard = new ConversionGuard();
        await guard.performHealthCheck();
        
        // 如果有问题，尝试自动修复
        const hasCriticalIssues = !guard.healthStatus.rustCLI || !guard.healthStatus.uiBindings;
        if (hasCriticalIssues) {
            await guard.attemptAutoFix();
        }
        
        // 导出到全局，便于调试
        window.PIXLY_CONVERSION_GUARD = guard;
    });
    
})(window.PIXLY = window.PIXLY || {});