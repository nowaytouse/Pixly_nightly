<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import type { Writable } from 'svelte/store';
    import type { NavigationId } from '@amp/web-app-components/src/types';
    import clickOutside from '@amp/web-app-components/src/actions/click-outside';
    import SearchSuggestions from '@amp/web-app-components/src/components/SearchSuggestions/SearchSuggestions.svelte';
    import type { NavigationItem } from '@amp/web-app-components/src/components/Navigation/types';
    import {
        ClearEventLocation,
        SEARCH_EVENTS,
    } from '@amp/web-app-components/src/constants';
    import { getUpdatedFocusedIndex } from '@amp/web-app-components/src/utils/getUpdatedFocusedIndex';
    import { debounce } from '@amp/web-app-components/src/utils/debounce';
    import type {
        HighlightedSearchSuggestion,
        SearchSuggestion,
    } from '@amp/web-app-components/src/utils/processTextSearchSuggestion';
    import SearchIcon from '@amp/web-app-components/assets/icons/search.svg';

    const {
        SEARCH_INPUT_HAS_FOCUS,
        MAKE_SEARCH_QUERY_FROM_SUGGESTION,
        MAKE_SEARCH_QUERY_FROM_INPUT,
        CLICKED_OUTSIDE_SUGGESTIONS,
        CLICKED_OUTSIDE,
        RESET_SEARCH_INPUT,
        MENU_ITEM_CLICK,
        SHOW_SEARCH_SUGGESTIONS,
    } = SEARCH_EVENTS;

    $: debouncedHandleSearchInput = debounce(handleSearchInput, 100);

    /**
     * The translate fn to be used to handle localization
     * @type {function}
     */
    export let translateFn: (key: string) => string;

    /**
     * The handler to be executed that retrieves suggestions for a given term
     * @type {function}
     */
    export let getSuggestionsForPartialTerm: (
        partialTerm: string,
    ) => Promise<SearchSuggestion[]> = async () => [];

    /**
     * The store containing the currently selected tab.
     */
    export let currentTab: Writable<NavigationId | null>;

    /**
     * The pre-filled value of the text field
     */
    export let defaultValue: string | null = null;

    /**
     * The menu item that should be selected when a search is performed or the
     * search field receives focus while not on this item.
     */
    export let menuItem: NavigationItem;

    /**
     * Optional argument to disable search suggestions completely
     */
    export let hideSuggestions = false;

    let suggestions = [];
    let cachedSuggestions = [];
    let partialTerm = !!defaultValue ? defaultValue : '';
    let focusedSearchSuggestionIndex = null;
    let searchInputElement: HTMLInputElement;
    let showSuggestion = false;
    let showCancelButton = false;

    $: showSuggestion = suggestions?.length > 0;
    $: handleShowSuggestion(showSuggestion);

    const dispatch = createEventDispatcher<{
        resetSearchInput: null; // no details returned
        menuItemClick: NavigationItem;
        searchInputHasFocus: null; // no details returned
        makeSearchQueryFromInput: { term: string };
        // Unfortunately SearchSuggestions uses Array<any> so no way to fully type this.
        // rdar://137049269 ((Shared/Components) Create Types for SearchSuggestions component)
        makeSearchQueryFromSuggestion: { suggestion: any };
        clickedOutsideSuggestions: null; // no details returned
        clickedOutside: null; // no details returned
        clear: { from: ClearEventLocation };
        showSearchSuggestions: { showSearchSuggestions: boolean };
    }>();

    function resetSearchInputState() {
        searchInputElement.value = '';
        partialTerm = '';
        suggestions = [];
        cachedSuggestions = [];
        focusedSearchSuggestionIndex = null;
        dispatch(RESET_SEARCH_INPUT);
    }

    /**
     * We use a click focus here (instead of input focus) as a
     * lighter touch way to detect interaction with the search input.
     *
     * See additional explanation here:
     *   rdar://83511986 (JMOTW AX Music: Focussing on Search Field should not trigger a Context Change in Routing)
     */
    function handleSearchInputClickFocus() {
        showCancelButton = true;
        const currentTerm = searchInputElement.value;
        if (currentTerm === partialTerm && cachedSuggestions.length > 0) {
            suggestions = cachedSuggestions;
            cachedSuggestions = [];
        }

        // Only switch to the search tab if we aren't already on it
        if ($currentTab !== menuItem.id) {
            currentTab.set(menuItem.id);
            dispatch(MENU_ITEM_CLICK, menuItem);
        }

        dispatch(SEARCH_INPUT_HAS_FOCUS);
    }

    function handleSearchInputSubmit(event: SubmitEvent) {
        const term = searchInputElement.value;
        event.preventDefault();

        if (term) {
            dispatch(MAKE_SEARCH_QUERY_FROM_INPUT, {
                term,
            });

            // Submitting a search always goes to the search tab
            currentTab.set(menuItem.id);

            // Cache the current list of suggestions in case searchInputElement
            // becomes focused again.
            cachedSuggestions = suggestions;
            suggestions = [];
            focusedSearchSuggestionIndex = null;

            // Also hides the suggestions if visible
            searchInputElement.blur();
        }
    }

    function onSearchSuggestionChosen(suggestion: HighlightedSearchSuggestion) {
        dispatch(MAKE_SEARCH_QUERY_FROM_SUGGESTION, { suggestion });

        // Clicking on a search suggestion always goes to the search tab
        currentTab.set(menuItem.id);

        resetSearchInputState();
        searchInputElement.value = suggestion.displayTerm;
    }

    function onSearchSuggestionFocused(index: number) {
        focusedSearchSuggestionIndex = index;
    }

    function containerHandleKeyDown(event: KeyboardEvent) {
        switch (event.key) {
            case 'ArrowDown':
            case 'ArrowUp':
                event.preventDefault();
                break;
        }
    }

    function containerHandleKeyUp(event: KeyboardEvent) {
        switch (event.key) {
            case 'ArrowDown':
                focusedSearchSuggestionIndex = getUpdatedFocusedIndex(
                    1,
                    focusedSearchSuggestionIndex,
                    suggestions.length,
                );
                break;

            case 'ArrowUp':
                focusedSearchSuggestionIndex = getUpdatedFocusedIndex(
                    -1,
                    focusedSearchSuggestionIndex,
                    suggestions.length,
                );
                break;

            case 'Escape':
                resetSearchInputState();
                break;

            case 'Tab':
            case 'Control':
            case 'Alt':
            case 'Meta':
            case 'Shift':
            case ' ': // Spacebar
                // Don't do anything for remaining navigation keys.
                break;

            default:
                // If this event is not a navigational key, or not a Tab the focus is returned to the input
                // allowing the user to type with the this key stroke. This is necesasry because
                // VoiceOver first lands on the container and not on the input field.
                searchInputElement.focus();
        }

        event.preventDefault();
    }

    async function handleSearchInput(input: HTMLInputElement) {
        const searchInput = input ?? searchInputElement;
        partialTerm = searchInput.value;

        if (!partialTerm) {
            suggestions = [];
            return;
        }

        let _suggestions = await getSuggestionsForPartialTerm(partialTerm);
        cachedSuggestions = _suggestions;

        // rdar://93009223 (JMOTW: Hitting enter in search field before suggestions loads leaves suggestions stuck)
        //
        // We only want to show suggestions here if the input is focused.
        // Without this condition, suggestions will show up after enter is pressed if
        // it takes too long for the api to return
        if (document.activeElement === searchInput) {
            suggestions = _suggestions;
            cachedSuggestions = [];
        }
    }

    /**
     * We don't want `menuItemClick` to also get debounced
     * Extrapolating logic here to handle the route switch as well as the input delay
     *
     * rdar://83511986 (AX Music: Focussing on Search Field should not trigger a Context Change in Routing)
     *
     * TODO: we currently have no way to re-render the search landing page if the currently selected tab
     * is already on the search tab. The best solution (as of now) to re-render the search landing page
     * is to check if the input value is empty.
     *
     * rdar://91073241 (JMOTW: Search - Find a way to stop re-renders of search landing page)
     */
    function handleSearchInputActivity(e: Event) {
        if (
            !(e instanceof InputEvent) &&
            (e.target as HTMLInputElement).value === ''
        ) {
            dispatch('clear', { from: ClearEventLocation.Input });
        }
        const shouldDispatchMenuClick =
            $currentTab !== menuItem.id || searchInputElement.value === '';

        // From svelte docs:
        // The store value gets set to the value of the argument if
        // the store value is not already equal to it.
        // https://svelte.dev/docs#run-time-svelte-store-writable
        currentTab.set(menuItem.id);

        if (shouldDispatchMenuClick) {
            menuItem.opaqueData = () => ({ from: 'searchInputClear' });
            dispatch(MENU_ITEM_CLICK, menuItem);
        }

        debouncedHandleSearchInput(e.target as HTMLInputElement);
    }

    function handleClickOutside(event: Event) {
        const element = (event.target as HTMLElement) || null;

        const eventPath = event.composedPath ? event.composedPath() : [];
        const didEventHappenInContextMenu = eventPath.some(
            (item) =>
                'nodeName' in item && item.nodeName === 'AMP-CONTEXTUAL-MENU',
        );

        // dont close menu if interacting with context menu
        if (
            (element && element.nodeName === 'AMP-CONTEXTUAL-MENU') ||
            didEventHappenInContextMenu
        ) {
            return;
        }

        if (suggestions.length > 0) {
            // Cache the current list of suggestions in case searchInputElement
            // becomes focused again.
            cachedSuggestions = suggestions;

            // Clear out the suggestions so the suggestions disappear
            suggestions = [];

            dispatch(CLICKED_OUTSIDE_SUGGESTIONS);
        }

        showCancelButton = false;
        dispatch(CLICKED_OUTSIDE);
    }

    function handleShowSuggestion(curShowSuggestions: boolean) {
        dispatch(SHOW_SEARCH_SUGGESTIONS, {
            showSearchSuggestions: curShowSuggestions,
        });
    }

    function handleCancelButton() {
        showCancelButton = false;
        searchInputElement.value = '';
        dispatch('clear', { from: ClearEventLocation.Cancel });
    }
