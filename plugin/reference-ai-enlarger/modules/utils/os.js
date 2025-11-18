'use strict';

module.exports = class {
    /**
     * 檢查是否有GPU
     * @returns {boolean} 是否有GPU
     * @throws {Error} 錯誤
     * @example
     */
    
    static isIntegratedGPU = () => {
        try {
            const gpu = this.getGPU();

            if (!gpu) {
                throw 'WebGL not supported';
            }

            eagle.log.info('GPU device check : ' + gpu);

            // 常見內建顯卡關鍵詞列表
            const integratedKeywords = [
                "Intel",
                "Radeon Vega", // AMD 內建顯卡
                "Radeon(TM) Graphics", // AMD 內建顯卡的新命名方式
                "APU",
                "SwiftShader" // 軟件渲染器
            ];
        
            // 常見獨立顯卡關鍵詞列表
            const dedicatedKeywords = [
                "NVIDIA",
                "GeForce",
                "Quadro",
                "RTX",
                "GTX",
                "Arc",
                "Apple M",
                "Radeon RX", // AMD 獨立顯卡
                "Radeon Pro" // AMD 高端專業繪圖卡
            ];
        
            const includesIntegrated = integratedKeywords.some(keyword => gpu?.toLowerCase().includes(keyword.toLowerCase()));
            const includesDedicated = dedicatedKeywords.some(keyword => gpu?.toLowerCase().includes(keyword.toLowerCase()));
            const isIntegrated = (includesIntegrated && !includesDedicated);

            return isIntegrated;
        }
        catch (error) {
            console.error(error);
            return false;
        }
    };
    static getGPU = () => {
        const canvas = document.createElement('canvas');

        const gl = canvas.getContext('webgl') || canvas.getContext('experimental-webgl');

        if (gl) {
            const debugInfo = gl.getExtension('WEBGL_debug_renderer_info');

            const vendor = gl.getParameter(debugInfo.UNMASKED_VENDOR_WEBGL);
            const renderer = gl.getParameter(debugInfo.UNMASKED_RENDERER_WEBGL);

            return renderer;
        } else {
            throw 'WebGL not supported';
        }
    };
};
