import Translator from './translator';
import type {
    Locale,
    InterpolationOptions,
    ILocaleJSON,
    ITranslator,
} from './types';
import type { Logger } from '@amp/web-apps-logger';

/** @internal */
const formatOptions = (
    options: InterpolationOptions | number,
): InterpolationOptions =>
    typeof options === 'number' ? { count: options } : options;

/**
 *
 * Adapter class to expose expected LOC interface
 * @category Localization
 */
export class I18N {
    private readonly log: Logger;
    private readonly locale: Locale;
    private readonly translator: ITranslator;
    private readonly keys: ILocaleJSON;
    private readonly alwaysShowScreamers: boolean;

    /**
     * builds a new I18N class
     * @param locale - the locale to use default:`'en-us'`
     * @param translation - translation object default: `{}`
     * @param alwaysShowScreamers - optional boolean that is set upstream
     * by a FeatureKit feature flag. This makes it so the LOC keys themselves are
     * printed to the DOM, rather than their translations, which is helpful for QA testing
     */
    constructor(
        log: Logger,
        locale: Locale = 'en-us',
        translation: ILocaleJSON = {},
        alwaysShowScreamers: boolean = false,
    ) {
        this.log = log;
        this.locale = locale;
        this.translator = new Translator(locale, translation, {
            onMissingKeyFn: (key: string): string => {
                log.warn('key missing:', key);
                return `**${key}**`;
            },
            onMissingInterpolationFn: (key: string, interpolation: string) => {
                log.warn(`key ${key} missing interpolation:`, interpolation);
            },
        });
        this.keys = translation;
        this.alwaysShowScreamers = alwaysShowScreamers;
    }

    get currentLocale(): Locale {
        return this.locale;
    }

    get currentKeys(): ILocaleJSON {
        return this.keys;
    }

    /**
     * Gets non-interpolated string.
     * @category Localization
     * @param key key to lookup in the translation.json
     * @returns an uninterpolated string value
     */
    getUninterpolatedString(key: string): string {
        if (this.alwaysShowScreamers) {
            return key;
        } else {
            return this.translator.getUninterpolatedString(key);
        }
    }

    /**
     * Method for fetching translation based on key.
     *
     * If alwaysShowScreamers is true, return the key itself for QA testing purposes
     * (our app tends to call into this method within Svelte templates)
     *
     * @category Localization
     * @param key key to lookup in the translation.json
     * @param options options for translations
     * @returns interpolated string
     */
    t = (key: string, options: number | InterpolationOptions = {}): string => {
        if (this.alwaysShowScreamers) {
            return key;
        }

        let internalOptions: InterpolationOptions = formatOptions(options);
        if (typeof key !== 'string') {
            this.log.warn('received non-string key:', key);
            return '';
        }
        return this.translator.translate(key, internalOptions);
    };
}

export default I18N;
