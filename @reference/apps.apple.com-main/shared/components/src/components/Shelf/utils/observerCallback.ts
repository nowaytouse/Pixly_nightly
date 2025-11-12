/**
 * @name checkItemPositionInShelf
 * @description determine if we need to hide/show navigation arrows.
 *
 * @param entry entry provided by the intersection observer
 * @param lastIndex index of the last item in the list
 *
 * @returns first/last item values ONLY when being intersected,
 * otherwise will return null.
 */

// eslint-disable-next-line import/prefer-default-export
export const checkItemPositionInShelf = (
    entry: IntersectionObserverEntry,
    lastIndex: number,
): [boolean | null, boolean | null] => {
    const item = entry.target as HTMLLIElement;
    const itemIndexInView = item.dataset.index;
    const isItemVisible = entry.isIntersecting;

    const FIRST_INDEX = '0';
    const LAST_INDEX = `${lastIndex}`;

    const isFirstItemAndInView =
        itemIndexInView === FIRST_INDEX ? isItemVisible : null;
    const isLastItemAndInView =
        itemIndexInView === LAST_INDEX ? isItemVisible : null;

    return [isFirstItemAndInView, isLastItemAndInView];
};
