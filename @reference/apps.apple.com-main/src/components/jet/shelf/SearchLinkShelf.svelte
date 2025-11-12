<script lang="ts" context="module">
    import type { Shelf, SearchLink } from '@jet-app/app-store/api/models';

    interface SearchLinkShelf extends Shelf {
        items: SearchLink[];
    }

    export function isSearchLinkShelf(shelf: Shelf): shelf is SearchLinkShelf {
        const { contentType, items } = shelf;
        return contentType === 'searchLink' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import SearchLinkItem from '~/components/jet/item/SearchLinkItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: SearchLinkShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="SearchLink" let:item>
        <SearchLinkItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
