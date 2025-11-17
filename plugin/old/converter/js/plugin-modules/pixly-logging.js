/**
 * 🎯 Pixly统一日志系统 (JavaScript)
 * 
 * Phase 46.8: 三端共享的结构化日志
 * 确保Go/Rust/JS使用相同的日志格式
 */

// 日志级别
const LogLevel = {
  DEBUG: 'DEBUG',
  INFO: 'INFO',
  WARNING: 'WARNING',
  ERROR: 'ERROR'
};

// 日志配置
let currentLogLevel = LogLevel.INFO;
let enableJSON = false; // 默认人类可读格式

/**
 * 日志条目结构
 */
class LogEntry {
  constructor(level, component, message, options = {}) {
    this.timestamp = new Date().toISOString();
    this.level = level;
    this.layer = 'js-plugin';
    this.component = component;
    this.message = message;
    this.code = options.code || null;
    this.context = options.context || {};
    this.trace_id = options.traceId || null;
    this.file = options.file || null;
    this.line = options.line || null;
  }

  /**
   * 转换为JSON字符串
   */
  toJSON() {
    const obj = {
      timestamp: this.timestamp,
      level: this.level,
      layer: this.layer,
      component: this.component,
      message: this.message
    };

    if (this.code) obj.code = this.code;
    if (Object.keys(this.context).length > 0) obj.context = this.context;
    if (this.trace_id) obj.trace_id = this.trace_id;
    if (this.file) obj.file = this.file;
    if (this.line) obj.line = this.line;

    return JSON.stringify(obj);
  }

  /**
   * 转换为人类可读字符串
   */
  toHuman() {
    const emoji = {
      [LogLevel.DEBUG]: '🔍',
      [LogLevel.INFO]: 'ℹ️',
      [LogLevel.WARNING]: '⚠️',
      [LogLevel.ERROR]: '❌'
    }[this.level] || '';

    let output = `${this.timestamp} [${this.level}] ${emoji} ${this.component}: ${this.message}`;

    if (this.code) {
      output += ` [${this.code}]`;
    }

    if (Object.keys(this.context).length > 0) {
      output += ` | Context: ${JSON.stringify(this.context)}`;
    }

    return output;
  }
}

/**
 * 设置日志级别
 */
function setLogLevel(level) {
  currentLogLevel = level;
}

/**
 * 启用JSON格式日志
 */
function enableJSONLog(enable) {
  enableJSON = enable;
}

/**
 * 判断是否应该记录该级别的日志
 */
function shouldLog(level) {
  const levels = {
    [LogLevel.DEBUG]: 0,
    [LogLevel.INFO]: 1,
    [LogLevel.WARNING]: 2,
    [LogLevel.ERROR]: 3
  };
  return levels[level] >= levels[currentLogLevel];
}

/**
 * 内部日志函数
 */
function log(level, component, message, options = {}) {
  if (!shouldLog(level)) {
    return;
  }

  const entry = new LogEntry(level, component, message, options);

  if (enableJSON) {
    console.log(entry.toJSON());
  } else {
    const humanOutput = entry.toHuman();
    switch (level) {
      case LogLevel.DEBUG:
        console.debug(humanOutput);
        break;
      case LogLevel.INFO:
        console.info(humanOutput);
        break;
      case LogLevel.WARNING:
        console.warn(humanOutput);
        break;
      case LogLevel.ERROR:
        console.error(humanOutput);
        break;
      default:
        console.log(humanOutput);
    }
  }
}

/**
 * 公共日志API
 */
const Logger = {
  /**
   * 调试日志
   */
  debug(component, message, context = {}) {
    log(LogLevel.DEBUG, component, message, { context });
  },

  /**
   * 信息日志
   */
  info(component, message, context = {}) {
    log(LogLevel.INFO, component, message, { context });
  },

  /**
   * 警告日志
   */
  warning(component, message, context = {}) {
    log(LogLevel.WARNING, component, message, { context });
  },

  /**
   * 错误日志
   */
  error(component, message, context = {}) {
    log(LogLevel.ERROR, component, message, { context });
  },

  /**
   * 记录PixlyError
   */
  logPixlyError(component, pixlyError) {
    log(LogLevel.ERROR, component, pixlyError.message, {
      code: pixlyError.code,
      context: pixlyError.context
    });
  },

  /**
   * 记录验证错误（便捷函数）
   */
  logValidationError(component, error) {
    if (error && error.code && error.code.startsWith('PIXLY')) {
      this.logPixlyError(component, error);
    } else {
      this.error(component, `Validation error: ${error.message || error}`);
    }
  },

  /**
   * 记录验证成功（便捷函数）
   */
  logValidationSuccess(component) {
    this.info(component, '✅ Validation passed');
  },

  /**
   * 记录验证警告（便捷函数）
   */
  logValidationWarning(component, message, context = {}) {
    this.warning(component, message, context);
  },

  /**
   * 性能日志辅助
   */
  startPerformanceLog(component, operation) {
    const startTime = Date.now();
    this.info(component, `🚀 Starting: ${operation}`);

    return {
      end: () => {
        const elapsed = Date.now() - startTime;
        this.info(component, `✅ Completed: ${operation} (took ${elapsed}ms)`);
      },
      endWithContext: (context = {}) => {
        const elapsed = Date.now() - startTime;
        context.duration_ms = elapsed;
        this.info(component, `✅ Completed: ${operation}`, context);
      }
    };
  }
};

/**
 * 导出
 */
module.exports = {
  LogLevel,
  LogEntry,
  Logger,
  setLogLevel,
  enableJSONLog
};
