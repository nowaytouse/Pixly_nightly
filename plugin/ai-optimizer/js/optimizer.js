/**
 * Pixly AI Optimizer - Optimization Flow Controller
 * Handles the complete optimization workflow
 */

class PixlyOptimizer {
    constructor(aiClient) {
        this.aiClient = aiClient;
        this.files = [];
        this.currentMode = 'balanced';
        this.results = [];
    }

    /**
     * Add files to optimization queue
     */
    addFiles(fileList) {
        this.files = Array.from(fileList).map(file => ({
            path: file.path || file.name,
            name: file.name,
            size: file.size,
            type: this.detectMediaType(file.name)
        }));
        
        return this.files;
    }

    /**
     * Detect media type from filename
     */
    detectMediaType(filename) {
        const ext = filename.split('.').pop().toLowerCase();
        
        const imageFormats = ['jpg', 'jpeg', 'png', 'webp', 'avif', 'jxl', 'gif', 'bmp'];
        const videoFormats = ['mp4', 'mkv', 'avi', 'mov', 'webm', 'flv'];
        const audioFormats = ['mp3', 'aac', 'm4a', 'opus', 'ogg', 'flac', 'wav'];
        
        if (imageFormats.includes(ext)) return 'image';
        if (videoFormats.includes(ext)) return 'video';
        if (audioFormats.includes(ext)) return 'audio';
        return 'unknown';
    }

    /**
     * Set optimization mode
     */
    setMode(mode) {
        this.currentMode = mode;
    }

    /**
     * Analyze files with AI
     */
    async analyzeFiles() {
        const analyses = [];
        
        for (const file of this.files) {
            try {
                const prediction = await this.aiClient.predictAuto(
                    file.path,
                    this.currentMode
                );
                
                analyses.push({
                    file: file,
                    prediction: prediction,
                    status: 'analyzed'
                });
            } catch (error) {
                analyses.push({
                    file: file,
                    error: error.message,
                    status: 'error'
                });
            }
        }
        
        return analyses;
    }

    /**
     * Execute optimization
     */
    async optimize(onProgress) {
        this.results = [];
        let completed = 0;
        
        for (const file of this.files) {
            try {
                // Step 1: AI Analysis
                const prediction = await this.aiClient.predictAuto(
                    file.path,
                    this.currentMode
                );
                
                // Step 2: Build conversion parameters
                const params = this.buildConversionParams(file, prediction);
                
                // Step 3: Execute conversion (would call Rust CLI here)
                // For now, we simulate the conversion
                const result = await this.executeConversion(file, params);
                
                this.results.push({
                    file: file,
                    prediction: prediction,
                    result: result,
                    status: 'success'
                });
                
                completed++;
                if (onProgress) {
                    onProgress(completed, this.files.length);
                }
                
            } catch (error) {
                this.results.push({
                    file: file,
                    error: error.message,
                    status: 'error'
                });
                
                completed++;
                if (onProgress) {
                    onProgress(completed, this.files.length);
                }
            }
        }
        
        return this.results;
    }

    /**
     * Build conversion parameters from AI prediction
     */
    buildConversionParams(file, prediction) {
        const params = {
            input: file.path,
            mode: this.currentMode
        };
        
        // Image parameters
        if (file.type === 'image') {
            params.format = prediction.recommended_format || 'avif';
            params.quality = prediction.quality || 90;
            params.effort = prediction.effort || 7;
            params.lossless = prediction.lossless || false;
        }
        
        // Video parameters
        else if (file.type === 'video') {
            params.codec = prediction.recommended_codec || prediction.encoder || 'h265';
            params.crf = prediction.crf || 23;
            params.preset = prediction.preset || 'medium';
        }
        
        // Audio parameters
        else if (file.type === 'audio') {
            params.codec = prediction.recommended_codec || prediction.encoder || 'aac';
            params.bitrate = prediction.bitrate || 192;
        }
        
        return params;
    }

    /**
     * Execute conversion using Rust CLI
     */
    async executeConversion(file, params) {
        try {
            // 构建输出路径
            const outputDir = await this.ensureOutputDir();
            const inputPath = file.path;
            const ext = params.format || 'avif';
            const outputPath = `${outputDir}/${file.name.split('.')[0]}_optimized.${ext}`;
            
            // 构建Rust CLI命令
            const rustBin = await this.findRustBinary();
            const args = this.buildCliArgs(inputPath, outputPath, params);
            
            const startTime = Date.now();
            const originalSize = file.size || await this.getFileSize(inputPath);
            
            // 执行Rust转换器
            const result = await this.executeRustCommand(rustBin, args);
            
            const executionTime = Date.now() - startTime;
            const optimizedSize = await this.getFileSize(outputPath);
            const savings = Math.round((1 - optimizedSize / originalSize) * 100);
            
            return {
                originalSize,
                optimizedSize,
                savings,
                format: params.format,
                quality: params.quality,
                executionTime,
                outputPath,
                success: true
            };
            
        } catch (error) {
            console.error('Conversion failed:', error);
            return {
                originalSize: file.size || 0,
                optimizedSize: 0,
                savings: 0,
                format: params.format,
                error: error.message,
                success: false
            };
        }
    }

