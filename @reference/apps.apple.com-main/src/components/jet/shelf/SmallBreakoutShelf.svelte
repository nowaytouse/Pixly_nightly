<script lang="ts" context="module">
    import type {
        LargeHeroBreakout,
        Shelf,
        SmallBreakout,
    } from '@jet-app/app-store/api/models';

    interface SmallBreakoutShelf extends Shelf {
        items: SmallBreakout[];
    }

    export function isSmallBreakoutShelf(
        shelf: Shelf,
    ): shelf is SmallBreakoutShelf {
        const { contentType, items } = shelf;
        return contentType === 'smallBreakout' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import SmallBreakoutItem from '~/components/jet/item/SmallBreakoutItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: SmallBreakoutShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="Spotlight" let:item>
        <SmallBreakoutItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
