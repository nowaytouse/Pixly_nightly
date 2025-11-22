import { getAspectRatio } from '@amp/web-app-components/src/components/Artwork/utils/artProfile';
import { setContext, getContext, hasContext } from 'svelte';
import { derived, writable } from 'svelte/store';
import type { Readable } from 'svelte/store';
import type { Profile } from '@amp/web-app-components/src/components/Artwork/types';
import type { AspectRatioOverrideConfig } from '@amp/web-app-components/src/components/Shelf/types';

const SHELF_ASPECT_RATIO_KEY = 'shelf-aspect-ratio';

export const getShelfAspectRatioContext = (): {
    shelfAspectRatio: Readable<string>;
    addProfile: (profile: string | Profile) => void;
} => {
    return getContext(SHELF_ASPECT_RATIO_KEY);
};

export const hasShelfAspectRatioContext = () =>
    hasContext(SHELF_ASPECT_RATIO_KEY);

const createShelfAspectRatioStore = (config: AspectRatioOverrideConfig) => {
    const { subscribe, update } = writable(new Map() as Map<string, number>);

    const addProfile = (profile: string) => {
        const ratio = getAspectRatio(profile).toFixed(2);

        update((ratiosCount) => {
            const currentCount = ratiosCount.get(ratio);
            const newCount = ratiosCount.has(ratio) ? currentCount + 1 : 0;
            ratiosCount.set(ratio, newCount);
            return ratiosCount;
        });
    };

    const aspectRatioStore = {
        subscribe,
        addProfile,
    };

    const shelfAspectRatio = derived(aspectRatioStore, ($store) => {
        let aspectRatio: string = null;

        // Don't set shelf aspect ratio when only 1 ratio is found
        //
        // This allows e.g. a shelf with only tall artwork Powerswooshes to use
        // their native 3:4 aspect ratio, even when the shelf is set to use the
        // fixed 1:1 aspect ratio or a dominant aspect ratio.
        if ($store.size > 1) {
            if (config.type === 'fixed') {
                aspectRatio = config.aspectRatio;
            } else if (config.type === 'dominant') {
                let highestCount = 0;
                for (const [ratio, count] of $store.entries()) {
                    if (highestCount < count) {
                        aspectRatio = ratio;
                        highestCount = count;
                    }
                }
            }
        }

        return aspectRatio;
    });

    return {
        shelfAspectRatio,
        addProfile,
    };
};

export const createShelfAspectRatioContext = (
    config: AspectRatioOverrideConfig,
) => {
    setContext(SHELF_ASPECT_RATIO_KEY, createShelfAspectRatioStore(config));
    return getShelfAspectRatioContext();
};
