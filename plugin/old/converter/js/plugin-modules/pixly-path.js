/**
 * ==========================================
 * PIXLY Path Resolver
 * ==========================================
 * 
 * 依赖: Node.js fs, path, execSync（Eagle环境提供）
 * 
 * 安全性增强：
 * - 兼容Eagle插件环境
 * - 自动fallback机制
 * - 统一错误处理
 * 
 * @module PixlyPath
 * @version 2.0.0 (增强安全性)
 * @date 2025-11-05
 */

(function() {
    'use strict';

    // Log instance for the module
    const log = window.pixlyLog;

    /**
     * 安全获取当前工作目录
     * @returns {string|null} 当前工作目录，失败返回null
     */
    const safeGetCwd = () => {
        try {
            if (typeof process !== 'undefined' && process.cwd) {
                return process.cwd();
            }
        } catch (e) {
            if (log) {
                log.debug('PIXLY Path', formatLog(LOG.PIXLY_PATH_CWD_FALLBACK, {}));
            }
        }
        return null;
    };

    /**
     * 安全获取__dirname
     * @returns {string|null} __dirname，失败返回null
     */
    const safeGetDirname = () => {
        try {
            if (typeof __dirname !== 'undefined') {
                return __dirname;
            }
        } catch (e) {
            if (log) {
                log.debug('PIXLY Path', formatLog(LOG.PIXLY_PATH_DIRNAME_FALLBACK, {}));
            }
        }
        return null;
    };

    /**
     * 安全获取__filename
     * @returns {string|null} __filename，失败返回null
     */
    const safeGetFilename = () => {
        try {
            if (typeof __filename !== 'undefined') {
                return __filename;
            }
        } catch (e) {
            if (log) {
                log.debug('PIXLY Path', formatLog(LOG.PIXLY_PATH_FILENAME_FALLBACK, {}));
            }
        }
        return null;
    };
    
    window.PIXLY = window.PIXLY || {};
    
    let cachedPixlyPath = null;
    
    /**
     * 智能获取 PIXLY 二进制filepath
     * 优先级：1. cache → 2. pluginbuilt-in → 3. systeminstalled → 4. null
     */
    function getPixlyBinary() {
        // 如果cache，直接返回
        if (cachedPixlyPath) {
            return cachedPixlyPath;
        }
        
        const fs = require('fs');
        const path = require('path');
        const os = require('os');
        const { execSync } = require('child_process');
        
        const platform = os.platform();
        let binDir = null;
        let detectionMethod = '';
        
        // 方法1: 使用 __dirname (安全访问)
        const dirname = safeGetDirname();
        if (dirname) {
            let testPath = path.join(dirname, '..', '..', 'bin');
            if (fs.existsSync(testPath)) {
                binDir = testPath;
                detectionMethod = '__dirname';
            }
        }
        
        // 方法2: 使用 window.location
        if (!binDir && typeof window !== 'undefined' && window.location) {
            const currentPath = window.location.pathname;
            const dirPath = currentPath.substring(0, currentPath.lastIndexOf('/'));
            const testPath = path.join(dirPath, 'bin');
            if (fs.existsSync(testPath)) {
                binDir = testPath;
                detectionMethod = 'window.location';
            }
        }
        
        // 方法3: 使用 process.cwd() (安全访问)
        if (!binDir) {
            const cwd = safeGetCwd();
            if (cwd) {
                let testPath = path.join(cwd, 'bin');
                if (fs.existsSync(testPath)) {
                    binDir = testPath;
                    detectionMethod = 'cwd';
                }
            }
        }
        
        // 确定二进制file名
        let binaryName = 'pixly';
        if (platform === 'darwin') {
            binaryName = 'pixly-darwin';
        } else if (platform === 'win32') {
            binaryName = 'pixly.exe';
        } else if (platform === 'linux') {
            binaryName = 'pixly-linux';
        }
        
        // 如果Foundpluginbuilt-in PIXLY
        if (binDir) {
            const pixlyPath = path.join(binDir, binaryName);
            if (fs.existsSync(pixlyPath)) {
                if (log) {
                    log.info('PIXLY Path', formatLog(LOG.PIXLY_PATH_FOUND_BUILTIN, { path: pixlyPath, method: detectionMethod }));
                }
                cachedPixlyPath = pixlyPath;
                return pixlyPath;
            }
        }
        
        // 2️⃣ trysysteminstalling PIXLY
        try {
            const systemPath = execSync('which pixly', { encoding: 'utf-8' }).trim();
            if (systemPath && fs.existsSync(systemPath)) {
                if (log) {
                    log.info('PIXLY Path', formatLog(LOG.PIXLY_PATH_FOUND_SYSTEM, { path: systemPath }));
                }
                cachedPixlyPath = systemPath;
                return systemPath;
            }
        } catch (e) {
            // system未installing PIXLY
        }
        
        // 3️⃣ 都没Found
        if (log) {
            log.warn('PIXLY Path', formatLog(LOG.PIXLY_PATH_NOT_FOUND, {}));
            log.warn('PIXLY Path', formatLog(LOG.PIXLY_PATH_ENSURE_1, {}));
            log.warn('PIXLY Path', formatLog(LOG.PIXLY_PATH_ENSURE_2, {}));
        }
        
        return null;
    }
    
    /**
     * detected PIXLY version
     */
    function detectPixlyVersion() {
        const pixlyPath = getPixlyBinary();
        if (!pixlyPath) return null;
        
        try {
            const { execSync } = require('child_process');
            const version = execSync(`"${pixlyPath}" version`, { 
                encoding: 'utf-8',
                timeout: 5000 
            }).trim();
            return version;
        } catch (e) {
            if (log) {
                log.error('PIXLY Path', formatLog(LOG.PIXLY_PATH_VERSION_FAILED, { error: e.message || e }));
            }
            return null;
        }
    }
    
    /**
     * Check PIXLY is否available
     */
    function isPixlyAvailable() {
        return getPixlyBinary() !== null;
    }
    
    /**
     * 清除pathcache
     */
    function clearPathCache() {
        cachedPixlyPath = null;
    }
    
    // 暴露to全局
    PIXLY.path = {
        getBinary: getPixlyBinary,
        detectVersion: detectPixlyVersion,
        isAvailable: isPixlyAvailable,
        clearCache: clearPathCache
    };
    
    // 兼容Old代码
    window.getPixlyBinary = getPixlyBinary;
    
    if (log) {
        log.info('PIXLY Path', formatLog(LOG.PIXLY_PATH_MODULE_LOADED, {}));
    }
    
})(window);
