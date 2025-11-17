/**
 * 🔄 Bridge集成示例
 * 
 * 展示如何在转换流程中使用新的Bridge系统
 */

// ============================================
// 示例1: 使用ValidationBridge进行转换前验证
// ============================================
async function examplePreConversionValidation() {
    const files = ['/path/to/image1.png', '/path/to/image2.jpg'];
    const config = {
        format: 'webp',
        quality: 80,
        speed: 4,
        lossless: false,
        mode: 'manual'
    };
    
    // 调用验证
    const validationResult = await window.PIXLY.validationBridge.validatePreConversion(files, config);
    
    if (!validationResult.valid) {
        console.error('验证失败:', validationResult.errors);
        // 显示错误给用户
        window.PIXLY.validationBridge.displayValidationErrors(validationResult);
        return false;
    }
    
    // 显示警告（如果有）
    if (validationResult.warnings.length > 0) {
        window.PIXLY.validationBridge.displayValidationWarnings(validationResult.warnings);
    }
    
    console.log('验证通过，可以开始转换');
    return true;
}

// ============================================
// 示例2: 使用AIBridge获取智能推荐
// ============================================
async function exampleSmartMode() {
    const imageInfo = {
        width: 1920,
        height: 1080,
        size: 2500000,
        format: 'jpeg',
        hasAlpha: false,
        isAnimated: false
    };
    
    // 获取智能配置
    const smartConfig = await window.PIXLY.aiBridge.getSmartConfig(imageInfo);
    
    console.log('AI推荐配置:', smartConfig);
    // {
    //   format: 'webp',
    //   quality: 80,
    //   speed: 4,
    //   lossless: false,
    //   confidence: 0.85,
    //   estimatedSize: 1200000,
    //   estimatedRatio: 0.48
    // }
    
    // 显示AI建议
    window.PIXLY.aiBridge.displayAISuggestion(smartConfig);
    
    // 应用到UI
    window.PIXLY.aiBridge.applyAISuggestionToUI(smartConfig);
    
    return smartConfig;
}

// ============================================
// 示例3: 完整的转换流程（带验证和AI）
// ============================================
async function exampleFullConversionFlow() {
    const files = ['/path/to/image.png'];
    
    // Step 1: 获取图像信息
    const imageInfo = await window.PIXLY.kernelBridge.getFileInfo(files[0]);
    
    // Step 2: 智能模式 - 获取AI推荐
    const smartConfig = await window.PIXLY.aiBridge.getSmartConfig(imageInfo);
    
    // Step 3: 验证转换配置
    const validationResult = await window.PIXLY.validationBridge.validatePreConversion(files, smartConfig);
    
    if (!validationResult.valid) {
        console.error('验证失败，取消转换');
        return;
    }
    
    // Step 4: 执行转换
    const conversionResult = await window.PIXLY.kernelBridge.convertImages(files, {
        format: smartConfig.format,
        quality: smartConfig.quality,
        speed: smartConfig.speed,
        lossless: smartConfig.lossless,
        preserveMetadata: true,
        keepAnimated: true,
        outputDir: '/output/path'
    });
    
    // Step 5: 验证输出
    if (conversionResult.success) {
        const outputValidation = await window.PIXLY.validationBridge.validatePostConversion(
            conversionResult.outputPath,
            imageInfo.size
        );
        
        if (outputValidation.valid) {
            console.log('转换成功并验证通过！');
        }
    }
}

// ============================================
// 示例4: 批量转换
// ============================================
async function exampleBatchConversion() {
    const files = [
        '/path/to/image1.png',
        '/path/to/image2.jpg',
        '/path/to/image3.gif'
    ];
    
    // 批量获取文件信息
    const fileInfos = await window.PIXLY.kernelBridge.batchGetFileInfo(files);
    
    // 为每个文件获取AI推荐
    const configs = [];
    for (const info of fileInfos) {
        const config = await window.PIXLY.aiBridge.predictParameters(
            info,
            'webp',  // 统一转换为webp
            'balanced'
        );
        configs.push(config);
    }
    
    // 批量验证
    const validationResults = [];
    for (let i = 0; i < files.length; i++) {
        const result = await window.PIXLY.validationBridge.validatePreConversion(
            [files[i]],
            configs[i]
        );
        validationResults.push(result);
    }
    
    // 只转换验证通过的文件
    const validFiles = files.filter((_, i) => validationResults[i].valid);
    
    if (validFiles.length > 0) {
        const result = await window.PIXLY.kernelBridge.convertImages(validFiles, {
            format: 'webp',
            quality: 80,
            speed: 4,
            lossless: false,
            outputDir: '/output/path'
        });
        
        console.log('批量转换完成:', result);
    }
}

// ============================================
// 示例5: 格式特定检查
// ============================================
async function exampleFormatSpecificChecks() {
    const files = ['/path/to/large.png'];
    const config = {
        format: 'webp',
        quality: 60,
        speed: 4,
        lossless: false
    };
    
    // 执行验证（会自动包含格式特定检查）
    const result = await window.PIXLY.validationBridge.validatePreConversion(files, config);
    
    // 警告示例:
    // - "⚠️ 文件 large.png 较大 (20.00 MB)，WebP编码可能较慢"
    // - "⚠️ WebP质量设置较低 (60), 可能出现明显压缩痕迹"
    
    if (result.warnings.length > 0) {
        console.log('格式特定警告:', result.warnings);
    }
}

// ============================================
// 示例6: 在现有代码中集成
// ============================================
async function integrateIntoExistingCode() {
    // 在 startConversion() 函数中添加:
    
    // 1. 转换前验证
    const files = window.selectedFiles.map(f => f.filePath);
    const config = getConversionConfig(); // 现有函数
    
    const validationResult = await window.PIXLY.validationBridge.validatePreConversion(files, config);
    
    if (!validationResult.valid) {
        // 显示错误并取消转换
        window.PIXLY.validationBridge.displayDetailedReport(validationResult);
        return;
    }
    
    // 2. 如果是智能模式，使用AI推荐
    if (config.mode === 'smart') {
        const imageInfo = await window.PIXLY.kernelBridge.getFileInfo(files[0]);
        const aiConfig = await window.PIXLY.aiBridge.getSmartConfig(imageInfo);
        
        // 应用AI推荐
        window.PIXLY.aiBridge.applyAISuggestionToUI(aiConfig);
        
        // 更新config
        Object.assign(config, aiConfig);
    }
    
    // 3. 执行转换（使用现有的rustCLI或新的kernelBridge）
    const result = await window.PIXLY.kernelBridge.convertImages(files, config);
    
    // 4. 转换后验证
    if (result.success) {
        await window.PIXLY.validationBridge.validatePostConversion(
            result.outputPath,
            window.selectedFiles[0].size
        );
    }
}

console.log('✅ Bridge集成示例已加载');
console.log('可用示例:');
console.log('  - examplePreConversionValidation()');
console.log('  - exampleSmartMode()');
console.log('  - exampleFullConversionFlow()');
console.log('  - exampleBatchConversion()');
console.log('  - exampleFormatSpecificChecks()');
console.log('  - integrateIntoExistingCode()');
