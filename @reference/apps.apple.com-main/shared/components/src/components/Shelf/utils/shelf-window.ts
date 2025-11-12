/* eslint-disable import/prefer-default-export */

/**
 * Keeps track of the items that are
 * within the viewport of a shelf.
 */
export class ShelfWindow {
    /**
     * List of indexes of visible shelf items.
     */
    private visibleShelfEntries: Set<number> = new Set();

    /**
     * The lowest visible index in the shelf viewport.
     */
    private lowestIndexInVisibleShelf: number | undefined;

    /**
     * The highest visible index in the shelf viewport.
     */
    private highestIndexInVisibleShelf: number | undefined;

    /**
     * Adds the index that has entered the viewport to to shelf item visibility set.
     * @param index item's index that has entered the viewport
     */
    enterValue(index: number) {
        this.visibleShelfEntries.add(index);
        this.setMinAndMaxValuesOfViewport();
    }

    /**
     * Removes index that has left viewport from shelf item visibility set.
     *
     * @param index item index that has left the viewport
     */
    exitValue(index: number) {
        this.visibleShelfEntries.delete(index);
        this.setMinAndMaxValuesOfViewport();
    }

    /**
     * Set the min and max based on indexes in shelf item visiblity set.
     */
    private setMinAndMaxValuesOfViewport() {
        this.lowestIndexInVisibleShelf = Math.min(...this.visibleShelfEntries);
        this.highestIndexInVisibleShelf = Math.max(...this.visibleShelfEntries);
    }

    /**
     * Get the current visible indexes for a given shelf.
     *
     * @returns
     * the first and last item indexes in a shelf viewport
     * or null if both values are not set.
     */
    getViewport(): [number, number] | null {
        const firstIndex = this.lowestIndexInVisibleShelf;
        const secondIndex = this.highestIndexInVisibleShelf;

        if (typeof firstIndex === 'number' && typeof secondIndex === 'number') {
            return [firstIndex, secondIndex];
        }

        return null;
    }
}
