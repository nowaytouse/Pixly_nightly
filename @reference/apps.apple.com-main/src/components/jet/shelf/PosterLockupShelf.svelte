<script lang="ts" context="module">
    import type { Shelf, PosterLockup } from '@jet-app/app-store/api/models';

    interface PosterLockupShelf extends Shelf {
        items: PosterLockup[];
    }

    export function isPosterLockupShelf(
        shelf: Shelf,
    ): shelf is PosterLockupShelf {
        const { contentType, items } = shelf;
        return contentType === 'posterLockup' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import mediaQueries from '~/utils/media-queries';
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import PosterLockupItem from '~/components/jet/item/PosterLockupItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: PosterLockupShelf;

    $: gridType = $mediaQueries === 'xsmall' ? 'Spotlight' : 'PosterLockup';
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} {gridType} let:item>
        <PosterLockupItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
