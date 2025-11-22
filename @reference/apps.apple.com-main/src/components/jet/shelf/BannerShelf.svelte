<script lang="ts" context="module">
    import type { Shelf, Banner } from '@jet-app/app-store/api/models';

    interface BannerShelf extends Shelf {
        contentType: 'banner';
        items: Banner[];
    }

    export function isBannerShelf(shelf: Shelf): shelf is BannerShelf {
        return shelf.contentType === 'banner' && Array.isArray(shelf.items);
    }
</script>

<script lang="ts">
    import BannerItem from '~/components/jet/item/BannerItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: BannerShelf;
</script>

<ShelfWrapper {shelf}>
    <div class="banner-items-container">
        {#each shelf.items as item}
            <BannerItem {item} />
        {/each}
    </div>
</ShelfWrapper>

<style>
    .banner-items-container {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }
</style>
