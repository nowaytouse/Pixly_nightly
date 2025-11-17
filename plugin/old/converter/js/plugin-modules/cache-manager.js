/**
 * PIXLY cache管理module
 * 依赖: PIXLY命名空间, Node.js APIs
 */
(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    /**
     * cache管理器
     */
    const CacheManager = {
        
        /**
         * cacheconfig
         */
        config: {
            cacheDir: null,  // 将ininit时settings
            maxAge: 7 * 24 * 60 * 60 * 1000, // 7天
            maxSize: 500 * 1024 * 1024, // 500MB
            checkInterval: 60 * 60 * 1000 // 1小时
        },
        
        /**
         * initializedcache管理器
         */
        init: function() {
            const os = require('os');
            const path = require('path');
            
            // settingscachedirectory
            this.config.cacheDir = path.join(os.tmpdir(), 'pixly-cache');
            
            // 确保cachedirectoryexists
            this.ensureCacheDir();
            
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_INITIALIZED, { dir: this.config.cacheDir }));
            }
        },
        
        /**
         * 确保cachedirectoryexists
         */
        ensureCacheDir: function() {
            const fs = require('fs');
            
            if (!fs.existsSync(this.config.cacheDir)) {
                try {
                    fs.mkdirSync(this.config.cacheDir, { recursive: true });
                    const log = window.pixlyLog;
                    if (log) {
                        log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_DIR_CREATED, {}));
                    }
                } catch (error) {
                    const log = window.pixlyLog;
                    if (log) {
                        log.error('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_DIR_CREATE_FAILED, { error: error.message || error }));
                    }
                }
            }
        },
        
        /**
         * 获取cache统计
         */
        getStats: function() {
            const fs = require('fs');
            const path = require('path');
            
            if (!fs.existsSync(this.config.cacheDir)) {
                return {
                    fileCount: 0,
                    totalSize: 0,
                    oldestFile: null,
                    newestFile: null
                };
            }
            
            try {
                const files = fs.readdirSync(this.config.cacheDir);
                let totalSize = 0;
                let oldestTime = Date.now();
                let newestTime = 0;
                let oldestFile = null;
                let newestFile = null;
                
                for (const file of files) {
                    const filePath = path.join(this.config.cacheDir, file);
                    const stats = fs.statSync(filePath);
                    
                    totalSize += stats.size;
                    
                    const mtime = stats.mtime.getTime();
                    if (mtime < oldestTime) {
                        oldestTime = mtime;
                        oldestFile = file;
                    }
                    if (mtime > newestTime) {
                        newestTime = mtime;
                        newestFile = file;
                    }
                }
                
                return {
                    fileCount: files.length,
                    totalSize: totalSize,
                    totalSizeFormatted: this.formatBytes(totalSize),
                    oldestFile: oldestFile,
                    newestFile: newestFile,
                    oldestTime: oldestTime,
                    newestTime: newestTime
                };
            } catch (error) {
                const log = window.pixlyLog;
                if (log) {
                    log.error('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_STATS_FAILED, { error: error.message || error }));
                }
                return {
                    fileCount: 0,
                    totalSize: 0,
                    error: error.message
                };
            }
        },
        
        /**
         * 清理过期cache
         */
        cleanExpired: async function() {
            const fs = require('fs');
            const path = require('path');
            
            if (!fs.existsSync(this.config.cacheDir)) {
                const log = window.pixlyLog;
                if (log) {
                    log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_NO_DIR, {}));
                }
                return { deletedCount: 0, freedSpace: 0 };
            }
            
            try {
                const now = Date.now();
                const files = fs.readdirSync(this.config.cacheDir);
                let deletedCount = 0;
                let freedSpace = 0;
                
                for (const file of files) {
                    const filePath = path.join(this.config.cacheDir, file);
                    const stats = fs.statSync(filePath);
                    const age = now - stats.mtime.getTime();
                    
                    if (age > this.config.maxAge) {
                        const size = stats.size;
                        fs.unlinkSync(filePath);
                        deletedCount++;
                        freedSpace += size;
                        const log = window.pixlyLog;
                        if (log) {
                            log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_EXPIRED_DELETED, { file }));
                        }
                    }
                }
                
                const log = window.pixlyLog;
                if (log) {
                    log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_CLEANUP_COMPLETE, { count: deletedCount, size: this.formatBytes(freedSpace) }));
                }
                
                return {
                    deletedCount,
                    freedSpace,
                    freedSpaceFormatted: this.formatBytes(freedSpace)
                };
            } catch (error) {
                const log = window.pixlyLog;
                if (log) {
                    log.error('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_CLEANUP_FAILED, { error: error.message || error }));
                }
                return {
                    deletedCount: 0,
                    freedSpace: 0,
                    error: error.message
                };
            }
        },
        
        /**
         * 清空allcache
         */
        clearAll: async function() {
            const fs = require('fs');
            const path = require('path');
            
            if (!fs.existsSync(this.config.cacheDir)) {
                const log = window.pixlyLog;
                if (log) {
                    log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_NO_DIR, {}));
                }
                return { deletedCount: 0, freedSpace: 0 };
            }
            
            try {
                const files = fs.readdirSync(this.config.cacheDir);
                let deletedCount = 0;
                let freedSpace = 0;
                
                for (const file of files) {
                    const filePath = path.join(this.config.cacheDir, file);
                    const stats = fs.statSync(filePath);
                    const size = stats.size;
                    
                    fs.unlinkSync(filePath);
                    deletedCount++;
                    freedSpace += size;
                }
                
                const log = window.pixlyLog;
                if (log) {
                    log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_CLEARED, { count: deletedCount, size: this.formatBytes(freedSpace) }));
                }
                
                return {
                    deletedCount,
                    freedSpace,
                    freedSpaceFormatted: this.formatBytes(freedSpace)
                };
            } catch (error) {
                const log = window.pixlyLog;
                if (log) {
                    log.error('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_CLEAR_FAILED, { error: error.message || error }));
                }
                return {
                    deletedCount: 0,
                    freedSpace: 0,
                    error: error.message
                };
            }
        },
        
        /**
         * 自动清理cache
         */
        autoClean: async function() {
                const log = window.pixlyLog;
                if (log) {
                    log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_AUTO_CLEANUP, {}));
                }
            
            // 先清理过期file
            const expiredResult = await this.cleanExpired();
            
            // Check总大小
            const stats = this.getStats();
            
            if (stats.totalSize > this.config.maxSize) {
                    const log = window.pixlyLog;
                    if (log) {
                        log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_EXCEEDS_LIMIT, { current: stats.totalSizeformatted, limit: this.formatBytes(this.config.maxSize) }));
                    }
                
                // 清理最Oldfile直to低于限制
                const cleanResult = await this.cleanOldest(stats.totalSize - this.config.maxSize);
                
                return {
                    expired: expiredResult,
                    oldest: cleanResult,
                    totalFreed: expiredResult.freedSpace + cleanResult.freedSpace
                };
            }
            
            return {
                expired: expiredResult,
                totalFreed: expiredResult.freedSpace
            };
        },
        
        /**
         * 清理最Oldfile
         */
        cleanOldest: async function(bytesToFree) {
            const fs = require('fs');
            const path = require('path');
            
            if (!fs.existsSync(this.config.cacheDir)) {
                return { deletedCount: 0, freedSpace: 0 };
            }
            
            try {
                const files = fs.readdirSync(this.config.cacheDir)
                    .map(file => {
                        const filePath = path.join(this.config.cacheDir, file);
                        const stats = fs.statSync(filePath);
                        return {
                            name: file,
                            path: filePath,
                            size: stats.size,
                            mtime: stats.mtime.getTime()
                        };
                    })
                    .sort((a, b) => a.mtime - b.mtime); // 最Oldin前
                
                let deletedCount = 0;
                let freedSpace = 0;
                
                for (const file of files) {
                    if (freedSpace >= bytesToFree) break;
                    
                    fs.unlinkSync(file.path);
                    deletedCount++;
                    freedSpace += file.size;
                        const log = window.pixlyLog;
                        if (log) {
                            log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_OLD_DELETED, { file: file.name }));
                        }
                }
                
                    const log = window.pixlyLog;
                    if (log) {
                        log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_FREED_SPACE, { size: this.formatBytes(freedSpace) }));
                    }
                
                return {
                    deletedCount,
                    freedSpace,
                    freedSpaceFormatted: this.formatBytes(freedSpace)
                };
            } catch (error) {
                const log = window.pixlyLog;
                if (log) {
                    log.error('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_OLD_CLEANUP_FAILED, { error: error.message || error }));
                }
                return {
                    deletedCount: 0,
                    freedSpace: 0,
                    error: error.message
                };
            }
        },
        
        /**
         * 格式化字节大小
         */
        formatBytes: function(bytes) {
            if (PIXLY.utils && PIXLY.utils.formatFileSize) {
                return PIXLY.utils.formatFileSize(bytes);
            }
            
            if (bytes === 0) return '0 B';
            const k = 1024;
            const sizes = ['B', 'KB', 'MB', 'GB'];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
        }
    };
    
    // 暴露to全局
    PIXLY.CacheManager = CacheManager;
    
    // 向后兼容
    window.autoCleanCache = function() {
        return CacheManager.autoClean();
    };
    
    window.getCacheStats = function() {
        return CacheManager.getStats();
    };
    
    window.clearAllCache = function() {
        return CacheManager.clearAll();
    };
    
    // initialized
    if (typeof require !== 'undefined') {
        CacheManager.init();
    }
    
    const log = window.pixlyLog;
    if (log) {
        log.info('PIXLY Cache', formatLog(LOG.CACHE_MANAGER_MODULE_LOADED, {}));
    }
    
})(window);
