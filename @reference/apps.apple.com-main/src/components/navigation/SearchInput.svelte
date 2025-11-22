<script lang="ts" context="module">
    import type { ComponentProps } from 'svelte';
    import { writable } from 'svelte/store';

    import SearchInput from '@amp/web-app-components/src/components/SearchInput/SearchInput.svelte';

    type UnusedSearchInputProps = Pick<
        ComponentProps<SearchInput>,
        'currentTab' | 'menuItem'
    >;

    // `SearchInput` requires a bunch of properties that are unnecessary
    // for our use-case; they're defined here to keep them grouped together
    const UNNEEDED_SEARCH_INPUT_PROPS: UnusedSearchInputProps = {
        currentTab: writable(null),
        menuItem: {
            id: { type: 'search' },
            // @ts-expect-error the `menuItem` is not relevant to us; we don't
            // need to provide an icon for this
            icon: null,
        },
    };
</script>

<script lang="ts">
    import type { WebSearchFlowAction } from '@jet-app/app-store/common/search/web-search-action';
    import { makeCanonicalSearchResultsPageUrl } from '@jet-app/app-store/common/search/search-page-url';

    import { getJet } from '~/jet';
    import { getI18n } from '~/stores/i18n';

    const i18n = getI18n();
    const jet = getJet();

    export let searchAction: WebSearchFlowAction;
    export let big: boolean = false;

    function dispatchSearchAction(event: CustomEvent<{ term: string }>) {
        const { term } = event.detail;

        searchAction.destination.term = term;

        searchAction.pageUrl = makeCanonicalSearchResultsPageUrl(
            jet.objectGraph,
            searchAction.destination,
        );

        jet.perform(searchAction);
    }
</script>

<div class="search-input-wrapper" class:big>
    <SearchInput
        {...UNNEEDED_SEARCH_INPUT_PROPS}
        defaultValue={searchAction?.destination?.term}
        translateFn={(key) => $i18n.t(key)}
        on:makeSearchQueryFromInput={dispatchSearchAction}
    />
</div>

<style>
    .search-input-wrapper {
        --searchBoxIconFill: var(--keyColor);
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    .search-input-wrapper.big :global(.search-input__text-field) {
        height: 48px;
        padding-inline-start: 40px;
        font: var(--title-2);
        border-radius: 8px;
    }

    .search-input-wrapper.big :global(.search-svg) {
        width: 16px;
        height: auto;
        inset: 16px 0 0 13px;
    }
</style>
