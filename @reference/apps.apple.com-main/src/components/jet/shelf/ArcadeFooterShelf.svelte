<script lang="ts" context="module">
    import type { Shelf, ArcadeFooter } from '@jet-app/app-store/api/models';

    interface ArcadeFooterShelf extends Shelf {
        items: [ArcadeFooter];
    }

    export function isArcadeFooterShelf(
        shelf: Shelf,
    ): shelf is ArcadeFooterShelf {
        const { contentType, items } = shelf;

        return contentType === 'arcadeFooter' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ArcadeFooterItem from '~/components/jet/item/ArcadeFooterItem.svelte';
    import HorizontalShelf from '~/components/jet/shelf/HorizontalShelf.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: ArcadeFooterShelf;

    $: gridRows = shelf.rowsPerColumn ?? undefined;
    $: items = shelf.items;
</script>

<ShelfWrapper {shelf} withBottomPadding={false}>
    <HorizontalShelf {gridRows} gridType="Spotlight" {items} let:item>
        <ArcadeFooterItem {item} />
    </HorizontalShelf>
</ShelfWrapper>
