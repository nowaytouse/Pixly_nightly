<script lang="ts">
    import type { SearchResultsPage } from '@jet-app/app-store/api/models';

    import type { Size } from '@amp/web-app-components/src/types';
    import { ShelfConfig } from '@amp/web-app-components/config/components/shelf';

    import DefaultPage from './DefaultPage.svelte';
    import { getI18n } from '~/stores/i18n';
    import mediaQueries from '~/utils/media-queries';
    import {
        isSearchResultShelf,
        isRenderableInSearchResultsShelf,
    } from '~/components/jet/shelf/SearchResultShelf.svelte';
    import { getPlatformFromPage } from '~/utils/seo/common';

    export let page: SearchResultsPage;

    const i18n = getI18n();

    $: resultsShelf = page?.shelves?.find(isSearchResultShelf) ?? null;

    $: renderableItems = (resultsShelf?.items ?? []).filter(
        isRenderableInSearchResultsShelf,
    );

    $: columnConfig = ShelfConfig.get().GRID_VALUES.SearchResult;
    $: numberOfColumns = columnConfig[$mediaQueries as Size] || 3;
    $: numberOfRows = Math.ceil(renderableItems.length / numberOfColumns);
    $: middleRow = Math.floor(numberOfRows / 2);
    $: insertAt = middleRow * numberOfColumns;

    /**
     * This is unfortunate but only these three platforms support the transparency link.
     * This link is enabled via the `transparencyLawEditorialItemId` bag key, but when defining
     * bag keys, we do not have access to the platform being viewed, so we can't opt-out there.
     * We could do this platform check in the Jet layer, but adding two forms of opting into this
     * link felt cumbersome and unintuitive, so we can just do it here.
     */
    $: transparencyLink =
        page.transparencyLink &&
        ['iphone', 'ipad', 'mac'].includes(
            getPlatformFromPage(page).toLowerCase(),
        );

    /**
     * Here we are building constructing a new array of shelves _if_ there is a result shelf _and_
     * a transparency link. This creates three shelves:
     * 1) the search results before the transparency banner in the linkable text shelf
     * 2) the transparency banner
     * 3) the search results after the transparency banner
     */
    $: shelves = resultsShelf
        ? transparencyLink && renderableItems.length
            ? [
                  insertAt > 0 && {
                      ...resultsShelf,
                      items: renderableItems.slice(0, insertAt),
                      title: null,
                      isValid: () => true,
                  },
                  {
                      contentType: 'linkableText',
                      items: [page.transparencyLink],
                  },
                  {
                      ...resultsShelf,
                      items: renderableItems.slice(insertAt),
                      title: null,
                      isValid: () => true,
                  },
              ]
            : [{ ...resultsShelf, items: renderableItems, title: null }]
        : [];
</script>

<DefaultPage
    page={{
        shelves,
        title: renderableItems.length > 0 ? resultsShelf?.title : null,
    }}
>
    <svelte:fragment slot="before-shelves">
        {#if renderableItems.length === 0}
            <div>
                <h1>
                    {$i18n.t('ASE.Web.AppStore.Search.NoResults.FirstLine')}
                </h1>
                <p>
                    {$i18n.t('ASE.Web.AppStore.Search.NoResults.SecondLine', {
                        term: page.searchTermContext?.term,
                    })}
                </p>
            </div>
        {/if}
    </svelte:fragment>
</DefaultPage>

<style>
    div {
        display: flex;
        align-items: center;
        justify-content: center;
        flex-direction: column;
        gap: 3px;
        height: 70vh;
        margin: var(--bodyGutter);
    }

    p {
        font: var(--title-3);
        color: var(--systemSecondary);
    }
</style>
