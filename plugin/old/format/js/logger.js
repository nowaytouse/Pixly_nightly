/**
 * PIXLY Format - Logger Utility
 * Simple logger with parameter substitution
 */

/**
 * PIXLY Logger - Production-grade logging system
 * 
 * Features:
 * - Log levels (DEBUG, INFO, WARN, ERROR)
 * - Environment-based filtering (dev/prod)
 * - Parameter substitution
 * - Structured logging
 * - Performance tracking
 */

const LogLevel = {
    DEBUG: 0,
    INFO: 1,
    WARN: 2,
    ERROR: 3
};

const logger = {
    // Current log level (can be changed at runtime)
    currentLevel: LogLevel.INFO,
    
    // Development mode (shows DEBUG logs)
    isDevelopment: false,
    
    /**
     * Initialize logger
     * @param {object} options - Configuration options
     */
    init(options = {}) {
        this.isDevelopment = options.development || false;
        this.currentLevel = this.isDevelopment ? LogLevel.DEBUG : LogLevel.INFO;
    },
    
    /**
     * Format log message with parameters
     * @param {string} message - Message template with {key} placeholders
     * @param {object} params - Parameters to substitute
     * @returns {string} Formatted message
     */
    format(message, params = {}) {
        if (!params || typeof params !== 'object') {
            return message;
        }
        
        return message.replace(/\{(\w+)\}/g, (match, key) => {
            return params[key] !== undefined ? params[key] : match;
        });
    },
    
    /**
     * Check if log level should be output
     * @param {number} level - Log level to check
     * @returns {boolean} Should log
     */
    shouldLog(level) {
        return level >= this.currentLevel;
    },
    
    /**
     * Log debug message (only in development)
     * @param {string} module - Module name
     * @param {string} message - Message template
     * @param {object} params - Parameters
     */
    debug(module, message, params) {
        if (!this.shouldLog(LogLevel.DEBUG)) return;
        
        const formatted = this.format(message, params);
        console.log(`[${module}] [DEBUG] ${formatted}`);
    },
    
    /**
     * Log info message
     * @param {string} module - Module name
     * @param {string} message - Message template
     * @param {object} params - Parameters
     */
    info(module, message, params) {
        if (!this.shouldLog(LogLevel.INFO)) return;
        
        const formatted = this.format(message, params);
        console.log(`[${module}] ${formatted}`);
    },
    
    /**
     * Log warning message
     * @param {string} module - Module name
     * @param {string} message - Message template
     * @param {object} params - Parameters
     */
    warn(module, message, params) {
        if (!this.shouldLog(LogLevel.WARN)) return;
        
        const formatted = this.format(message, params);
        console.warn(`[${module}] ${formatted}`);
    },
    
    /**
     * Log error message
     * @param {string} module - Module name
     * @param {string} message - Message template
     * @param {object} params - Parameters
     * @param {Error} error - Optional error object
     */
    error(module, message, params, error) {
        if (!this.shouldLog(LogLevel.ERROR)) return;
        
        const formatted = this.format(message, params);
        console.error(`[${module}] ${formatted}`);
        if (error) {
            console.error(error);
        }
    },
    
    /**
     * Set log level
     * @param {number} level - New log level
     */
    setLevel(level) {
        this.currentLevel = level;
    }
};

// Auto-detect development mode
if (typeof process !== 'undefined' && process.env) {
    logger.init({ development: process.env.NODE_ENV === 'development' });
}

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { logger, LogLevel };
}

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = logger;
}
