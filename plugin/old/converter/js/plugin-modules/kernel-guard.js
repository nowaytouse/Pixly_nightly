/**
 * 🛡️ Kernel Guard - 内核守护模块
 * 
 * 功能:
 * - 强制内核检测（Rust CLI必需）
 * - 全屏警告UI（视觉冲击力强）
 * - 积极修复指导（帮助用户解决问题）
 * - 禁用所有功能（未检测到内核时）
 * 
 * 设计原则:
 * - 零容忍：内核未检测到 = 插件完全不可用
 * - 强视觉：全屏红色警告，无法忽视
 * - 积极帮助：提供详细修复步骤和自动修复
 * - 无fallback：删除所有JS转换fallback
 */

(function() {
    'use strict';

    // ===== 全局状态 =====
    // 🎯 单一统一Kernel - 纯本地Rust实现
    window.PIXLY_KERNEL_STATUS = {
        kernel: {
            detected: false,
            path: null,
            version: null,
            error: null,
            type: 'unified-rust'  // 统一的Rust Kernel
        },
        lastCheck: null,
        checkCount: 0
    };

    // ===== UI组件 =====
    
    /**
     * 创建全屏警告遮罩
     */
    function createKernelWarningUI() {
        const overlay = document.createElement('div');
        overlay.id = 'kernelWarningOverlay';
        overlay.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: rgba(0, 0, 0, 0.96);
            z-index: 999999;
            display: flex;
            justify-content: center;
            align-items: center;
            backdrop-filter: blur(10px);
            animation: fadeIn 0.3s ease-in;
        `;

        const warning = document.createElement('div');
        warning.style.cssText = `
            max-width: 700px;
            padding: 50px;
            background: linear-gradient(135deg, #d32f2f 0%, #c62828 100%);
            border-radius: 16px;
            color: white;
            text-align: center;
            box-shadow: 0 20px 60px rgba(211, 47, 47, 0.5);
            border: 3px solid rgba(255, 255, 255, 0.2);
            animation: slideUp 0.4s ease-out;
        `;

        warning.innerHTML = `
            <div style="font-size: 80px; margin-bottom: 20px; animation: pulse 2s infinite;">
                ⚠️
            </div>
            <h1 style="font-size: 32px; margin-bottom: 15px; font-weight: bold;">
                内核未检测到
            </h1>
            <p style="font-size: 16px; margin-bottom: 25px; opacity: 0.95;">
                PIXLY插件需要<strong>统一Rust Kernel</strong>才能运行<br>
                所有功能已被禁用，请立即修复
            </p>
            <div id="kernelErrorDetails" style="
                background: rgba(0,0,0,0.2);
                padding: 15px;
                border-radius: 8px;
                margin-bottom: 25px;
                font-size: 13px;
                text-align: left;
                font-family: 'Monaco', 'Courier New', monospace;
            "></div>
            <div style="display: flex; gap: 12px; justify-content: center;">
                <button id="kernelFixBtn" style="
                    padding: 15px 30px;
                    font-size: 16px;
                    background: white;
                    color: #d32f2f;
                    border: none;
                    border-radius: 8px;
                    cursor: pointer;
                    font-weight: bold;
                    transition: all 0.3s;
                    box-shadow: 0 4px 12px rgba(0,0,0,0.2);
                ">
                    🔧 立即修复
                </button>
                <button id="kernelRetryBtn" style="
                    padding: 15px 30px;
                    font-size: 16px;
                    background: rgba(255,255,255,0.2);
                    color: white;
                    border: 2px solid white;
                    border-radius: 8px;
                    cursor: pointer;
                    font-weight: bold;
                    transition: all 0.3s;
                ">
                    🔄 重新检测
                </button>
                <button id="kernelHelpBtn" style="
                    padding: 15px 30px;
                    font-size: 16px;
                    background: rgba(255,255,255,0.1);
                    color: white;
                    border: 2px solid rgba(255,255,255,0.3);
                    border-radius: 8px;
                    cursor: pointer;
                    font-weight: bold;
                    transition: all 0.3s;
                ">
                    📖 帮助文档
                </button>
            </div>
            <div style="margin-top: 20px; font-size: 12px; opacity: 0.7;">
                检测次数: <span id="kernelCheckCount">0</span> | 
                上次检测: <span id="kernelLastCheck">从未</span>
            </div>
        `;

        // 添加CSS动画
        const style = document.createElement('style');
        style.textContent = `
            @keyframes fadeIn {
                from { opacity: 0; }
                to { opacity: 1; }
            }
            @keyframes slideUp {
                from { transform: translateY(50px); opacity: 0; }
                to { transform: translateY(0); opacity: 1; }
            }
            @keyframes pulse {
                0%, 100% { transform: scale(1); }
                50% { transform: scale(1.1); }
            }
            #kernelFixBtn:hover {
                transform: scale(1.05);
                box-shadow: 0 6px 16px rgba(0,0,0,0.3);
            }
            #kernelRetryBtn:hover, #kernelHelpBtn:hover {
                background: rgba(255,255,255,0.3);
                transform: scale(1.05);
            }
        `;
        document.head.appendChild(style);

        overlay.appendChild(warning);
        document.body.appendChild(overlay);

        // 绑定事件
        document.getElementById('kernelFixBtn').addEventListener('click', startKernelFix);
        document.getElementById('kernelRetryBtn').addEventListener('click', retryKernelDetection);
        document.getElementById('kernelHelpBtn').addEventListener('click', showKernelHelp);

        return overlay;
    }

    /**
     * 显示内核警告
     */
    function showKernelWarning(error) {
        const log = window.pixlyLog;
        if (log) {
            log.error('KERNEL GUARD', formatLog(LOG.KERNEL_GUARD_NOT_DETECTED, { error: error || 'unknown' }));
        }
        
        let overlay = document.getElementById('kernelWarningOverlay');
        if (!overlay) {
            overlay = createKernelWarningUI();
        }
        
        overlay.style.display = 'flex';
        
        // 更新错误详情
        const details = document.getElementById('kernelErrorDetails');
        if (details) {
            details.innerHTML = `
                <strong>❌ 错误详情:</strong><br>
                ${error || 'Rust CLI可执行文件未找到或无法执行'}
            `;
        }
        
        // 更新检测信息
        const status = window.PIXLY_KERNEL_STATUS;
        document.getElementById('kernelCheckCount').textContent = status.checkCount;
        document.getElementById('kernelLastCheck').textContent = 
            status.lastCheck ? new Date(status.lastCheck).toLocaleTimeString() : '从未';
        
        // 禁用所有功能
        disableAllFeatures();
    }

    /**
     * 隐藏内核警告
     */
    function hideKernelWarning() {
        const overlay = document.getElementById('kernelWarningOverlay');
        if (overlay) {
            overlay.style.display = 'none';
        }
        
        // 恢复所有功能
        enableAllFeatures();
    }

    /**
     * 禁用所有插件功能
     */
    function disableAllFeatures() {
        const log = window.pixlyLog;
        if (log) {
            log.warn('KERNEL GUARD', formatLog(LOG.KERNEL_GUARD_DISABLE_FEATURES, {}));
        }
        
        // 禁用所有转换按钮
        const i18n = window.i18n || { t: (key) => key };
        document.querySelectorAll('button[id*="convert"], button[id*="quick"], button[id*="start"]').forEach(btn => {
            if (btn.id !== 'kernelFixBtn' && btn.id !== 'kernelRetryBtn' && btn.id !== 'kernelHelpBtn') {
                btn.disabled = true;
                btn.style.opacity = '0.3';
                btn.style.cursor = 'not-allowed';
                btn.title = i18n.t('title.rustNotDetected');
            }
        });
        
        // 禁用所有输入控件
        document.querySelectorAll('input, select, textarea').forEach(input => {
            input.disabled = true;
            input.style.opacity = '0.3';
        });
        
        // 添加全局禁用标志
        window.PIXLY_FEATURES_DISABLED = true;
    }

    /**
     * 恢复所有插件功能
     */
    function enableAllFeatures() {
        const log = window.pixlyLog;
        if (log) {
            log.info('KERNEL GUARD', formatLog(LOG.KERNEL_GUARD_ENABLE_FEATURES, {}));
        }
        
        // 恢复所有按钮
        document.querySelectorAll('button').forEach(btn => {
            btn.disabled = false;
            btn.style.opacity = '1';
            btn.style.cursor = 'pointer';
            btn.title = '';
        });
        
        // 恢复所有输入控件
        document.querySelectorAll('input, select, textarea').forEach(input => {
            input.disabled = false;
            input.style.opacity = '1';
        });
        
        // 移除全局禁用标志
        window.PIXLY_FEATURES_DISABLED = false;
    }

    // ===== 内核检测 =====

    /**
     * 🎯 检测统一Rust Kernel
     * 单一纯本地Kernel，比之前更强大
     */
    async function detectUnifiedKernel() {
        const log = window.pixlyLog;
        if (log) {
            log.info('KERNEL GUARD', '🔍 检测统一Rust Kernel...');
        }
        
        const status = window.PIXLY_KERNEL_STATUS;
        status.checkCount++;
        status.lastCheck = Date.now();
        
        try {
            // 检查window.rustCLI是否已初始化
            if (!window.rustCLI) {
                throw new Error('统一Rust Kernel未初始化');
            }
            
            // 检查available状态
            if (!window.rustCLI.available) {
                throw new Error(window.rustCLI.error || '统一Rust Kernel不可用');
            }
            
            // 获取版本信息
            const version = window.rustCLI.version || 'unknown';
            const path = window.rustCLI.path || 'unknown';
            
            // 更新状态
            status.kernel.detected = true;
            status.kernel.path = path;
            status.kernel.version = version;
            status.kernel.error = null;
            
            if (log) {
                log.info('KERNEL GUARD', `✅ 统一Rust Kernel已检测到 (v${version})`);
                log.debug('KERNEL GUARD', `Kernel路径: ${path}`);
            }
            
            return true;
            
        } catch (error) {
            status.kernel.detected = false;
            status.kernel.error = error.message;
            
            if (log) {
                log.error('KERNEL GUARD', `❌ Kernel检测失败: ${error.message}`);
            }
            return false;
        }
    }
    
    // 向后兼容的别名
    const detectRustCLI = detectUnifiedKernel;

    /**
     * 重新检测内核
     */
    async function retryKernelDetection() {
        const log = window.pixlyLog;
        if (log) {
            log.info('KERNEL GUARD', formatLog(LOG.KERNEL_GUARD_RETRY, {}));
        }
        
        const btn = document.getElementById('kernelRetryBtn');
        if (btn) {
            btn.disabled = true;
            const i18n = window.i18n || { t: (key) => key };
            btn.textContent = i18n.t('kernel.detecting');
        }
        
        // 延迟1秒，给系统时间
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        const detected = await detectRustCLI();
        
        if (detected) {
            hideKernelWarning();
            if (log) {
                log.info('KERNEL GUARD', formatLog(LOG.KERNEL_GUARD_RETRY_SUCCESS, {}));
            }
        } else {
            const error = window.PIXLY_KERNEL_STATUS.rust.error;
            showKernelWarning(error);
            if (log) {
                log.warn('KERNEL GUARD', formatLog(LOG.KERNEL_GUARD_RETRY_FAILED, {}));
            }
        }
        
        if (btn) {
            btn.disabled = false;
            btn.textContent = i18n.t('kernel.retryDetection');
        }
    }

    // ===== 修复功能 =====

    /**
     * 开始内核修复流程
     */
    async function startKernelFix() {
        const log = window.pixlyLog;
        if (log) {
            log.info('KERNEL GUARD', formatLog(LOG.KERNEL_GUARD_FIX_START, {}));
        }
        
        const issues = await diagnoseKernelIssues();
        showFixGuide(issues);
    }

    /**
     * 诊断内核问题
     */
    async function diagnoseKernelIssues() {
        const issues = [];
        
        // 1. 检查Rust CLI路径
        if (!window.rustCLI || !window.rustCLI.path) {
            issues.push({
                severity: 'critical',
                problem: 'Rust CLI可执行文件未找到',
                solution: '请确保已编译Rust核心：运行 `cd pixly-rust && cargo build --release`',
                autoFix: false
            });
        }
        
        // 2. 检查文件是否存在
        if (window.rustCLI && window.rustCLI.path) {
            const fs = require('fs');
            if (!fs.existsSync(window.rustCLI.path)) {
                issues.push({
                    severity: 'critical',
                    problem: `Rust CLI文件不存在: ${window.rustCLI.path}`,
                    solution: '文件可能被删除，请重新编译或下载',
                    autoFix: false
                });
            }
        }
        
        // 3. 检查执行权限 (仅Unix)
        if (process.platform !== 'win32' && window.rustCLI && window.rustCLI.path) {
            const fs = require('fs');
            try {
                const stats = fs.statSync(window.rustCLI.path);
                const isExecutable = (stats.mode & parseInt('111', 8)) !== 0;
                if (!isExecutable) {
                    issues.push({
                        severity: 'high',
                        problem: 'Rust CLI文件无执行权限',
                        solution: `运行: chmod +x "${window.rustCLI.path}"`,
                        autoFix: true,
                        fixCommand: `chmod +x "${window.rustCLI.path}"`
                    });
                }
            } catch (e) {
                issues.push({
                    severity: 'medium',
                    problem: '无法检查文件权限',
                    solution: '请手动检查文件权限',
                    autoFix: false
                });
            }
        }
        
        // 4. 检查依赖库
        if (window.rustCLI && window.rustCLI.error) {
            if (window.rustCLI.error.includes('library') || window.rustCLI.error.includes('dylib')) {
                issues.push({
                    severity: 'high',
                    problem: '系统依赖库缺失',
                    solution: 'macOS: 运行 `brew install imagemagick jpeg-xl`\nLinux: 运行 `apt install libimage-dev`',
                    autoFix: false
                });
            }
        }
        
        return issues;
    }

    /**
     * 显示修复指南
     */
    function showFixGuide(issues) {
        const overlay = document.getElementById('kernelWarningOverlay');
        if (!overlay) return;
        
        const guide = document.createElement('div');
        guide.style.cssText = `
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            max-width: 800px;
            max-height: 80vh;
            overflow-y: auto;
            background: white;
            color: #333;
            padding: 30px;
            border-radius: 12px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.3);
            z-index: 1000000;
        `;
        
        let html = `
            <h2 style="margin-bottom: 20px; color: #d32f2f;">🔧 内核修复指南</h2>
            <p style="margin-bottom: 20px;">发现以下问题，请按照建议修复：</p>
        `;
        
        if (issues.length === 0) {
            html += `<p style="color: green;">✅ 未发现明显问题，可能是临时错误，请重试检测。</p>`;
        } else {
            issues.forEach((issue, index) => {
                const color = issue.severity === 'critical' ? '#d32f2f' : 
                             issue.severity === 'high' ? '#ff9800' : '#2196f3';
                html += `
                    <div style="margin-bottom: 20px; padding: 15px; background: #f5f5f5; border-left: 4px solid ${color}; border-radius: 4px;">
                        <h3 style="margin: 0 0 10px 0; color: ${color};">
                            ${index + 1}. ${issue.problem}
                        </h3>
                        <p style="margin: 0 0 10px 0; white-space: pre-wrap;">
                            <strong>解决方案:</strong><br>${issue.solution}
                        </p>
                        ${issue.autoFix ? `
                            <button onclick="window.PIXLY_AUTO_FIX_${index}()" style="
                                padding: 8px 16px;
                                background: ${color};
                                color: white;
                                border: none;
                                border-radius: 4px;
                                cursor: pointer;
                            ">
                                🔧 自动修复
                            </button>
                        ` : ''}
                    </div>
                `;
                
                // 注册自动修复函数
                if (issue.autoFix) {
                    window[`PIXLY_AUTO_FIX_${index}`] = async () => {
                        try {
                            const { exec } = require('child_process');
                            exec(issue.fixCommand, (error) => {
                                if (error) {
                                    if (log) {
                                        log.error('KERNEL GUARD', formatLog(LOG.KERNEL_GUARD_AUTOFIX_FAILED, { error: error.message }));
                                    }
                                } else {
                                    if (log) {
                                        log.info('KERNEL GUARD', formatLog(LOG.KERNEL_GUARD_AUTOFIX_SUCCESS, {}));
                                    }
                                    retryKernelDetection();
                                }
                            });
                        } catch (e) {
                            if (log) {
                                log.error('KERNEL GUARD', formatLog(LOG.KERNEL_GUARD_AUTOFIX_FAILED, { error: e.message }));
                            }
                        }
                    };
                }
            });
        }
        
        html += `
            <div style="margin-top: 20px; text-align: center;">
                <button onclick="this.parentElement.parentElement.remove()" style="
                    padding: 12px 24px;
                    background: #2196f3;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    cursor: pointer;
                    font-size: 14px;
                ">
                    关闭指南
                </button>
            </div>
        `;
        
        guide.innerHTML = html;
        overlay.appendChild(guide);
    }

    /**
     * 显示帮助文档
     */
    function showKernelHelp() {
        const url = 'https://github.com/your-repo/pixly/wiki/Kernel-Setup';
        if (window.eagle && window.eagle.app) {
            window.eagle.app.openURL(url);
        } else {
            window.open(url, '_blank');
        }
    }

    // ===== 初始化 =====

    /**
     * 初始化内核守护
     * 🎯 统一Rust Kernel检测
     */
    async function initKernelGuard() {
        const log = window.pixlyLog;
        if (log) {
            log.info('KERNEL GUARD', '🛡️ 初始化Kernel守护...');
        }
        
        // 延迟1秒，确保rustCLI完全初始化
        if (log) {
            log.info('KERNEL GUARD', '⏳ 等待Rust CLI初始化...');
        }
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        if (log) {
            log.debug('KERNEL GUARD', `检查状态: rustCLI=${!!window.rustCLI}, available=${window.rustCLI?.available}`);
        }
        
        const detected = await detectUnifiedKernel();
        
        // 🔥 更新UI状态显示 (统一Kernel)
        const imageGoCoreStatus = document.getElementById('imageGoCoreStatus');
        const imageManualStatus = document.getElementById('imageManualCoreStatus');
        const videoManualStatus = document.getElementById('videoManualRustCoreStatus');
        
        if (!detected) {
            const error = window.PIXLY_KERNEL_STATUS.kernel.error;
            showKernelWarning(error);
            
            // ❌ 更新状态为离线
            const i18n = window.i18n || { t: (key) => key };
            const offlineText = `<span style="color: #EF4444;">❌ Kernel ${i18n.t('ai.offline')}</span>`;
            const offlineTitle = '统一Rust Kernel未检测到';
            
            if (imageGoCoreStatus) {
                imageGoCoreStatus.innerHTML = offlineText;
                imageGoCoreStatus.title = offlineTitle;
            }
            if (imageManualStatus) {
                imageManualStatus.innerHTML = offlineText;
                imageManualStatus.title = offlineTitle;
            }
            if (videoManualStatus) {
                videoManualStatus.innerHTML = offlineText;
                videoManualStatus.title = offlineTitle;
            }
        } else {
            if (log) {
                log.info('KERNEL GUARD', '✅ 统一Rust Kernel初始化成功');
            }
            
            // ✅ 更新状态为在线
            const version = window.rustCLI?.version || 'unknown';
            const i18n = window.i18n || { t: (key) => key };
            const onlineText = `<span style="color: #10B981;">✅ Kernel ${i18n.t('common.online')}</span>`;
            const onlineTitle = `统一Rust Kernel v${version}`;
            
            if (imageGoCoreStatus) {
                imageGoCoreStatus.innerHTML = onlineText;
                imageGoCoreStatus.title = onlineTitle;
            }
            if (imageManualStatus) {
                imageManualStatus.innerHTML = onlineText;
                imageManualStatus.title = onlineTitle;
            }
            if (videoManualStatus) {
                videoManualStatus.innerHTML = onlineText;
                videoManualStatus.title = onlineTitle;
            }
        }
        
        // 定期检测（每30秒）
        setInterval(async () => {
            if (!window.PIXLY_KERNEL_STATUS.kernel.detected) {
                await detectUnifiedKernel();
            }
        }, 30000);
    }

    // ===== 导出API =====
    window.PIXLY_KERNEL_GUARD = {
        detect: detectRustCLI,
        retry: retryKernelDetection,
        fix: startKernelFix,
        diagnose: diagnoseKernelIssues,
        showWarning: showKernelWarning,
        hideWarning: hideKernelWarning,
        status: () => window.PIXLY_KERNEL_STATUS
    };

    // 自动初始化
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initKernelGuard);
    } else {
        initKernelGuard();
    }

})();
