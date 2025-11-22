import { getCookie } from '@amp/web-app-components/src/utils/cookie';
import type { LoggerFactory } from '@amp/web-apps-logger';
import { isSome } from '@amp/web-apps-utils';
import { deserializeServerData, stableStringify } from './server-data';
import { type PrefetchedIntent, isPrefetchedIntents } from './types';

export function getPrefetchedIntents(
    loggerFactory: LoggerFactory,
    options?: { evenIfSignedIn?: boolean; featureKitItfe?: string },
): Map<string, unknown> {
    const logger = loggerFactory.loggerFor('getPrefetchedIntents');
    const evenIfSignedIn = options?.evenIfSignedIn;
    const itfe = options?.featureKitItfe;

    const data = deserializeServerData();
    if (!data || !isPrefetchedIntents(data)) {
        return new Map();
    }

    // We avoid prefetched intents in two scenarios:
    //
    // Condition 1: User is signed in (and evenIfSignedIn is false)
    // It's possible/likely that dispatching an intent when signed in behaves
    // differently.
    //
    // Condition 2: ITFE is enabled in Feature Kit
    // When ITFE is active, we discard prefetched intents so that media API
    // calls are triggered in the browser, allowing Feature Kit to inject ITFE
    // into those calls.
    if ((!evenIfSignedIn && getCookie('media-user-token')) || itfe) {
        logger.info(
            'Discarding prefetched intents - signed in user or ITFE enabled',
        );
        return new Map();
    }

    logger.debug('received prefetched intents from the server:', data);
    return new Map(
        data
            .map(
                ({
                    intent,
                    data,
                }: PrefetchedIntent): [string, unknown] | null => {
                    try {
                        if (intent.$kind.includes('Library')) {
                            return null;
                        }
                        // NOTE: PrefetchedIntents.get depends on stableStringify
                        return [stableStringify(intent), data];
                    } catch (e) {
                        return null;
                    }
                },
            )
            .filter(isSome),
    );
}
