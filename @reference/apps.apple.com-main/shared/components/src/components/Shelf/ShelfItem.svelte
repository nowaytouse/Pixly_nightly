<script lang="ts">
    import { getRafQueue } from '@amp/web-app-components/src/utils/rafQueue';
    import { onDestroy } from 'svelte';
    import { get, type Readable } from 'svelte/store';
    import type { VisibleIndexData } from '@amp/web-app-components/src/components/Shelf/store/visibleStore';

    export let index: number;
    export let visibleStore: Readable<VisibleIndexData>;

    const rafQueue = getRafQueue();
    const isBetween = (start: number, end: number, value: number) => {
        return value >= start && value <= end;
    };
    // get value but dont subscribe to it.
    let { startIndex, endIndex } = get(visibleStore);
    $: isRendered = isBetween(startIndex, endIndex, index);
    $: isSubscribed = true;

    // Elements should only be subscribed
    // to the store if they are not rendered.
    const unsubscribe = visibleStore.subscribe((store) => {
        const { startIndex, endIndex } = store;
        const currentIsRendered = isBetween(startIndex, endIndex, index);
        // Manually handling subscription to
        // update DOM using RAF in browser for smoother scrolling
        if (currentIsRendered && !isRendered) {
            rafQueue.add(() => {
                isRendered = currentIsRendered;
            });
        }
    });

    /**
     * Unsubscribe to the store only if `isSubscribed` is true
     *
     * This helps ensure that we do not accidentally call `unsubscribe` twice,
     * which can cause errors in Svelte. One way that can happen is by unsubscribing
     * both using `onDestory` and with the callback added to the `rafQueue`
     *
     * See https://github.com/sveltejs/svelte/issues/4765#issuecomment-1379243063
     */
    function unsubscribeIfNeeded() {
        if (isSubscribed) {
            unsubscribe();
            isSubscribed = false;
        }
    }

    $: if (isSubscribed && isRendered) {
        rafQueue.add(() => {
            unsubscribeIfNeeded();
        });
    }

    onDestroy(() => {
        unsubscribeIfNeeded();
    });
</script>

<slot {isRendered} />
