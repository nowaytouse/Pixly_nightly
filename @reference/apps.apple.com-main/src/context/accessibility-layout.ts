import { getContext, setContext } from 'svelte';

import type { Shelf } from '@jet-app/app-store/api/models';
import { isAccessibilityHeaderShelf } from '~/components/jet/shelf/AccessibilityHeaderShelf.svelte';
import { isAccessibilityFeaturesShelf } from '~/components/jet/shelf/AccessibilityFeaturesShelf.svelte';
import { isAccessibilityDeveloperLinkShelf } from '~/components/jet/shelf/AccessibilityDeveloperLinkShelf.svelte';

/**
 * Describes the layout configuration for accessibility shelves
 */
interface AccessibilityLayoutConfiguration {
    withBottomPadding: boolean;
}

const ACCESSIBILITY_LAYOUT_FALLBACK: AccessibilityLayoutConfiguration =
    Object.freeze({
        withBottomPadding: false,
    });

type AccessibilityLayoutStore = WeakMap<
    Shelf,
    AccessibilityLayoutConfiguration
>;
type AccessibilityLayoutStoreContext = AccessibilityLayoutStore | undefined;

const ACCESSIBILITY_LAYOUT_CONTEXT_ID = 'accessibility-layout-context';

/**
 * Check if a shelf is accessibility-related
 */
function isAccessibilityRelated(shelf: Shelf): boolean {
    return (
        shelf.contentType === 'accessibilityParagraph' ||
        shelf.contentType === 'accessibilityFeatures'
    );
}

/**
 * Check if a shelf is one of the target accessibility shelves
 */
function isTargetAccessibilityShelf(shelf: Shelf): boolean {
    return (
        isAccessibilityHeaderShelf(shelf) ||
        isAccessibilityFeaturesShelf(shelf) ||
        isAccessibilityDeveloperLinkShelf(shelf)
    );
}

/**
 * Store the {@linkcode AccessibilityLayoutConfiguration} for each accessibility shelf
 * in "context", so it can be retrieved at the shelf-component level
 *
 * This determines bottom padding based on whether the next shelf is accessibility-related
 */
export function setAccessibilityLayoutContext(page: { shelves: Shelf[] }) {
    const store: AccessibilityLayoutStore = new WeakMap();

    for (let i = 0; i < page.shelves.length; i++) {
        const shelf = page.shelves[i];

        // Only process target accessibility shelves
        if (!isTargetAccessibilityShelf(shelf)) {
            continue;
        }

        // Check if the next shelf is accessibility-related
        const nextShelf = page.shelves[i + 1];
        const hasAccessibilityNext =
            nextShelf && isAccessibilityRelated(nextShelf);

        store.set(shelf, {
            withBottomPadding: !hasAccessibilityNext,
        });
    }

    setContext<AccessibilityLayoutStoreContext>(
        ACCESSIBILITY_LAYOUT_CONTEXT_ID,
        store,
    );
}

/**
 * Retrieve the {@linkcode AccessibilityLayoutConfiguration} for a given accessibility shelf
 */
export function getAccessibilityLayoutConfiguration(
    shelf: Shelf,
): AccessibilityLayoutConfiguration {
    const accessibilityLayout = getContext<AccessibilityLayoutStoreContext>(
        ACCESSIBILITY_LAYOUT_CONTEXT_ID,
    );

    return accessibilityLayout?.get(shelf) ?? ACCESSIBILITY_LAYOUT_FALLBACK;
}
