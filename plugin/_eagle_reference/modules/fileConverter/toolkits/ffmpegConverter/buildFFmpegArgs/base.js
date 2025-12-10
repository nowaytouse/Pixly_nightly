const path = require('path');
const os = require('os');
const { execSync } = require('child_process');

/**
 * FFmpeg 命令構建器基礎類
 * 負責構建 FFmpeg 轉換參數，包含濾鏡處理、品質設定等核心功能
 */
class BaseBuilder {
    constructor() {
        this.args = [];
        this._initializeConstants();
    }

    /**
     * 初始化常量配置
     * @private
     */
    _initializeConstants() {
        // 支援透明背景的輸出格式白名單
        this.transparentFormats = [
            'png',
            'webp',
            'gif',
            'ico',
            'tga',
            'exr',
            'jxl',
            'dds',
            'tiff',
            'tif',
            'webm'
        ];

        // 可能含有透明背景的輸入格式
        this.transparentInputFormats = [
            '.png', '.svg', '.webp', '.gif', '.ico', '.tga', '.exr', '.jxl', '.dds', '.tiff', '.tif'
        ];

        // 濾鏡模板
        this.filterTemplates = {
            // 白色背景覆蓋濾鏡
            whiteBackground: 'split[orig][bg];[bg]drawbox=color=white@1.0:replace=1:t=fill[bgwhite];[bgwhite][orig]overlay'
        };
    }

    buildArgs(src, dest, options) {
        this.args = [
            ...this.getQualityArgs(options),
            ...this.getVideoFilterArgs(src, options),
            ...this.getFramesArgs(options),
            this.getOutputPath(dest, options),
        ];
        return this.args.filter(arg => arg);
    }

    /**
     * 如果輸出不是動畫格式的話，則需要添加 -frames:v 1 參數
     */
    getFramesArgs(options) {
        const format = options.format.toLowerCase();

        // 定義幀數策略
        const frameStrategies = {
            // 動畫格式：不限制幀數
            animated: {
                formats: ['webp', 'gif', 'png', 'mp4', 'webm'],
                getFrames: () => []
            },
            // 靜態格式：限制為單幀
            static: {
                formats: ['jpg', 'jpeg', 'bmp', 'tiff', 'tif', 'ico', 'jxl', 'avif'],
                getFrames: () => ['-frames:v', '1']
            }
        };

        // 根據格式選擇策略
        for (const strategy of Object.values(frameStrategies)) {
            if (strategy.formats.includes(format)) {
                return strategy.getFrames();
            }
        }

        // 預設策略：對未知格式限制為單幀
        return ['-frames:v', '1'];
    }

    /**
     * 檢查格式是否支援透明
     */
    isTransparentFormat(format) {
        return this.transparentFormats.includes(format.toLowerCase());
    }

    /**
     * 獲取視頻濾鏡參數（包含背景和尺寸）
     * @param {string} src - 源文件路徑
     * @param {Object} options - 轉換選項
     * @returns {Array} FFmpeg 濾鏡參數陣列
     */
    getVideoFilterArgs(src, options) {
        const filters = this._buildFilterChain(src, options);

        return filters.length > 0 ? ['-vf', filters.join(',')] : [];
    }

    /**
     * 構建濾鏡鏈
     * @param {string} src - 源文件路徑  
     * @param {Object} options - 轉換選項
     * @returns {Array} 濾鏡字串陣列
     * @private
     */
    _buildFilterChain(src, options) {
        const filters = [];

        // 添加背景處理濾鏡
        const backgroundFilter = this._getBackgroundFilter(src, options);
        if (backgroundFilter) {
            filters.push(backgroundFilter);
        }

        // 添加尺寸調整濾鏡
        const sizeFilter = this.getSizeFilter(options);
        if (sizeFilter) {
            filters.push(sizeFilter);
        }

        return filters;
    }

    /**
     * 獲取背景處理濾鏡
     * @param {string} src - 源文件路徑
     * @param {Object} options - 轉換選項 
     * @returns {string|null} 背景濾鏡字串，若無需處理則回傳 null
     * @private
     */
    _getBackgroundFilter(src, options) {
        const srcExt = path.extname(src).toLowerCase();

        // 只有當來源可能有透明背景，且輸出格式不支援透明時，才添加白色背景
        if (this._needsBackgroundProcessing(srcExt, options.format)) {
            return this.filterTemplates.whiteBackground;
        }

        return null;
    }