    /**
     * Get optimization statistics
     */
    getStats() {
        const successful = this.results.filter(r => r.status === 'success');
        const failed = this.results.filter(r => r.status === 'error');
        
        const totalOriginalSize = successful.reduce((sum, r) => 
            sum + (r.result.originalSize || 0), 0
        );
        
        const totalOptimizedSize = successful.reduce((sum, r) => 
            sum + (r.result.optimizedSize || 0), 0
        );
        
        const savedSize = totalOriginalSize - totalOptimizedSize;
        const savedRatio = totalOriginalSize > 0 
            ? (savedSize / totalOriginalSize) * 100 
            : 0;
        
        return {
            total: this.results.length,
            successful: successful.length,
            failed: failed.length,
            originalSize: totalOriginalSize,
            optimizedSize: totalOptimizedSize,
            savedSize: savedSize,
            savedRatio: savedRatio.toFixed(1)
        };
    }

    /**
     * Format file size
     */
    static formatSize(bytes) {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return (bytes / Math.pow(k, i)).toFixed(2) + ' ' + sizes[i];
    }

    /**
     * Format duration
     */
    static formatDuration(seconds) {
        if (seconds < 60) return `${seconds.toFixed(1)}s`;
        const minutes = Math.floor(seconds / 60);
        const secs = Math.floor(seconds % 60);
        return `${minutes}m ${secs}s`;
    }
    
    /**
     * Find Rust binary path
     */
    async findRustBinary() {
        // Eagle插件中Rust二进制文件的可能位置
        const possiblePaths = [
            '../../../core/rust/target/release/pixly-rust',
            '../../core/rust/target/release/pixly-rust', 
            './bin/pixly-rust',  // 插件内置binary
            'pixly-rust'  // 系统PATH
        ];
        
        for (const path of possiblePaths) {
            try {
                if (await this.fileExists(path)) {
                    return path;
                }
            } catch (e) {
                continue;
            }
        }
        
        throw new Error('Rust binary (pixly-rust) not found. Please ensure it is built and accessible.');
    }
    
    /**
     * Build CLI arguments for Rust converter
     */
    buildCliArgs(inputPath, outputPath, params) {
        const args = ['convert', inputPath, outputPath];
        
        if (params.format) args.push('--format', params.format);
        if (params.quality !== undefined) args.push('--quality', params.quality.toString());
        if (params.effort !== undefined) args.push('--effort', params.effort.toString());
        if (params.codec) args.push('--codec', params.codec);
        if (params.crf !== undefined) args.push('--crf', params.crf.toString());
        if (params.preset) args.push('--preset', params.preset);
        if (params.bitrate !== undefined) args.push('--bitrate', params.bitrate.toString());
        
        return args;
    }
    
    /**
     * Execute Rust command (Eagle环境适配)
     */
    async executeRustCommand(binaryPath, args) {
        return new Promise((resolve, reject) => {
            const command = `"${binaryPath}" ${args.map(arg => `"${arg}"`).join(' ')}`;
            
            // 使用Eagle的系统调用API（如果可用）
            if (typeof window !== 'undefined' && window.eagle && window.eagle.system) {
                window.eagle.system.exec(command)
                    .then(result => {
                        if (result.code === 0) {
                            resolve(result);
                        } else {
                            reject(new Error(`Conversion failed with code ${result.code}: ${result.stderr}`));
                        }
                    })
                    .catch(reject);
            } else {
                // 🔥 Phase 47.18: 删除fallback模拟，响亮报错
                reject(new Error('❌ 转换器不可用！请确保Rust CLI已正确安装'));
            }
        });
    }
    
    /**
     * 🔥 Phase 47.19 (O-004): 批量转换支持
     */
    async executeBatchConversion(files, outputDir, format, params) {
        const binaryPath = await this.findRustBinary();
        const args = ['batch'];
        
        // 添加文件列表
        files.forEach(file => args.push(file));
        
        // 输出目录和格式
        args.push(outputDir, format);
        
        // 转换参数
        if (params.quality !== undefined) args.push('--quality', params.quality.toString());
        if (params.effort !== undefined) args.push('--effort', params.effort.toString());
        if (params.lossless) args.push('--lossless');
        if (params.preserve_metadata) args.push('--preserve-metadata');
        if (params.keep_animated) args.push('--keep-animated');
        
        // 使用并行处理（Phase 47.19优化）
        args.push('--parallel');
        
        return this.executeRustCommand(binaryPath, args);
    }
    
