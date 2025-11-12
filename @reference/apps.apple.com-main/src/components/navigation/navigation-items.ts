import {
    isSome,
    unwrapOptional as unwrap,
} from '@jet/environment/types/optional';

import type { NavigationItem } from '@amp/web-app-components/src/components/Navigation/types';
import type { NavigationId } from '@amp/web-app-components/src/types';
import type {
    WebNavigation,
    WebNavigationLink,
} from '@jet-app/app-store/api/models/web-navigation';

import {
    isSystemImageArtwork,
    getIconNameFromTemplate,
} from '~/components/SystemImage.svelte';
import { getIconComponentByName } from '../SFSymbol.svelte';
import type { Artwork } from '@jet-app/app-store/api/models';
import CategoryTabItem from '~/components/jet/web-navigation/CategoryTabItem.svelte';

/**
 * A {@linkcode NavigationItem} that includes the original {@linkcode WebNavigationLink}
 * it was defined from, which is needed for the
 */
export interface NavigationItemWithTab extends NavigationItem {
    tab: WebNavigationLink;
    artwork?: Artwork;
    isActive?: boolean;
}

export function navigationIdFromLink(link: WebNavigationLink): NavigationId {
    const intent = unwrap(link.action.destination);

    return {
        type: intent.$kind,
        // `intent.$kind` will be unique for each `Intent` used here as they are
        // each a different `LandingPageIntent`
        resourceId: link.action.pageUrl ?? intent.$kind,
    };
}

/**
 * Transform the "tabs" in the `WebNavigation` into a shape that works with our
 * shared navigation side-bar components.
 */
export function makeNavLinks(
    tabs: WebNavigationLink[],
    { shouldShowSearchTab = false },
): NavigationItemWithTab[] {
    return tabs
        .filter((tab) => {
            const isSearchTab =
                tab.action?.destination?.['$kind'].includes('search_Intent');

            // Allows for filtering our the search tab, which we use when the sidebar is visible,
            // since there is a search input in the sidebar
            return isSearchTab ? shouldShowSearchTab : true;
        })
        .map((tab) => {
            const { action, artwork: tabArtwork } = tab;
            const { artwork } = action || {};
            const hasSystemImageArtwork =
                isSome(artwork) && isSystemImageArtwork(artwork);

            return {
                id: navigationIdFromLink(tab),
                label: unwrap(tab.action.title),
                url: action.pageUrl ?? undefined,
                icon: hasSystemImageArtwork
                    ? getIconComponentByName(
                          getIconNameFromTemplate(artwork.template),
                      )
                    : undefined,
                artwork: tabArtwork,
                component: !hasSystemImageArtwork ? CategoryTabItem : undefined,
                tab,
            };
        });
}
