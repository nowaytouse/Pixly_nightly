import { writable, type Readable } from 'svelte/store';

export type VisibleIndexData = {
    startIndex: number;
    endIndex: number;
};

export interface VisibleStore extends Readable<VisibleIndexData> {
    updateStartIndex: (num: number) => void;
    updateEndIndex: (num: number) => void;
}

/**
 * Store for keeping track of items rendered in shelf.
 */
export const createVisibleIndexStore = (): VisibleStore => {
    const { subscribe, update } = writable({
        startIndex: 0,
        endIndex: 0,
    });

    return {
        subscribe,
        updateStartIndex: (startIndex: number) =>
            update((visibleItems) => {
                return { ...visibleItems, startIndex };
            }),
        updateEndIndex: (endIndex: number) =>
            update((visibleItems) => {
                return { ...visibleItems, endIndex };
            }),
    };
};
