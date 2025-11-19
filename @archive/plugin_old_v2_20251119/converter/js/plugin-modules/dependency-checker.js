/**
 * PIXLY 依赖Check工具module
 * 依赖: PIXLY命名空间, Node.js APIs
 */
(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    // Log instance for the module
    const log = window.pixlyLog;
    
    /**
     * 依赖Check器
     */
    const DependencyChecker = {
        
        /**
         * Checkall依赖
         */
        checkAll: async function() {
        if (log) {
            log.info('PIXLY Dep', formatLog(LOG.DEPENDENCY_CHECK_START, {}));
        }
            
            const results = {
                // pixly: await this.checkPixly(), // ❌ 已废弃 - 不存在的工具
                cjxl: await this.checkCjxl(),
                avifenc: await this.checkAvifenc(),
                cwebp: await this.checkCwebp(),
                ffmpeg: await this.checkFfmpeg(),
                exiftool: await this.checkExiftool()
            };
            
            const allInstalled = Object.values(results).every(r => r.installed);
            
        if (log) {
            log.info('PIXLY Dep', formatLog(LOG.DEPENDENCY_CHECK_COMPLETE, { status: allInstalled ? '✅' : '⚠️' }));
        }
            
            return {
                allInstalled,
                results
            };
        },
        
        /**
         * CheckPIXLY
         */
        checkPixly: async function() {
            return await this.checkCommand('pixly --version', 'PIXLY');
        },
        
        /**
         * CheckCJXL
         */
        checkCjxl: async function() {
            return await this.checkCommand('cjxl --version', 'JPEG XL (cjxl)');
        },
        
        /**
         * Checkavifenc
         */
        checkAvifenc: async function() {
            return await this.checkCommand('avifenc --version', 'AVIF Encoder');
        },
        
        /**
         * Checkcwebp
         */
        checkCwebp: async function() {
            return await this.checkCommand('cwebp -version', 'WebP Encoder');
        },
        
        /**
         * Checkffmpeg
         */
        checkFfmpeg: async function() {
            return await this.checkCommand('ffmpeg -version', 'FFmpeg');
        },
        
        /**
         * Checkexiftool
         */
        checkExiftool: async function() {
            return await this.checkCommand('exiftool -ver', 'ExifTool');
        },
        
        /**
         * Check命令is否available
         */
        checkCommand: async function(command, name) {
            try {
                const { exec } = require('child_process');
                const { promisify } = require('util');
                const execAsync = promisify(exec);
                
                // settings完整 PATH
                const env = Object.assign({}, require('process').env);
                env.PATH = `/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:${env.PATH || ''}`;
                
                const { stdout, stderr } = await execAsync(command, { env });
                const version = this.extractVersion(stdout || stderr);
                
                if (log) {
                    log.info('PIXLY Dep', formatLog(LOG.DEPENDENCY_FOUND, { name, version: version || 'installed' }));
                }
                
                return {
                    name,
                    installed: true,
                    version: version || 'unknown',
                    command: command.split(' ')[0]
                };
            } catch (error) {
                if (log) {
                    log.warn('PIXLY Dep', formatLog(LOG.DEPENDENCY_NOT_FOUND, { name }));
                }
                
                return {
                    name,
                    installed: false,
                    version: null,
                    command: command.split(' ')[0],
                    error: error.message
                };
            }
        },
        
        /**
         * from输出中提取version号
         */
        extractVersion: function(output) {
            if (!output) return null;
            
            // 匹配常见version号格式
            const patterns = [
                /version\s+(\d+\.\d+\.\d+)/i,
                /v(\d+\.\d+\.\d+)/i,
                /(\d+\.\d+\.\d+)/,
                /(\d+\.\d+)/
            ];
            
            for (const pattern of patterns) {
                const match = output.match(pattern);
                if (match && match[1]) {
                    return match[1];
                }
            }
            
            return null;
        },
        
        /**
         * 获取installed建议
         */
        getInstallSuggestions: function(missingDeps) {
            const suggestions = {
                // 'pixly': 'brew install pixly', // ❌ 已废弃 - 不存在的工具
                'cjxl': 'brew install jpeg-xl',
                'avifenc': 'brew install libavif',
                'cwebp': 'brew install webp',
                'ffmpeg': 'brew install ffmpeg',
                'exiftool': 'brew install exiftool'
            };
            
            return missingDeps.map(dep => ({
                name: dep.name,
                command: suggestions[dep.command] || `brew install ${dep.command}`
            }));
        },
        
        /**
         * 自动installed缺失依赖
         */
        autoInstall: async function(missingDeps) {
        if (log) {
            log.info('PIXLY Dep', formatLog(LOG.DEPENDENCY_AUTO_INSTALL_START, {}));
        }
            
            const { exec } = require('child_process');
            const { promisify } = require('util');
            const execAsync = promisify(exec);
            
            const results = [];
            
            for (const dep of missingDeps) {
                const suggestions = this.getInstallSuggestions([dep]);
                const installCmd = suggestions[0].command;
                
                try {
                    if (log) {
                        log.info('PIXLY Dep', formatLog(LOG.DEPENDENCY_INSTALLING, { name: dep.name }));
                    }
                    await execAsync(installCmd);
                    
                    results.push({
                        name: dep.name,
                        success: true
                    });
                    
                    if (log) {
                        log.info('PIXLY Dep', formatLog(LOG.DEPENDENCY_INSTALL_SUCCESS, { name: dep.name }));
                    }
                } catch (error) {
                    results.push({
                        name: dep.name,
                        success: false,
                        error: error.message
                    });
                    
                if (log) {
                    log.error('PIXLY Dep', formatLog(LOG.DEPENDENCY_INSTALL_FAILED, { name: dep.name, error: error.message || error }));
                }
                }
            }
            
            return results;
        },
        
        /**
         * 生成依赖报告
         */
        generateReport: function(checkResults) {
            const report = [];
            
            report.push('='.repeat(50));
            report.push('PIXLY 依赖Check报告');
            report.push('='.repeat(50));
            report.push('');
            
            for (const [key, result] of Object.entries(checkResults.results)) {
                const status = result.installed ? '✅' : '❌';
                const version = result.version || 'N/A';
                report.push(`${status} ${result.name.padEnd(20)} ${version}`);
            }
            
            report.push('');
            report.push('='.repeat(50));
            report.push(`Overall Status: ${checkResults.allInstalled ? '✅ All dependencies installed' : '⚠️ Missing some dependencies'}`);
            report.push('='.repeat(50));
            
            return report.join('\n');
        }
    };
    
    // 暴露to全局
    PIXLY.DependencyChecker = DependencyChecker;
    
    // 向后兼容
    window.checkAllDependencies = function() {
        return DependencyChecker.checkAll();
    };
    
    // ❌ 已废弃 - Pixly不是独立CLI工具
    // window.checkPixlyQuietly = function() {
    //     return DependencyChecker.checkPixly().then(r => r.installed);
    // };
    
    window.autoInstallToolsSilently = function() {
        return DependencyChecker.checkAll().then(results => {
            const missing = Object.values(results.results).filter(r => !r.installed);
            if (missing.length > 0) {
                return DependencyChecker.autoInstall(missing);
            }
            return [];
        });
    };
    
    if (log) {
        log.info('PIXLY Dep', formatLog(LOG.DEPENDENCY_CHECKER_MODULE_LOADED, {}));
    }
    
})(window);
