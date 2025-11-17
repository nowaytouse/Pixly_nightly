/**
 * PIXLY module加载器
 * 负责按顺序加载all modules file
 * version: v3.0.0 (modular version)
 */
(function() {
    'use strict';
    
    // 🎯 统一日志实例（延迟获取，避免初始化顺序问题）
    const getLog = () => window.pixlyLog || console;
    
    getLog().info('[PIXLY Loader] 🚀 Loading modular plugin');
    
    // 🔥 Phase 40: 模块文件列表（去除数字前缀，使用直白命名）
    // 🎯 按依赖关系分层加载
    const modules = [
        // === Layer -3: 日志常量 (最最先加载) ===
        'log-constants.js',        // 📋 日志消息常量定义 v1.0.0
        
        // === Layer -2: 基础工具 ===
        'log-manager.js',          // 📝 分级日志管理器 v1.0.0 (增强版:去重+节流)
        'cross-platform-log-collector.js',  // 🌐 跨平台日志收集器 v1.0.0 (统一JS/Rust/GO)
        'performance-utils.js',    // 🚀 性能优化工具 v1.0.0 (LRU缓存+快速hash+节流)
        'performance-monitor.js',  // 🔬 性能监控工具 v1.0.0 (可选)
        
        // === Layer -1: 路径解析 ===
        'path-resolver.js',        // 🔧 统一路径解析器 v1.0.0
        
        // === Layer 0: 配置系统 ===
        'config-manager.js',       // 配置管理器 v1.0.0
        
        // === Phase 41: HTML模板加载器 ===
        'template-loader.js',      // HTML模板动态加载 (fetch API)
        
        // === Layer 1: 核心基础 ===
        'globals.js',              // 全局变量和config
        'logger.js',               // 日志系统
        'utils.js',                // 工具函数
        'pixly-path.js',           // PIXLY路径检测
        'gpu-detection.js',        // GPU检测
        
        // === Layer 2: 统一日志系统 ===
        'logger.js',               // 统一日志 v3.0.0 (First)
        'pixly-logging.js',        // 日志打印器 (扩展)
        'log-constants.js',        // 日志常量定义 (原38-log-constants)
        'error-handler.js',        // 错误处理增强     // 工具函数
        
        // === Layer 2: 系统检查 ===
        'dependency-checker.js',   // 依赖检查
        'cache-manager.js',        // 缓存管理
        // 'validation.js',        // 已迁移到Rust (media_analyzer.rs::OutputValidator)
        // 'validation.js',        // ❌ 已迁移到Rust (media_analyzer.rs::OutputValidator)
        
        // === Layer 3: 文件与转换 ===
        'file-validator.js',       // 🔒 Phase 45.4: AI文件类型验证（Magika集成）
        'file-handler.js',      // ✅ UI层: selectFiles, getSelectedFiles (Eagle API)
        
        // === 🔄 Phase 2: Bridge层（在转换模块之前加载）===
        'kernel-bridge.js',        // 🌉 Kernel通信桥接
        'validation-bridge.js',    // 🔍 验证系统桥接
        'ai-bridge.js',            // 🤖 AI系统桥接
        'param-compatibility.js',  // 🎛️ 参数兼容性检查
        
        // === 🔥 Phase 3: 参数收集器（模块化拆分）===
        'format-params-collector.js',      // 📦 格式专属参数收集器
        'conversion-config-collector.js',  // ⚙️ 转换配置收集器
        'conversion-executor.js',          // 🚀 转换执行器
        
        'image-conversion.js',     // 🔥 图像转换核心（Rust CLI）
        'video-conversion.js',     // 🎬 Phase 40.34: 视频转换核心（Rust CLI）
        // 'video-processing.js',  // ❌ 已废弃 - JS不应直接调用ffmpeg → 使用 Rust CLI video 子命令
        'video-params.js',         // 视频编码参数
        
        // === Layer 4: UI基础组件 ===
        // 'i18n-helpers.js',      // ❌ 已废弃（重复i18n.js）
        'theme.js',                // 主题切换
        'toast.js',                // Toast通知
        'quality-slider.js',       // 质量滑块
        // 'format-selector.js',   // ❌ 已废弃（过度封装UI逻辑）
        
        // === Layer 5: UI交互 ===
        'ui-handlers.js',          // UI事件处理
        'eagle-lifecycle.js',      // Eagle生命周期
        
        // === Layer 6: 高级功能 ===
        // 'params-builder.js',    // ❌ 已废弃 - JS不应构建处理参数 → Rust CLI直接构建，Go AI预测
        // 'deduplicator.js',      // ❌ 已废弃（应由Rust处理） → Rust file_manager.rs
        'eagle-dialog.js',         // Eagle对话框
        
        // === Layer 7: AI功能 ===
        'ai-client.js',            // 🧠 AI智能预测客户端 v4.2.0
        // 'format-recommender.js', // ❌ 已废弃（违反架构，Go AI用ML预测）
        // 'ssim-validator.js',    // ❌ 已废弃 - SSIM迁移到Rust media_analyzer.rs
        'observation-recorder.js', // 📊 观测数据记录器 (仅HTTP调用Go AI)
        'video-ai-client.js',      // 🎬 视频AI客户端
        
        // === Layer 8: Rust集成 ===
        // 'rust-client.js',  // ❌ DEPRECATED: 违反真实性原则 (34处fallback)          // 🦀 Rust HTTP客户端 (已废弃)
        'rust-cli-executor.js',    // 🦀 Rust CLI执行器 v2.0.0
        'timer-manager.js',        // 🕐 定时器管理器
        // 'log-manager.js',       // ❌ 已废弃（重复logger.js）
        
        // === Layer 9: 内核保护 ===
        'kernel-guard.js',         // 🛡️ 内核守护
        'conversion-guard.js',     // 🔒 转换保障系统 v1.0.0 (Phase 40.24)
        'ai-integration.js',       // 🤖 AI集成
    ];
    
    // 获取当前脚本基础path
    const currentScript = document.currentScript;
    const scriptPath = currentScript ? currentScript.src : '';
    const basePath = scriptPath.substring(0, scriptPath.lastIndexOf('/')) + '/plugin-modules/';
    
    getLog().info('[PIXLY Loader] 📁 Module base path:', basePath);
    
    // loadedmodule计数
    let loadedCount = 0;
    const totalModules = modules.length;
    
    // load单个module
    function loadModule(index) {
        if (index >= modules.length) {
            onAllModulesLoaded();
            return;
        }
        
        const moduleName = modules[index];
        const script = document.createElement('script');
        script.type = 'text/javascript';
        script.src = basePath + moduleName;
        
        script.onload = function() {
            loadedCount++;
            getLog().info(`[PIXLY Loader] ✅ [${loadedCount}/${totalModules}] ${moduleName} loaded successfully`);
            
            // load下一个module
            loadModule(index + 1);
        };
        
        script.onerror = function(error) {
            getLog().error(`[PIXLY Loader] ❌ ${moduleName} failed to load:`, error);
            
            // 即使failed也继续load下一个module
            loadModule(index + 1);
        };
        
        document.head.appendChild(script);
    }
    
    // all modulesloadcomplete后回调
    function onAllModulesLoaded() {
        getLog().info('[PIXLY Loader] ========================================');
        getLog().info('[PIXLY Loader] 🎉 All modules loaded successfully!');
        getLog().info('[PIXLY Loader] 📊 Loaded:', loadedCount, '/', totalModules);
        getLog().info('[PIXLY Loader] ========================================');
        
        // 检查PIXLY命名空间
        if (window.PIXLY) {
            getLog().info('[PIXLY Loader] ✅ PIXLY namespace:', Object.keys(window.PIXLY));
            getLog().info('[PIXLY Loader] ✅ Version:', window.PIXLY.VERSION || 'unknown');
        } else {
            getLog().error('[PIXLY Loader] ❌ PIXLY namespace not created!');
        }
        
        // 触发自定义事件，通知plugin准备就绪
        const event = new CustomEvent('PIXLYModulesReady', {
            detail: {
                version: window.PIXLY?.VERSION,
                modulesLoaded: loadedCount,
                totalModules: totalModules
            }
        });
        document.dispatchEvent(event);
        
        getLog().info('[PIXLY Loader] 🎯 PIXLYModulesReady event triggered');
        
        // 立即initializepluginUI（不waitingEagle事件）
        getLog().info('[PIXLY Loader] 🔧 Initializing plugin...');
        
        // waitingDOM完全load
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', initializePluginUI);
        } else {
            // DOMalreadyloadcomplete，立即initialize
            initializePluginUI();
        }
    }
    
    // 防止重复初始化
    let _uiInitialized = false;
    
    // initializepluginUI
    async function initializePluginUI() {
        // 防止重复调用
        if (_uiInitialized) {
            getLog().warn('[PIXLY Loader] ⚠️ UI already initialized, skipping...');
            return;
        }
        _uiInitialized = true;
        
        const log = window.pixlyLog || console;
        getLog().info?.('PIXLY Loader', '🎨 Initializing UI...')
        
        // Phase 41: 先加载HTML模板
        if (window.pixlyTemplateLoader) {
            try {
                getLog().debug?.('PIXLY Loader', '📥 Loading HTML templates...')
                await window.pixlyTemplateLoader.loadAll();
                getLog().info?.('PIXLY Loader', '✅ HTML templates loaded')
            } catch (error) {
                getLog().error?.('PIXLY Loader', '❌ Template loading failed:', error)
                alert('插件模板加载失败，请刷新后重试');
                _uiInitialized = false;  // 重置标志以允许重试
                return;
            }
        } else {
            getLog().error?.('PIXLY Loader', '❌ Template loader not found')
        }
        
        // 调用initializePlugin（来自06-ui-handlers.js）
        if (typeof window.initializePlugin === 'function') {
            try {
                window.initializePlugin();
                getLog().info?.('PIXLY Loader', '✅ initializePlugin() called successfully')
            } catch (error) {
                getLog().error?.('PIXLY Loader', '❌ initializePlugin() failed to call:', error);
            }
        } else {
            getLog().error?.('[PIXLY Loader] ❌ initializePlugin function not found');
        }
        
        // 注册Eagle生命周期
        if (window.PIXLY && window.PIXLY.EagleLifecycle) {
            getLog().info('[PIXLY Loader] 🦅 Registering Eagle lifecycle...');
            if (typeof eagle !== 'undefined') {
                window.PIXLY.EagleLifecycle.register();
                getLog().info('[PIXLY Loader] ✅ Eagle lifecycle registered');
            } else {
                getLog().warn('[PIXLY Loader] ⚠️ eagle API not yet available, will auto-register when available');
            }
        }
        
        getLog().info('[PIXLY Loader] ========================================');
        getLog().info('[PIXLY Loader] 🎉 Plugin initialization complete!');
        getLog().info('[PIXLY Loader] ========================================');
    }
    
    // startload第一个module
    getLog().info('[PIXLY Loader] 🔄 Loading', totalModules, 'modules in sequence...');
    loadModule(0);
    
})();
