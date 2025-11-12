import { getPageDir } from './getPageDir';

/**
 * Checks if a string contains language script
 * ex. "zh-Hant-HK", "zh-Hant-TW", "zh-Hans-CN"
 * @param {string} locale
 * @returns {boolean}
 */
const hasSupportedLanguageScript = (locale: string): boolean => {
    const SUPPORTED_SCRIPTS = ['-hans-', '-hant-'];

    const formattedLocale = locale.toLowerCase();
    return SUPPORTED_SCRIPTS.some((item) => formattedLocale.includes(item));
};

/**
 *
 * BCP47 https://www.w3.org/International/articles/language-tags/
 *
 * @param {string} language https://en.wikipedia.org/wiki/ISO_639
 * @param {string} region https://en.wikipedia.org/wiki/ISO_3166-1
 * @param {string} script https://en.wikipedia.org/wiki/ISO_15924

 */
const buildBcp47String = (
    language: string,
    region: string,
    script?: string,
): string => {
    let capitalizeScript: string | null = null;
    if (script) {
        capitalizeScript =
            script[0].toUpperCase() + script.substring(1).toLowerCase();
    }
    let bcp47Arr = [
        language.toLowerCase(),
        capitalizeScript,
        region.toUpperCase(),
    ];

    return bcp47Arr.filter((item) => item !== null).join('-');
};

/**
 * @description
 * get values to be used in <html> tag lang and dir attributes.
 *
 * @param {string} locale
 * @returns { { dir: 'rtl' | 'ltr', lang: string }} HTML dir + lang values
 */

export function getLocAttributes(locale: string): {
    dir: 'rtl' | 'ltr';
    lang: string;
} {
    const pageDir = getPageDir(locale);
    let bcp47 = locale;

    const localeStrings = locale.split('-');

    // region index in array
    const regionIndex = hasSupportedLanguageScript(locale) ? 2 : 1;

    const language = localeStrings[0];
    const script = hasSupportedLanguageScript(locale)
        ? localeStrings[1]
        : undefined;
    const region = localeStrings[regionIndex];

    if (language && region) {
        bcp47 = buildBcp47String(language, region, script);
    }

    return {
        dir: pageDir,
        lang: bcp47,
    };
}
