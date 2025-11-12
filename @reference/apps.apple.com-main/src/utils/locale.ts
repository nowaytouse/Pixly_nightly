import type { Opt } from '@jet/environment';
import { DEFAULT_STOREFRONT_CODE } from '~/constants/storefront';

import type {
    NormalizedLocale,
    NormalizedStorefront,
    NormalizedLanguage,
} from '@jet-app/app-store/api/locale';
import type { Locale } from '@jet-app/app-store/foundation/dependencies/locale/locale';

import { TEXT_DIRECTION } from '@amp/web-app-components/src/constants';
import { getLocAttributes } from '@amp/web-apps-localization';

import { regions } from '~/utils/storefront-data';
import { getJet } from '~/jet/svelte';

export type NormalizedLocaleWithDefault = NormalizedLocale & {
    isDefaultLanguage: boolean;
};

type LanguageDetails = {
    languages: NormalizedLanguage[];
    defaultLanguage: NormalizedLanguage;
};

export function normalizeStorefront(storefront: Opt<string>): {
    storefront: NormalizedStorefront;
    languages: NormalizedLanguage[];
    defaultLanguage: NormalizedLanguage;
} {
    const storefronts: Record<NormalizedStorefront, LanguageDetails> = {};

    for (const { locales } of regions) {
        for (const { id, language, isDefault } of locales) {
            if (isDefault) {
                storefronts[id as NormalizedStorefront] = {
                    languages: [],
                    defaultLanguage: language as NormalizedLanguage,
                };
            }

            if (id in storefronts) {
                storefronts[id as NormalizedStorefront].languages.push(
                    language as NormalizedLanguage,
                );
            }
        }
    }

    const normalizedStorefront = (storefront || '').toLowerCase();
    const chosenStorefront =
        normalizedStorefront in storefronts
            ? (normalizedStorefront as NormalizedStorefront)
            : DEFAULT_STOREFRONT_CODE;

    return {
        storefront: chosenStorefront,
        ...storefronts[chosenStorefront],
    };
}

export function normalizeLanguage(
    language: string,
    languages: NormalizedLanguage[],
    defaultLanguage: NormalizedLanguage,
): { language: NormalizedLanguage; isDefaultLanguage: boolean } {
    function annotateReturn(language: NormalizedLanguage): {
        language: NormalizedLanguage;
        isDefaultLanguage: boolean;
    } {
        return {
            language,
            isDefaultLanguage: language === defaultLanguage,
        };
    }

    // Prefer an exact match (ex. en-US matches en-US)
    const exactMatch = findMatch(language, languages, (a, b) => a === b);
    if (exactMatch) {
        return annotateReturn(exactMatch);
    }

    // Try partial match (ex. fr-CA or fr matches fr-FR)
    const partialMatch = findMatch(
        language,
        languages,
        (a, b) => a.split('-')[0] === b.split('-')[0],
    );
    if (partialMatch) {
        return annotateReturn(partialMatch);
    }

    // The only remaining choice is the storefront default
    return annotateReturn(defaultLanguage);
}

function findMatch<T extends string>(
    needle: string,
    haystack: T[],
    matches: (a: string, b: string) => boolean,
): Opt<T> {
    return haystack.find((possibility) =>
        matches(possibility.toLowerCase(), needle.toLowerCase()),
    );
}

/**
 * Gets the current Locale instance from the Svelte context.
 *
 * @return the active {@linkcode NormalizedLocale}
 */
export function getLocale(): NormalizedLocale {
    let locale: Locale | undefined;

    try {
        const { objectGraph } = getJet();

        locale = objectGraph.locale;
    } catch {
        throw new Error('`getLocale` called before `Jet.load`');
    }

    return {
        storefront: locale.activeStorefront,
        language: locale.activeLanguage,
    };
}

/**
 * Returns whether or not the document is in RTL mode, first based on the document's direction,
 * with a fallback to the storefronts default writing direction.
 */
export function isRtl() {
    const { storefront } = getLocale();
    const { dir } = getLocAttributes(storefront);

    return (
        (typeof document !== 'undefined' &&
            document.dir === TEXT_DIRECTION.RTL) ||
        dir === TEXT_DIRECTION.RTL
    );
}
