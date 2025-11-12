/**
 * UI Modules Index - 统一导出所有UI模块
 * 
 * 将原本4440行的ui-handlers.js拆分为多个职责单一的模块
 * 每个模块负责特定功能领域
 */

// 核心初始化
export { UIInitializer } from './ui-initializer.js';

// 事件管理
export { EventManager } from './event-manager.js';

// 模式管理（智能/手动）
export { ModeManager } from './mode-manager.js';

// 格式管理
export { FormatManager } from './format-manager.js';

// AI功能管理
export { AIFeatureManager } from './ai-feature-manager.js';

// 范围过滤
export { ScopeFilterManager } from './scope-filter-manager.js';

// UI更新管理
export { UIUpdater } from './ui-updater.js';

// 服务检测
export { ServiceDetector } from './service-detector.js';

// 全局状态管理
export { StateManager } from './state-manager.js';
