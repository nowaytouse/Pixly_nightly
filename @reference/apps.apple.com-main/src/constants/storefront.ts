import type {
    NormalizedLanguage,
    NormalizedStorefront,
} from '@jet-app/app-store/api/locale';

export const DEFAULT_STOREFRONT_CODE = 'us' as NormalizedStorefront;
export const DEFAULT_LANGUAGE_BCP47 = 'en-US' as NormalizedLanguage;

export const EU_STOREFRONTS = [
    'at',
    'be',
    'bg',
    'cy',
    'cz',
    'dk',
    'ee',
    'fi',
    'fr',
    'de',
    'gr',
    'hr',
    'hu',
    'ie',
    'it',
    'lv',
    'lt',
    'lu',
    'mt',
    'nl',
    'pl',
    'pt',
    'ro',
    'sk',
    'si',
    'es',
    'se',
    'uk',
];

export const SUPPORTED_STOREFRONTS_FOR_VISION = new Set<NormalizedStorefront>([
    'us',
    'cn',
    'hk',
    'jp',
    'sg',
    'au',
    'ca',
    'fr',
    'de',
    'gb',
    'kr',
    'ae',
    'tw',
] as NormalizedStorefront[]);

export const UNSUPPORTED_STOREFRONTS_FOR_ARCADE = new Set([
    'cn',
    'hk',
    'mo',
] as NormalizedStorefront[]);
