<script lang="ts" context="module">
    import type { Brick, Shelf } from '@jet-app/app-store/api/models';

    interface LargeBrickShelf extends Shelf {
        items: Brick[];
    }

    export function isLargeBrickShelf(shelf: Shelf): shelf is LargeBrickShelf {
        const { contentType, items } = shelf;
        return contentType === 'largeBrick' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import LargeBrickItem from '~/components/jet/item/LargeBrickItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: LargeBrickShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="LargeBrick" let:item>
        <LargeBrickItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
