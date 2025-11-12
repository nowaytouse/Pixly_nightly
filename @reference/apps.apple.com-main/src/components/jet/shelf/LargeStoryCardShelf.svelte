<script lang="ts" context="module">
    import type { Shelf, TodayCard } from '@jet-app/app-store/api/models';

    interface LargeStoryCardShelf extends Shelf {
        items: TodayCard[];
    }

    export function isLargeStoryCardShelf(
        shelf: Shelf,
    ): shelf is LargeStoryCardShelf {
        return (
            shelf.contentType === 'largeStoryCard' && Array.isArray(shelf.items)
        );
    }
</script>

<script lang="ts">
    import HeroCarousel from '~/components/hero/Carousel.svelte';
    import LargeStoryCardItem from '~/components/jet/item/LargeStoryCardItem.svelte';

    export let shelf: LargeStoryCardShelf;

    $: items = shelf.items;

    function deriveBackgroundArtworkFromItem(item: TodayCard) {
        return item.heroMedia?.artworks[0];
    }
</script>

<HeroCarousel {shelf} {items} {deriveBackgroundArtworkFromItem} let:item>
    <LargeStoryCardItem {item} />
</HeroCarousel>