</script>

<div
    data-testid="amp-search-input"
    aria-controls="search-suggestions"
    aria-expanded={suggestions && suggestions.length > 0}
    aria-haspopup="listbox"
    aria-owns="search-suggestions"
    class="search-input-container"
    tabindex="-1"
    role={showSuggestion ? 'combobox' : ''}
    use:clickOutside={handleClickOutside}
    on:keydown={containerHandleKeyDown}
    on:keyup={containerHandleKeyUp}
>
    <div class="flex-container">
        <form
            role="search"
            id="search-input-form"
            on:submit={handleSearchInputSubmit}
        >
            <SearchIcon class="search-svg" aria-hidden="true" />

            <input
                value={defaultValue}
                aria-activedescendant={Number.isInteger(
                    focusedSearchSuggestionIndex,
                ) && focusedSearchSuggestionIndex >= 0
                    ? `search-suggestion-${focusedSearchSuggestionIndex}`
                    : undefined}
                aria-autocomplete="list"
                aria-multiline="false"
                aria-controls="search-suggestions"
                placeholder={translateFn('AMP.Shared.SearchInput.Placeholder')}
                spellcheck={false}
                autocomplete="off"
                autocorrect="off"
                autocapitalize="off"
                type="search"
                class="search-input__text-field"
                bind:this={searchInputElement}
                data-testid="search-input__text-field"
                on:input={handleSearchInputActivity}
                on:click={handleSearchInputClickFocus}
            />
        </form>

        {#if showCancelButton}
            <div
                class="search-input__cancel-button-container"
                data-testid="search-input__cancel-button-container"
            >
                <button
                    data-testid="search-input__cancel-button"
                    on:click={handleCancelButton}
                    aria-label={translateFn('FUSE.Search.Cancel')}
                >
                    {translateFn('FUSE.Search.Cancel')}
                </button>
            </div>
        {/if}
    </div>

    <div data-testid="search-scope-bar"><slot name="searchScopeBar" /></div>

    <!-- https://github.com/sveltejs/svelte/issues/5604 -->
    {#if !hideSuggestions && suggestions && suggestions.length > 0}
        {#if $$slots['suggestion']}
            <SearchSuggestions
                on:suggestionClicked={(e) =>
                    onSearchSuggestionChosen(e.detail.suggestion)}
                on:suggestionFocused={(e) =>
                    onSearchSuggestionFocused(e.detail.index)}
                {suggestions}
                focusedSuggestionIndex={focusedSearchSuggestionIndex}
                {translateFn}
            >
                <svelte:fragment slot="suggestion" let:suggestion>
                    <slot name="suggestion" {suggestion} />
                </svelte:fragment>
            </SearchSuggestions>
        {:else}
            <SearchSuggestions
                on:suggestionClicked={(e) =>
                    onSearchSuggestionChosen(e.detail.suggestion)}
                on:suggestionFocused={(e) =>
                    onSearchSuggestionFocused(e.detail.index)}
                {suggestions}
                focusedSuggestionIndex={focusedSearchSuggestionIndex}
                {translateFn}
            />
        {/if}
    {/if}
</div>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use '@amp/web-shared-styles/app/core/mixins/focus' as *;

    $search-input-text-height: 32px;
    $search-svg-size-hide-sidebar: 12px;

    .search-input-container {
        @media (--sidebar-visible) {
            position: relative;
            z-index: var(--z-default);
        }

        @media (--range-sidebar-hidden-down) {
            width: 100%;
        }

        :global(.search-svg) {
            width: 16px;
            height: 16px;
            top: 10px;
            bottom: 10px;
            position: absolute;
            fill: var(--searchBoxIconFill);
            inset-inline-start: 10px;
            z-index: var(--z-default);

            @media (--sidebar-visible) {
                width: $search-svg-size-hide-sidebar;
                height: $search-svg-size-hide-sidebar;
            }
        }

        :global(.search-suggestion-svg) {
            fill: var(--searchBoxIconFill);
        }
    }

    .search-input__text-field {
        background-color: var(--pageBG);
        border-radius: 4px;
        border-style: solid;
        border-width: 1px;
        border-color: var(--searchBarBorderColor);
        color: var(--systemPrimary-vibrant);
        font-size: 12px;
        font-weight: 400;
        height: $search-input-text-height;
        letter-spacing: 0;
        line-height: 1.25;
        padding-top: 6px;
        padding-bottom: 5px;
        width: 100%;
        padding-inline-end: 5px;

        @media (--range-sidebar-hidden-down) {
            height: 38px;
            border-radius: 9px;
            padding-inline-start: 34px;
            font: var(--title-3-tall);
            font-size: 16px;
        }

        @media (--sidebar-visible) {
            padding-inline-start: 28px;
        }
    }

    input::-webkit-search-decoration,
    input::-webkit-search-results-decoration {
        appearance: none;
    }

    input::placeholder {
        color: var(--systemTertiary-vibrant);

        @media (prefers-color-scheme: dark) {
            color: var(--systemSecondary-vibrant);
        }
    }

    input:focus {
        @include focus-shadow;
    }

    input::-webkit-search-cancel-button {
        $cancelButtonSize: 14px;
        appearance: none;
        background-position: center;
        background-repeat: no-repeat;
        background-size: $cancelButtonSize $cancelButtonSize;
        height: $cancelButtonSize;
        width: $cancelButtonSize;
        background-image: url('/assets/icons/sidebar-searchfield-close-on-light.svg');

        @media (prefers-color-scheme: dark) {
            background-image: url('/assets/icons/sidebar-searchfield-close-on-dark.svg');
        }
    }

    .search-input__cancel-button-container {
        align-self: center;
        color: var(--keyColor);
        font: var(--title-3-tall);
        margin-inline-start: 14px;

        @media (--sidebar-visible) {
            display: none;
        }
    }

    .flex-container {
        @media (--range-sidebar-hidden-down) {
            display: flex;

            form {
                flex-grow: 1;
            }
        }
    }
</style>
