// Based on https://github.com/cibernox/svelte-media
import { readable } from 'svelte/store';
import { ArtworkConfig } from '@amp/web-app-components/config/components/artwork';
import { getMediaConditions } from '@amp/web-app-components/src/utils/getMediaConditions';

const { BREAKPOINTS } = ArtworkConfig.get();
const mqConditions = getMediaConditions(BREAKPOINTS);

const DEFAULT_SETTING = 'medium';

/**
 * Filters media query results and outputs the breakpoint name with a matching media query.
 *
 * @param {Object} mqls media query configurations (pulled from getMediaConditions())
 * @returns {String|undefined} breakpoint string that matches current media query
 */
function calculateMediaQuery(mqls: Record<string, MediaQueryList>): string {
    return Object.entries(mqls)
        .filter(([_, query]) => query.matches)
        .map(([name, _]) => name)[0];
}

/**
 * This function allows to build a store that tracks which of the given media query conditions matches.
 * @param initialValue The inital value for the store. It only bears importance in server side rendering
 * as it will update immediately in the browser
 * @param mediaQueryConditions The dictionary with the media query names and the MQ condition to match against.
 * @returns Svelte.Store<string> The name of the matching media query
 */
export function buildMediaQueryStore(
    initialValue: string,
    mediaQueryConditions: Record<string, string> = mqConditions,
) {
    return readable(initialValue, (set) => {
        if (
            typeof window === 'undefined' ||
            typeof matchMedia === 'undefined'
        ) {
            set(initialValue);
            return;
        }

        let mqls = {};
        let updateMediaQuery = () => set(calculateMediaQuery(mqls));

        for (const key in mediaQueryConditions) {
            mqls[key] = window.matchMedia(mediaQueryConditions[key]);
            // `addListener` is deprecated but should still be used for compatibility with more browsers.
            mqls[key].addListener(updateMediaQuery);
        }

        updateMediaQuery();

        return function (): void {
            for (let key in mqls) {
                // `removeListener` is deprecated but should still be used for compatibility with more browsers.
                mqls[key].removeListener(updateMediaQuery);
            }
        };
    });
}

export const mediaQueries = buildMediaQueryStore(DEFAULT_SETTING, mqConditions);
