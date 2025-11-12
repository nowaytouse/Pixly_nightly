//TODO: rdar://73157363 (Limit loc plural functions to only use supported locales)
import * as cardinals from 'make-plural/cardinals';
import type {
    Locale,
    ILocaleJSON,
    InterpolationOptions,
    TranslatorOptions,
    ImissingInterpolationFn,
    ImissingKeyFn,
    ITranslator,
} from './types';

const DEFAULT_MISSING_FN: ImissingKeyFn = (key: string): string => `**${key}**`;
const DEFAULT_INTERPOLATION_REGEX: RegExp = /@@(.*?)@@/g;

/**
 * Interpolates string and returns result.
 * @category Localization
 * @param phrase phrase to be interpolated ex. ```"hello my name is @@name@@" ```
 * @param options object containing values to subsitute ex. ``` { name: "Joe" } ```
 * @param onMissingInterpolationFn callback to be called if options object does not contain a value for the interpolation schema
 *
 * @returns translated string ex ``` "hello my name is Joe" ```
 */
export function interpolateString(
    key: string,
    phrase: string,
    options: InterpolationOptions,
    onMissingInterpolationFn: ImissingInterpolationFn | null,
    locale: Locale,
): string {
    const result = phrase.replace(
        DEFAULT_INTERPOLATION_REGEX,
        function (expression: string, argument: string) {
            const optionHasProperty = options.hasOwnProperty(argument);
            const optionType = typeof options[argument];
            const argumentIsUndefined = optionType === 'undefined';
            const argumentIsValid =
                optionType === 'string' || optionType === 'number';
            let value: string = expression;
            if (optionHasProperty && argumentIsValid) {
                let validValue: string | number = options[argument];
                if (
                    optionType === 'number' &&
                    options.hasOwnProperty('count')
                ) {
                    validValue = (validValue as number).toLocaleString([
                        locale,
                        'en-US',
                    ]);
                }
                value = validValue as string;
            } else if (onMissingInterpolationFn && argumentIsUndefined) {
                onMissingInterpolationFn(key, value);
            }
            return value;
        },
    );

    return result;
}

type Cardinal = (n: number | string) => cardinals.PluralCategory;

function getCardinal(selectedLang: string): Cardinal | undefined {
    // @ts-ignore-error TypeScript does not allow us to index into a namespace dynamically
    return cardinals[selectedLang];
}

/**
 * TODO: rdar://73157363 (Limit loc plural functions to only use supported locales)
 * Used to select the locale specific cardinal plural form key.
 * @category Localization
 * @param count number to determine the cardinal value
 * @param key base key
 * @param locale to lookup plural
 *
 * Reference:
 * https://confluence.sd.apple.com/pages/viewpage.action?spaceKey=ASL&title=Pluralization+Rules
 *
 * @returns key + correct plural ex. ```[key].[ 'zero' | 'one' | 'two' | 'few' | 'many' | 'other'] ```
 */

export const getPlural = (
    count: number,
    key: string,
    locale: Locale,
): string => {
    const lang = locale.split('-')[0];

    // use english plural for dev strings
    const selectedLang = lang === 'dev' ? 'en' : lang;
    const cardinal = getCardinal(selectedLang);

    let plural: cardinals.PluralCategory | null = null;
    if (cardinal) {
        plural = cardinal(count);
        // TODO: rdar://93665757 (JMOTW: investigate where to use 'few' and 'many' loc keys)
        if (plural === 'few' || plural === 'many') plural = 'other';
    }
    return plural ? `${key}.${plural}` : key;
};

/**
 * Class that manages translations, plural rules,
 * and interpolation for a single locale.
 * @category Localization
 */
class Translator implements ITranslator {
    private translationMap: Map<string, string>;
    private locale: Locale;
    private onMissingKeyFn: ImissingKeyFn;
    private onMissingInterpolationFn: ImissingInterpolationFn | null;
    constructor(
        locale: Locale,
        phrases: ILocaleJSON,
        options: TranslatorOptions = {},
    ) {
        const {
            onMissingKeyFn = DEFAULT_MISSING_FN,
            onMissingInterpolationFn = null,
        } = options;
        this.locale = locale;
        this.translationMap = new Map(Object.entries(phrases));
        this.onMissingKeyFn = onMissingKeyFn;
        this.onMissingInterpolationFn = onMissingInterpolationFn;
    }

    /**
     * Gets the correct value from the translation map.
     * @category Localization
     * @param key used to look up the value
     */
    private getValue(key: string): string | null {
        return this.translationMap.get(key) || null;
    }
    /**
     * Gets an uniterpolated value of key.
     * @category Localization
     * @param key used to look up the value
     */
    getUninterpolatedString(key: string) {
        const keyValue = this.getValue(key);
        return keyValue ? keyValue : this.onMissingKeyFn(key);
    }
    /**
     * Translate string based on translation map, plural rules interpolates values.
     * @category Localization
     * @param key used to look up the value
     * @param options used for interpolation
     * @returns translated string
     */
    translate(key: string, options: InterpolationOptions = {}): string {
        let internalKey = key;
        const { count } = options;

        if (count && !isNaN(count)) {
            internalKey = getPlural(count, key, this.locale);
        }

        const keyValue = this.getValue(internalKey);
        return keyValue
            ? interpolateString(
                  internalKey,
                  keyValue,
                  options,
                  this.onMissingInterpolationFn,
                  this.locale,
              )
            : this.onMissingKeyFn(internalKey);
    }
}

export default Translator;
