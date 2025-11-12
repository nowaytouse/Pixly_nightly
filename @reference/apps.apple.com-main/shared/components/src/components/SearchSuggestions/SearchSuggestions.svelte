<script lang="ts">
    import focusNode from '@amp/web-app-components/src/actions/focus-node';
    import { onMount, onDestroy } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    import { SEARCH_EVENTS } from '@amp/web-app-components/src/constants';
    import type { HighlightedSearchSuggestion } from '@amp/web-app-components/src/utils/processTextSearchSuggestion';
    import TextSearchSuggestion from '@amp/web-app-components/src/components/TextSearchSuggestion/TextSearchSuggestion.svelte';

    /**
     * The list of suggestions
     * @type {Array}
     */
    export let suggestions: Array<any> = [];

    /**
     * The current focused suggestion index
     * @type {number}
     */
    export let focusedSuggestionIndex: number | null = null;

    /**
     * The translate fn to be used to handle localization
     * @type {function}
     */
    export let translateFn: (
        str: string,
        values?: Record<string, string | number>,
    ) => string;

    const dispatch = createEventDispatcher();

    let searchSuggestionsScrimElement: HTMLDivElement;
    let domPortalElement: HTMLDivElement;

    onMount(() => {
        domPortalElement = document.createElement('div');
        domPortalElement.className = 'portal';
        domPortalElement.appendChild(searchSuggestionsScrimElement);

        // All onyx based apps use `.app-container` as top level of app elements.
        // For z-indexing to be correct we need to create portal at same level as app.
        const appTarget =
            document.querySelector('.app-container') ?? document.body;
        appTarget.appendChild(domPortalElement);

        // this is a cleanup task, same as 'onDestroy',
        // if for whatever reason the onMount becomes async
        // move this into an `onDestroy`
        return () => {
            if (domPortalElement) {
                appTarget.removeChild(domPortalElement);
            }
        };
    });

    function handleSuggestionClicked(suggestion: HighlightedSearchSuggestion) {
        dispatch(SEARCH_EVENTS.SUGGESTION_CLICKED, { suggestion });
    }

    function handleSuggestionKeyUp(
        suggestion: HighlightedSearchSuggestion,
        event: KeyboardEvent,
    ) {
        switch (event.key) {
            case 'Enter':
            case ' ': // Spacebar
                dispatch(SEARCH_EVENTS.SUGGESTION_CLICKED, { suggestion });
                break;
        }
    }

    function handleSuggestionFocused(
        suggestion: HighlightedSearchSuggestion,
        index: number,
    ) {
        dispatch(SEARCH_EVENTS.SUGGESTION_FOCUSED, { suggestion, index });
    }
</script>

<ul
    aria-label={translateFn('AMP.Shared.SearchInput.Suggestions')}
    role="listbox"
    data-testid="search-suggestions"
    id="search-suggestions"
    class="search-suggestions"
