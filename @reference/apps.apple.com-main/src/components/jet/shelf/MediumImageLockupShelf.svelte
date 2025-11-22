<script lang="ts" context="module">
    import type { ImageLockup, Shelf } from '@jet-app/app-store/api/models';

    interface MediumImageLockupShelf extends Shelf {
        items: ImageLockup[];
    }

    export function isMediumImageLockupShelf(
        shelf: Shelf,
    ): shelf is MediumImageLockupShelf {
        const { contentType, items } = shelf;
        return contentType === 'mediumImageLockup' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import MediumImageLockupItem from '~/components/jet/item/MediumImageLockupItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: MediumImageLockupShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="B" let:item>
        <MediumImageLockupItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