    /**
     * 判斷是否需要背景處理
     * @param {string} srcExt - 源文件副檔名
     * @param {string} outputFormat - 輸出格式
     * @returns {boolean} 是否需要背景處理
     * @private
     */
    _needsBackgroundProcessing(srcExt, outputFormat) {
        // 只有當輸出格式不支援透明背景時，才需要背景處理
        // 例如：JPG、BMP 等格式不支援透明背景        
        return !this.isTransparentFormat(outputFormat) && this._hasTransparentInput(srcExt);
    }

    /**
     * 判斷輸入是否可能包含透明背景
     * @param {string} srcExt - 源文件副檔名
     * @returns {boolean} 是否可能包含透明背景
     * @private
     */
    _hasTransparentInput(srcExt) {
        return this.transparentInputFormats.includes(srcExt);
    }

    /**
     * 獲取尺寸調整濾鏡（僅返回濾鏡字串，不包含 -vf）
     * @param {Object} options - 轉換選項
     * @param {string} options.sizeType - 尺寸調整類型
     * @param {number} options.sizeValue - 尺寸數值
     * @param {number} [options.width] - 明確寬度（exact 類型使用）
     * @param {number} [options.height] - 明確高度（exact 類型使用）
     * @returns {string|null} 尺寸調整濾鏡字串，若無需調整則回傳 null
     */
    getSizeFilter(options) {
        if (!options.sizeType) {
            return null;
        }

        const sizeHandlers = {
            maxWidth: () => this._buildScaleFilter(`min'(iw,${options.sizeValue})'`, '-1'),
            maxHeight: () => this._buildScaleFilter('-1', `min'(ih,${options.sizeValue})'`),
            minWidth: () => this._buildScaleFilter(`max'(iw,${options.sizeValue})'`, '-1'),
            minHeight: () => this._buildScaleFilter('-1', `max'(ih,${options.sizeValue})'`),
            maxSide: () => this._buildMaxSideScaleFilter(options.sizeValue),
            minSide: () => this._buildMinSideScaleFilter(options.sizeValue),
            exact: () => this._buildExactScaleFilter(options)
        };

        const handler = sizeHandlers[options.sizeType];
        return handler ? handler() : null;
    }

    /**
     * 建構基本縮放濾鏡
     * @param {string} width - 寬度表達式
     * @param {string} height - 高度表達式  
     * @returns {string} 縮放濾鏡字串
     * @private
     */
    _buildScaleFilter(width, height) {
        return `scale=${width}:${height}`;
    }

    /**
     * 建構最大邊縮放濾鏡
     * @param {number} maxSize - 最大尺寸
     * @returns {string} 縮放濾鏡字串
     * @private
     */
    _buildMaxSideScaleFilter(maxSize) {
        const widthExpr = `if(gt(iw,ih),${maxSize},-1)`;
        const heightExpr = `if(gt(iw,ih),-1,${maxSize})`;
        return `scale='${widthExpr}':'${heightExpr}'`;
    }

    /**
     * 建構最小邊縮放濾鏡
     * @param {number} minSize - 最小尺寸
     * @returns {string} 縮放濾鏡字串
     * @private
     */
    _buildMinSideScaleFilter(minSize) {
        const widthExpr = `if(gt(iw,ih),-1,${minSize})`;
        const heightExpr = `if(gt(iw,ih),${minSize},-1)`;
        return `scale='${widthExpr}':'${heightExpr}'`;
    }

    /**
     * 建構精確尺寸縮放濾鏡
     * @param {Object} options - 包含 width 和 height 的選項
     * @returns {string|null} 縮放濾鏡字串
     * @private
     */
    _buildExactScaleFilter(options) {
        if (options.width && options.height) {
            return this._buildScaleFilter(options.width, options.height);
        }
        if (options.width) {
            return this._buildScaleFilter(options.width, '-1');
        }
        if (options.height) {
            return this._buildScaleFilter('-1', options.height);
        }
        return null;
    }

    /**
     * 獲取尺寸調整參數（舊版 API，已棄用）
     * @deprecated 請使用 getSizeFilter() 並配合 getVideoFilterArgs() 以支持濾鏡鏈
     * @param {Object} options - 轉換選項
     * @returns {Array} 包含 -vf 參數的陣列，若無需調整則回傳空陣列
     */
    getSizeArgs(options) {
        const sizeFilter = this.getSizeFilter(options);
        return sizeFilter ? ['-vf', sizeFilter] : [];
    }

    /**
     * 獲取品質控制參數
     * @param {Object} options - 轉換選項
     * @param {number} [options.quality] - 品質百分比 (1-100)
     * @returns {Array|undefined} 品質參數陣列，若未設置品質則回傳 undefined
     */
    getQualityArgs(options) {
        if (!options.quality) {
            return undefined;
        }

        // 將百分比品質轉換為 FFmpeg 品質值 (2-31，數值越小品質越高)
        const ffmpegQuality = Math.round(31 - ((options.quality / 100) * 29));
        return ['-q:v', ffmpegQuality];
    }

