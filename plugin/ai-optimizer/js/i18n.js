/**
 * Pixly AI Optimizer - Internationalization
 * Simple i18n helper
 */

const I18n = {
    currentLocale: 'en',
    translations: {},
    
    /**
     * Initialize i18n with locale
     */
    async init(locale = 'en') {
        this.currentLocale = locale;
        await this.loadTranslations(locale);
    },
    
    /**
     * Load translations for locale
     */
    async loadTranslations(locale) {
        try {
            const response = await fetch(`_locales/${locale}.json`);
            if (response.ok) {
                this.translations = await response.json();
                return true;
            }
        } catch (error) {
            console.warn(`Failed to load translations for ${locale}:`, error);
        }
        return false;
    },
    
    /**
     * Get translated text by key
     */
    t(key, defaultText = '') {
        const keys = key.split('.');
        let value = this.translations;
        
        for (const k of keys) {
            if (value && typeof value === 'object') {
                value = value[k];
            } else {
                return defaultText || key;
            }
        }
        
        return value || defaultText || key;
    },
    
    /**
     * Apply translations to DOM
     */
    applyToDom() {
        document.querySelectorAll('[data-i18n]').forEach(element => {
            const key = element.getAttribute('data-i18n');
            const text = this.t(key);
            if (text) {
                element.textContent = text;
            }
        });
        
        // Handle placeholders
        document.querySelectorAll('[data-i18n-placeholder]').forEach(element => {
            const key = element.getAttribute('data-i18n-placeholder');
            const text = this.t(key);
            if (text) {
                element.placeholder = text;
            }
        });
        
        // Handle titles
        document.querySelectorAll('[data-i18n-title]').forEach(element => {
            const key = element.getAttribute('data-i18n-title');
            const text = this.t(key);
            if (text) {
                element.title = text;
            }
        });
    },
    
    /**
     * Detect user locale
     */
    detectLocale() {
        // Try to get from browser
        const browserLang = navigator.language || navigator.userLanguage;
        
        // Map to supported locales
        if (browserLang.startsWith('zh-CN') || browserLang.startsWith('zh-Hans')) {
            return 'zh_CN';
        } else if (browserLang.startsWith('zh-TW') || browserLang.startsWith('zh-Hant')) {
            return 'zh_TW';
        } else if (browserLang.startsWith('ja')) {
            return 'ja_JP';
        }
        
        return 'en';
    }
};

// Export for use in other scripts
if (typeof module !== 'undefined' && module.exports) {
    module.exports = I18n;
}
