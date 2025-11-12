import { readable } from 'svelte/store';
import I18N from '@amp/web-apps-localization';
import { getContext } from 'svelte';
import type { Readable } from 'svelte/store';
import type { Locale, ILocaleJSON } from '@amp/web-apps-localization';
import type { Logger, LoggerFactory } from '@amp/web-apps-logger';
import { isEnabled } from '@amp/web-apps-featurekit';

export type { Locale } from '@amp/web-apps-localization';

import { __FF_SHOW_LOC_KEYS } from '~/utils/features/consts';

const CONTEXT_NAME = 'i18n';

export async function setup(
    context: Map<string, unknown>,
    loggerFactory: LoggerFactory,
    locale: Locale,
): Promise<I18N> {
    const log = loggerFactory.loggerFor('i18n');

    let alwaysShowScreamers = false;
    if (isEnabled(__FF_SHOW_LOC_KEYS)) {
        alwaysShowScreamers = true;
    }

    const translations = await getTranslations(log, locale);
    const i18n = new I18N(log, locale, translations, alwaysShowScreamers);
    const store = readable(i18n);

    context.set(CONTEXT_NAME, store);

    return i18n;
}

/**
 * Gets the current i18n store from the Svelte context.
 *
 * @return i18n The i18n store
 */
export function getI18n(): Readable<I18N> {
    const i18n = getContext(CONTEXT_NAME) as Readable<I18N> | undefined;

    if (!i18n) {
        throw new Error('getI18n called before setup');
    }

    return i18n;
}

async function getTranslations(
    log: Logger,
    locale: Locale,
): Promise<ILocaleJSON> {
    try {
        // TODO: Shoebox logic here
        const translations = await importLocale(locale);
        return translations.default;
    } catch (err) {
        log.error('failed to load:', err);
        throw new Error('i18n failed to load');
    }
}

interface IDynamicImportJSON {
    default: ILocaleJSON;
}

//TODO: rdar://73157638 (Determine if we can use ES modules based on browser matrix)
// Possibly switch this to fetch instead of dynamic imports?
function importLocale(locale: Locale): Promise<IDynamicImportJSON> {
    return import(`../../tmp/locales/${locale}/translations.json`);
}
