<script lang="ts" context="module">
    import type { Lockup, Shelf } from '@jet-app/app-store/api/models';

    interface SmallLockupShelf extends Shelf {
        items: Lockup[];
    }

    export function isSmallLockupShelf(
        shelf: Shelf,
    ): shelf is SmallLockupShelf {
        const { contentType, items } = shelf;
        return contentType === 'smallLockup' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import SmallLockupItem from '~/components/jet/item/SmallLockupItem.svelte';
    import SmallLockupWithOrdinalItem, {
        isSmallLockupWithOrdinalItem,
    } from '~/components/jet/item/SmallLockupWithOrdinalItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: SmallLockupShelf;

    $: ({ isArticleContext = false } = shelf.presentationHints ?? {});
    $: itemHasOrdinal = shelf.items.some((item) => item.ordinal);
    $: gridType = (() => {
        if (itemHasOrdinal) {
            return 'SmallLockupWithOrdinal';
        }

        if (isArticleContext) {
            return 'Spotlight';
        }

        return 'SmallLockup';
    })();
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout
        {shelf}
        {gridType}
        rowsPerColumnOverride={gridType === 'SmallLockup' ? 3 : null}
        let:item
    >
        {#if isSmallLockupWithOrdinalItem(item)}
            <SmallLockupWithOrdinalItem {item} />
        {:else}
            <SmallLockupItem {item} --margin-inline-end="16px" />
        {/if}
    </ShelfItemLayout>
</ShelfWrapper>
