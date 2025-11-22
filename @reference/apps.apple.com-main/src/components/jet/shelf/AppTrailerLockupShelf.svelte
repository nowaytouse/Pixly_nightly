<script lang="ts" context="module">
    import type {
        TrailersLockup,
        MixedMediaLockup,
        Shelf,
    } from '@jet-app/app-store/api/models';

    type AppTrailerLockupItem = TrailersLockup | MixedMediaLockup;

    interface AppTrailerLockupShelf extends Shelf {
        contentType: 'appTrailerLockup';
        items: AppTrailerLockupItem[];
    }

    export function isAppTrailerLockupShelf(
        shelf: Shelf,
    ): shelf is AppTrailerLockupShelf {
        return (
            shelf.contentType === 'appTrailerLockup' &&
            Array.isArray(shelf.items)
        );
    }

    function isMixedMediaLockup(
        item: AppTrailerLockupItem,
    ): item is MixedMediaLockup {
        return Array.isArray(item.trailers);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import MixedMediaLockupItem from '~/components/jet/item/MixedMediaLockupItem.svelte';
    import TrailersLockupItem from '~/components/jet/item/TrailersLockupItem.svelte';

    export let shelf: AppTrailerLockupShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="B" let:item>
        {#if isMixedMediaLockup(item)}
            <MixedMediaLockupItem {item} />
        {:else}
            <TrailersLockupItem {item} />
        {/if}
    </ShelfItemLayout>
</ShelfWrapper>
