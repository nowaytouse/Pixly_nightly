/**
 * TODO: rdar://73010072 (Make localization utils its own package)
 * Copied from:
 * https://github.pie.apple.com/amp-ui/desktop-music-app/blob/main/app/utils/page-dir.js
 */

// these overrides were determined to always show page in RTL, even if the global elements dont contain
// an he_il entry
// <rdar://problem/49297213> LOC: IW-IL: RTL: Web Preview Pages: The Preview Pages are not RTL.
const RTL_LANG_CODES_OVERRIDE = [
    'he', // hebrew
];

const RTL_LANG_CODES = [
    'ar', // arabic
    'he', // hebrew
    'ku', // kurdish
    'ur', // urdu
    'ps', // pashto
    'yi', // yiddish
];

/**
 * Determine the page-direction for a given locale
 *
 * @param {String} localeCode - A string containing a language code and region code separated by a hyphen.
 * @param {String|undefined|null} langParam - A language code passed from the `l=` query param.
 */
export function getPageDir(
    localeCode: string,
    langParam: string | undefined | null = null,
) {
    const twoLettersLangCode = localeCode.split('-')[0];
    const isRTLLang = RTL_LANG_CODES.includes(twoLettersLangCode);
    const isRTLLangOverride =
        typeof langParam === 'string' &&
        RTL_LANG_CODES_OVERRIDE.includes(langParam);

    return isRTLLang || isRTLLangOverride ? 'rtl' : 'ltr';
}
