<script lang="ts" context="module">
    import type {
        AppSearchResult,
        Shelf,
        SearchResult,
        AppEventSearchResult,
    } from '@jet-app/app-store/api/models';

    import AppSearchResultItem, {
        isAppSearchResult,
        isAppEventSearchResult,
    } from '~/components/jet/item/SearchResult/AppSearchResultItem.svelte';

    /**
     * All sub-classes of {@linkcode SearchResult} that this component can handle rendering
     */
    type RenderableSearchResult = AppSearchResult | AppEventSearchResult;

    interface SearchResultShelf extends Shelf {
        items: SearchResult[];
    }

    export function isSearchResultShelf(
        shelf: Shelf,
    ): shelf is SearchResultShelf {
        return (
            shelf.contentType === 'searchResult' && Array.isArray(shelf.items)
        );
    }

    export function isRenderableInSearchResultsShelf(
        item: SearchResult,
    ): item is RenderableSearchResult {
        return isAppSearchResult(item) || isAppEventSearchResult(item);
    }
</script>

<script lang="ts">
    import Grid from '~/components/Grid.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: SearchResultShelf;
</script>

<ShelfWrapper {shelf}>
    <Grid gridType="SearchResult" items={shelf.items} let:item>
        <AppSearchResultItem {item} />
    </Grid>
</ShelfWrapper>
