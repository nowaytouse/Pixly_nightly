<!--
    LazyLoader Component
    This component provides loading="lazy"
    functionality for browsers that do not support it.
    It uses Intersection Observers to evaluate
    if an image needs to be loaded.

    DO NOT USE DIRECTLY use LoaderSelector
-->
<script context="module" lang="ts">
    import { get } from 'svelte/store';
    import { shouldUseLazyLoader } from '@amp/web-app-components/src/components/Artwork/constants';
    import { createArtworkLoaderStore } from '@amp/web-app-components/src/components/Artwork/stores/artworkLoader';
    import type { ArtworkLoaderStore } from '@amp/web-app-components/src/components/Artwork/stores/artworkLoader';
    import { getRafQueue } from '@amp/web-app-components/src/utils/rafQueue';

    const rafQueue = getRafQueue();

    let artworkLookupTable: ArtworkLoaderStore | null = null;
    let observer: IntersectionObserver | null = null;

    const setupObserver = () => {
        let options = {
            root: null, // go off viewport
            rootMargin: '0px',
            threshold: 0.0,
        };

        return new IntersectionObserver((entries) => {
            entries.forEach((item) => {
                rafQueue.add(() => {
                    const storeValue = get(artworkLookupTable);
                    const isItemAlreadyVisible = storeValue.get(item.target);
                    if (!isItemAlreadyVisible) {
                        artworkLookupTable.addEntry(
                            item.target,
                            item.isIntersecting,
                        );
                    }
                });
            });
        }, options);
    };
    if (shouldUseLazyLoader) {
        observer = setupObserver();
        artworkLookupTable = createArtworkLoaderStore();
    }
</script>

<script lang="ts">
    import { onDestroy } from 'svelte';

    let isSubscribed = false;

    let container: Element;
    let isVisible: boolean = false;
    let unsubscribeToStore: () => void = () => {};

    const cleanup = () => {
        unsubscribeToStore();
        observer.unobserve(container);
        artworkLookupTable.cleanupEntry(container);
    };

    $: {
        if (isVisible && isSubscribed) {
            cleanup();
            isSubscribed = false;
        }
    }

    export function onSlotMount(artworkComponent: Element) {
        container = artworkComponent;
        isSubscribed = true;
        observer.observe(container);

        unsubscribeToStore = artworkLookupTable.subscribe((map) => {
            isVisible = map.get(container);
        });
    }

    onDestroy(() => {
        if (isSubscribed) {
            cleanup();
        }
    });
</script>

<slot {isVisible} />
