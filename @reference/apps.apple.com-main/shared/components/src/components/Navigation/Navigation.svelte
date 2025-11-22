<script lang="ts">
    import { createEventDispatcher, afterUpdate } from 'svelte';
    import type { Writable } from 'svelte/store';
    import {
        menuIsExpanded,
        menuIsTransitioning,
    } from '@amp/web-app-components/src/components/Navigation/store/menu-state';
    import type { NavigationId } from '@amp/web-app-components/src/types';
    import type { NavigationItem } from '@amp/web-app-components/src/components/Navigation/types';
    import MenuIcon from './MenuIcon.svelte';
    import NavigationItems from './NavigationItems.svelte';
    import { allowDrop } from '@amp/web-app-components/src/actions/allow-drop';
    import { sidebarIsHidden } from '@amp/web-app-components/src/stores/sidebar-hidden';

    const dispatch = createEventDispatcher();

    /**
     * The local storage key that contains the user-selected library items to show
     * @type {string}
     */
    export let visibilityPreferencesKey: string | null = null;

    /**
     * A list of links to be in the navigation
     * @type {Array<NavigationItem>}
     */
    export let items: NavigationItem[];

    /**
     * A list of links to be in the library navigation
     * @type {Array<NavigationItem>}
     */
    export let libraryItems: NavigationItem[] = [];

    /**
     * A list of personalized items in the navigation such as a user's playlists or stations
     * @type {Array<NavigationItem>}
     */
    export let personalizedItems: NavigationItem[] = [];

    /**
     * Header to be used for the personalized items list
     */
    export let personalizedItemsHeader: string = '';

    /**
     * translate function provided by the parent app.
     */
    export let translateFn: (key: string) => string;

    /**
     * The store containing the currently selected tab.
     */
    export let currentTab: Writable<NavigationId | null>;

    /**
     * Whether you should be able to drop on the library section
     * @type {boolean}
     */
    export let libraryDropEnabled: boolean = false;

    /**
     * Boolean or method to indicate if it allows drop on navigation header.
     * The header type can be passed in to have a conditional drop area.
     * Use together with on:dropOnHeader
     */
    export let headerDropEnabled: boolean | ((type: string) => boolean) = false;

    /**
     * Function that maps the item to drag data.
     * Uses the item by default when not set.
     */
    export let getItemDragData: (item: NavigationItem) => any = null;

    /**
     * Boolean or method to indicate if it allows items to be dragged.
     * The item can be passed in to have conditional dragging.
     * Use together with getItemDragData
     */
    export let itemDragEnabled: boolean | ((item: NavigationItem) => boolean) =
        false;

    /**
     * Boolean or method to indicate if it allows drop on an item.
     * The item can be passed in to have a conditional drop area.
     * Use together with on:dropOnItem
     */
    export let itemDropEnabled: boolean | ((item: NavigationItem) => boolean) =
        false;

    const navigationId: string = 'navigation';

    // If the viewport changes to show the sidebar while menu is expanded, update menu store.
    // This ensures `aria-hidden="false"` on the main section and player bar.
    $: if (!$sidebarIsHidden) {
        $menuIsExpanded = false;
    }

    let navigatableContainer: HTMLElement;
</script>

<nav
    data-testid="navigation"
    class="navigation"
    class:is-transitioning={$menuIsTransitioning}
    class:is-expanded={$menuIsExpanded}
    on:transitionend|self={() => ($menuIsTransitioning = false)}
