import { getContext, setContext } from 'svelte';

import type { TodayPage } from '@jet-app/app-store/api/models';
import {
    type TodayCardShelf,
    isTodayCardShelf,
} from '~/components/jet/shelf/TodayCardShelf.svelte';

/**
 * Describes the configuration of the card layout within a {@linkcode TodayCardShelf}
 */
interface LayoutConfiguration {
    wrap: {
        shouldStretchFirstCard: boolean;
    };
    nowrap: {
        shouldStretchFirstCard: boolean;
    };
}

const LAYOUT_CONFIGURATION_FALLBACK: LayoutConfiguration = Object.freeze({
    wrap: {
        shouldStretchFirstCard: true,
    },
    nowrap: {
        shouldStretchFirstCard: true,
    },
});

type TodayCardLayoutStore = WeakMap<TodayCardShelf, LayoutConfiguration>;
type TodayCardLayoutStoreContext = TodayCardLayoutStore | undefined;

const TODAY_CARD_LAYOUT_CONTEXT_ID = 'today-card-layout-context';

/**
 * Store the {@linkcode LayoutConfiguration} for each {@linkcode TodayCardShelf} in a
 * {@linkcode TodayPage} in "context", so it can be retrieved at the shelf-component level
 *
 * This is necessary because the layout of the cards within each shelf of a {@linkcode TodayPage}
 * is only knowable given information about the shelves that were rendered before it
 *
 * The information about the shelf layout is persisted through the "context" API so that the
 * rendering of a {@linkcode TodayPage} can defer to the "default" page component, which requires
 * that we pass no additional arguments into each shelf component
 *
 * {@linkcode getTodayCardLayoutConfiguration} can be used to look up the {@linkcode LayoutConfiguration}
 * stored for a given {@linkcode TodayCardShelf}
 */
export function setTodayCardLayoutContext(page: Pick<TodayPage, 'shelves'>) {
    const store: TodayCardLayoutStore = new WeakMap();

    let shouldStretchFirstCardMultiline = false;
    let shouldStretchFirstCardInline = false;

    for (const shelf of page.shelves) {
        // Skip any non-`TodayCard` shelves
        if (!isTodayCardShelf(shelf)) {
            continue;
        }

        store.set(shelf, {
            wrap: {
                shouldStretchFirstCard: shouldStretchFirstCardMultiline,
            },
            nowrap: {
                shouldStretchFirstCard: shouldStretchFirstCardInline,
            },
        });

        // In the multi-line card configuration, shelves with two or three cards in them will
        // require that the next shelf swaps to stretching the cards at the opposite end
        if (shelf.items.length === 2 || shelf.items.length === 3) {
            shouldStretchFirstCardMultiline = !shouldStretchFirstCardMultiline;
        }

        // In the "inline" card configuration, each shelf should always alternate which end the
        // card is stretched on
        shouldStretchFirstCardInline = !shouldStretchFirstCardInline;
    }

    setContext<TodayCardLayoutStoreContext>(
        TODAY_CARD_LAYOUT_CONTEXT_ID,
        store,
    );
}

/**
 * Retrieve the {@linkcode LayoutConfiguration} for a given {@linkcode TodayCardShelf}
 */
export function getTodayCardLayoutConfiguration(
    shelf: TodayCardShelf,
): LayoutConfiguration {
    const todayCardLayout = getContext<TodayCardLayoutStoreContext>(
        TODAY_CARD_LAYOUT_CONTEXT_ID,
    );

    return todayCardLayout?.get(shelf) ?? LAYOUT_CONFIGURATION_FALLBACK;
}
