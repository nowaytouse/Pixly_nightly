<script lang="ts" context="module">
    import type { Shelf, EditorialCard } from '@jet-app/app-store/api/models';

    interface EditorialCardShelf extends Shelf {
        items: EditorialCard[];
    }

    export function isEditorialCardShelf(
        shelf: Shelf,
    ): shelf is EditorialCardShelf {
        const { contentType, items } = shelf;

        return contentType === 'editorialCard' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import HeroCarousel from '~/components/hero/Carousel.svelte';
    import EditorialCardItem from '~/components/jet/item/EditorialCardItem.svelte';

    export let shelf: EditorialCardShelf;

    $: items = shelf.items;

    function deriveBackgroundArtworkFromItem(item: EditorialCard) {
        return item.artwork;
    }
</script>

<HeroCarousel {shelf} {items} {deriveBackgroundArtworkFromItem} let:item>
    <EditorialCardItem {item} />
</HeroCarousel>
