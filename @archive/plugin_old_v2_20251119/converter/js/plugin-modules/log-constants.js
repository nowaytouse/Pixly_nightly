/**
 * 📋 Log Constants & Keys
 * 统一管理所有日志消息，便于搜索、修改和国际化（如需要）
 * 使用方式: pixlyLog.info('PIXLY UI', LOG.UI_PRESET_CHANGED, { preset: 'balanced' })
 */

const LOG = {
    // ========== Template Loading ==========
    TEMPLATE_INJECTED: 'Injected {template} into {container}',
    TEMPLATE_LOADED: 'Loaded: {name} ({size} chars)',
    TEMPLATE_LOADING: 'Loading: {path}',
    TEMPLATE_ALL_LOADED: 'All {count} templates loaded in {time}ms',
    
    // ========== UI Handlers ==========
    UI_SELECTOR_BOUND: 'Log level selector bound',
    UI_DESCRIPTION_BOUND: 'Feature description click event bound',
    UI_HELP_MODAL_CLOSE_BOUND: 'Help modal close button bound',
    UI_VIDEO_AI_ENABLED: 'Video AI always enabled in smart mode',
    UI_ADVANCED_FEATURES: 'Advanced features: {status}',
    UI_FORCE_TRANSFORMER: 'Force Transformer: {status}',
    UI_VMAF_VALIDATION: 'VMAF validation: {status}',
    UI_IMAGE_PRESET_BOUND: 'Image preset radio buttons bound with visual feedback',
    UI_PRESET_UPDATED: 'Updated to {mode} mode info',
    UI_PRESET_CHANGED: 'Image preset: {preset}',
    UI_VIDEO_PRESET_CHANGED: 'Video AI preset: {preset}',
    UI_RADIO_BOUND: 'Radio buttons bound with visual feedback',
    UI_MODE_TABS_BOUND: 'Mode tabs (smart/manual) bound',
    UI_SWITCHED_TO_IMAGE: 'Switched to Image Conversion',
    UI_SWITCHED_TO_VIDEO: 'Switched to {type} Processing',
    UI_FILE_REFRESH_TRIGGERED: 'File list refresh triggered (conversion type switch)',
    UI_LOG_LEVEL_BOUND: 'Log level selector bound',
    UI_VIDEO_AI_PRESETS_BOUND: 'Video AI preset radio buttons bound with visual feedback',
    UI_VIDEO_AI_CONTROLS_BOUND: 'Video AI advanced controls and presets bound',
    UI_VIDEO_MODE: 'Video mode: {mode}',
    UI_VIDEO_MODE_TABS_BOUND: 'Video mode tabs (smart/manual) bound',
    UI_DROPDOWN_BOUND: 'Dropdown click logic bound',
    UI_DROPDOWN_TOGGLED: 'Dropdown toggled',
    UI_CPU_CORES_DETECTED: 'CPU cores: {count}, max workers: {max}',
    UI_CPU_CORES_DEFAULT: 'Cannot detect CPU cores, using default value 16',
    UI_JXL_MODE_SWITCHED: 'Switched to JXL mode, dropdown shows: Normal',
    UI_NORMAL_MODE_SWITCHED: 'Switched to normal mode, dropdown shows: JXL',
    UI_SMART_MODE_ENABLED: 'Smart mode enabled: JPEG lossless + HEIC preprocessing + animation detection',
    UI_NORMAL_CONVERSION_SELECTED: 'Normal conversion selected',
    UI_JXL_CONVERSION_SELECTED: 'JPEG→JXL lossless conversion selected',
    UI_JXL_BATCH_COMPLETE: 'JPEG→JXL batch conversion completed: {success}/{total} success',
    UI_JPEG_LOSSLESS_CHANGED: 'JPEG Lossless checkbox changed: {status}',
    UI_CHECKBOX_BOUND: 'JPEG Lossless checkbox bound',
    UI_AI_VALIDATION_CHANGED: 'AI file validation: {status}',
    UI_FORMAT_CORRECTION_CHANGED: 'Auto format correction: {status}',
    UI_QUICK_TOOLS_BOUND: 'Quick Tools: AI validation & format correction bound',
    UI_CRF_SLIDER_BOUND: 'CRF slider bound',
    UI_LANGUAGE_CHANGED: 'Language changed, refreshing dynamic content...',
    UI_DYNAMIC_REFRESHED: 'Dynamic content refreshed',
    UI_INIT_COMPLETE: 'Plugin initialization complete',
    UI_IMAGE_SMART_ACTIVATED: 'Image panel: Smart mode activated by default',
    UI_VIDEO_SMART_ACTIVATED: 'Video panel: Smart mode activated by default',
    UI_DETECTING_CORE: 'Detecting PIXLY core...',
    UI_CORE_NOT_DETECTED: 'PIXLY core not detected',
    UI_SCOPE_FILTER_INIT: 'Scope filter initialized',
    UI_AI_TOGGLE_NOT_FOUND: 'No advanced AI toggle buttons found',
    UI_DEPENDENCY_INIT: 'Image video-for-animation dependency initialized',
    UI_VIDEO_ANIM_READY: 'Video-for-animation ready',
    UI_SMART_MODE_ACTIVATED: 'Smart mode activated by default',
    UI_PRORES_MODE: 'Switched to ProRes mode',
    UI_ENCODER_SWITCHED: 'Switched to {encoder} encoder',
    UI_CONTAINER_SWITCHED: 'Switched to {container} container',
    UI_COMPATIBILITY_UPDATED: 'Updated compatibility: {compat}',
    UI_LOG_LEVEL_SWITCHED: 'Log Level switched to {level}',
    UI_LANGUAGE_SWITCH: 'Switch language: {locale}',
    UI_LANGUAGE_CHANGED: 'Language changed, refreshing dynamic content...',
    UI_DYNAMIC_REFRESHED: 'Dynamic content refreshed',
    UI_ENCODER_SWITCHED: 'Switched to {encoder} encoder',
    UI_CONTAINER_SWITCHED: 'Switched to {container} container',
    UI_MODE_SWITCHED: 'Switched to {mode}',
    UI_VIDEO_INPUT_SWITCH: 'Video input type switch: {type}',
    UI_ANIMATION_MODE: 'Animation mode: Simplified UI',
    UI_VIDEO_MODE_FULL: 'Video mode: Full UI',
    UI_VIDEO_CONV_STARTED: 'Started unified video conversion: {type} mode, {count} files',
    UI_QUICK_ANIMATION_CONV: 'Quick conversion: Animation→MP4',
    UI_QUICK_VIDEO_OPT: 'Quick conversion: Video optimization',
    UI_QUICK_NORMALIZE: 'Quick action: Normalize filenames',
    UI_CONVERSION_FAILED: 'Conversion failed',
    UI_OPTIMIZE_FAILED: 'Optimize failed',
    UI_NORMALIZE_FAILED: 'Normalization failed',
    UI_ACTIVE_TAB_TYPE: 'Active tab type: {type}',
    UI_VIDEO_TAB_START: 'Video tab active, calling startVideoConversion()',
    UI_IMAGE_TAB_START: 'Image tab active, calling startConversion()',
    UI_CONVERSION_NOT_AVAILABLE: 'Conversion function not available',
    UI_CANCEL_VIDEO_CONV: 'Cancelling video conversion',
    UI_CANCEL_IMAGE_CONV: 'Cancelling image conversion',
    UI_AI_OPTIONS_STATUS: 'AI options {status}',
    UI_FORMAT_CHANGED: 'Format changed, triggering JPEG notice update',
    UI_FOCUS_REFRESH: 'Focus event triggered file refresh (debounced)',
    UI_CONVERSION_SWITCH: 'File list refresh triggered (conversion type switch)',
    UI_HELP_MODAL_BOUND: 'Help modal close button bound',
    UI_PARAMS_UPDATED: 'Params availability updated',
    UI_JPEG_NOTICE_UPDATE: 'JPEG Notice Update - Format: {format} isJXL: {isJXL} hasJPEG: {hasJPEG} files: {count}',
    UI_LABEL_CHANGED: 'Changed label to {label} for JXL format',
    UI_JPEG_OPTION_SHOW: 'Showing JPEG lossless option',
    UI_JPEG_OPTION_HIDE: 'Hiding JPEG lossless option',
    UI_MODE_SWITCHED_JPEG: 'Triggered JPEG notice update on mode switch',
    UI_AI_OPTIONS_ENABLED: 'AI options enabled',
    UI_AI_OPTIONS_DISABLED: 'AI options disabled',
    UI_MANUAL_CONTROLS_ENABLED: 'Enabling all manual mode controls',
    UI_MANUAL_PRE_ENABLE: 'Pre-enabling controls before detection...',
    UI_MANUAL_CHECK_SERVICES: 'Checking core services for manual mode...',
    UI_MANUAL_RUST_AVAILABLE: 'Rust CLI available, enabling controls immediately...',
    UI_VIDEO_FOR_ANIM_DISABLED: 'Video for animation disabled (format: {format})',
    UI_PIXLY_READY: 'PIXLY ready: {name}',
    UI_PIXLY_PATH: 'Path: {path}',
    UI_SCOPE_NARROWING: 'Scope narrowing: {status}',
    UI_SIZE_FILTER: 'File size filter: {status}',
    UI_RESOLUTION_FILTER: 'Resolution filter: {status}',
    UI_AI_CLIENT_INIT: 'AI client initialization complete',
    UI_AI_CLIENT_INIT_FAILED: 'AI client initialization failed',
    UI_CORE_DETECTION_START: 'Starting initial core status detection...',
    UI_AI_FORCE_DETECT: 'Forcing AI service detection...',
    
    // ========== File Handler ==========
    FILE_SELECTION_START: 'Starting file selection...',
    FILE_SELECTION_COMPLETE: 'File selection completed: {count} files in {time}ms',
    FILE_SELECTED: 'Selected {count} files',
    FILE_EAGLE_RETURNED: 'Eagle returned {count} items',
    FILE_CONVERSION_TYPE: 'Conversion type: {type}',
    FILE_UI_UPDATED: 'UI updated: {count} files displayed',
    FILE_XMP_DETECTED: 'XMP files detected, auto-merge will be applied',
    FILE_VIDEO_PLACEHOLDER: 'Video info placeholder: {text}',
    FILE_PANEL_STYLES: 'Forced panel styles',
    FILE_LIST_INFO: 'filesList info',
    
    // ========== GO Service ==========
    GO_CORE_READY: 'GO core ready: {version}',
    GO_OFFLINE: 'GO service not available',
    GO_HEALTH_CHECK: 'GO service health check: {status}',
    GO_CACHED_STATUS: 'Using cached GO service status',
    GO_DETECTING: 'Detecting GO service...',
    GO_SERVICE_DETECTED: 'Detected service running on port {port}',
    GO_VERSION_RESPONSE: 'Version response',
    GO_VERSION_DETECTED: 'Service version detected: {version} on port {port}',
    GO_TOOLS_READY: 'Tools + GO ready',
    
    // ========== AI Integration ==========
    AI_TESTING_CONNECTION: 'Testing AI service connection...',
    AI_SERVICE_RESPONDED: 'AI service responded: HTTP {status}',
    AI_SERVICE_ONLINE: 'AI service is online (Go service port {port})',
    AI_INTEGRATION_INIT: 'AI integration initialized',
    AI_MODEL_CONTAINER_NOT_FOUND: 'AI model container not found',
    AI_STATS_CONTAINER_NOT_FOUND: 'AI stats container not found',
    AI_FETCHING_MODELS: 'Fetching active models...',
    AI_MODELS_RETRIEVED: 'Retrieved models',
    AI_FEATURES_DISABLED: 'AI features disabled (service unavailable)',
    
    // ========== Theme ==========
    THEME_INIT: 'Initialized new theme system...',
    THEME_FORCE_APPLY: 'Force apply theme: {theme}',
    THEME_PANELS_MODIFIED: 'Forcefully modified {count} panels (Image & Video AI options exempted)',
    THEME_SWITCHED: 'Theme switched: {from} → {to}',
    THEME_CODEC_STYLES: 'Applied codec card {theme} mode styles',
    THEME_OBSERVER_STARTED: 'Theme observer started (smart mode)',
    THEME_OBSERVER_STOPPED: 'Theme observer stopped',
    THEME_AUTO_APPLY: 'Auto-applying theme to new elements...',
    THEME_SWITCH_COMPLETED: 'Theme switch completed',
    THEME_BUTTON_BOUND: 'Theme switch button bound',
    THEME_MODULE_LOADED: 'Module loaded',
    THEME_INIT_COMPLETE: 'New Theme System initialized completed',
    
    // ========== i18n ==========
    I18N_USER_PREFERENCE: 'User preference found: {locale}',
    I18N_LOADING: 'Loading translation file: {locale}',
    I18N_LOADED: 'Translation loaded: {locale}',
    I18N_USING_MANUAL: 'Using manual translation: {locale}',
    I18N_ALREADY_USING: 'Already using {locale}',
    I18N_USER_SWITCHING: 'User switching to: {locale}',
    I18N_SWITCHED: 'Switched to manual translation: {locale}',
    I18N_SELECTOR_DEBUG: 'Selector debug: found={found}, value={value}, target={target}',
    I18N_SELECTOR_SYNCED: 'Language selector already synced: {locale}',
    I18N_TEST_TRANSLATION: 'Test translation for \'{key}\': "{value}"',
    I18N_MODE: 'Mode: {mode} | Locale: {locale}',
    I18N_ELEMENTS_UPDATED: 'Updated {count} elements (skipped {skipped})',
    I18N_INITIALIZED: 'Initialized with locale: {locale}',
    I18N_LANGUAGE_SWITCHED: 'Language switched to: {locale}',
    
    // ========== Eagle Lifecycle ==========
    EAGLE_PLUGIN_CREATED: 'Plugin created',
    EAGLE_PLUGIN_RUNNING: 'Plugin running',
    EAGLE_LIFECYCLE_REGISTERED: 'Lifecycle events registered',
    
    // ========== Kernel Guard ==========
    KERNEL_CHECKING_STATUS: 'Checking window.rustCLI status',
    KERNEL_DETECTING: 'Detecting Rust CLI...',
    KERNEL_CLI_DETECTED: 'Rust CLI detected',
    KERNEL_GUARD_INIT: 'Kernel guard initialized successfully',
    KERNEL_STILL_NOT_DETECTED: 'Rust kernel still not detected',
    
    // ========== Dependency Checker ==========
    DEP_CHECK_START: 'Starting dependency check...',
    DEP_CHECK_COMPLETE: 'Check complete. All dependencies: ✅',
    DEP_FOUND: '{name}: {version}',
    
    // ========== Cache Manager ==========
    CACHE_STATS_FAILED: 'Failed to get cache stats',
    CACHE_DIR_NOT_EXIST: 'Cache directory does not exist, no need to {action}',
    CACHE_EXPIRED_DELETED: 'Deleted expired cache: {file}',
    CACHE_CLEANUP_COMPLETE: 'Cleanup completed: Deleted {count} files, freed {size}',
    CACHE_CLEANUP_FAILED: 'Failed to clean cache',
    CACHE_CLEARED: 'Cache cleared: Deleted {count} files, freed {size}',
    CACHE_CLEAR_FAILED: 'Failed to clear cache',
    CACHE_EXCEEDS_LIMIT: 'Cache exceeds limit ({current} > {max})',
    CACHE_FREED: 'Freed {size}',
    CACHE_OLD_CLEANUP_FAILED: 'Failed to clean old cache',
    CACHE_AUTO_CLEANUP_START: 'Started auto cache cleanup...',
    
    // ========== Loader ==========
    LOADER_HTML_LOADED: 'HTML templates loaded',
    LOADER_INIT_CALLED: 'initializePlugin() called successfully',
    LOADER_EAGLE_REGISTERING: 'Registering Eagle lifecycle...',
    LOADER_EAGLE_REGISTERED: 'Eagle lifecycle registered',
    LOADER_INIT_COMPLETE: 'Plugin initialization complete!',
    
    // ========== Log Manager ==========
    LOG_LEVEL_SET: 'Log level set to: {level}',
    LOG_DEBUG_ENABLED: 'Debug mode enabled',
    LOG_DEBUG_DISABLED: 'Debug mode disabled',
    LOG_MANAGER_LOADED: 'Log Manager loaded | Current level: {level}',
    
    // ========== Main ==========
    MAIN_DOM_LOADING: 'DOM Loading complete, Starting Initializing...',
    MAIN_EAGLE_READY: 'Eagle API ready',
    MAIN_EAGLE_NOT_READY: 'Eagle API not ready',
    
    // ========== Conversion ==========
    CONV_JPEG_JXL_SUCCESS: '[{current}/{total}] {file} → JXL success',
    CONV_JPEG_JXL_ERROR: '[{current}/{total}] {file} conversion error',
    CONV_BATCH_COMPLETE: 'JPEG→JXL batch conversion completed: {success}/{total} success',
    CONV_VALIDATION_STANDALONE: 'AI file validation (standalone)',
    CONV_VALIDATION_RESULTS: 'Validation results',
    CONV_CORRECTION_STANDALONE: 'Format correction (standalone)',
    CONV_CORRECTION_SUCCESS: 'Correction success: {old} → {new}',
    CONV_CORRECTION_FAILED: 'Correction failed: {file}',
    CONV_QUICK_ANIMATION_MP4: 'Quick conversion: Animation→MP4',
    CONV_QUICK_VIDEO_OPTIMIZE: 'Quick conversion: Video optimization',
    CONV_QUICK_NORMALIZE: 'Quick action: Normalize filenames',
    CONV_NORMALIZE_FAILED: 'Normalization failed',
    CONV_ANIMATION_TO_VIDEO: 'Animation to video: {ext} → VIDEO',
    CONV_CONVERTING: 'Converting: {input} → {output}',
    
    // ========== Quick Tools ==========
    TOOLS_AI_VALIDATION: 'AI file validation: {status}',
    TOOLS_FORMAT_CORRECTION: 'Auto format correction: {status}',
    
    // ========== PIXLY Path ==========
    PATH_BINARY_NOT_FOUND: 'PIXLY binary file not found',
    PATH_ENSURE_1: 'Plugin bin/ directory exists',
    PATH_ENSURE_2: 'Or PIXLY is installed in system',
    PATH_SYSTEM_VERSION_FOUND: 'Found system-installed version: {path}',
    
    // ========== Video ==========
    VIDEO_GOP_SIZE: 'GOP size: {size}',
    VIDEO_PARAMS: 'Video Encoding Parameters',
    VIDEO_MANUAL_MODE: 'Video mode: Manual (Rust core only, no AI prediction)',
    VIDEO_SMART_MODE: 'Video mode: Smart (AI-driven, GO+Rust cores active)',
    VIDEO_RUST_READY: 'Rust core ready: {version}',
    
    // ========== JPEG Detection ==========
    JPEG_DETECTION: 'JPEG detection: {hasJPEG}, files: {count}',
    
    // ========== Debug Tools ==========
    DEBUG_DROPDOWN_INFO: 'Dropdown Debug Info',
    DEBUG_TEST_SUGGESTIONS: 'Test suggestions',
    DEBUG_FORCE_SHOW: 'Force showing menu for 5 seconds...',
    DEBUG_RESTORE_NORMAL: '5 seconds ended, restored to normal',
    DEBUG_MONITORING_START: 'Started monitoring events... (10s)',
    DEBUG_MONITORING_END: 'Monitoring ended, cleaned up event listeners',
    DEBUG_TOOLS_LOADED: 'Debug tools loaded!',
    DEBUG_RUN_IN_CONSOLE: 'Run in console',
    
    // ========== Retry ==========
    RETRY_ATTEMPT_FAILED: 'Attempt {current}/{max} failed, retrying in {delay}ms...',
    
    // ========== Image Conversion Core ==========
    IMAGE_CONV_DUPLICATE_CALL: 'Conversion already in progress, ignoring duplicate call',
    IMAGE_CONV_START: 'startConversion() called',
    IMAGE_CONV_TIME: 'Time: {time}',
    IMAGE_CONV_SELECTED_FILES: 'Selected files: {count}',
    IMAGE_CONV_FLAG_SET: 'Setting conversion flag to prevent refresh interference',
    IMAGE_CONV_BTN_HIDDEN: 'Start button hidden during conversion',
    IMAGE_CONV_CANCEL_SHOWN: 'Cancel button shown',
    IMAGE_CONV_NO_FILES: 'No files selected',
    IMAGE_CONV_RUST_NOT_AVAILABLE: 'Rust CLI not available',
    IMAGE_CONV_HEIC_WARNING: 'User cancelled: animated images + HEIC format incompatible',
    IMAGE_CONV_HEIC_CONFIRMED: 'User confirmed: converting animated images to HEIC (will lose animation)',
    IMAGE_CONV_CANCELLED: 'Conversion cancelled, all flags cleared',
    IMAGE_CONV_SUSPICIOUS_FILES: 'Conversion cancelled (suspicious files), all flags cleared',
    IMAGE_CONV_PROGRESS_FOUND: 'progressSection found: {found}',
    IMAGE_CONV_INLINE_PROGRESS_FOUND: 'inlineProgressSection found: {found}',
    IMAGE_CONV_INLINE_SHOWN: 'Inline progress bar shown',
    IMAGE_CONV_UPDATE_PROGRESS: 'updateProgress called',
    IMAGE_CONV_PROGRESS_NOT_FOUND: 'progressSection not found!',
    IMAGE_CONV_CONVERTING_FILE: '[{current}/{total}] Converting: {file}',
    IMAGE_CONV_FILE_ANALYZE_START: 'Analyzing file: {file}',
    IMAGE_CONV_FILE_ANALYZED: 'File analyzed: {size}MB, {type}, ~{time}s',
    IMAGE_CONV_FILE_DETAILS: 'File: {size}MB, {type}, estimated: {time}s',
    IMAGE_CONV_ANALYZE_ERROR: 'File analysis FAILED!',
    IMAGE_CONV_ANALYZE_ERROR_FILE: 'File: {file}',
    IMAGE_CONV_ANALYZE_ERROR_PATH: 'Path: {path}',
    IMAGE_CONV_ANALYZE_ERROR_MSG: 'Error: {error}',
    IMAGE_CONV_ANALYZE_CAUSES: 'Possible causes',
    IMAGE_CONV_REAL_PROGRESS: 'Real progress: {percent}%',
    IMAGE_CONV_RESULT: 'Result',
    IMAGE_CONV_SUCCESS_CHECK: 'result.success: {success}',
    IMAGE_CONV_FILE_SUCCESS: 'File converted successfully: {file}',
    IMAGE_CONV_FILE_FAILED: 'File conversion failed: {file}',
    IMAGE_CONV_FILE_SKIPPED: 'File skipped: {file}',
    IMAGE_CONV_COMPLETE: 'Conversion complete',
    IMAGE_CONV_FORCE_LOCK: 'Forcing file selection lock after conversion',
    IMAGE_CONV_RECORDED_FILES: 'Recorded converted files: {count}',
    IMAGE_CONV_SELECTION_LOCKED: 'PIXLY_SELECTION_LOCKED = true (blocking auto-refresh)',
    IMAGE_CONV_BTN_DISABLED: 'Convert button disabled until file reselection',
    IMAGE_CONV_BTN_VISIBLE_DISABLED: 'Start button visible but disabled',
    IMAGE_CONV_NOTIFICATION: 'Final notification displayed',
    IMAGE_CONV_FINAL_SUMMARY: 'Final summary - Success: {success} Failed: {failed}',
    IMAGE_CONV_PARTIAL_SUCCESS: 'Showing partial-success notification',
    IMAGE_CONV_ALL_SUCCESS: 'Showing all-success notification',
    IMAGE_CONV_ALL_FAILED: 'Showing all-failed notification',
    IMAGE_CONV_CLEARING_FLAG: 'Conversion complete, clearing flag',
    IMAGE_CONV_SKIP_REFRESH: 'Skipping auto-refresh (user must select new files)',
    IMAGE_CONV_OPTIMIZING: 'Optimizing images...',
    IMAGE_CONV_OPTIMIZE_MODE: 'Optimize mode: {mode}',
    IMAGE_CONV_SMART_MODE: 'Smart mode: Delegating to Go AI ML prediction',
    IMAGE_CONV_USER_FORMAT: 'User selected format (radio): {format}',
    IMAGE_CONV_AI_PREDICT: 'AI prediction: Quality={quality}, Speed={speed}, Lossless={lossless}',
    IMAGE_CONV_CALLING_RUST: 'Calling Rust CLI for conversion',
    IMAGE_CONV_CONVERSION_OPTIONS: 'Conversion options',
    
    // 🔥 Batch 2: Progress & Completion (行377-660)
    IMAGE_CONV_PROGRESS_SIM_DISABLED: 'Progress simulation disabled by feature flag',
    IMAGE_CONV_REAL_PROGRESS: 'Real progress: {progress}%',
    IMAGE_CONV_RESULT_DATA: 'Result: {result}',
    IMAGE_CONV_RESULT_SUCCESS: 'result.success: {success}',
    IMAGE_CONV_UPDATE_PROGRESS_CALLED: 'updateProgress called',
    IMAGE_CONV_FILE_SUCCESS: 'File converted successfully: {filename}',
    IMAGE_CONV_FORMAT_CORRECTED: 'Format corrected: {oldPath} → {newPath}',
    IMAGE_CONV_FORMAT_CORRECTION_FAILED: 'Format correction failed: {error}',
    IMAGE_CONV_FILE_FAILED: 'File conversion failed: {filename} - {error}',
    IMAGE_CONV_LOCK_FORCING: 'Forcing file selection lock after conversion',
    IMAGE_CONV_FILES_RECORDED: 'Recorded converted files: {count} files',
    IMAGE_CONV_UNLOCK_CLICKED: 'User clicked unlock button',
    IMAGE_CONV_OVERLAY_HIDDEN: 'Lock overlay hidden',
    IMAGE_CONV_SELECTION_UNLOCKED: 'PIXLY_SELECTION_LOCKED = false',
    IMAGE_CONV_FILES_CLEARED: 'Cleared last converted files record',
    IMAGE_CONV_BTN_ENABLED: 'Convert button enabled',
    IMAGE_CONV_READY_NEW: 'Ready for new conversion',
    
    // 🔥 Batch 3: Config & Initialization (行680-835)
    IMAGE_CONV_OPTIMIZE_MODE_READ: 'Optimize mode: {mode}',
    IMAGE_CONV_SMART_MODE_DELEGATING: 'Smart mode: Delegating to Go AI ML prediction',
    IMAGE_CONV_USER_FORMAT_AI: 'User selected format (AI expected): {format}',
    IMAGE_CONV_USER_FORMAT_RADIO: 'User selected format (radio): {format}',
    IMAGE_CONV_MANUAL_MODE_CONFIG: 'Manual mode: {config}',
    IMAGE_CONV_CANCEL_REQUESTED: 'User requested cancellation',
    IMAGE_CONV_SELECTION_LOCKED_INIT: 'PIXLY_SELECTION_LOCKED initialized to false',
    IMAGE_CONV_LAST_FILES_INIT: 'PIXLY_LAST_CONVERTED_FILES initialized',
    IMAGE_CONV_MODULE_LOADED: 'Module loaded (Rust CLI only)',
    
    // 🔥 RUST CLI EXECUTOR (rust-cli-executor.js)
    // Batch 1: Initialization & Execution (行1-250)
    RUST_CLI_INITIALIZING: 'Initializing (Unified Path Resolver)...',
    RUST_CLI_CHECKING: 'Checking availability...',
    RUST_CLI_NOT_AVAILABLE: 'CLI not available: {error}',
    RUST_CLI_USING_PATH: 'Using CLI: {path}',
    RUST_CLI_AVAILABLE: 'Available - Version: {version}',
    RUST_CLI_INIT_FAILED: 'Init failed: {error}',
    RUST_CLI_EXECUTING: 'Executing: {command}',
    RUST_CLI_USING_ENV_PATH: 'Using PATH: {path}',
    RUST_CLI_STDOUT_HEADER: 'STDOUT:',
    RUST_CLI_STDOUT_LINE: '  | {line}',
    RUST_CLI_STDERR_HEADER: 'STDERR:',
    RUST_CLI_STDERR_LINE: '  | {line}',
    RUST_CLI_SPAWN_ERROR: 'Spawn error: {error}',
    RUST_CLI_CMD_FAILED: 'Command failed with exit code: {code}',
    RUST_CLI_EXEC_ERROR: 'Execution error: {error}',
    RUST_CLI_ASYNC_START: 'Async executing with progress: {command}',
    RUST_CLI_PROGRESS_PERCENT: '{percent}% - {message}',
    RUST_CLI_DURATION: 'Duration: {duration}s, estimated frames: {frames}',
    RUST_CLI_PROGRESS_FRAMES: '{current}/{total} frames = {percent}%',
    RUST_CLI_FRAME_NO_TOTAL: 'Frame {frame}',
    RUST_CLI_ASYNC_SPAWN_ERROR: 'Async spawn error: {error}',
    RUST_CLI_ASYNC_COMPLETED: 'Async execution completed',
    RUST_CLI_ASYNC_FAILED: 'Async execution failed with code: {code}',
    RUST_CLI_STDERR_EXCERPT: 'STDERR excerpt:',
    
    // Batch 2: Conversion & Metadata (行250-624)
    RUST_CLI_EXEC_WITH_PROGRESS: 'Executing (async): {command}',
    RUST_CLI_OUTPUT_LINE: '| {line}',
    RUST_CLI_STDERR_LINE_PREFIX: '⚠️  {line}',
    RUST_CLI_PROCESS_ERROR: 'Process error: {error}',
    RUST_CLI_CONVERTING: 'Converting: {input} → {output}',
    RUST_CLI_XMP_MERGE_ENABLED: 'XMP merge enabled: --merge-xmp',
    RUST_CLI_OPTIMIZE_MODE_SET: 'Optimize mode: {mode}',
    RUST_CLI_COMMAND_ARGS: 'Command: {args}',
    RUST_CLI_XMP_DETECTED: 'XMP file detected in selection, auto-merge enabled',
    RUST_CLI_AI_OPTIONS: 'AI Options: {options}',
    RUST_CLI_CONVERTING_IMAGE: 'Converting image (convertImage wrapper)...',
    RUST_CLI_CONVERT_OPTIONS: 'Options: {options}',
    RUST_CLI_CONVERT_FAILED: 'convertImage failed: {error}',
    RUST_CLI_TESTING: 'Testing...',
    RUST_CLI_FILE_ANALYSIS: 'File analysis: {analysis}',
    RUST_CLI_ANALYZE_FAILED: 'Failed to analyze file: {error}',
    RUST_CLI_ANALYZE_REQUIRED: 'File analysis is REQUIRED for accurate progress estimation',
    RUST_CLI_CREATING_INSTANCE: 'Creating global instance...',
    RUST_CLI_MODULE_LOADED: 'Module loaded (Unified Path Resolver)',
    RUST_CLI_STATUS: 'Status: {status}',
    RUST_CLI_VERSION_INFO: 'Version: {version}',
    RUST_CLI_PATH_INFO: 'Path: {path}',
    
    // 🔥 FILE HANDLER (file-handler.js)
    // No Fallback - 符合质量宣言
    FILE_SELECTED_COUNT: 'Selected {count} files',
    FILE_SELECTION_START: 'Starting file selection...',
    FILE_SELECTION_BLOCKED: 'File selection blocked: conversion in progress',
    FILE_KERNEL_NOT_AVAILABLE: 'Rust kernel not available!',
    FILE_SELECTED: 'Selected {count} files',
    FILE_CONVERSION_TYPE: 'Conversion type: {type}',
    FILE_SELECTION_COMPLETE: 'File selection completed in {time}ms',
    FILE_SELECTION_FAILED: 'File selection failed: {error}',
    FILE_UI_UPDATED: 'UI updated: {count} files displayed',
    FILE_XMP_DETECTED: 'XMP files detected, auto-merge will be applied',
    FILE_VIDEO_INFO_UPDATE: 'Updating video info panel for: {name}',
    FILE_RUST_CLI_NOT_AVAILABLE: 'Rust CLI not available, cannot get video info',
    FILE_LOCK_STATUS_CHANGE: 'PIXLY_SELECTION_LOCKED = {status}',
    FILE_LOCK_OVERLAY_HIDDEN: 'Lock overlay hidden (new files selected)',
    FILE_SAME_FILES_KEEPING_LOCK: 'Same files selected, keeping lock',
    FILE_FORCED_PANEL_STYLES: 'Forced panel styles - display: {display}, minHeight: {minHeight}',
    FILE_LIST_DEBUG_INFO: 'filesList info - child count: {childCount}, innerHTML length: {htmlLength}',
    
    // 🔥 CONVERSION GUARD (conversion-guard.js)
    // No Fallback - 符合质量宣言
    CONV_GUARD_HEALTH_CHECK_START: 'Starting health check...',
    CONV_GUARD_HEALTH_CHECK_DIVIDER: '━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━',
    CONV_GUARD_CHECKING_RUST_CLI: 'Checking Rust CLI...',
    CONV_GUARD_RUST_CLI_NOT_FOUND: 'Rust CLI not found',
    CONV_GUARD_RUST_CLI_NOT_AVAILABLE: 'Rust CLI not available: {error}',
    CONV_GUARD_RUST_CLI_OK: 'Rust CLI OK, version: {version}',
    CONV_GUARD_RUST_CLI_TEST_FAILED: 'Rust CLI test failed: {error}',
    CONV_GUARD_CHECKING_PATH_RESOLVER: 'Checking Path Resolver...',
    CONV_GUARD_PATH_RESOLVER_NOT_FOUND: 'Path Resolver not found',
    CONV_GUARD_PATH_RESOLVER_INCOMPLETE: 'Path Resolver incomplete',
    CONV_GUARD_PATH_RESOLVER_OK: 'Path Resolver OK, CLI path: {path}',
    CONV_GUARD_PATH_RESOLVER_FAILED: 'Path Resolver failed: {error}',
    CONV_GUARD_CHECKING_EAGLE_API: 'Checking Eagle API...',
    CONV_GUARD_EAGLE_API_NOT_FOUND: 'Eagle API not found',
    CONV_GUARD_EAGLE_API_INCOMPLETE: 'Eagle API incomplete: {apis}',
    CONV_GUARD_EAGLE_API_OK: 'Eagle API OK',
    CONV_GUARD_CHECKING_UI_BINDINGS: 'Checking UI bindings...',
    CONV_GUARD_UI_ELEMENTS_MISSING: 'UI elements missing: {elements}',
    CONV_GUARD_CONVERT_BTN_NO_LISTENER: 'Convert button listener not detected',
    CONV_GUARD_UI_BINDINGS_OK: 'UI bindings OK',
    CONV_GUARD_CHECKING_SELECTED_FILES: 'Checking selected files...',
    CONV_GUARD_SELECTED_FILES_NOT_FOUND: 'selectedFiles not found',
    CONV_GUARD_SELECTED_FILES_NOT_ARRAY: 'selectedFiles is not an array',
    CONV_GUARD_NO_FILES_SELECTED: 'No files selected yet (this is OK at startup)',
    CONV_GUARD_FILE_OBJECTS_INCOMPLETE: 'File objects incomplete: missing {props}',
    CONV_GUARD_FILES_SELECTED: '{count} files selected and valid',
    CONV_GUARD_HEALTH_CHECK_COMPLETE: 'Health check complete - {status}',
    CONV_GUARD_VALIDATION_FAILED: 'Validation failed: {errors}',
    
    // 🔥 VIDEO CONVERSION (video-conversion.js)
    // No Fallback - 符合质量宣言
    VIDEO_CONV_ALREADY_IN_PROGRESS: 'Conversion already in progress, ignoring duplicate call',
    VIDEO_CONV_DIVIDER: '━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━',
    VIDEO_CONV_START_CALLED: 'startVideoConversion() called',
    VIDEO_CONV_TIME: 'Time: {time}',
    VIDEO_CONV_SELECTED_FILES: 'Selected files: {count}',
    VIDEO_CONV_SETTING_FLAG: 'Setting conversion flag to prevent refresh interference',
    VIDEO_CONV_START_BTN_HIDDEN: 'Start button hidden during conversion',
    VIDEO_CONV_CANCEL_BTN_SHOWN: 'Cancel button shown',
    VIDEO_CONV_CONFIG: 'Conversion config: {config}',
    VIDEO_CONV_PROGRESS_SHOWN: 'Progress bar shown',
    VIDEO_CONV_CANCELLED: 'Conversion cancelled by user',
    VIDEO_CONV_ERROR: 'Conversion error: {error}',
    VIDEO_CONV_COMPLETE: 'Conversion complete, clearing flag',
    VIDEO_CONV_SMART_MODE: 'Smart mode: Using AI prediction',
    VIDEO_CONV_MANUAL_MODE: 'Manual mode: Reading UI parameters',
    VIDEO_CONV_MANUAL_PARAMS: 'Manual params: {params}',
    VIDEO_CONV_ANIMATION_TO_VIDEO: 'Animation to video: {ext} → VIDEO',
    VIDEO_CONV_CONVERTING: 'Converting: {input} → {output}',
    VIDEO_CONV_CLI_ARGS: 'Rust CLI args: {args}',
    VIDEO_CONV_RUST_OUTPUT: 'Rust: {data}',
    VIDEO_CONV_ANIM_COMPLETE: 'Phase 46.1: Animated image → Video conversion complete',
    VIDEO_CONV_RUST_HANDLES_REPLACEMENT: 'Rust CLI will handle in-place replacement (delete original + update Eagle metadata)',
    VIDEO_CONV_ARCHITECTURE: 'Architecture: JS (UI) → Rust (file operations)',
    VIDEO_CONV_CANCEL_REQUESTED: 'User requested cancellation',
    
    // 🔥 THEME SYSTEM (theme.js)
    // No Fallback - 符合质量宣言
    THEME_INIT: 'Initializing new theme system',
    THEME_FORCE_APPLY: 'Force applying theme: {theme}',
    THEME_PANELS_MODIFIED: 'Forcefully modified {count} panels (Image & Video AI options exempted)',
    THEME_SWITCHED: 'Theme switched: {from} → {to}',
    THEME_CODEC_STYLES: 'Applied codec card {theme} mode styles',
    THEME_AUTO_APPLY: 'Auto-applying theme to new elements',
    THEME_OBSERVER_STARTED: 'Theme observer started (smart mode)',
    THEME_OBSERVER_STOPPED: 'Theme observer stopped',
    THEME_SWITCH_COMPLETED: 'Theme switch completed',
    THEME_BUTTON_BOUND: 'Theme switch button bound',
    THEME_INIT_COMPLETE: 'Theme system initialization complete',
    THEME_MODULE_LOADED: 'Theme module loaded',
    
    // 🔥 PATH RESOLVER (path-resolver.js)
    // No Fallback - 符合质量宣言
    PATH_RESOLVER_INIT: 'Path Resolver initializing',
    PATH_RESOLVER_SYMLINK_DETECTED: 'Symlink detected: {path}',
    PATH_RESOLVER_REAL_PATH: 'Real path: {path}',
    PATH_RESOLVER_SYMLINK_FAILED: 'Failed to resolve symlink: {error}',
    PATH_RESOLVER_PLUGIN_ROOT: 'Plugin root: {path}',
    PATH_RESOLVER_PROJECT_ROOT: 'Project root: {path}',
    PATH_RESOLVER_RUST_CORE: 'Rust core: {path}',
    PATH_RESOLVER_GO_CORE: 'GO core: {path}',
    PATH_RESOLVER_PROJECT_DETECTED: 'Project root detected at: {path}',
    PATH_RESOLVER_PROJECT_NOT_DETECTED: 'Project root not detected, using plugin root',
    PATH_RESOLVER_NO_PROJECT_ROOT: 'Project root not available',
    PATH_RESOLVER_SEARCHING: 'Searching for executable: {name}',
    PATH_RESOLVER_CHECKING: 'Checking path [{index}/{total}]: {path}',
    PATH_RESOLVER_FOUND: 'Found executable: {path}',
    PATH_RESOLVER_NOT_EXECUTABLE: 'Found but not executable: {path}',
    PATH_RESOLVER_NOT_FOUND: 'Executable not found: {name}',
    PATH_RESOLVER_INITIALIZED: 'Path Resolver initialized',
    PATH_RESOLVER_INFO: 'Path Resolver info: {info}',
    
    // 🔥 KERNEL GUARD (kernel-guard.js)
    // No Fallback - 符合质量宣言
    KERNEL_GUARD_INIT: 'Kernel guard initializing',
    KERNEL_GUARD_WAIT_RUST: 'Waiting for rustCLI initialization ({seconds}s)',
    KERNEL_GUARD_CHECK_STATUS: 'Checking rustCLI status: {status}',
    KERNEL_GUARD_NOT_DETECTED: 'Kernel not detected: {error}',
    KERNEL_GUARD_DISABLE_FEATURES: 'Disabling all features',
    KERNEL_GUARD_ENABLE_FEATURES: 'Enabling all features',
    KERNEL_GUARD_DETECTING: 'Detecting Rust CLI',
    KERNEL_GUARD_DETECTED: 'Rust CLI detected: {info}',
    KERNEL_GUARD_DETECTION_FAILED: 'Rust CLI detection failed: {error}',
    KERNEL_GUARD_RETRY: 'Retrying kernel detection',
    KERNEL_GUARD_RETRY_SUCCESS: 'Rust kernel detected successfully',
    KERNEL_GUARD_RETRY_FAILED: 'Rust kernel still not detected',
    KERNEL_GUARD_FIX_START: 'Starting kernel fix',
    KERNEL_GUARD_AUTOFIX_FAILED: 'Auto-fix failed: {error}',
    KERNEL_GUARD_AUTOFIX_SUCCESS: 'Auto-fix successful, please redetect',
    KERNEL_GUARD_INITIALIZED: 'Kernel guard initialized successfully',
    
    // 🔥 EAGLE LIFECYCLE (eagle-lifecycle.js)
    // No Fallback - 符合质量宣言
    EAGLE_CREATED: 'Plugin created',
    EAGLE_SHOWN: 'Plugin shown',
    EAGLE_DEBOUNCED: 'Debounced: skipping duplicate onShow call',
    EAGLE_CONVERTING: 'Conversion in progress, skipping file refresh',
    EAGLE_LOCKED: 'Selection locked, skipping file refresh',
    EAGLE_LOCKED_TIP: 'User must select NEW files to unlock',
    EAGLE_REFRESHED: 'File list auto-refreshed',
    EAGLE_SELECT_NOT_FOUND: 'selectFiles function not found',
    EAGLE_SELECT_TIP: 'Please ensure file-handler.js is loaded',
    EAGLE_RUNNING: 'Plugin running',
    EAGLE_HIDDEN: 'Plugin hidden',
    EAGLE_EXIT: 'Plugin about to exit',
    EAGLE_GRACEFUL_SHUTDOWN: 'Attempting graceful process shutdown (SIGTERM)',
    EAGLE_FORCE_KILL: 'Process still running, force killing (SIGKILL)',
    EAGLE_CLEANUP_FAILED: 'Cleanup process failed: {error}',
    EAGLE_CLEAR_TIMERS: 'Clearing {count} active timers',
    EAGLE_TIMER_CLEANUP_FAILED: 'Timer cleanup failed: {error}',
    EAGLE_API_NOT_AVAILABLE: 'Eagle API not available',
    EAGLE_REGISTERED: 'Lifecycle events registered',
    EAGLE_MODULE_LOADED: 'Lifecycle module loaded',
    
    // 🔥 GLOBALS (globals.js)
    // No Fallback - 符合质量宣言
    GLOBALS_PROGRESS_UPDATE: 'Progress v3.1.2 - {progress}% | [{current}/{total}] | {message}',
    GLOBALS_PROGRESS_DOM_CHECK: 'DOM elements check: {status}',
    GLOBALS_PROGRESS_ELEMENTS_NOT_FOUND: 'Progress elements not found! Missing: {missing}',
    GLOBALS_PROGRESS_ROLLBACK: 'Failed rollback animation triggered (1.5s)',
    GLOBALS_UPDATE_PROGRESS_REGISTERED: 'updateProgress registered: {type}',
    GLOBALS_UPDATE_SUBPROGRESS_REGISTERED: 'updateSubProgress registered: {type}',
    GLOBALS_MODULE_LOADED: 'v{version} Global Variables module loaded',
    
    // 🔥 LOGGER (logger.js)
    // No Fallback - 符合质量宣言
    LOGGER_EAGLE_API_ERROR: 'Eagle API error: {error}',
    LOGGER_MODULE_LOADED: 'Logger System module loaded',
    
    // 🔥 UTILS (utils.js)
    // No Fallback - 符合质量宣言
    UTILS_TOOLTIP_INITIALIZED: 'Tooltip system initialized with i18n support',
    UTILS_RETRY_ATTEMPT: 'Retry attempt {attempt}/{maxAttempts} failed, retrying in {delay}ms',
    UTILS_MODULE_LOADED: 'Utility Functions module loaded',
    
    // 🔥 FILE VALIDATOR (file-validator.js)
    // No Fallback - 符合质量宣言
    FILE_VALIDATOR_CACHED: 'Using cached result: {file}',
    FILE_VALIDATOR_VALIDATING: 'Validating: {file}',
    FILE_VALIDATOR_FAILED: 'Validation failed: {error}',
    FILE_VALIDATOR_BATCH_START: 'Batch validation: {count} files',
    FILE_VALIDATOR_BATCH_COMPLETE: 'Batch validation complete: {results}',
    FILE_VALIDATOR_RUST_CLI_FAILED: 'Rust CLI call failed: {error}',
    FILE_VALIDATOR_FORMAT_MISMATCH: 'Format mismatch recorded: {file} (.{from} → .{to})',
    FILE_VALIDATOR_CORRECTION_DISABLED: 'Format correction disabled, skipping',
    FILE_VALIDATOR_NO_CORRECTION: 'No format correction needed for: {file}',
    FILE_VALIDATOR_APPLYING_CORRECTION: 'Applying format correction: {file} (.{from} → .{to})',
    FILE_VALIDATOR_RENAME_SUCCESS: 'File renamed: {from} → {to}',
    FILE_VALIDATOR_RENAME_FAILED: 'File rename failed: {error}',
    FILE_VALIDATOR_PLUGIN_ERROR: 'Plugin error during rename: {error}',
    FILE_VALIDATOR_NO_ERRORS: 'No validation errors found',
    FILE_VALIDATOR_ERRORS_FOUND: 'Found {count} validation errors',
    FILE_VALIDATOR_GENERATE_REPORT: 'Validation report: {report}',
    FILE_VALIDATOR_METADATA_ERROR: 'Metadata extraction error: {error}',
    FILE_VALIDATOR_CACHE_STATS: 'Cache stats: {stats}',
    FILE_VALIDATOR_CACHE_CLEARED: 'Cache cleared',
    FILE_VALIDATOR_MODULE_LOADED: 'File Validator loaded (Phase 45.4)',
    
    // 🔥 PERFORMANCE MONITOR (performance-monitor.js)
    // No Fallback - 符合质量宣言
    PERF_MONITOR_INIT: 'Performance Monitor initialized',
    PERF_MONITOR_STARTED: 'Started: {name}',
    PERF_MONITOR_ENDED: 'Ended: {name} - Duration: {duration}ms',
    PERF_MONITOR_PHASE_START: 'Phase {phase} started',
    PERF_MONITOR_PHASE_END: 'Phase {phase} completed in {duration}ms',
    PERF_MONITOR_WARNING: 'Performance warning: {operation} took {duration}ms',
    PERF_MONITOR_CRITICAL: 'Critical performance issue: {operation} took {duration}ms',
    PERF_MONITOR_REPORT: 'Performance report: {report}',
    PERF_MONITOR_MEMORY: 'Memory usage: {usage}MB / {limit}MB',
    PERF_MONITOR_OPTIMIZATION: 'Optimization applied: {type}',
    PERF_MONITOR_THRESHOLD_EXCEEDED: 'Threshold exceeded for {operation}: {duration}ms > {threshold}ms',
    PERF_MONITOR_MODULE_LOADED: 'Performance Monitor loaded',
    
    // 🔥 AI INTEGRATION (ai-integration.js)
    // No Fallback - 符合质量宣言
    AI_INTEGRATION_INIT: 'AI Integration initializing',
    AI_INTEGRATION_ENABLED: 'AI features enabled',
    AI_INTEGRATION_DISABLED: 'AI features disabled',
    AI_INTEGRATION_PREDICTING: 'AI predicting for: {file}',
    AI_INTEGRATION_PREDICTION_SUCCESS: 'AI prediction successful: {result}',
    AI_INTEGRATION_PREDICTION_FAILED: 'AI prediction failed: {error}',
    AI_INTEGRATION_FALLBACK_USED: 'Using fallback prediction',
    AI_INTEGRATION_RECORDING: 'Recording observation: {data}',
    AI_INTEGRATION_RECORDING_FAILED: 'Recording failed: {error}',
    AI_INTEGRATION_VALIDATION: 'Validating with AI: {file}',
    AI_INTEGRATION_VALIDATION_SUCCESS: 'AI validation successful',
    AI_INTEGRATION_VALIDATION_FAILED: 'AI validation failed: {error}',
    AI_INTEGRATION_GO_UNAVAILABLE: 'Go AI unavailable, features limited',
    AI_INTEGRATION_MODULE_LOADED: 'AI Integration module loaded',
    
    // 🔥 SMALL MODULES - Quick cleanup
    // No Fallback - 符合质量宣言
    TOAST_MODULE_LOADED: 'Toast module loaded',
    GPU_DETECTION_MODULE_LOADED: 'GPU Detection module loaded',
    AI_CLIENT_MODULE_LOADED: 'AI Client module loaded',
    
    // 🔥 QUALITY SLIDER (quality-slider.js)
    // No Fallback - 符合质量宣言
    QUALITY_SLIDER_NOT_FOUND: 'Quality slider element not found',
    QUALITY_SLIDER_INITIALIZED: 'Quality slider initialized',
    QUALITY_SLIDER_SAVED: 'Quality saved: {quality}',
    QUALITY_SLIDER_RECOMMENDED: 'Set recommended quality: {mode} -> {quality}',
    QUALITY_SLIDER_MODULE_LOADED: 'Quality Slider module loaded',
    
    // 🔥 EAGLE DIALOG (eagle-dialog.js)
    // No Fallback - 符合质量宣言
    EAGLE_DIALOG_API_UNAVAILABLE: 'Eagle API not available, using Toast fallback',
    EAGLE_DIALOG_FAILED: 'Dialog failed: {error}',
    EAGLE_DIALOG_ALERT_FAILED: 'Alert dialog failed: {error}',
    EAGLE_DIALOG_NO_NOTIFICATION: 'No notification system available',
    EAGLE_DIALOG_CONFIRM_FAILED: 'Confirm dialog failed: {error}',
    EAGLE_DIALOG_CONFIRM_FALLBACK: 'Confirm fallback - auto-confirming',
    EAGLE_DIALOG_MODULE_LOADED: 'Unified dialog system loaded',
    
    // 🔥 VIDEO AI CLIENT (video-ai-client.js)
    // No Fallback - 符合质量宣言
    VIDEO_AI_CLIENT_PRESET_APPLIED: 'Applied preset: {preset}',
    VIDEO_AI_CLIENT_MODULE_LOADED: 'Video AI Client module loaded (v3.0 - Enhanced Prediction)',
    
    // 🔥 CACHE MANAGER (cache-manager.js)
    // No Fallback - 符合质量宣言
    CACHE_MANAGER_INITIALIZED: 'Cache Manager initialized: {dir}',
    CACHE_MANAGER_DIR_CREATED: 'Cache directory created',
    CACHE_MANAGER_DIR_CREATE_FAILED: 'Create cache directory failed: {error}',
    CACHE_MANAGER_STATS_FAILED: 'Failed to get cache stats: {error}',
    CACHE_MANAGER_NO_DIR: 'Cache directory does not exist, no need to clean',
    CACHE_MANAGER_EXPIRED_DELETED: 'Deleted expired cache: {file}',
    CACHE_MANAGER_CLEANUP_COMPLETE: 'Cleanup completed: Deleted {count} files, freed {size}',
    CACHE_MANAGER_CLEANUP_FAILED: 'Failed to clean cache: {error}',
    CACHE_MANAGER_CLEARED: 'Cache cleared: Deleted {count} files, freed {size}',
    CACHE_MANAGER_CLEAR_FAILED: 'Failed to clear cache: {error}',
    CACHE_MANAGER_AUTO_CLEANUP: 'Started auto cache cleanup',
    CACHE_MANAGER_EXCEEDS_LIMIT: 'Cache exceeds limit ({current} > {limit})',
    CACHE_MANAGER_OLD_DELETED: 'Deleted old cache: {file}',
    CACHE_MANAGER_FREED_SPACE: 'Freed {size}',
    CACHE_MANAGER_OLD_CLEANUP_FAILED: 'Failed to clean old cache: {error}',
    CACHE_MANAGER_MODULE_LOADED: 'Cache Manager module loaded',
    
    // 🔥 TEMPLATE LOADER (template-loader.js)
    // No Fallback - 符合质量宣言
    TEMPLATE_LOADER_INIT: 'Template Loader initializing',
    TEMPLATE_LOADER_ALREADY_LOADED: '{template} already loaded',
    TEMPLATE_LOADER_LOADING: 'Loading: {url}',
    TEMPLATE_LOADER_LOADED: 'Loaded: {template} ({size} chars)',
    TEMPLATE_LOADER_LOAD_FAILED: 'Failed to load {template}: {error}',
    TEMPLATE_LOADER_INJECTED: 'Injected {template} into #{container}',
    TEMPLATE_LOADER_LOADING_ALL: 'Loading all templates',
    TEMPLATE_LOADER_OUTER_START: '[1/2] Loading outer templates',
    TEMPLATE_LOADER_OUTER_COMPLETE: '[1/2] Outer templates loaded',
    TEMPLATE_LOADER_INNER_START: '[2/2] Loading inner panel templates',
    TEMPLATE_LOADER_INNER_COMPLETE: '[2/2] Inner panel templates loaded',
    TEMPLATE_LOADER_MODALS_START: '[3/3] Loading modals',
    TEMPLATE_LOADER_MODALS_COMPLETE: '[3/3] Modals loaded',
    TEMPLATE_LOADER_ALL_COMPLETE: 'All {count} templates loaded in {duration}ms',
    TEMPLATE_LOADER_ALL_FAILED: 'Template loading failed: {error}',
    TEMPLATE_LOADER_MODULE_LOADED: 'Template Loader module loaded',
    
    // 🔥 FEATURE FLAGS (feature-flags.js)
    // No Fallback - 符合质量宣言
    FEATURE_FLAGS_LOADING: 'Feature Flags loading',
    FEATURE_FLAGS_LOADED_STORAGE: 'Loaded from localStorage: {count} overrides',
    FEATURE_FLAGS_USING_DEFAULTS: 'Using default configuration',
    FEATURE_FLAGS_LOAD_FAILED: 'Failed to load: {error}',
    FEATURE_FLAGS_SAVED: 'Saved: {count} overrides',
    FEATURE_FLAGS_SAVE_FAILED: 'Failed to save: {error}',
    FEATURE_FLAGS_TOGGLED: '{status} {flag}',
    FEATURE_FLAGS_RESET: 'Reset to defaults',
    FEATURE_FLAGS_LISTENER_ERROR: 'Listener error for {flag}: {error}',
    FEATURE_FLAGS_IMPORTED: 'Imported configuration',
    FEATURE_FLAGS_IMPORT_FAILED: 'Failed to import: {error}',
    FEATURE_FLAGS_MODULE_LOADED: 'Feature Flags module loaded',
    FEATURE_FLAGS_ACTIVE_COUNT: 'Active features: {count}',
    
    // 🔥 VIDEO PARAMS (video-params.js)
    // Hybrid Strategy - preserve console.group for dev tools + pixlyLog for production
    VIDEO_PARAMS_DISPLAYED: 'Video encoding parameters displayed',
    VIDEO_PARAMS_MODULE_LOADED: 'Video params module loaded',
    
    // 🔥 DEPENDENCY CHECKER (dependency-checker.js)
    // No Fallback - 符合质量宣言
    DEPENDENCY_CHECK_START: 'Starting dependency check',
    DEPENDENCY_CHECK_COMPLETE: 'Check complete. All dependencies: {status}',
    DEPENDENCY_FOUND: '{name}: {version}',
    DEPENDENCY_NOT_FOUND: '{name}: Not found',
    DEPENDENCY_AUTO_INSTALL_START: 'Starting auto-install dependencies',
    DEPENDENCY_INSTALLING: 'Installing {name}',
    DEPENDENCY_INSTALL_SUCCESS: '{name} installed successfully',
    DEPENDENCY_INSTALL_FAILED: '{name} installation failed: {error}',
    DEPENDENCY_CHECKER_MODULE_LOADED: 'Dependency checker module loaded',
    
    // 🔥 CROSS-PLATFORM LOG COLLECTOR (cross-platform-log-collector.js)
    // No Fallback - 符合质量宣言
    LOG_COLLECTOR_INIT: 'Log collector initializing',
    LOG_COLLECTOR_INITIALIZED: 'Log collector initialized',
    LOG_COLLECTOR_RUST_NOT_FOUND: 'Rust CLI not found, skipping integration',
    LOG_COLLECTOR_GO_DETECTED: 'GO service detected, starting log polling',
    LOG_COLLECTOR_GO_NOT_AVAILABLE: 'GO service not available, skipping integration',
    LOG_COLLECTOR_GO_API_UNAVAILABLE: 'GO log API not available, stopping polling',
    LOG_COLLECTOR_SUBSCRIBER_ERROR: 'Subscriber error: {error}',

    // === Conversion Validator ===
    // 🔥 Phase 40.33: Multi-level validation system
    // No fallback - All logs use pixlyLog
    CONVERSION_VALIDATOR_START: 'Starting multi-level validation',
    CONVERSION_VALIDATOR_INPUT_FAILED: 'Input validation failed: {errors}',
    CONVERSION_VALIDATOR_LEVEL1_PASSED: 'Level 1 passed: Input files validated',
    CONVERSION_VALIDATOR_PARAM_FAILED: 'Parameter validation failed: {errors}',
    CONVERSION_VALIDATOR_LEVEL2_PASSED: 'Level 2 passed: Parameters validated',
    CONVERSION_VALIDATOR_MODE_FAILED: 'Mode consistency validation failed: {errors}',
    CONVERSION_VALIDATOR_LEVEL3_PASSED: 'Level 3 passed: Mode consistency validated',
    CONVERSION_VALIDATOR_ALL_PASSED: 'All validation levels passed',
    CONVERSION_VALIDATOR_WARNINGS: 'Warnings: {warnings}',
    CONVERSION_VALIDATOR_MODULE_LOADED: 'Multi-level validation system loaded',

    // === PIXLY Path Resolver ===
    // Path detection and binary resolution
    // No fallback - All logs use pixlyLog
    PIXLY_PATH_CWD_FALLBACK: 'Cannot access process.cwd(), using fallback',
    PIXLY_PATH_DIRNAME_FALLBACK: 'Cannot access __dirname, using fallback',
    PIXLY_PATH_FILENAME_FALLBACK: 'Cannot access __filename, using fallback',
    PIXLY_PATH_FOUND_BUILTIN: 'Found built-in version: {path} ({method})',
    PIXLY_PATH_FOUND_SYSTEM: 'Found system-installed version: {path}',
    PIXLY_PATH_NOT_FOUND: 'PIXLY binary file not found',
    PIXLY_PATH_ENSURE_1: 'Please ensure: Plugin bin/ directory exists',
    PIXLY_PATH_ENSURE_2: 'Or PIXLY is installed in system',
    PIXLY_PATH_VERSION_FAILED: 'Version detection failed: {error}',
    PIXLY_PATH_MODULE_LOADED: 'Path detected module loaded',

    // === File Handler ===
    // UI layer file selection and video info
    // No fallback - Loud errors > Silent degradation
    FILE_HANDLER_RUST_CLI_CALL: 'Calling Rust CLI: {path}',
    FILE_HANDLER_RUST_CLI_ARGS: 'Args: {args}',
    FILE_HANDLER_RUST_CLI_FILE: 'File path: {filePath}',
    FILE_HANDLER_RUST_CLI_FAILED: 'Rust CLI failed with code: {code}',
    FILE_HANDLER_RUST_CLI_COMMAND: 'Command: {command}',
    FILE_HANDLER_RUST_CLI_STDERR: 'Stderr: {stderr}',
    FILE_HANDLER_RUST_CLI_STDOUT: 'Stdout: {stdout}',
    FILE_HANDLER_VIDEO_INFO_SUCCESS: 'Got video info: {info}',
    FILE_HANDLER_VIDEO_INFO_DETAILS: 'Info details: duration={duration}, frames={frames}, fps={fps}',
    FILE_HANDLER_VIDEO_INFO_PARSE_FAILED: 'Failed to parse video info: {error}',
    FILE_HANDLER_VIDEO_INFO_ERROR: 'Error getting video info: {error}',
    FILE_HANDLER_VIDEO_INFO_PLACEHOLDER: 'Video info placeholder: {message}',
    FILE_HANDLER_FILE_REMOVED: 'Removed file at index {index}',

    // === Performance Monitor ===
    // Performance tracking and reporting - HYBRID strategy (console.group preserved)
    PERF_MONITOR_ENABLED: 'Performance monitoring enabled',
    PERF_MONITOR_DISABLED: 'Performance monitoring disabled', 
    PERF_MONITOR_WARNING: '{warning}',
    PERF_MONITOR_MEMORY_NOT_AVAILABLE: 'Memory API not available',
    PERF_MONITOR_MODULE_LOADED: 'Performance Monitor loaded'
};

/**
 * 🔧 格式化日志消息（支持占位符替换）
 * @param {string} template - 消息模板，如 "File: {name}, Size: {size}"
 * @param {Object} params - 参数对象，如 { name: 'test.jpg', size: '1.2MB' }
 * @returns {string} 格式化后的消息
 * 
 * @example
 * formatLog(LOG.FILE_SELECTED, { count: 5 })
 * // 返回: "Selected 5 files"
 */
function formatLog(template, params = {}) {
    if (!params || typeof params !== 'object') {
        return template;
    }
    
    return template.replace(/\{(\w+)\}/g, (match, key) => {
        return params.hasOwnProperty(key) ? params[key] : match;
    });
}

// 导出
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { LOG, formatLog };
}
if (typeof window !== 'undefined') {
    window.LOG = LOG;
    window.formatLog = formatLog;
}
