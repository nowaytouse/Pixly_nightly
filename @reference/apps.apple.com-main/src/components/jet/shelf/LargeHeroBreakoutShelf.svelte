<script lang="ts" context="module">
    import type {
        LargeHeroBreakout,
        Shelf,
    } from '@jet-app/app-store/api/models';

    interface LargeHeroBreakoutShelf extends Shelf {
        items: LargeHeroBreakout[];
    }

    export function isLargeHeroBreakoutShelf(
        shelf: Shelf,
    ): shelf is LargeHeroBreakoutShelf {
        const { contentType, items } = shelf;
        return contentType === 'largeHeroBreakout' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import LargeHeroBreakoutItem from '~/components/jet/item/LargeHeroBreakoutItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: LargeHeroBreakoutShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="Spotlight" let:item>
        <LargeHeroBreakoutItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
