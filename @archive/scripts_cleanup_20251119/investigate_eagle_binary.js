const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('🔍 深入调查Eagle使用的Binary');
console.log('=====================================\n');

const binaryPath = path.join(__dirname, 'pixly-rust/target/release/pixly-rust');

// 1. 检查binary文件信息
console.log('1️⃣ Binary文件信息:');
const stats = fs.statSync(binaryPath);
console.log(`   路径: ${binaryPath}`);
console.log(`   大小: ${(stats.size / 1024 / 1024).toFixed(2)} MB`);
console.log(`   修改时间: ${stats.mtime}`);
console.log('');

// 2. 检查binary版本
console.log('2️⃣ Binary版本:');
try {
    const version = execSync(`"${binaryPath}" --version`, { encoding: 'utf8' });
    console.log(`   ${version.trim()}`);
} catch (e) {
    console.log(`   ❌ 获取版本失败: ${e.message}`);
}
console.log('');

// 3. 测试转换（捕获所有输出）
console.log('3️⃣ 测试转换（完整输出）:');
const testInput = '/Users/nyamiiko/Documents/git/@test/test.library/images/MHISY2WJBCNJY.info/1a2bc2f1fc9484535dd149147d25afd0.gif';
const testOutput = '/tmp/test_investigation.jxl';

try {
    const result = execSync(
        `"${binaryPath}" convert "${testInput}" "${testOutput}" --format jxl --quality 90 --effort 7 2>&1`,
        { encoding: 'utf8', maxBuffer: 10 * 1024 * 1024 }
    );
    
    console.log('   输出内容:');
    result.split('\n').forEach(line => {
        if (line.trim()) console.log(`   | ${line}`);
    });
    
    // 检查关键日志是否存在
    console.log('\n   关键日志检查:');
    console.log(`   ${result.includes('📋 Registering conversion strategies') ? '✅' : '❌'} 包含"Registering conversion strategies"`);
    console.log(`   ${result.includes('CLI JXL (cjxl) - Available') ? '✅' : '❌'} 包含"CLI JXL available"`);
    console.log(`   ${result.includes('✅ Conversion successful') ? '✅' : '❌'} 转换成功`);
    
} catch (error) {
    console.log(`   ❌ 转换失败:`);
    console.log(`   Exit code: ${error.status}`);
    console.log(`   输出:\n${error.stdout}`);
    console.log(`   错误:\n${error.stderr}`);
}

console.log('\n=====================================');
console.log('🎯 调查完成');
