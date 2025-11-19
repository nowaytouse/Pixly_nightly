/**
 * PIXLY Format - Log Constants
 * Centralized log message management
 */

const LOG = {
    // ==================== i18n Module ====================
    I18N_MODULE_LOADED: 'i18n module loaded',
    I18N_INITIALIZED: 'Initialized with locale: {locale}',
    I18N_UNSUPPORTED_LOCALE: 'Unsupported locale: {locale}',
    I18N_LANGUAGE_SWITCHED: 'Language switched to: {locale}',
    I18N_ELEMENTS_UPDATED: 'Updated {count} elements',
    
    // ==================== Initialization ====================
    INIT_COMPLETE: 'Initialization complete',
    INIT_THEME: 'Theme initialized',
    
    // ==================== Format Switching ====================
    FORMAT_SWITCHED: 'Format switched to: {format}',
    TYPE_SWITCHED: 'Conversion type switched to: {type}',
    
    // ==================== Eagle Lifecycle ====================
    EAGLE_API_UNAVAILABLE: 'Eagle API unavailable',
    EAGLE_PLUGIN_CREATED: 'Plugin created',
    EAGLE_PLUGIN_SHOWN: 'Plugin shown',
    EAGLE_PLUGIN_RUNNING: 'Plugin running',
    EAGLE_PLUGIN_HIDDEN: 'Plugin hidden',
    EAGLE_PLUGIN_EXITING: 'Plugin exiting',
    
    // ==================== File Loading ====================
    FILE_LOADING_START: 'Starting file selection',
    FILE_LOADING_EAGLE_CALL: 'Calling eagle.item.getSelected()',
    FILE_LOADING_EAGLE_RETURNED: 'Eagle returned {count} items',
    FILE_LOADING_NO_FILES: 'No files selected',
    FILE_LOADING_TYPE_INFO: 'Conversion type: {type}, supported extensions: {exts}',
    FILE_LOADING_FILE_CHECK: 'File: {name}, ext: {ext}, supported: {supported}',
    FILE_LOADING_FILTERED: 'Filtered files count: {count}',
    FILE_LOADING_COMPLETE: 'Loaded {count} files',
    FILE_LOADING_ERROR: 'Failed to load files',
    FILE_LOADING_ERROR_STACK: 'Error stack',
    
    // ==================== File Management ====================
    FILES_CLEARED: 'File list cleared',
    
    // ==================== Rust Core Detection ====================
    RUST_CORE_FOUND: 'Rust core found at: {path}',
    RUST_CORE_VERSION: 'Version: {version}',
    RUST_CORE_NOT_FOUND: 'Rust core not found',
    RUST_CORE_DETECTION_FAILED: 'Rust core detection failed',
    
    // ==================== Conversion ====================
    CONVERSION_START: 'Starting conversion',
    CONVERSION_TYPE: 'Conversion type: {type}',
    CONVERSION_PARAMS: 'Conversion parameters',
    CONVERSION_FILE_FAILED: 'Conversion failed for file: {name}',
    CONVERSION_COMPLETE: 'Conversion complete',
    CONVERSION_ERROR: 'Conversion failed',
    
    // ==================== Progress ====================
    PROGRESS_UPDATE: 'Progress: {current}/{total} - {filename}',
    
    // ==================== Results ====================
    RESULTS_SUCCESS: 'Successfully converted {count} files',
    RESULTS_FAILED: 'Failed to convert {count} files',
};

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = LOG;
}
