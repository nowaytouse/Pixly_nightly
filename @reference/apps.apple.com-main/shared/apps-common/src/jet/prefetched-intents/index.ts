import type { LoggerFactory } from '@amp/web-apps-logger';
import type { Intent, IntentReturnType } from '@jet/environment/dispatching';
import { serializeServerData, stableStringify } from './server-data';
import type { PrefetchedIntent } from './types';
import { getPrefetchedIntents } from './get-prefetched-intents';

export type { PrefetchedIntent } from './types';

export function serializePrefetchedIntents(
    loggerFactory: LoggerFactory,
    prefetchedIntents: PrefetchedIntent[],
): string {
    const serialized = serializeServerData(
        prefetchedIntents.map(removeSeoData),
    );

    if (serialized.length === 0) {
        const logger = loggerFactory.loggerFor('serializePrefetchedIntents');
        logger.warn('failed to serialize prefetched intents');
    }

    return serialized;
}

// SEO data is never needed for the first clientside render since the server
// already adds SEO tags. The seoData convention is ubiquitous across the apps.
// See: rdar://144581413 (Etag constantly changes on pages with songs due to seoData.ogSongs)
function removeSeoData(intent: PrefetchedIntent): PrefetchedIntent {
    const { data } = intent;

    // We very intentionally return the original intent to prevent
    // needlessly allocating new objects.

    if (data === null || typeof data !== 'object' || !('seoData' in data)) {
        return intent;
    }

    const { seoData } = data;
    if (seoData === null || typeof seoData !== 'object') {
        return intent;
    }

    let partialSeoData:
        | { pageTitle?: unknown; titleHeader?: unknown }
        | undefined = undefined;
    if ('pageTitle' in seoData || 'titleHeader' in seoData) {
        partialSeoData = {};

        if ('pageTitle' in seoData) {
            partialSeoData['pageTitle'] = seoData.pageTitle;
        }

        if ('titleHeader' in seoData) {
            partialSeoData['titleHeader'] = seoData.titleHeader;
        }
    }

    // Only if we're actually going to do the removal do we spread
    return {
        ...intent,
        data: {
            ...data,
            // Page title is desirable to keep as it is occasionally consulted
            // outside of MetaTags.svelte
            seoData: partialSeoData,
        },
    };
}

export class PrefetchedIntents {
    static empty(): PrefetchedIntents {
        return new PrefetchedIntents(new Map());
    }

    static fromDom(
        loggerFactory: LoggerFactory,
        options?: { evenIfSignedIn?: boolean; featureKitItfe?: string },
    ): PrefetchedIntents {
        return new PrefetchedIntents(
            getPrefetchedIntents(loggerFactory, options),
        );
    }

    private intents: Map<string, unknown>;

    private constructor(intents: Map<string, unknown>) {
        this.intents = intents;
    }

    get<I extends Intent<unknown>>(intent: I): IntentReturnType<I> | undefined {
        if (this.intents.size === 0) {
            return;
        }

        let subject: string | void;
        try {
            subject = stableStringify(intent);
        } catch (e) {
            // It's possible the intents don't stringify. If that's that case,
            // then we won't find it in this.intents, since the keys of that
            // are successfully stringified intents. We could try something
            // sophisticated here, but it's probably not worth it as most
            // intents will serialize.
            return;
        }

        const data = this.intents.get(subject);

        // Remove the prefetched data so that it can only be used once
        this.intents.delete(subject);

        // NOTE: There really isn't a good way to be safe with types here. We
        // don't have a type guard for arbitrary IntentReturnType<I>. We just
        // have to trust that the serialized data is of the correct type. This
        // isn't unreasonable since we control serialization.
        return data as unknown as IntentReturnType<I> | undefined;
    }
}
