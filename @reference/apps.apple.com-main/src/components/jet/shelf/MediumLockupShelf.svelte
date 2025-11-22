<script lang="ts" context="module">
    import type { Lockup, Shelf } from '@jet-app/app-store/api/models';

    interface MediumLockupShelf extends Shelf {
        items: Lockup[];
    }

    export function isMediumLockupShelf(
        shelf: Shelf,
    ): shelf is MediumLockupShelf {
        const { contentType, items } = shelf;
        return contentType === 'mediumLockup' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import MediumLockupItem from '~/components/jet/item/MediumLockupItem.svelte';
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: MediumLockupShelf;

    $: isArticleContext = shelf.presentationHints?.isArticleContext;
    $: gridType = isArticleContext ? 'Spotlight' : 'MediumLockup';
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} {gridType} rowsPerColumnOverride={2} let:item>
        <MediumLockupItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
