const LOCALES_REGEX = {
    ARABIC: /^(ar-?)/,
    HEBREW: /^(he-?)/,
    HINDI: /^(hi-?)/,
    JAPANESE: /^(ja-?)/,
    KOREAN: /^(ko-?)/,
    THAI: /^(th-?)/,
};

export const BASE = '//www.apple.com/wss/fonts';

/**
 *
 * @param locale %7C String %7C The locale to get the font URL for
 * @param includeNewYork %7C Boolean %7C Used to specifiy whether to include the New York font which is an additional font in addition to SF Pro
 * @returns URL string to fetch fonts
 */
export function getFontURL(locale: string, includeNewYork?: boolean): string {
    let fonts = 'SF+Pro,v4%7CSF+Pro+Icons,v1';
    // Check for any Arabic locales first (full list here: https://confluence.sd.apple.com/display/AMPRighttoLeft/BRD)
    if (LOCALES_REGEX.ARABIC.test(locale)) {
        fonts = `${fonts}%7CArabic+UI,v1`;
    } else if (LOCALES_REGEX.HEBREW.test(locale)) {
        fonts = `${fonts}%7CArial+Hebrew,v1`;
    } else if (LOCALES_REGEX.HINDI.test(locale)) {
        fonts = `${fonts}%7CKohinoor+Devanagari,v1`;
    } else if (LOCALES_REGEX.JAPANESE.test(locale)) {
        fonts = `${fonts}%7CSF+Pro+JP,v1`;
    } else if (LOCALES_REGEX.KOREAN.test(locale)) {
        fonts = `${fonts}%7CSF+Pro+KR,v2`;
    } else if (LOCALES_REGEX.THAI.test(locale)) {
        fonts = `${fonts}%7CThonburi+Pro,v1`;
    }

    switch (locale) {
        case 'zh-cn':
            fonts = `SF+Pro,v4%7CSF+Pro+SC,v1%7CSF+Pro+Icons,v1`;
            break;
        case 'zh-hk':
            fonts = `SF+Pro,v4%7CSF+Pro+HK,v1%7CSF+Pro+Icons,v1`;
            break;
        case 'zh-mo':
        case 'zh-tw':
            fonts = `SF+Pro,v4%7CSF+Pro+TC,v1%7CSF+Pro+Icons,v1`;
            break;
    }

    if (includeNewYork) {
        fonts = `${fonts}%7CNew+York+Small,v1%7CNew+York+Medium,v1%7CNew+York+Large,v1`;
    }

    return `${BASE}?families=${fonts}&display=swap`;
}
