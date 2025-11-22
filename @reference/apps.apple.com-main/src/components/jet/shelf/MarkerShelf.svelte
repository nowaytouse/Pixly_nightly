<script lang="ts" context="module">
    import type { ShelfBasedProductPage } from '@jet-app/app-store/api/models';
    import type {
        Lockup,
        Shelf,
        ShelfMarker,
    } from '@jet-app/app-store/api/models';

    export interface MarkerShelf extends Shelf {
        contentType: 'marker';
        marker: ShelfMarker;
        items: Lockup[];
    }

    export function isMarkerShelf(shelf: Shelf): shelf is MarkerShelf {
        const { contentType, marker, items } = shelf;

        return (
            contentType === 'marker' &&
            typeof marker === 'string' &&
            Array.isArray(items)
        );
    }
</script>

<script lang="ts">
    import ProductTopLockup from '~/components/jet/marker-shelf/ProductTopLockup.svelte';

    export let shelf: MarkerShelf;

    export let page: ShelfBasedProductPage;
</script>

{#if shelf.marker === 'productTopLockup'}
    <ProductTopLockup {page} />
{/if}
