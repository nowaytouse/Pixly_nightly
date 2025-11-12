/**
 * 🎨 PIXLY v3 主题切换module - 性能优化版
 * 策略：MutationObserver监听DOM变化 + 防抖机制
 */

(function() {
    'use strict';
    const log = window.pixlyLog;
    if (!log) {
        throw new Error('pixlyLog is required for theme.js');
    }
    log.debug('PIXLY Theme', formatLog(LOG.THEME_INIT, {}));

    // 主题颜色定义
    const THEMES = {
        dark: {
            bgBase: '#1a1a2e',
            bgCard: '#25293c',
            bgElevated: '#2d3250',
            textPrimary: '#e2e8f0',
            textSecondary: '#cbd5e1',
            border: '#3a4261',
            primary: '#667eea',  // 蓝紫色主题色
            icon: '🌞'
        },
        light: {
            bgBase: '#f8fafc',
            bgCard: '#ffffff',
            bgElevated: '#ffffff',
            textPrimary: '#1e293b',
            textSecondary: '#475569',
            border: '#e2e8f0',
            primary: '#667eea',  // 蓝紫色主题色（亮暗模式统一）
            icon: '🌙'
        }
    };

    let currentTheme = 'light';
    let themeObserver = null;
    let themeUpdateTimeout = null;
    
    // 🔥 Phase 40.32: 日志优化 - 减少重复的主题应用日志
    let lastLoggedTheme = null;
    let themeApplyCount = 0;

    /**
     * 🎨 应用主题：直接修改所有元素style + 更新CSS变量
     */
    function bruteForceApplyTheme(theme) {
        const colors = THEMES[theme];
        if (!colors) return;

        log.debug('PIXLY Theme', formatLog(LOG.THEME_FORCE_APPLY, { theme }));
        
        // 🚫 AI options区域 - 永久豁免主题修改（包括图像和视频AI区域）
        const aiSection = document.getElementById('aiOptionsSection');
        const videoAISection = document.getElementById('videoAIOptionsSection');

        // 0. 🎨 更新CSS变量（关键修复！）
        const root = document.documentElement;
        root.style.setProperty('--text-primary', colors.textPrimary);
        root.style.setProperty('--text-secondary', colors.textSecondary);
        root.style.setProperty('--bg-base', colors.bgBase);
        root.style.setProperty('--bg-card', colors.bgCard);
        root.style.setProperty('--bg-elevated', colors.bgElevated);
        root.style.setProperty('--border', colors.border);
        root.style.setProperty('--primary', colors.primary);

        // 1. 修改body
        document.body.style.setProperty('background-color', colors.bgBase, 'important');
        document.body.style.setProperty('color', colors.textPrimary, 'important');

        // 2. 修改all面板（skip AI options）
        const panels = [
            ...document.querySelectorAll('.conversion-panel'),
            ...document.querySelectorAll('.main-panel'),
            ...document.querySelectorAll('.panel-wrapper'),
            ...document.querySelectorAll('.panel-content'),
            ...document.querySelectorAll('details[open] .panel-content')
        ].filter(panel => {
            const inImageAI = aiSection && aiSection.contains(panel);
            const inVideoAI = videoAISection && videoAISection.contains(panel);
            return !inImageAI && !inVideoAI;
        });

        panels.forEach(panel => {
            panel.style.setProperty('background', colors.bgCard, 'important');
            panel.style.setProperty('background-color', colors.bgCard, 'important');
            panel.style.setProperty('color', colors.textPrimary, 'important');
        });

        // 3. 修改all容器（skip AI options）
        const containers = document.querySelectorAll('.container, .content, .wrapper, main, section');
        containers.forEach(el => {
            if ((aiSection && aiSection.contains(el)) || (videoAISection && videoAISection.contains(el))) return; // skip AI区域
            
            const currentBg = window.getComputedStyle(el).backgroundColor;
            // 只修改有背景色elements
            if (currentBg !== 'rgba(0, 0, 0, 0)' && currentBg !== 'transparent') {
                el.style.setProperty('background-color', colors.bgCard, 'important');
            }
            el.style.setProperty('color', colors.textPrimary, 'important');
        });

        // 4. 修改all文本elements（skip AI options）
        const textElements = document.querySelectorAll('p, span, label, div, h1, h2, h3, h4, h5, h6');
        textElements.forEach(el => {
            if ((aiSection && aiSection.contains(el)) || (videoAISection && videoAISection.contains(el))) return; // 🚫 skip AI区域
            
            // 只修改那些没有特殊颜色elements
            const currentColor = window.getComputedStyle(el).color;
            if (!el.style.color || currentColor.includes('rgb(')) {
                el.style.setProperty('color', colors.textPrimary, 'important');
            }
        });

        log.debug('PIXLY Theme', formatLog(LOG.THEME_PANELS_MODIFIED, { count: panels.length }));
        
    // 🔥 Phase 40.32: 只在主题切换或每10次应用时记录
    if (lastLoggedTheme !== theme || themeApplyCount % 10 === 0) {
        if (lastLoggedTheme !== theme) {
            log.info('PIXLY Theme', formatLog(LOG.THEME_SWITCHED, { from: lastLoggedTheme, to: theme }));
        }
        lastLoggedTheme = theme;
    }
    themeApplyCount++;
        
        // 🔥 强制应用编码器卡片暗色模式样式
        if (theme === 'dark') {
            // 编码器名称 - 纯白色
            document.querySelectorAll('.codec-name').forEach(el => {
                el.style.setProperty('color', '#ffffff', 'important');
                el.style.setProperty('font-weight', '700', 'important');
            });
            
            // 统计信息 - 亮灰色
            document.querySelectorAll('.codec-stats').forEach(el => {
                el.style.setProperty('color', '#b0b8c5', 'important');
            });
            
            // 🎨 标签配色方案 - 协调美观的暗色模式配色
            
            // H.265 "推荐" - 金橙色渐变（重要/突出）
            const h265Card = document.querySelector('input[value="h265"]')?.closest('.codec-card');
            if (h265Card) {
                const badge = h265Card.querySelector('.codec-badge');
                if (badge) {
                    badge.style.setProperty('background', 'linear-gradient(135deg, #f59e0b 0%, #d97706 100%)', 'important');
                    badge.style.setProperty('color', '#ffffff', 'important');
                    badge.style.setProperty('font-weight', '700', 'important');
                    badge.style.setProperty('border', '1px solid rgba(251, 191, 36, 0.3)', 'important');
                }
            }
            
            // H.266 "最新标准" - 紫色渐变
            const h266Card = document.querySelector('input[value="h266"]')?.closest('.codec-card');
            if (h266Card) {
                const badge = h266Card.querySelector('.codec-badge');
                if (badge) {
                    badge.style.setProperty('background', 'linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%)', 'important');
                    badge.style.setProperty('color', '#ffffff', 'important');
                    badge.style.setProperty('font-weight', '700', 'important');
                    badge.style.setProperty('border', '1px solid rgba(139, 92, 246, 0.3)', 'important');
                }
            }
            
            // AV1 "次世代" - 青蓝色渐变
            const av1Card = document.querySelector('input[value="av1"]')?.closest('.codec-card');
            if (av1Card) {
                const badge = av1Card.querySelector('.codec-badge');
                if (badge) {
                    badge.style.setProperty('background', 'linear-gradient(135deg, #06b6d4 0%, #0891b2 100%)', 'important');
                    badge.style.setProperty('color', '#ffffff', 'important');
                    badge.style.setProperty('font-weight', '600', 'important');
                    badge.style.setProperty('border', '1px solid rgba(6, 182, 212, 0.3)', 'important');
                }
            }
            
            // ProRes "专业后期" - 翠绿色渐变
            const proresCard = document.querySelector('input[value="prores"]')?.closest('.codec-card');
            if (proresCard) {
                const badge = proresCard.querySelector('.codec-badge');
                if (badge) {
                    badge.style.setProperty('background', 'linear-gradient(135deg, #10b981 0%, #059669 100%)', 'important');
                    badge.style.setProperty('color', '#ffffff', 'important');
                    badge.style.setProperty('font-weight', '600', 'important');
                    badge.style.setProperty('border', '1px solid rgba(16, 185, 129, 0.3)', 'important');
                }
            }
            
            log.debug('PIXLY Theme', formatLog(LOG.THEME_CODEC_STYLES, { theme: 'dark' }));
            
            // 🎯 Smart mode按钮 - 暗色模式
            document.querySelectorAll('.optimize-mode-btn:not(:has(input:checked)):not(.active)').forEach(btn => {
                btn.style.setProperty('color', '#e2e8f0', 'important');
            });
            document.querySelectorAll('.optimize-mode-btn:has(input:checked), .optimize-mode-btn.active').forEach(btn => {
                btn.style.setProperty('color', '#ffffff', 'important');
            });
        } else {
            // 亮色模式 - 移除强制样式，恢复默认
            document.querySelectorAll('.codec-name, .codec-stats, .codec-badge').forEach(el => {
                el.style.removeProperty('color');
                el.style.removeProperty('background');
                el.style.removeProperty('background-color');
                el.style.removeProperty('font-weight');
                el.style.removeProperty('border');
                el.style.removeProperty('border-color');
            });
            
            // 🎯 Smart mode按钮 - 浅色模式
            document.querySelectorAll('.optimize-mode-btn').forEach(btn => {
                btn.style.setProperty('color', '#1e293b', 'important');
            });
            log.debug('PIXLY Theme', formatLog(LOG.THEME_CODEC_STYLES, { theme: 'light' }));
        }
        
        // �� 更New智能模式卡片样式（使用NewCSS变量值）
        if (window.PIXLY && window.PIXLY.updateOptimizeModeStyles) {
            window.PIXLY.updateOptimizeModeStyles();
        }
    }

    /**
     * �� 智能主题监听器 - 仅在DOM变化时应用主题
     */
    function startThemeObserver(theme) {
        // 停止旧observer
        if (themeObserver) {
            themeObserver.disconnect();
            themeObserver = null;
        }
        
        // 创建新observer - 监听DOM变化
        themeObserver = new MutationObserver((mutations) => {
            let needsUpdate = false;
            
            // 检查是否有新元素添加
            for (const mutation of mutations) {
                if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {
                    // 过滤掉文本节点和注释节点
                    for (const node of mutation.addedNodes) {
                        if (node.nodeType === Node.ELEMENT_NODE) {
                            needsUpdate = true;
                            break;
                        }
                    }
                }
                if (needsUpdate) break;
            }
            
            if (needsUpdate) {
                // 防抖：200ms内只执行一次
                if (themeUpdateTimeout) {
                    clearTimeout(themeUpdateTimeout);
                }
                themeUpdateTimeout = setTimeout(() => {
                    log.debug('PIXLY Theme', formatLog(LOG.THEME_AUTO_APPLY, {}));
                    bruteForceApplyTheme(theme);
                    themeUpdateTimeout = null;
                }, 200);
            }
        });
        
        // 监听整个body的DOM变化
        themeObserver.observe(document.body, {
            childList: true,
            subtree: true
        });
        
        log.debug('PIXLY Theme', formatLog(LOG.THEME_OBSERVER_STARTED, {}));
    }

    /**
     * 停止主题监听器
     */
    function stopThemeObserver() {
        if (themeObserver) {
            themeObserver.disconnect();
            themeObserver = null;
            log.debug('PIXLY Theme', formatLog(LOG.THEME_OBSERVER_STOPPED, {}));
        }
        if (themeUpdateTimeout) {
            clearTimeout(themeUpdateTimeout);
            themeUpdateTimeout = null;
        }
    }

    /**
     * 🔄 切换主题
     */
    function toggleTheme() {
        currentTheme = currentTheme === 'dark' ? 'light' : 'dark';
        
        log.info('PIXLY Theme', formatLog(LOG.THEME_SWITCHED, { from: null, to: currentTheme }));

        // 更NewlocalStorage
        localStorage.setItem('pixly-theme', currentTheme);

        // 更NewHTMLattribute
        if (currentTheme === 'dark') {
            document.documentElement.removeAttribute('data-theme');
        } else {
            document.documentElement.setAttribute('data-theme', 'light');
        }

        // 更New图标
        const themeToggle = document.getElementById('themeToggle');
        if (themeToggle) {
            const i18n = window.i18n || { t: (key) => key };
            themeToggle.textContent = THEMES[currentTheme].icon;
            themeToggle.title = currentTheme === 'dark' ? i18n.t('title.switchToLight') : i18n.t('title.switchToDark');
        }
        
        // 立即应用
        bruteForceApplyTheme(currentTheme);

        // 启动智能监听器
        startThemeObserver(currentTheme);

        log.debug('PIXLY Theme', formatLog(LOG.THEME_SWITCH_COMPLETED, {}));
    }

    /**
     * 🚀 initialized主题system
     */
    function initTheme() {
        log.info('PIXLY Theme', formatLog(LOG.THEME_INIT, {}));
        
        // readsave主题
        const savedTheme = localStorage.getItem('pixly-theme') || 'light';
        currentTheme = savedTheme;
        
        // 应用初始主题
        if (currentTheme === 'dark') {
            document.documentElement.removeAttribute('data-theme');
        } else {
            document.documentElement.setAttribute('data-theme', 'light');
        }

        // 绑定切换按钮
        const themeToggle = document.getElementById('themeToggle');
        if (themeToggle) {
            const i18n = window.i18n || { t: (key) => key };
            themeToggle.textContent = THEMES[currentTheme].icon;
            themeToggle.title = currentTheme === 'dark' ? i18n.t('title.switchToLight') : i18n.t('title.switchToDark');
            
            // 移除allOld事件监听器（passed克隆elements）
            const newThemeToggle = themeToggle.cloneNode(true);
            themeToggle.parentNode.replaceChild(newThemeToggle, themeToggle);
        
            // 绑定New事件
            newThemeToggle.addEventListener('click', (e) => {
                e.preventDefault();
                e.stopPropagation();
                toggleTheme();
            }, true);

            log.debug('PIXLY Theme', formatLog(LOG.THEME_BUTTON_BOUND, {}));
        }

        // 立即应用主题
        bruteForceApplyTheme(currentTheme);

        // 启动智能监听器
        startThemeObserver(currentTheme);

        log.info('PIXLY Theme', formatLog(LOG.THEME_INIT_COMPLETE, {}));
    }

    // 延迟initialized，确保DOMloadcomplete
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            setTimeout(initTheme, 100);
        });
    } else {
        setTimeout(initTheme, 100);
    }

    // 导出to全局
    if (typeof window.PIXLY === 'undefined') {
        window.PIXLY = {};
    }

    window.PIXLY.Theme = {
        toggle: toggleTheme,
        apply: bruteForceApplyTheme,
        getCurrentTheme: () => currentTheme
    };
    
    log.debug('PIXLY Theme', formatLog(LOG.THEME_MODULE_LOADED, {}));
})();
