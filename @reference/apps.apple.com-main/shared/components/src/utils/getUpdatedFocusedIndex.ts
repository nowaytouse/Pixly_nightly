export function getUpdatedFocusedIndex(
    incrementAmount: number,
    currentFocusedIndex: number | null,
    numberOfItems: number,
): number {
    const potentialFocusedIndex = incrementAmount + currentFocusedIndex;

    if (incrementAmount > 0) {
        if (currentFocusedIndex === null) {
            return 0;
        } else {
            return potentialFocusedIndex >= numberOfItems
                ? 0
                : potentialFocusedIndex;
        }
    } else {
        if (currentFocusedIndex === null) {
            return numberOfItems - 1;
        } else {
            return potentialFocusedIndex < 0
                ? numberOfItems - 1
                : potentialFocusedIndex;
        }
    }
}
