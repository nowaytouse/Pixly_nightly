<script lang="ts">
    import SearchIcon from '@amp/web-app-components/assets/icons/search.svg';
    import type { HighlightedSearchSuggestion } from '../../utils/processTextSearchSuggestion';

    export let suggestion: HighlightedSearchSuggestion;
    $: autofillBefore = suggestion.autofillBefore;
    $: highlighted = suggestion.highlighted;
    $: autofillAfter = suggestion.autofillAfter;
</script>

<SearchIcon class="search-suggestion-svg" aria-hidden="true" />
<span class="suggestion">
    <!--
        These spans cannot be broken down onto separate lines until Svelte
        supports trimming of whitespace on-demand: https://github.com/sveltejs/svelte/issues/189
        TODO: rdar://101681389 (Onxy: Remove whitespace trimming workarounds)
    -->

    <!-- prettier-ignore -->
    <span data-testid="suggestion-autofill-before">{autofillBefore}</span><span
        class="highlighted"
        data-testid="suggestion-autofill-highlighted">{highlighted}</span
    ><span data-testid="suggestion-autofill-after">{autofillAfter}</span>
</span>

<style lang="scss">
    @use 'amp/stylekit/core/mixins/line-clamp' as *;

    .suggestion {
        color: var(--systemSecondary);
        margin: 0 6px;
        font: var(--title-2);

        @include line-clamp(var(--searchSuggestionClampedLines, 1));

        @media (--sidebar-visible) {
            font: var(--callout);
        }
    }

    .highlighted {
        color: var(--systemPrimary);
    }
</style>
