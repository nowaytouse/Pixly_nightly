<script lang="ts" context="module">
    import type { Brick, Shelf } from '@jet-app/app-store/api/models';

    interface CategoryBrickShelf extends Shelf {
        items: Brick[];
    }

    export function isCategoryBrickShelf(
        shelf: Shelf,
    ): shelf is CategoryBrickShelf {
        const { contentType, items } = shelf;
        return contentType === 'categoryBrick' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import BrickItem from '~/components/jet/item/BrickItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: CategoryBrickShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="C" let:item>
        <BrickItem {item} shouldOverlayDescription />
    </ShelfItemLayout>
</ShelfWrapper>
