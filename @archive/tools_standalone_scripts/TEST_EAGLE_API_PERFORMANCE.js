#!/usr/bin/env node
/**
 * Eagle API 性能测试
 * 测试不同文件获取方法的性能和可靠性
 */

const fs = require('fs');
const path = require('path');

const LIBRARY_PATH = '/Users/nyamiiko/Documents/git/plxy-easy2jxlavif/1.library';

console.log('🔬 Eagle API 性能测试\n');
console.log('=' .repeat(60));

// ==================== 方法1: 直接读取metadata.json ====================
console.log('\n📊 方法1: 直接扫描.info目录 + 读取metadata.json');
const t1Start = Date.now();

const imagesDir = path.join(LIBRARY_PATH, 'images');
const infoDirs = fs.readdirSync(imagesDir).filter(name => name.endsWith('.info'));

let files1 = [];
let readCount = 0;
let errorCount = 0;

for (const infoDir of infoDirs) {
    const metadataPath = path.join(imagesDir, infoDir, 'metadata.json');
    
    try {
        if (fs.existsSync(metadataPath)) {
            const metadata = JSON.parse(fs.readFileSync(metadataPath, 'utf8'));
            
            // 构建文件路径
            const filePath = path.join(imagesDir, infoDir, `${metadata.name}.${metadata.ext}`);
            
            files1.push({
                id: metadata.id,
                name: metadata.name,
                ext: metadata.ext,
                size: metadata.size,
                width: metadata.width,
                height: metadata.height,
                filePath: filePath,
                exists: fs.existsSync(filePath)
            });
            
            readCount++;
        }
    } catch (error) {
        errorCount++;
    }
}

const t1End = Date.now();
const t1Time = t1End - t1Start;

console.log(`   ✅ 扫描 ${infoDirs.length} 个.info目录`);
console.log(`   ✅ 成功读取 ${readCount} 个metadata.json`);
console.log(`   ❌ 失败 ${errorCount} 个`);
console.log(`   📁 找到 ${files1.length} 个文件`);
console.log(`   ⏱️  耗时: ${t1Time}ms`);

// 文件存在性统计
const existsCount = files1.filter(f => f.exists).length;
const notExistsCount = files1.filter(f => !f.exists).length;
console.log(`   📊 文件存在: ${existsCount} / 不存在: ${notExistsCount}`);

// ==================== 方法2: 读取metadata.json但不验证文件存在性 ====================
console.log('\n📊 方法2: 直接扫描 (不验证文件存在性)');
const t2Start = Date.now();

let files2 = [];

for (const infoDir of infoDirs) {
    const metadataPath = path.join(imagesDir, infoDir, 'metadata.json');
    
    try {
        if (fs.existsSync(metadataPath)) {
            const metadata = JSON.parse(fs.readFileSync(metadataPath, 'utf8'));
            
            // 不验证文件存在性，直接构建路径
            const filePath = path.join(imagesDir, infoDir, `${metadata.name}.${metadata.ext}`);
            
            files2.push({
                id: metadata.id,
                name: metadata.name,
                ext: metadata.ext,
                size: metadata.size,
                filePath: filePath
            });
        }
    } catch (error) {
        // 忽略
    }
}

const t2End = Date.now();
const t2Time = t2End - t2Start;

console.log(`   📁 找到 ${files2.length} 个文件`);
console.log(`   ⏱️  耗时: ${t2Time}ms`);
console.log(`   🚀 比方法1快: ${((1 - t2Time/t1Time) * 100).toFixed(1)}%`);

// ==================== 方法3: 只读取metadata.json名称，不解析JSON ====================
console.log('\n📊 方法3: 最小化读取 (只扫描目录)');
const t3Start = Date.now();

let files3 = [];

for (const infoDir of infoDirs) {
    const infoPath = path.join(imagesDir, infoDir);
    const id = infoDir.replace('.info', '');
    
    files3.push({
        id: id,
        infoPath: infoPath
    });
}

const t3End = Date.now();
const t3Time = t3End - t3Start;

console.log(`   📁 找到 ${files3.length} 个.info目录`);
console.log(`   ⏱️  耗时: ${t3Time}ms`);
console.log(`   🚀 比方法1快: ${((1 - t3Time/t1Time) * 100).toFixed(1)}%`);

// ==================== 性能对比总结 ====================
console.log('\n' + '='.repeat(60));
console.log('📊 性能对比总结:\n');
console.log(`   方法1 (完整验证):      ${t1Time}ms (基准)`);
console.log(`   方法2 (无验证):        ${t2Time}ms (快 ${((1 - t2Time/t1Time) * 100).toFixed(1)}%)`);
console.log(`   方法3 (最小化):        ${t3Time}ms (快 ${((1 - t3Time/t1Time) * 100).toFixed(1)}%)`);

// ==================== Eagle API 可靠性分析 ====================
console.log('\n' + '='.repeat(60));
console.log('💡 Eagle API 最佳实践建议:\n');
console.log('   1. 🚀 使用 eagle.item.getSelected() 获取选中文件');
console.log('      - 优点: Eagle已经处理好了选中状态');
console.log('      - 缺点: 可能较慢，需要通过IPC通信');
console.log('      - 建议: 直接使用，不做额外验证\n');

console.log('   2. ⚡ 延迟验证策略');
console.log('      - 在文件选择时: 不验证文件存在性');
console.log('      - 在转换开始时: 才验证和解析完整路径');
console.log('      - 好处: 文件选择响应快速\n');

console.log('   3. 📦 Eagle数据结构');
console.log('      - metadata.json 包含所有必要信息');
console.log('      - 文件路径: {library}/images/{id}.info/{name}.{ext}');
console.log('      - 缩略图: {library}/images/{id}.info/{name}_thumbnail.png\n');

console.log('   4. 🎯 优化建议');
console.log('      - 减少 fs.existsSync 调用');
console.log('      - 减少 fs.statSync 调用');
console.log('      - 批量操作比逐个操作快\n');

// ==================== 格式统计 ====================
console.log('='.repeat(60));
console.log('📊 文件格式统计:\n');

const formatCounts = {};
files1.forEach(f => {
    const ext = f.ext.toUpperCase();
    formatCounts[ext] = (formatCounts[ext] || 0) + 1;
});

Object.entries(formatCounts)
    .sort((a, b) => b[1] - a[1])
    .forEach(([format, count]) => {
        const percentage = ((count / files1.length) * 100).toFixed(1);
        console.log(`   ${format.padEnd(8)}: ${count.toString().padStart(3)} (${percentage}%)`);
    });

console.log('\n' + '='.repeat(60));
console.log('✅ 测试完成\n');