>
    {#each suggestions as suggestion, index}
        <!--
            Events using `self` modifier have this in order to filter out
            events that are directed to a child (i.e. pressing `Enter` or
            focusing on a context menu button).
         -->
        <li
            class="search-hint"
            class:search-hint--text={suggestion.kind === 'text'}
            class:search-hint--lockup={suggestion.kind !== 'text'}
            use:focusNode={focusedSuggestionIndex}
            data-index={index}
            data-testid={`suggestion-index-${index}`}
            role="option"
            tabindex="0"
            aria-selected={focusedSuggestionIndex === index ? true : undefined}
            id={`search-suggestion-${index}`}
            on:click={() => handleSuggestionClicked(suggestion)}
            on:keyup|self={(e) => handleSuggestionKeyUp(suggestion, e)}
            on:focusin|self={() => handleSuggestionFocused(suggestion, index)}
        >
            {#if $$slots['suggestion']}
                <slot name="suggestion" {suggestion} />
            {:else}
                <TextSearchSuggestion {suggestion} />
            {/if}
        </li>
    {/each}
</ul>

<div
    class="search-suggestions-scrim"
    data-testid="search-suggestions-scrim"
    bind:this={searchSuggestionsScrimElement}
/>

<style lang="scss">
    @use 'amp/stylekit/core/mixins/browser-targets' as *;
    @use 'amp/stylekit/core/mixins/materials' as *;
    @use '@amp/web-shared-styles/app/core/globalvars' as *;
    @use '@amp/web-shared-styles/app/core/mixins/absolute-center' as *;

    $search-hints-vertical-padding: 6px;

    @mixin search-hint-border {
        &::before {
            top: 0;
            inset-inline-start: var(--searchHintBorderStart, 6px);
            inset-inline-end: var(--searchHintBorderEnd, 6px);
            position: absolute;
            content: '';
            border-top: var(--keyline-border-style);

            @content;
        }
    }

    .search-suggestions {
        margin-top: 12px;

        @media (--sidebar-visible) {
            padding: $search-hints-vertical-padding 0;
            margin-top: 0;
            width: 302px;
            // Calculate the distance from the top of the window so we can get the height right to allow it to scroll within the page
            // with exactly 25px (our $-web-navigation-inline-padding sizing).
            // 3px is the distance difference in the spec from the calculations we have here.
            max-height: calc(
                100vh - #{$global-player-bar-height} - #{$web-search-input-height} -
                    #{$web-navigation-inline-padding} + 3px
            );
            position: absolute;
            top: 36px;
            border-radius: 9px;
            overflow-x: hidden;
            overflow-y: auto;
            border: $dialog-border;
            box-shadow: $dialog-inset-shadow, $dialog-shadow;
            text-align: start;
            z-index: calc(var(--z-contextual-menus) + 2);

            @include system-standard-thick-material;

            li:not(.search-hint--text) {
                &:focus-visible {
                    outline: none; // Hide default focus ring as background color serves as focus state
                }
            }
        }
    }

    @include target-safari {
        // Safari Safari 14.1 fails to render contents of `search-hint--text`, with `background-filter`, when content does not overflow
        // `search-hint--text` container. `1px` of extra negative `margin-bottom` and `padding-bottom` on last element, helps trigger overflow.
        // This issue is not reproducible in Safari 14.2.
        li:last-child {
            margin-bottom: -$search-hints-vertical-padding - 1;
            padding-bottom: $search-hints-vertical-padding + 1;
        }
    }

    .search-hint {
        position: relative;
        border-radius: var(
            --global-border-radius-xsmall,
            #{$global-border-radius-xsmall}
        );
        z-index: var(--z-default);

        // Hover/focus styles for desktop only
        @media (--sidebar-visible) {
            &:hover,
            &:focus-visible,
            &:focus-within {
                // Ensure favorited badge is visible when focused
                --favoriteBadgeColor: white;
                background-color: var(--keyColor);
                outline: none; // Hide default focus ring as background color serves as focus state

                :global(svg) {
                    fill: white;
                }

                // Applies to all text in child <span> tags -- works for text and lockup suggestions
                :global(span) {
                    color: white;
                }
            }
        }
    }

    .search-hint--lockup {
        @include search-hint-border;

        @media (--range-sidebar-hidden-down) {
            --searchHintBorderStart: var(
                --searchHintBorderStartOverride,
                68px
            ); // Border starts after artwork. This is overridden using `:has` in child
            --searchHintBorderEnd: calc(-1 * var(--bodyGutter));

            // Show full divider before first child, and between text and lockup hints
            &:first-child,
            .search-hint--text + & {
                --searchHintBorderStart: 0;
            }
        }

        @media (--sidebar-visible) {
            $top-search-list-gutter: 6px;
            width: calc(100% - #{$top-search-list-gutter * 2});
            margin-inline-start: $top-search-list-gutter;
            margin-inline-end: $top-search-list-gutter;

            // Hide border on currently hovered/focused item
            &:hover,
            &:focus-visible,
            &:focus-within {
                &::before {
                    border-color: transparent;
                }
            }

            // Hide border on item directly after currently hovered/focused item
            &:hover + &,
            &:focus-visible + &,
            &:focus-within + & {
                &::before {
                    border-color: transparent;
                }
            }
        }
    }

    .search-hint--text {
        align-items: center;
        display: grid;
        grid-template-columns: 20px auto;

        // Add borders between text search hints on sidebar hidden
        @media (--range-sidebar-hidden-down) {
            --searchHintBorderStart: 26px; // Border starts after search icon
            --searchHintBorderEnd: calc(-1 * var(--bodyGutter));
            padding-block: 15px;

            @include search-hint-border;

            &:first-child {
                --searchHintBorderStart: 0;
            }
        }

        @media (--sidebar-visible) {
            grid-template-columns: 16px auto;
            margin: 0 6px;
            padding: 4px;
            font: var(--body);

            &:focus-within {
                background-color: var(--keyColor);
                outline: none; // Hide default focus ring as background color serves as focus state

                :global(.search-suggestion-svg) {
                    fill: white;
                }

                :global(span) {
                    color: white;
                }
            }
        }

        :global(.search-suggestion-svg) {
            justify-self: center;
            align-self: start;
            width: 16px;
            height: 16px;
            transform: translateY(4px);

            @media (--sidebar-visible) {
                width: 11px;
                height: 11px;
                transform: translateY(2.5px);
            }
        }

        + .search-hint--lockup {
            @media (--sidebar-visible) {
                margin-top: 6px; // Add small margin between '.search-hint--text' and '.search-hint--lockup' on larger viewports per spec
            }
        }
    }

    .search-suggestions-scrim {
        @include absolute-center;

        @media (--range-sidebar-hidden-down) {
            display: none;
        }

        @media (--sidebar-visible) {
            z-index: calc(var(--z-default) + 1);
        }
    }
</style>