>
    <div class="navigation__header">
        {#if $sidebarIsHidden}
            <MenuIcon {navigationId} {translateFn} on:toggleExpansion />
            <slot name="logo" />
            <slot name="auth" />
        {:else}
            <slot name="logo" />
            <slot name="search" />
        {/if}
    </div>

    <div
        data-testid="navigation-content"
        class="navigation__content"
        id={navigationId}
        aria-hidden={$sidebarIsHidden && !$menuIsExpanded ? 'true' : 'false'}
    >
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
            bind:this={navigatableContainer}
            class="navigation__scrollable-container"
        >
            {#if typeof window === 'undefined' || navigatableContainer}
                <NavigationItems
                    type="primary"
                    {items}
                    {translateFn}
                    {currentTab}
                    visibilityPreferencesKey={null}
                    header={null}
                    listGroupElement={navigatableContainer}
                    on:menuItemClick
                />

                {#if libraryItems.length > 0}
                    <div
                        use:allowDrop={libraryDropEnabled && {
                            dropEnabled: true,
                            onDrop: (dropData) =>
                                dispatch('libraryDrop', dropData),
                        }}
                        data-testid="navigation-library-section"
                    >
                        <NavigationItems
                            type="library"
                            header={translateFn('AMP.Shared.Library')}
                            items={libraryItems}
                            listGroupElement={navigatableContainer}
                            {visibilityPreferencesKey}
                            {translateFn}
                            {currentTab}
                            {itemDragEnabled}
                            {itemDropEnabled}
                            on:dropOnItem
                            on:menuItemClick
                        />
                    </div>
                {/if}

                {#if personalizedItems.length > 0}
                    <NavigationItems
                        type="personalized"
                        header={personalizedItemsHeader}
                        items={personalizedItems}
                        visibilityPreferencesKey={null}
                        listGroupElement={navigatableContainer}
                        {translateFn}
                        {currentTab}
                        {getItemDragData}
                        {itemDragEnabled}
                        {itemDropEnabled}
                        {headerDropEnabled}
                        on:menuItemClick
                        on:dropOnItem
                        on:dropOnHeader
                    />
                {/if}
            {/if}
            <slot name="after-navigation-items" />
        </div>

        <div class="navigation__native-cta">
            <slot name="native-cta" />
        </div>
    </div>
</nav>

<style lang="scss">
    @use '@amp/web-shared-styles/app/core/globalvars' as *;

    // Default Values
    $amp-nav-element-transition: height 0.56s cubic-bezier(0.52, 0.16, 0.24, 1);

    .navigation {
        width: 100%;
        display: flex;
        flex-direction: column;
        z-index: var(--z-web-chrome);

        @media (--range-sidebar-hidden-down) {
            height: $global-header-mobile-contracted-height;
            position: fixed;
            overflow: hidden;
            background-color: var(--mobileNavigationBG);
            box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);

            &.is-expanded {
                height: 100%;
            }

            // The transition property should only be applied when the
            // navigation is actively being set to expand / contract.
            // This is to prevent unintended transitions when moving from
            // `sidebar:visible` to `sidebar:hidden`.
            &.is-transitioning {
                transition: $amp-nav-element-transition;
            }

            // Remove transition when user prefers reduced motion
            @media (prefers-reduced-motion: 'reduce') {
                transition: none;
            }
        }

        @media (--sidebar-visible) {
            height: 100%;
            position: relative;
            background-color: var(--navSidebarBG);
            box-shadow: none;
            border-inline-end: 1px solid var(--labelDivider);
        }
    }

    .navigation__header {
        display: grid;

        // Mobile styles -- horizontal icons
        @media (--range-sidebar-hidden-down) {
            grid-template-columns: repeat(3, 1fr);
            align-items: center;
            margin-inline-start: 12px;
            margin-inline-end: 11px;

            // Position each child correctly relative to grid cell
            & > :global(:nth-child(1)) {
                justify-self: start;
            }

            & > :global(:nth-child(2)) {
                justify-self: center;
            }

            & > :global(:nth-child(3)) {
                justify-self: end;
            }
        }

        // Desktop styles -- stacked logo + search
        @media (--sidebar-visible) {
            :global(.search-input-wrapper) {
                min-height: $web-search-input-height;
            }
        }
    }

    .navigation__content {
        display: flex;
        flex-direction: column;
        overflow: hidden;

        // Explicitly set sidebar content container width to include border, per spec
        @media (--sidebar-visible) {
            width: var(--web-navigation-width);
            flex: 1;
        }
    }

    .navigation__scrollable-container {
        overflow-y: auto;
        scroll-behavior: smooth;

        @media (--range-sidebar-hidden-down) {
            padding-top: 23px;
        }

        @media (--sidebar-visible) {
            flex: 1; // Push CTA to bottom of sidebar
        }
    }
</style>
