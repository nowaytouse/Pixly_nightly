import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';

export type ArtworkLoaderStore = {
    subscribe: Writable<WeakMap<Element, boolean>>['subscribe'];
    addEntry: (entry: Element, isVisible: boolean) => void;
    cleanupEntry: (entry: Element) => void;
};

export function createArtworkLoaderStore(): ArtworkLoaderStore {
    const value = new WeakMap();
    const { subscribe, update } = writable(value);

    return {
        subscribe,
        addEntry: (entry: Element, isVisible: boolean) => {
            update((map) => {
                map.set(entry, isVisible);
                return map;
            });
        },

        cleanupEntry: (entry: Element) => {
            update((map) => {
                map.delete(entry);
                return map;
            });
        },
    };
}
