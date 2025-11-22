import type { GenericPage } from '@jet-app/app-store/api/models';
import { type Writable, writable, type Readable } from 'svelte/store';

interface Page {
    page: GenericPage;
    pageDetail?: string;
}

const modalPageStore: Writable<Page | undefined> = (() => {
    // prevent global store on the server
    if (typeof window === 'undefined') {
        return {
            subscribe: () => {
                return () => {};
            },
            set: () => {},
            update: () => {},
        } as unknown as Writable<Page | undefined>;
    }

    return writable();
})();

interface ModalPageStore extends Readable<Page | undefined> {
    setPage: (page: Page) => void;
    clearPage: () => void;
}

export const getModalPageStore = (): ModalPageStore => {
    return {
        subscribe: modalPageStore.subscribe,
        setPage: (page) => modalPageStore.set(page),
        clearPage: () => modalPageStore.set(undefined),
    };
};