    /**
     * 獲取輸出路徑
     * @param {string} dest - 目標目錄路徑
     * @param {Object} options - 轉換選項
     * @param {string} options.fileName - 檔案名稱（不含副檔名）
     * @param {string} options.format - 輸出格式
     * @returns {string} 完整的輸出文件路徑
     */
    getOutputPath(dest, options) {
        return path.join(dest, `${options.fileName}.${options.format}`);
    }

    /**
     * 獲取系統資訊和GPU類型
     * @returns {Object} 包含作業系統類型和GPU類型的物件
     */
    getSystemInfo() {
        const osType = os.type();
        let gpuType = null;

        if (osType === 'Windows_NT') {
            // 嘗試多種方法來偵測 GPU
            gpuType = this.detectWindowsGPU();
        }

        return { osType, gpuType };
    }

    /**
     * 偵測 Windows 系統的 GPU 類型
     * @returns {string|null} GPU 類型 ('NVIDIA', 'AMD', 'Intel') 或 null
     */
    detectWindowsGPU() {
        // 方法 1: 嘗試使用 PowerShell (更現代的方法)
        try {
            const gpuInfo = execSync('powershell -Command "Get-WmiObject Win32_VideoController | Select-Object -ExpandProperty Name"', {
                encoding: 'utf8',
                windowsHide: true
            }).toString();
            
            const gpuName = gpuInfo.trim().toUpperCase();
            if (gpuName) {
                return this.identifyGPUType(gpuName);
            }
        } catch (error) {
            console.log('PowerShell method failed, trying wmic...');
        }

        // 方法 2: 嘗試使用 wmic (舊版 Windows)
        try {
            const gpuInfo = execSync('wmic path win32_VideoController get name', {
                encoding: 'utf8',
                windowsHide: true
            }).toString();
            
            const gpuLines = gpuInfo
                .split('\n')
                .map((line) => line.trim())
                .filter((line) => line && line !== 'Name');
            
            if (gpuLines[0]) {
                return this.identifyGPUType(gpuLines[0].toUpperCase());
            }
        } catch (error) {
            console.log('wmic method failed');
        }

        // 如果兩種方法都失敗，回傳 null 使用 CPU 軟體編碼
        console.warn('Unable to detect GPU type, will use CPU software encoding');
        return null;
    }

    /**
     * 根據 GPU 名稱識別類型
     * @param {string} gpuName - GPU 名稱（已轉為大寫）
     * @returns {string|null} GPU 類型或 null
     */
    identifyGPUType(gpuName) {
        // NVIDIA GPU 識別
        if (gpuName.includes('NVIDIA') || 
            gpuName.includes('GEFORCE') || 
            gpuName.includes('RTX') || 
            gpuName.includes('GTX') ||
            gpuName.includes('QUADRO') ||
            gpuName.includes('TESLA')) {
            return 'NVIDIA';
        }
        
        // AMD GPU 識別
        if (gpuName.includes('AMD') || 
            gpuName.includes('RADEON') || 
            gpuName.includes('RX') ||
            gpuName.includes('VEGA') ||
            gpuName.includes('FIREPRO')) {
            return 'AMD';
        }
        
        // Intel GPU 識別
        if (gpuName.includes('INTEL') || 
            gpuName.includes('UHD') || 
            gpuName.includes('HD GRAPHICS') || 
            gpuName.includes('IRIS') ||
            gpuName.includes('ARC')) {
            return 'Intel';
        }

        // 無法識別
        console.log(`Unable to identify GPU type from: ${gpuName}`);
        return null;
    }

    /**
     * 通用的編解碼器參數獲取方法
     * @param {Object} options - 轉換選項
     * @param {Function} getGPUArgsCallback - 子類別提供的GPU參數獲取回調函數
     * @returns {Array} 編解碼器參數陣列
     */
    getCodecArgs(options, getGPUArgsCallback) {
        const { osType, gpuType } = this.getSystemInfo();

        console.log(osType, gpuType);

        if (osType === 'Darwin') {
            return getGPUArgsCallback(options, osType, gpuType);
        }

        if (osType === 'Windows_NT') {
            return getGPUArgsCallback(options, osType, gpuType);
        }

        // 其他作業系統預設使用軟體編解碼器
        return getGPUArgsCallback(options, osType, null);
    }
}

module.exports = BaseBuilder;