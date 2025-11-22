<script lang="ts" context="module">
    import type {
        Shelf,
        HeroCarousel as HeroCarouselModel,
        HeroCarouselItem as HeroCarouselItemModel,
    } from '@jet-app/app-store/api/models';

    interface HeroCarouselShelf extends Shelf {
        items: [HeroCarouselModel];
    }

    export function isHeroCarouselShelf(
        shelf: Shelf,
    ): shelf is HeroCarouselShelf {
        const { contentType, items } = shelf;

        return contentType === 'heroCarousel' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import HeroCarousel from '~/components/hero/Carousel.svelte';
    import HeroCarouselItem from '~/components/jet/item/HeroCarouselItem.svelte';
    import { isRtl } from '~/utils/locale';

    export let shelf: HeroCarouselShelf;

    $: ({ items: ltrItems, rtlItems } = shelf.items[0]);
    $: items = isRtl() && rtlItems.length ? rtlItems : ltrItems;

    function deriveBackgroundArtworkFromItem(item: HeroCarouselItemModel) {
        return item.artwork || item.video?.preview;
    }
</script>

<HeroCarousel {shelf} {items} {deriveBackgroundArtworkFromItem} let:item>
    <HeroCarouselItem {item} />
</HeroCarousel>
