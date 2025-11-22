<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { throttle } from '@amp/web-app-components/src/utils/throttle';

    const dispatch = createEventDispatcher();

    export let resizeThrottleLimit = 100; // Limit on how often to fire resize event
    export let resizeTimeoutLimit = 250; // If resize event hasn't fired in this much time, we are no longer resizing

    let isResizing: boolean = false;
    let resizeTimeoutId;

    const handleResize = () => {
        isResizing = true;

        if (resizeTimeoutId) {
            clearInterval(resizeTimeoutId);
        }

        resizeTimeoutId = setTimeout(
            () => (isResizing = false),
            resizeTimeoutLimit,
        );
    };

    // Dispatch event whenever isResizing updates
    $: dispatch('resizeUpdate', { isResizing });
</script>

<svelte:window on:resize={throttle(handleResize, resizeThrottleLimit)} />
