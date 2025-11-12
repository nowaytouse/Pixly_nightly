<script lang="ts" context="module">
    import type { Shelf, ImageLockup } from '@jet-app/app-store/api/models';

    interface LargeImageLockupShelf extends Shelf {
        items: ImageLockup[];
    }

    export function isLargeImageLockupShelf(
        shelf: Shelf,
    ): shelf is LargeImageLockupShelf {
        return (
            shelf.contentType === 'largeImageLockup' &&
            Array.isArray(shelf.items)
        );
    }
</script>

<script lang="ts">
    import LargeImageLockupItem from '~/components/jet/item/LargeImageLockupItem.svelte';
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: LargeImageLockupShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="A" let:item>
        <LargeImageLockupItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