    /**
     * 批量优化入口
     */
    async optimizeBatch(selectedFiles) {
        const startTime = Date.now();
        const outputDir = await this.ensureOutputDir();
        
        try {
            // 批量AI分析
            const analysisResults = await this.analyzeBatch(selectedFiles);
            
            // 根据分析结果确定最佳格式和参数
            const commonFormat = this.determineCommonFormat(analysisResults);
            const averageParams = this.calculateAverageParams(analysisResults);
            
            // 执行批量转换
            const result = await this.executeBatchConversion(
                selectedFiles,
                outputDir,
                commonFormat,
                averageParams
            );
            
            const elapsed = (Date.now() - startTime) / 1000;
            
            return {
                success: true,
                filesProcessed: selectedFiles.length,
                format: commonFormat,
                params: averageParams,
                time: elapsed,
                output: result
            };
            
        } catch (error) {
            console.error('Batch optimization failed:', error);
            throw error;
        }
    }
    
    /**
     * 批量AI分析
     */
    async analyzeBatch(files) {
        const results = [];
        for (const file of files) {
            const result = await this.analyzeImage(file);
            results.push(result);
        }
        return results;
    }
    
    /**
     * 确定批量转换的通用格式
     */
    determineCommonFormat(analysisResults) {
        const formatCounts = {};
        analysisResults.forEach(result => {
            const format = result.recommended_format || 'avif';
            formatCounts[format] = (formatCounts[format] || 0) + 1;
        });
        
        // 返回最常见的格式
        return Object.keys(formatCounts).reduce((a, b) => 
            formatCounts[a] > formatCounts[b] ? a : b
        );
    }
    
    /**
     * 计算平均参数
     */
    calculateAverageParams(analysisResults) {
        const params = {
            quality: 0,
            effort: 0,
            count: 0
        };
        
        analysisResults.forEach(result => {
            if (result.quality) {
                params.quality += result.quality;
                params.count++;
            }
            if (result.effort) {
                params.effort += result.effort;
            }
        });
        
        return {
            quality: Math.round(params.quality / params.count),
            effort: Math.round(params.effort / params.count),
            preserve_metadata: true,
            keep_animated: true
        };
    }
    
    /**
     * Utility functions
     */
    async ensureOutputDir() {
        const outputDir = './optimized_output';
        if (window.eagle && window.eagle.fs) {
            await window.eagle.fs.mkdir(outputDir, { recursive: true });
        }
        return outputDir;
    }
    
    async fileExists(path) {
        if (window.eagle && window.eagle.fs) {
            try {
                await window.eagle.fs.stat(path);
                return true;
            } catch {
                return false;
            }
        }
        return false; // 开发环境默认
    }
    
    async getFileSize(path) {
        if (window.eagle && window.eagle.fs) {
            try {
                const stats = await window.eagle.fs.stat(path);
                return stats.size;
            } catch {
                return 0;
            }
        }
        return 1024 * 1024; // 开发环境默认1MB
    }
    
    /**
     * 错误分类
     */
    categorizeError(error) {
        const msg = error.message.toLowerCase();
        
        if (msg.includes('not found') || msg.includes('no such file')) {
            return 'file_not_found';
        } else if (msg.includes('permission') || msg.includes('access denied')) {
            return 'permission_denied';
        } else if (msg.includes('timeout') || msg.includes('time out')) {
            return 'timeout';
        } else if (msg.includes('jxl') && msg.includes('failed')) {
            return 'jxl_encoding_error';
        } else if (msg.includes('ai') || msg.includes('predict')) {
            return 'ai_service_error';
        } else if (msg.includes('format') || msg.includes('codec')) {
            return 'format_not_supported';
        } else {
            return 'unknown_error';
        }
    }
    
    /**
     * 获取错误解决建议
     */
    getSuggestion(errorType) {
        const suggestions = {
            'file_not_found': '检查文件路径是否正确，文件是否存在',
            'permission_denied': '检查文件权限，确保有读写权限',
            'timeout': '文件可能过大，尝试减少质量设置或使用较小的文件',
            'jxl_encoding_error': '该JPEG文件可能有兼容性问题，尝试使用其他格式如AVIF',
            'ai_service_error': 'AI服务不可用，请确保Python AI服务正在运行',
            'format_not_supported': '不支持该格式转换，请选择其他目标格式',
            'unknown_error': '未知错误，请查看详细错误信息'
        };
        
        return suggestions[errorType] || suggestions['unknown_error'];
    }
}

// Export for use in other scripts
if (typeof module !== 'undefined' && module.exports) {
    module.exports = PixlyOptimizer;
}
