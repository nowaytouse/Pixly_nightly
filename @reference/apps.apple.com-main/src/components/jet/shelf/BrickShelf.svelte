<script lang="ts" context="module">
    import type { Brick, Shelf } from '@jet-app/app-store/api/models';

    interface BrickShelf extends Shelf {
        items: Brick[];
    }

    export function isBrickShelf(shelf: Shelf): shelf is BrickShelf {
        const { contentType, items } = shelf;
        return contentType === 'brick' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import BrickItem from '~/components/jet/item/BrickItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: BrickShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout
        {shelf}
        gridTypeForShelf="Brick"
        gridTypeForGrid="F"
        let:item
    >
        <BrickItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
