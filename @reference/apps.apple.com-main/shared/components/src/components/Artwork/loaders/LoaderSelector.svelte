<script context="module" lang="ts">
    export const LOADER_TYPE = {
        LAZY: 'LAZY',
        NONE: 'NONE',
    } as const;
</script>

<script lang="ts">
    import LazyLoader from '@amp/web-app-components/src/components/Artwork/loaders/LazyLoader.svelte';
    import NoLoader from '@amp/web-app-components/src/components/Artwork/loaders/NoLoader.svelte';
    import { shouldUseLazyLoader } from '@amp/web-app-components/src/components/Artwork/constants';
    import type { ValueOf } from '@amp/web-app-components/src/types';
    import type { SvelteComponent } from 'svelte';

    type LoaderOptions = ValueOf<typeof LOADER_TYPE>;

    export let loaderType: LoaderOptions = LOADER_TYPE.LAZY;

    interface LoaderComponent extends SvelteComponent {
        onSlotMount: (component: Element) => void;
    }

    let currentComponent: LoaderComponent;

    export function onSlotMount(component: Element) {
        currentComponent.onSlotMount(component);
    }
</script>

{#if loaderType === LOADER_TYPE.LAZY && shouldUseLazyLoader}
    <LazyLoader bind:this={currentComponent} let:isVisible
        ><slot {isVisible} /></LazyLoader
    >
{:else}
    <NoLoader bind:this={currentComponent} let:isVisible
        ><slot {isVisible} /></NoLoader
    >
{/if}
