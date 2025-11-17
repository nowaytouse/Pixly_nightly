/**
 * ==========================================
 * PIXLY Configuration Manager
 * ==========================================
 *
 * 统一配置管理系统
 * - 从配置文件读取
 * - 支持环境变量覆盖
 * - 配置验证
 * - 默认值fallback
 *
 * @module ConfigManager
 * @version 1.0.0
 * @date 2025-11-05
 * @layer 0 (最先加载)
 */

(function(window) {
    'use strict';

    const Logger = window.Logger || console;

    /**
     * 默认配置（fallback）
     */
    const DEFAULT_CONFIG = {
        version: '2.0.0',
        services: {
            ai: {
                host: 'localhost',
                port: 50052,
                protocol: 'http',
                timeout: 30000,
                enabled: true
            },
            conversion: {
                host: 'localhost',
                port: 8080,
                protocol: 'http',
                timeout: 120000,
                enabled: false,
                deprecated: true
            },
            rustCLI: {
                enabled: true,
                cliPath: 'pixly-rust',
                timeout: 300000
            }
        },
        environment: {
            allowEnvOverride: true,
            envPrefix: 'PIXLY_'
        },
        logging: {
            level: 'info',
            enableDebug: false
        }
    };

    /**
     * 安全获取环境变量
     */
    function getEnvVar(key, defaultValue) {
        try {
            if (typeof process !== 'undefined' && process.env && process.env[key]) {
                return process.env[key];
            }
        } catch (e) {
            // Eagle环境可能不支持process.env
        }
        return defaultValue;
    }

    /**
     * 构建服务URL
     */
    function buildServiceUrl(serviceConfig) {
        const protocol = serviceConfig.protocol || 'http';
        const host = serviceConfig.host || 'localhost';
        const port = serviceConfig.port;
        return `${protocol}://${host}:${port}`;
    }

    /**
     * 环境变量覆盖配置
     */
    function applyEnvOverrides(config) {
        if (!config.environment.allowEnvOverride) {
            return config;
        }

        const prefix = config.environment.envPrefix || 'PIXLY_';

        // AI服务覆盖
        const aiHost = getEnvVar(`${prefix}AI_HOST`, null);
        const aiPort = getEnvVar(`${prefix}AI_PORT`, null);
        if (aiHost) config.services.ai.host = aiHost;
        if (aiPort) config.services.ai.port = parseInt(aiPort, 10);

        // 转换服务覆盖
        const convHost = getEnvVar(`${prefix}CONV_HOST`, null);
        const convPort = getEnvVar(`${prefix}CONV_PORT`, null);
        if (convHost) config.services.conversion.host = convHost;
        if (convPort) config.services.conversion.port = parseInt(convPort, 10);

        // Rust CLI覆盖
        const rustCLIPath = getEnvVar(`${prefix}RUST_CLI`, null);
        if (rustCLIPath) config.services.rustCLI.cliPath = rustCLIPath;

        return config;
    }

    /**
     * 验证配置
     */
    function validateConfig(config) {
        const errors = [];

        // 验证服务配置
        for (const [name, service] of Object.entries(config.services)) {
            if (service.port && (service.port < 1 || service.port > 65535)) {
                errors.push(`Invalid port for service '${name}': ${service.port}`);
            }
            if (service.protocol && !['http', 'https'].includes(service.protocol)) {
                errors.push(`Invalid protocol for service '${name}': ${service.protocol}`);
            }
        }

        if (errors.length > 0) {
            Logger.error('[Config] Validation errors:', errors);
            return false;
        }

        return true;
    }

    /**
     * 加载配置
     */
    async function loadConfig() {
        try {
            // 尝试从配置文件加载
            const configPath = 'config.json';
            let config = DEFAULT_CONFIG;

            try {
                const fs = require('fs');
                const path = require('path');
                
                // 安全获取__dirname（兼容Eagle环境）
                let dirname = null;
                try {
                    if (typeof __dirname !== 'undefined') {
                        dirname = __dirname;
                    }
                } catch (e) {
                    // Eagle环境可能不支持__dirname
                }
                
                if (dirname) {
                    const fullPath = path.join(dirname, '..', '..', configPath);
                    
                    if (fs.existsSync(fullPath)) {
                        const configData = fs.readFileSync(fullPath, 'utf8');
                        const fileConfig = JSON.parse(configData);
                        config = { ...DEFAULT_CONFIG, ...fileConfig };
                        Logger.info('[Config] ✅ Loaded from file:', fullPath);
                    } else {
                        Logger.info('[Config] ⚠️ Config file not found, using defaults');
                    }
                }
            } catch (e) {
                Logger.warn('[Config] ⚠️ Cannot read config file (Eagle mode?), using defaults');
            }

            // 应用环境变量覆盖
            config = applyEnvOverrides(config);

            // 验证配置
            if (!validateConfig(config)) {
                throw new Error('Configuration validation failed');
            }

            Logger.info('[Config] ✅ Configuration loaded:', {
                version: config.version,
                aiService: buildServiceUrl(config.services.ai),
                conversionService: buildServiceUrl(config.services.conversion),
                rustCLI: config.services.rustCLI.cliPath
            });

            return config;

        } catch (error) {
            Logger.error('[Config] ❌ Error loading config:', error);
            Logger.info('[Config] 🔄 Falling back to default configuration');
            return DEFAULT_CONFIG;
        }
    }

    /**
     * 配置管理器类
     */
    class ConfigManager {
        constructor() {
            this.config = null;
            this.initialized = false;
        }

        async initialize() {
            if (this.initialized) {
                return this.config;
            }

            this.config = await loadConfig();
            this.initialized = true;
            return this.config;
        }

        get(path) {
            if (!this.config) {
                Logger.warn('[Config] Config not initialized, using default');
                return this._getFromObject(DEFAULT_CONFIG, path);
            }
            return this._getFromObject(this.config, path);
        }

        _getFromObject(obj, path) {
            const keys = path.split('.');
            let current = obj;
            for (const key of keys) {
                if (current[key] === undefined) {
                    return undefined;
                }
                current = current[key];
            }
            return current;
        }

        getServiceUrl(serviceName) {
            const service = this.get(`services.${serviceName}`);
            if (!service) {
                return null;
            }
            return buildServiceUrl(service);
        }

        isServiceEnabled(serviceName) {
            const enabled = this.get(`services.${serviceName}.enabled`);
            return enabled !== false;
        }

        getConfig() {
            return this.config || DEFAULT_CONFIG;
        }
    }

    // ==========================================
    // 初始化
    // ==========================================

    (async () => {
        try {
            Logger.info('[PIXLY Config Manager] 🔧 Initializing...');

            const configManager = new ConfigManager();
            await configManager.initialize();

            // 暴露到全局
            window.PIXLY = window.PIXLY || {};
            window.PIXLY.config = configManager;

            // 便捷访问方法
            window.PIXLY.getConfig = () => configManager.getConfig();
            window.PIXLY.getServiceUrl = (service) => configManager.getServiceUrl(service);

            Logger.info('[PIXLY Config Manager] ✅ Ready');

        } catch (error) {
            Logger.error('[PIXLY Config Manager] ❌ Initialization failed:', error);
        }
    })();

})(window);
