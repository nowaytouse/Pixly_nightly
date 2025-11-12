/**
 * PIXLY 质量滑块处理module
 * 依赖: PIXLY命名空间
 */
(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    /**
     * 质量滑块管理器
     */
    const QualitySlider = {
        
        slider: null,
        valueDisplay: null,
        
        /**
         * initialized质量滑块
         */
        init: function() {
            this.slider = document.getElementById('quality');
            this.valueDisplay = document.getElementById('qualityValue');
            
            if (!this.slider || !this.valueDisplay) {
                const log = window.pixlyLog;
                if (log) {
                    log.warn('PIXLY Quality', formatLog(LOG.QUALITY_SLIDER_NOT_FOUND, {}));
                }
                return false;
            }
            
            this.setupEventListeners();
            this.updateDisplay();
            
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY Quality', formatLog(LOG.QUALITY_SLIDER_INITIALIZED, {}));
            }
            return true;
        },
        
        /**
         * settings事件监听器
         */
        setupEventListeners: function() {
            if (!this.slider) return;
            
            // 实时更New显示
            this.slider.addEventListener('input', () => {
                this.updateDisplay();
            });
            
            // saveconfig
            this.slider.addEventListener('change', () => {
                this.saveQuality();
            });
        },
        
        /**
         * 更New显示值
         */
        updateDisplay: function() {
            if (!this.slider || !this.valueDisplay) return;
            
            const value = parseInt(this.slider.value);
            this.valueDisplay.textContent = value;
            
            // 根据质量值改变颜色
            const color = this.getQualityColor(value);
            this.valueDisplay.style.color = color;
            
            // 更New滑块样式
            this.updateSliderStyle(value);
        },
        
        /**
         * 根据质量值获取颜色
         */
        getQualityColor: function(quality) {
            if (quality >= 90) return '#4caf50'; // 绿色 - 高质量
            if (quality >= 75) return '#2196f3'; // 蓝色 - 好质量
            if (quality >= 60) return '#ff9800'; // 橙色 - 中等质量
            return '#f44336'; // 红色 - 低质量
        },
        
        /**
         * 更New滑块渐变样式
         */
        updateSliderStyle: function(value) {
            if (!this.slider) return;
            
            const percent = ((value - this.slider.min) / (this.slider.max - this.slider.min)) * 100;
            const color = this.getQualityColor(value);
            
            this.slider.style.background = `linear-gradient(to right, ${color} 0%, ${color} ${percent}%, #ddd ${percent}%, #ddd 100%)`;
        },
        
        /**
         * save质量settings
         */
        saveQuality: function() {
            if (!this.slider) return;
            
            const quality = parseInt(this.slider.value);
            
            if (PIXLY.setConfig) {
                PIXLY.setConfig('defaultQuality', quality);
            }
            
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY Quality', formatLog(LOG.QUALITY_SLIDER_SAVED, { quality }));
            }
        },
        
        /**
         * 获取当前质量值
         */
        getValue: function() {
            return this.slider ? parseInt(this.slider.value) : 85;
        },
        
        /**
         * settings质量值
         */
        setValue: function(value) {
            if (!this.slider) return;
            
            this.slider.value = value;
            this.updateDisplay();
        },
        
        /**
         * 根据Optimize模式settings推荐质量
         */
        setRecommendedForMode: function(mode) {
            const recommendations = {
                'size': 75,      // 体积优先
                'balanced': 85,  // 平衡
                'quality': 95,   // 质量优先
                'universal': 100 // 通用（无损）
            };
            
            const quality = recommendations[mode] || 85;
            this.setValue(quality);
            
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY Quality', formatLog(LOG.QUALITY_SLIDER_RECOMMENDED, { mode, quality }));
            }
        },
        
        /**
         * 禁用/启用滑块
         */
        setEnabled: function(enabled) {
            if (!this.slider) return;
            
            this.slider.disabled = !enabled;
            
            if (!enabled) {
                this.slider.style.opacity = '0.5';
                this.slider.style.cursor = 'not-allowed';
            } else {
                this.slider.style.opacity = '1';
                this.slider.style.cursor = 'pointer';
            }
        },
        
        /**
         * 获取质量描述
         */
        getQualityDescription: function(quality) {
            if (quality >= 95) return '极高质量 - 几乎无损';
            if (quality >= 85) return '高质量 - 推荐';
            if (quality >= 75) return '良好质量 - 较小体积';
            if (quality >= 60) return '中等质量 - 明显压缩';
            return '低质量 - 大量压缩';
        }
    };
    
    // 暴露to全局
    PIXLY.QualitySlider = QualitySlider;
    
    // 向后兼容
    window.initQualitySlider = function() {
        QualitySlider.init();
    };
    
    const log = window.pixlyLog;
    if (log) {
        log.info('PIXLY Quality', formatLog(LOG.QUALITY_SLIDER_MODULE_LOADED, {}));
    }
    
})(window);
