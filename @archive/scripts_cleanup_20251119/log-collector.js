#!/usr/bin/env node

/**
 * 三端统一日志收集器
 * 
 * 功能：
 * - 收集 JS、Rust、Go 三端的 JSON 格式日志
 * - 按 request_id 聚合跨端请求
 * - 提供实时日志查看
 * - 支持日志过滤和搜索
 * 
 * 使用：
 * - 开发环境: node scripts/log-collector.js --dev
 * - 生产环境: node scripts/log-collector.js --prod --output logs/
 */

const fs = require('fs');
const path = require('path');
const readline = require('readline');

// 配置
const CONFIG = {
    dev: {
        output: null, // 实时输出到终端
        filter: 'info', // 最低日志级别
        pretty: true, // 美化输出
    },
    prod: {
        output: 'logs/pixly.log', // 输出到文件
        filter: 'warn', // 只记录警告和错误
        pretty: false, // JSON格式
    }
};

// 日志级别权重
const LOG_LEVELS = {
    trace: 0,
    debug: 1,
    info: 2,
    warn: 3,
    error: 4
};

// 日志统计
const stats = {
    total: 0,
    byLevel: { trace: 0, debug: 0, info: 0, warn: 0, error: 0 },
    byModule: {}
};

// 当前配置
let config = CONFIG.dev;
let outputStream = null;

/**
 * 解析命令行参数
 */
function parseArgs() {
    const args = process.argv.slice(2);
    
    if (args.includes('--prod')) {
        config = CONFIG.prod;
    }
    
    const outputIdx = args.indexOf('--output');
    if (outputIdx !== -1 && args[outputIdx + 1]) {
        config.output = args[outputIdx + 1];
    }
    
    const filterIdx = args.indexOf('--filter');
    if (filterIdx !== -1 && args[filterIdx + 1]) {
        config.filter = args[filterIdx + 1];
    }
}

/**
 * 初始化输出流
 */
function initOutput() {
    if (config.output) {
        const dir = path.dirname(config.output);
        if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir, { recursive: true });
        }
        outputStream = fs.createWriteStream(config.output, { flags: 'a' });
        console.log(`📋 Logging to: ${config.output}`);
    } else {
        console.log('📋 Real-time log collector started');
    }
}

/**
 * 格式化日志条目
 */
function formatLog(logEntry) {
    if (!config.pretty) {
        return JSON.stringify(logEntry);
    }
    
    const { timestamp, level, module, message, args } = logEntry;
    const time = new Date(timestamp).toLocaleTimeString();
    
    const emoji = {
        error: '❌',
        warn: '⚠️ ',
        info: '✅',
        debug: '🔍',
        trace: '🔬'
    };
    
    const colors = {
        error: '\x1b[31m', // red
        warn: '\x1b[33m',  // yellow
        info: '\x1b[32m',  // green
        debug: '\x1b[36m', // cyan
        trace: '\x1b[35m'  // magenta
    };
    
    const reset = '\x1b[0m';
    const color = colors[level] || '';
    const icon = emoji[level] || '';
    
    let output = `${color}${icon} [${time}] [${module}] ${message}${reset}`;
    
    if (args && args.length > 0) {
        output += ` ${JSON.stringify(args)}`;
    }
    
    return output;
}

/**
 * 处理日志条目
 */
function processLog(logEntry) {
    // 过滤日志级别
    const entryLevel = LOG_LEVELS[logEntry.level] || 0;
    const filterLevel = LOG_LEVELS[config.filter] || 0;
    
    if (entryLevel < filterLevel) {
        return; // 跳过低级别日志
    }
    
    // 更新统计
    stats.total++;
    stats.byLevel[logEntry.level] = (stats.byLevel[logEntry.level] || 0) + 1;
    stats.byModule[logEntry.module] = (stats.byModule[logEntry.module] || 0) + 1;
    
    // 格式化输出
    const output = formatLog(logEntry);
    
    if (outputStream) {
        outputStream.write(output + '\n');
    } else {
        console.log(output);
    }
}

/**
 * 解析日志行
 */
function parseLine(line) {
    try {
        // 尝试解析 JSON
        const logEntry = JSON.parse(line);
        
        // 验证必需字段
        if (logEntry.timestamp && logEntry.level && logEntry.message) {
            processLog(logEntry);
        }
    } catch (e) {
        // 非 JSON 行，直接输出
        if (config.pretty && !config.output) {
            console.log(`📝 ${line}`);
        }
    }
}

/**
 * 显示统计信息
 */
function showStats() {
    console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('📊 Log Statistics');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log(`Total logs: ${stats.total}`);
    console.log('\nBy Level:');
    Object.entries(stats.byLevel).forEach(([level, count]) => {
        if (count > 0) {
            console.log(`  ${level}: ${count}`);
        }
    });
    console.log('\nTop Modules:');
    const topModules = Object.entries(stats.byModule)
        .sort((a, b) => b[1] - a[1])
        .slice(0, 10);
    topModules.forEach(([module, count]) => {
        console.log(`  ${module}: ${count}`);
    });
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
}

/**
 * 主函数
 */
function main() {
    parseArgs();
    initOutput();
    
    console.log(`🎯 Filter level: ${config.filter}`);
    console.log(`📋 Format: ${config.pretty ? 'pretty' : 'json'}`);
    console.log('');
    
    // 从标准输入读取
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
        terminal: false
    });
    
    rl.on('line', parseLine);
    
    rl.on('close', () => {
        if (outputStream) {
            outputStream.end();
        }
        showStats();
    });
    
    // Ctrl+C 处理
    process.on('SIGINT', () => {
        console.log('\n\n👋 Stopping log collector...');
        rl.close();
    });
}

// 运行
if (require.main === module) {
    main();
}

module.exports = { processLog, formatLog };
