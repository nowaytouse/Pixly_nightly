import type {
    ShelfBasedProductPage,
    Shelf,
} from '@jet-app/app-store/api/models';
import { isProductMediaShelf } from '~/components/jet/shelf/ProductMediaShelf.svelte';

type ShelfWithExpandedMedia = Shelf & {
    expandedMedia?: ShelfWithExpandedMedia[];
};

export const getProductPageShelvesForOrdering = (
    page: ShelfBasedProductPage,
    shelfOrder: string,
): Shelf[] => {
    return (
        page.shelfOrderings[shelfOrder]
            ?.map((shelfIdentifier) => page.shelfMapping[shelfIdentifier])
            // The type system doesn't reflect this, but ordering identifier may be provided for
            // shelves that do not exist. We should probably filter those out
            .filter((shelf): shelf is Shelf => !!shelf)
    );
};

export const getProductPageShelvesWithExpandedMedia = (
    page: ShelfBasedProductPage,
): ShelfWithExpandedMedia[] => {
    const { defaultShelfOrdering = 'notPurchasedOrdering' } = page;

    const shelves = getProductPageShelvesForOrdering(
        page,
        defaultShelfOrdering,
    ) as ShelfWithExpandedMedia[];

    // find the location of the product media of selected platform in shelves
    const mainMediaShelfIndex = shelves.findIndex((shelf) =>
        isProductMediaShelf(shelf),
    );

    let expandedMedia: ShelfWithExpandedMedia[] | undefined;

    if (mainMediaShelfIndex !== -1) {
        expandedMedia = getProductPageShelvesForOrdering(
            page,
            'notPurchasedOrdering_ExpandedMedia',
        )
            .filter((shelf) => isProductMediaShelf(shelf))
            // filter out the product media shelf of selected platform to avoid duplicate shelves
            .filter(({ id }) => id !== shelves[mainMediaShelfIndex].id);
    }

    if (expandedMedia) {
        shelves[mainMediaShelfIndex].expandedMedia = expandedMedia;
    }

    return shelves;
};
