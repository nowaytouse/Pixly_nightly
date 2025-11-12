/**
 * Normalizes and makes sure we include some unicode option for number formating.
 */
function localeWithOptionsForNumbers(locale: string) {
    locale = locale.toLowerCase().replace('_', '-');

    if (locale === 'hi-in') {
        // nu-latn makes the formatter use latin numbers.
        // See BCP47 Unicode extensions for number (nu):
        // http://unicode.org/repos/cldr/trunk/common/bcp47/number.xml
        // TL;DR -u- means the start of unicode extension.
        // nu-latn means numeric (nu) extension, latn value
        return 'hi-in-u-nu-latn';
    } else if (locale === 'my') {
        // For the `my` locale, we want to display functional numbers as Latin numerals rather than in Burmese,
        // so we are overriding the locale to give us the Latin functional numbers. See radar for more context:
        // rdar://155236306 (LOC: MS-MY: ASOTW | Product Page: Functional: Numbers are not displayed in MS/EN format)
        return 'my-u-nu-latn';
    }

    return locale;
}

/**
 * Abbreviate a number into a compact shorthand
 *
 * @example
 * const abbr = abbreviateNumber(10_000, 'en-US'); // '10K'
 */
export function abbreviateNumber(value: number, locale: string): string {
    const formatter = new Intl.NumberFormat(
        localeWithOptionsForNumbers(locale),
        {
            notation: 'compact',
        },
    );

    return formatter.format(value);
}
