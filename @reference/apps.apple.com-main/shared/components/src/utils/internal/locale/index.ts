/* istanbul ignore file */

//TODO rdar://93379311 (Solution for sharing context between app + shared components)
import { getContext, setContext } from 'svelte';
import type { Locale } from '@amp/web-app-components/src/types';

const CONTEXT_NAME = 'shared:locale';

// WARNING these signatures can change after rdar://93379311
export function setLocale(context: Map<string, unknown>, locale: Locale) {
    context.set(CONTEXT_NAME, locale);
}

// WARNING these signatures can change after rdar://93379311
export function getLocale(): Locale {
    return getContext(CONTEXT_NAME) as Locale | undefined;
}
