const fs = require('fs').promises;
const path = require('path');

/**
 * Validates conversion output file
 * @param {string} filePath - Path to output file
 * @param {Object} options - Validation options
 * @returns {Promise<boolean>} True if valid, throws error if invalid
 */
async function validateOutputFile(filePath, options = {}) {
    const {
        minSize = 1,           // Minimum file size in bytes
        maxSize = Infinity,    // Maximum file size in bytes
        timeout = 5000         // Max wait time for file to appear
    } = options;

    // Wait for file to appear (handle async file operations)
    const startTime = Date.now();
    while (Date.now() - startTime < timeout) {
        try {
            const stats = await fs.stat(filePath);
            
            // Check file size
            if (stats.size < minSize) {
                throw new Error(i18next.t('error.fileSizeBelowMin', { size: stats.size, minSize: minSize }));
            }
            
            if (stats.size > maxSize) {
                throw new Error(i18next.t('error.fileSizeExceedsMax', { size: stats.size, maxSize: maxSize }));
            }
            
            return true;
        } catch (error) {
            if (error.code === 'ENOENT') {
                // File doesn't exist yet, wait and retry
                await new Promise(resolve => setTimeout(resolve, 100));
                continue;
            }
            throw error;
        }
    }
    
    throw new Error(i18next.t('error.outputFileNotFound', { filePath: filePath }));
}

/**
 * Constructs output file path from options
 * @param {string} dest - Destination directory
 * @param {Object} options - Conversion options
 * @returns {string} Full output file path
 */
function getOutputPath(dest, options) {
    return path.join(dest, `${options.fileName}.${options.format}`);
}

module.exports = {
    validateOutputFile,
    getOutputPath
};