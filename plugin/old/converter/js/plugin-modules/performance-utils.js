/**
 * Performance Optimization Utilities
 * 
 * Provides various performance optimization helpers:
 * - Fast hash functions
 * - LRU cache
 * - Throttle/debounce with RAF
 * - Memory-efficient data structures
 */
(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    /**
     * Fast hash function (FNV-1a algorithm)
     * Much faster than string concatenation for cache keys
     */
    const fastHash = (str) => {
        let hash = 2166136261; // FNV offset basis
        for (let i = 0; i < str.length; i++) {
            hash ^= str.charCodeAt(i);
            hash += (hash << 1) + (hash << 4) + (hash << 7) + (hash << 8) + (hash << 24);
        }
        return hash >>> 0; // Convert to unsigned 32-bit integer
    };
    
    /**
     * Simple fast hash for multiple strings
     * Handles null/undefined gracefully
     */
    const hashStrings = (...strings) => {
        let hash = 2166136261;
        for (const str of strings) {
            // Skip null/undefined
            if (str == null) continue;
            
            // Convert to string if not already
            const s = String(str);
            for (let i = 0; i < s.length; i++) {
                hash ^= s.charCodeAt(i);
                hash += (hash << 1) + (hash << 4) + (hash << 7) + (hash << 8) + (hash << 24);
            }
            hash ^= 0; // Mix between strings
        }
        return hash >>> 0;
    };
    
    /**
     * LRU (Least Recently Used) Cache
     * Automatically evicts oldest entries when full
     */
    class LRUCache {
        constructor(maxSize = 1000, ttl = 60000) {
            this.maxSize = maxSize;
            this.ttl = ttl;
            this.cache = new Map();
            this.accessOrder = [];
        }
        
        get(key) {
            const entry = this.cache.get(key);
            if (!entry) return null;
            
            // Check TTL
            if (Date.now() - entry.timestamp > this.ttl) {
                this.cache.delete(key);
                this._removeFromAccessOrder(key);
                return null;
            }
            
            // Update access order (move to end)
            this._updateAccessOrder(key);
            
            return entry.data;
        }
        
        set(key, data) {
            // Remove oldest if full
            if (this.cache.size >= this.maxSize && !this.cache.has(key)) {
                const oldestKey = this.accessOrder[0];
                this.cache.delete(oldestKey);
                this.accessOrder.shift();
            }
            
            this.cache.set(key, {
                data,
                timestamp: Date.now()
            });
            
            this._updateAccessOrder(key);
        }
        
        has(key) {
            return this.cache.has(key);
        }
        
        delete(key) {
            this.cache.delete(key);
            this._removeFromAccessOrder(key);
        }
        
        clear() {
            this.cache.clear();
            this.accessOrder = [];
        }
        
        get size() {
            return this.cache.size;
        }
        
        _updateAccessOrder(key) {
            this._removeFromAccessOrder(key);
            this.accessOrder.push(key);
        }
        
        _removeFromAccessOrder(key) {
            const index = this.accessOrder.indexOf(key);
            if (index > -1) {
                this.accessOrder.splice(index, 1);
            }
        }
    }
    
    /**
     * RAF-based throttle for UI updates
     * Ensures updates happen at most once per frame
     */
    class RAFThrottle {
        constructor(callback) {
            this.callback = callback;
            this.rafId = null;
            this.pendingArgs = null;
        }
        
        call(...args) {
            this.pendingArgs = args;
            
            if (!this.rafId) {
                this.rafId = requestAnimationFrame(() => {
                    if (this.pendingArgs) {
                        this.callback(...this.pendingArgs);
                        this.pendingArgs = null;
                    }
                    this.rafId = null;
                });
            }
        }
        
        cancel() {
            if (this.rafId) {
                cancelAnimationFrame(this.rafId);
                this.rafId = null;
                this.pendingArgs = null;
            }
        }
    }
    
    /**
     * Time-based throttle with configurable interval
     */
    class TimeThrottle {
        constructor(callback, interval = 100) {
            this.callback = callback;
            this.interval = interval;
            this.lastCall = 0;
            this.timeoutId = null;
            this.pendingArgs = null;
        }
        
        call(...args) {
            const now = Date.now();
            const timeSinceLastCall = now - this.lastCall;
            
            if (timeSinceLastCall >= this.interval) {
                this.callback(...args);
                this.lastCall = now;
            } else {
                // Schedule for later
                this.pendingArgs = args;
                
                if (!this.timeoutId) {
                    const remaining = this.interval - timeSinceLastCall;
                    this.timeoutId = setTimeout(() => {
                        if (this.pendingArgs) {
                            this.callback(...this.pendingArgs);
                            this.pendingArgs = null;
                        }
                        this.lastCall = Date.now();
                        this.timeoutId = null;
                    }, remaining);
                }
            }
        }
        
        cancel() {
            if (this.timeoutId) {
                clearTimeout(this.timeoutId);
                this.timeoutId = null;
                this.pendingArgs = null;
            }
        }
    }
    
    /**
     * Debounce helper
     */
    class Debounce {
        constructor(callback, delay = 300) {
            this.callback = callback;
            this.delay = delay;
            this.timeoutId = null;
        }
        
        call(...args) {
            if (this.timeoutId) {
                clearTimeout(this.timeoutId);
            }
            
            this.timeoutId = setTimeout(() => {
                this.callback(...args);
                this.timeoutId = null;
            }, this.delay);
        }
        
        cancel() {
            if (this.timeoutId) {
                clearTimeout(this.timeoutId);
                this.timeoutId = null;
            }
        }
        
        flush() {
            if (this.timeoutId) {
                clearTimeout(this.timeoutId);
                this.callback();
                this.timeoutId = null;
            }
        }
    }
    
    /**
     * Batch processor for reducing function calls
     */
    class BatchProcessor {
        constructor(processFn, maxBatchSize = 100, maxWaitMs = 50) {
            this.processFn = processFn;
            this.maxBatchSize = maxBatchSize;
            this.maxWaitMs = maxWaitMs;
            this.batch = [];
            this.timeoutId = null;
            this.firstItemTime = null;
        }
        
        add(item) {
            if (this.batch.length === 0) {
                this.firstItemTime = Date.now();
            }
            
            this.batch.push(item);
            
            // Process immediately if batch is full
            if (this.batch.length >= this.maxBatchSize) {
                this.flush();
                return;
            }
            
            // Otherwise schedule processing
            if (!this.timeoutId) {
                const elapsed = Date.now() - this.firstItemTime;
                const remaining = Math.max(0, this.maxWaitMs - elapsed);
                
                this.timeoutId = setTimeout(() => {
                    this.flush();
                }, remaining);
            }
        }
        
        flush() {
            if (this.batch.length > 0) {
                this.processFn(this.batch);
                this.batch = [];
                this.firstItemTime = null;
            }
            
            if (this.timeoutId) {
                clearTimeout(this.timeoutId);
                this.timeoutId = null;
            }
        }
    }
    
    // Export utilities
    window.PIXLY.PerformanceUtils = {
        fastHash,
        hashStrings,
        LRUCache,
        RAFThrottle,
        TimeThrottle,
        Debounce,
        BatchProcessor
    };
    
    const log = window.pixlyLog;
    if (log) {
        log.info('Performance Utils', 'Performance utilities loaded');
    }
    
})(window);
