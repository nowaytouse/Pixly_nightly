/**
 * ==========================================
 * PIXLY 路径解析模块 (Path Resolver)
 * ==========================================
 * 
 * 统一管理所有路径解析，避免硬编码
 * 
 * Eagle Plugin API: https://developer.eagle.cool/plugin-api/v/zh-cn
 * 
 * @module PathResolver
 * @version 1.0.0
 * @date 2025-11-06
 */
(function(window) {
    'use strict';
    
    const log = window.pixlyLog;
    if (!log) {
        throw new Error('pixlyLog is required for path-resolver');
    }

    log.info('PIXLY Path Resolver', formatLog(LOG.PATH_RESOLVER_INIT, {}));

    const path = require('path');
    const fs = require('fs');

    /**
     * 路径解析器类
     */
    class PathResolver {
        constructor() {
            this.pluginDir = __dirname;  // 插件模块目录
            this.pluginRoot = null;      // 插件根目录
            this.projectRoot = null;     // 项目根目录（Pixly_Nightly）
            this.coreRoots = {           // 各个核心的根目录
                rust: null,
                go: null
            };
            
            this.init();
        }

        /**
         * 初始化路径解析
         */
        init() {
            // 0. 解析符号链接（如果有）
            try {
                const realPath = fs.realpathSync(this.pluginDir);
                if (realPath !== this.pluginDir) {
                    log.debug('Path Resolver', formatLog(LOG.PATH_RESOLVER_SYMLINK_DETECTED, { path: this.pluginDir }));
                    log.debug('Path Resolver', formatLog(LOG.PATH_RESOLVER_REAL_PATH, { path: realPath }));
                    this.pluginDir = realPath;
                }
            } catch (e) {
                log.warn('Path Resolver', formatLog(LOG.PATH_RESOLVER_SYMLINK_FAILED, { error: e.message }));
            }
            
            // 1. 确定插件根目录（从模块目录向上一级）
            this.pluginRoot = path.dirname(this.pluginDir);
            log.info('Path Resolver', formatLog(LOG.PATH_RESOLVER_PLUGIN_ROOT, { path: this.pluginRoot }));

            // 2. 自动检测项目根目录
            this.projectRoot = this.detectProjectRoot();
            log.info('Path Resolver', formatLog(LOG.PATH_RESOLVER_PROJECT_ROOT, { path: this.projectRoot }));

            // 3. 检测各核心目录
            if (this.projectRoot) {
                this.coreRoots.rust = path.join(this.projectRoot, 'rust');
                this.coreRoots.go = path.join(this.projectRoot, 'go');
                
                log.info('Path Resolver', formatLog(LOG.PATH_RESOLVER_RUST_CORE, { path: this.coreRoots.rust }));
                log.info('Path Resolver', formatLog(LOG.PATH_RESOLVER_GO_CORE, { path: this.coreRoots.go }));
            }
        }

        /**
         * 自动检测项目根目录
         * 通过查找特征文件/目录来确定
         */
        detectProjectRoot() {
            let current = this.pluginRoot;
            const maxDepth = 5;  // 最多向上查找5层
            
            for (let i = 0; i < maxDepth; i++) {
                // 检查特征标记
                const markers = [
                    'rust',            // Rust核心目录
                    'go',              // GO核心目录
                    'plugin',          // 插件目录
                ];
                
                // 检查是否所有标记都存在
                const hasAllMarkers = markers.every(marker => {
                    const markerPath = path.join(current, marker);
                    return fs.existsSync(markerPath);
                });
                
                if (hasAllMarkers) {
                    log.info('Path Resolver', formatLog(LOG.PATH_RESOLVER_PROJECT_DETECTED, { path: current }));
                    return current;
                }
                
                // 向上一级
                const parent = path.dirname(current);
                if (parent === current) {
                    // 已经到达根目录
                    break;
                }
                current = parent;
            }
            
            log.warn('Path Resolver', formatLog(LOG.PATH_RESOLVER_PROJECT_NOT_DETECTED, {}));
            return this.pluginRoot;
        }

        /**
         * 查找可执行文件
         * @param {string} name - 可执行文件名（如 'pixly-rust'）
         * @param {string[]} searchPaths - 搜索路径列表（相对于projectRoot）
         * @returns {string|null} 找到的完整路径
         */
        findExecutable(name, searchPaths) {
            if (!this.projectRoot) {
                log.error('Path Resolver', formatLog(LOG.PATH_RESOLVER_NO_PROJECT_ROOT, {}));
                return null;
            }

            log.info('Path Resolver', formatLog(LOG.PATH_RESOLVER_SEARCHING, { name }));
            
            // 遍历搜索路径
            for (let i = 0; i < searchPaths.length; i++) {
                const searchPath = searchPaths[i];
                const fullPath = path.join(this.projectRoot, searchPath, name);
                
                log.debug('Path Resolver', formatLog(LOG.PATH_RESOLVER_CHECKING, { index: i+1, total: searchPaths.length, path: fullPath }));
                
                if (fs.existsSync(fullPath)) {
                    // 检查是否可执行（Unix系统）
                    try {
                        fs.accessSync(fullPath, fs.constants.X_OK);
                        log.info('Path Resolver', formatLog(LOG.PATH_RESOLVER_FOUND, { path: fullPath }));
                        return fullPath;
                    } catch (e) {
                        log.warn('Path Resolver', formatLog(LOG.PATH_RESOLVER_NOT_EXECUTABLE, { path: fullPath }));
                    }
                }
            }
            
            log.warn('Path Resolver', formatLog(LOG.PATH_RESOLVER_NOT_FOUND, { name }));
            return null;
        }

        /**
         * 获取Rust CLI路径
         * 🎯 统一Rust Kernel位置
         * @returns {string|null}
         */
        getRustCLIPath() {
            const searchPaths = [
                'converter/bin',           // 主要位置：插件内置binary
                'rust/target/release',     // 开发环境：release构建
                'rust/target/debug',       // 开发环境：debug构建
                'bin'                      // 备用位置
            ];
            return this.findExecutable('pixly-rust', searchPaths);
        }

        /**
         * 获取相对于项目根的路径
         * @param {string} relativePath
         * @returns {string}
         */
        resolve(...relativePath) {
            if (!this.projectRoot) {
                log.error('Path Resolver', formatLog(LOG.PATH_RESOLVER_NO_PROJECT_ROOT, {}));
                return path.join(...relativePath);
            }
            return path.join(this.projectRoot, ...relativePath);
        }

        /**
         * 获取相对于插件根的路径
         * @param {string} relativePath
         * @returns {string}
         */
        resolvePlugin(...relativePath) {
            return path.join(this.pluginRoot, ...relativePath);
        }

        /**
         * 检查路径是否存在
         * @param {string} filePath
         * @returns {boolean}
         */
        exists(filePath) {
            return fs.existsSync(filePath);
        }

        /**
         * 获取路径信息（调试用）
         * @returns {object}
         */
        getInfo() {
            return {
                pluginDir: this.pluginDir,
                pluginRoot: this.pluginRoot,
                projectRoot: this.projectRoot,
                coreRoots: this.coreRoots,
                rustCLI: this.getRustCLIPath()
            };
        }
    }

    // 创建全局实例
    window.PIXLY_PATH_RESOLVER = new PathResolver();
    
    log.info('PIXLY Path Resolver', formatLog(LOG.PATH_RESOLVER_INITIALIZED, {}));
    log.debug('PIXLY Path Resolver', formatLog(LOG.PATH_RESOLVER_INFO, { info: JSON.stringify(window.PIXLY_PATH_RESOLVER.getInfo()) }));

})(window);
