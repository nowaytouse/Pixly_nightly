import type { Locale as JetLocaleDependency } from '@jet-app/app-store/foundation/dependencies/locale/locale';
import type {
    NormalizedLanguage,
    NormalizedStorefront,
    NormalizedLocale,
    UnnormalizedLocale,
} from '@jet-app/app-store/api/locale';
import type I18N from '@amp/web-apps-localization';
import type { Logger, LoggerFactory } from '@amp/web-apps-logger';

import type { Jet } from '~/jet/jet';
import {
    DEFAULT_STOREFRONT_CODE,
    DEFAULT_LANGUAGE_BCP47,
} from '~/constants/storefront';
import {
    type NormalizedLocaleWithDefault,
    normalizeStorefront,
    normalizeLanguage,
} from '~/utils/locale';
import type { Optional } from '@jet/environment';

/**
 * Contains information related to the locale of the request currently being
 * made to the application.
 *
 * Typically, localization information is expected to be known when the Jet
 * instance is initialized. The Web, however, will not know the current
 * locale and langauge until after routing has already taken place.
 *
 * This object exists to contain that lazily-determined locale information,
 * so that other dependencies can retreive it from here. It is to be created
 * with the rest of the dependencies and passed to them when they are created.
 *
 * Localization information is set in the {@linkcode Jet#setLocale} method
 */
export class Locale implements JetLocaleDependency {
    private readonly logger: Logger;

    private _storefront: NormalizedStorefront | undefined;
    private _language: NormalizedLanguage | undefined;

    i18n: I18N | undefined;

    constructor(loggerFactory: LoggerFactory) {
        this.logger = loggerFactory.loggerFor('locale');
    }

    get activeStorefront(): NormalizedStorefront {
        if (!this._storefront) {
            this.logger.warn('`storefront` was accessed before being set');
            return DEFAULT_STOREFRONT_CODE;
        }

        return this._storefront;
    }

    get activeLanguage(): NormalizedLanguage {
        if (!this._language) {
            this.logger.warn('`language` was accessed before being set');
            return DEFAULT_LANGUAGE_BCP47;
        }

        return this._language;
    }

    setActiveLocale(locale: NormalizedLocale): void {
        this._storefront = locale.storefront;
        this._language = locale.language;
    }

    normalize({
        storefront,
        language,
    }: UnnormalizedLocale): NormalizedLocaleWithDefault {
        const {
            storefront: normalizedStorefront,
            languages,
            defaultLanguage,
        } = normalizeStorefront(storefront);

        return {
            storefront: normalizedStorefront,
            ...normalizeLanguage(language || '', languages, defaultLanguage),
        };
    }

    deriveLocaleForUrl(locale: NormalizedLocale): {
        storefront: string;
        language: Optional<string>;
    } {
        const { isDefaultLanguage } = this.normalize(locale);

        return {
            storefront: locale.storefront,
            language: isDefaultLanguage ? undefined : locale.language,
        };
    }
}
