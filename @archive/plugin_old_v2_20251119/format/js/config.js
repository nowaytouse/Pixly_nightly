/**
 * PIXLY Format Configuration
 * 
 * This file controls plugin behavior including logging levels
 */

const config = {
    // Development mode (enables DEBUG logs)
    development: false,  // Set to true for development
    
    // Log level: 'DEBUG', 'INFO', 'WARN', 'ERROR'
    logLevel: 'INFO',
    
    // Feature flags
    features: {
        enableAI: true,
        enableValidation: true,
        enableMetadata: true
    },
    
    // Performance settings
    performance: {
        maxConcurrentConversions: 4,
        conversionTimeout: 300000  // 5 minutes
    }
};

// Auto-detect development mode from environment
if (typeof process !== 'undefined' && process.env) {
    if (process.env.NODE_ENV === 'development') {
        config.development = true;
        config.logLevel = 'DEBUG';
    }
}

// Export
if (typeof module !== 'undefined' && module.exports) {
    module.exports = config;
}
