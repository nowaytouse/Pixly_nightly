import type {
    Region,
    Languages,
} from '@amp/web-app-components/src/components/buttons/LocaleSwitcherButton/types';
import type { StorefrontNames } from '@amp/web-app-components/src/components/banners/types';
import {
    regions as outputtedRegions,
    languages as outputtedLanguages,
} from 'virtual:storefronts';
import { getFormattedStorefrontNameTranslations } from '@amp/web-app-storefronts';

export const regions: Region[] = outputtedRegions;
export const languages: Languages = outputtedLanguages;
export const storefrontNameTranslations: StorefrontNames =
    getFormattedStorefrontNameTranslations(regions);
