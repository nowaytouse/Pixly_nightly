<script lang="ts">
    import { writable } from 'svelte/store';
    import { isSome } from '@jet/environment/types/optional';
    import type {
        WebNavigation,
        WebNavigationLink,
    } from '@jet-app/app-store/api/models/web-navigation';
    import type { WebSearchFlowAction } from '@jet-app/app-store/common/search/web-search-action';
    import { isSearchResultsPageIntent } from '@jet-app/app-store/api/intents/search-results-page-intent';

    import Navigation from '@amp/web-app-components/src/components/Navigation/Navigation.svelte';
    import { sidebarIsHidden } from '@amp/web-app-components/src/stores/sidebar-hidden';

    import AppStoreLogo from '~/components/icons/AppStoreLogo.svg';
    import PlatformSelectorDropdown from '~/components/jet/web-navigation/PlatformSelectorDropdown.svelte';
    import FlowAction from '~/components/jet/action/FlowAction.svelte';
    import SystemImage, {
        isSystemImageArtwork,
    } from '~/components/SystemImage.svelte';
    import SearchInput from '~/components/navigation/SearchInput.svelte';
    import SFSymbol from '~/components/SFSymbol.svelte';

    import { getJetPerform } from '~/jet';
    import { getI18n } from '~/stores/i18n';
    import {
        type NavigationItemWithTab,
        navigationIdFromLink,
        makeNavLinks,
    } from '~/components/navigation/navigation-items';
    import mediaQueries from '~/utils/media-queries';

    import { fade, type EasingFunction } from 'svelte/transition';
    import { circOut } from 'svelte/easing';
    import { flyAndBlur } from '~/utils/transition';
    import { makeCategoryTabsIntent } from '@jet-app/app-store/api/intents/category-tabs-intent';
    import { getJet } from '~/jet';
    import { getPlatformFromPage } from '~/utils/seo/common';
    import type { NavigationId } from '@amp/web-app-components/src/types';

    const i18n = getI18n();
    const perform = getJetPerform();
    const jet = getJet();

    const categoryTabsCache: Record<string, WebNavigationLink[]> = {};
    let categoryTabLinks: WebNavigationLink[] = [];
    let currentTabStore = writable<NavigationId | null>(null);

    export let webNavigation: WebNavigation;

    $: isXSmallViewport = $mediaQueries === 'xsmall';
    $: searchAction = webNavigation.searchAction as WebSearchFlowAction;
    // Mobile first means the inline items are hidden
    // However, we still want the list visible in SSR (which is fine for mobile
    // since the menu won't be expanded by default)
    $: inlinePlatformItems =
        isXSmallViewport || typeof window === 'undefined'
            ? webNavigation.platforms
            : [];

    $: if (webNavigation && typeof window !== 'undefined') {
        fetchCategoryTabs(webNavigation);
    }

    async function fetchCategoryTabs(nav: WebNavigation) {
        const platform = getPlatformFromPage({
            webNavigation: nav,
        });

        if (!platform) {
            categoryTabLinks = [];
            return;
        }

        if (categoryTabsCache[platform]) {
            categoryTabLinks = updateActiveStates(categoryTabsCache[platform]);
        } else {
            try {
                const data = await jet.dispatch(
                    makeCategoryTabsIntent({
                        platform,
                    }),
                );

                categoryTabsCache[platform] = data;
                categoryTabLinks = updateActiveStates(data);
            } catch (error) {
                categoryTabLinks = [];
            }
        }

        updateCurrentTab();
    }

    function updateActiveStates(
        tabs: WebNavigationLink[],
    ): WebNavigationLink[] {
        return tabs.map((link) => ({
            ...link,
            isActive: link.action?.destination?.id
                ? window.location.pathname.includes(link.action.destination.id)
                : false,
        }));
    }

    function updateCurrentTab() {
        const allLinks: WebNavigationLink[] = [
            ...categoryTabLinks,
            ...webNavigation.tabs,
        ];

        const activeLink = allLinks.find((link) => link.isActive);
        currentTabStore.set(
            activeLink ? navigationIdFromLink(activeLink) : null,
        );
    }

    function handleMenuItemClick(event: CustomEvent<NavigationItemWithTab>) {
        const navigationItem = event.detail;
        const tab = navigationItem.tab;

        perform(tab.action);
    }

    const BASE_DELAY = 80;
    const BASE_DURATION = 150;
    const DURATION_SPREAD = 300;

    // Returns an eased duration for a list item based on its index, e.g. items later in the list
    // get longer durations, between BASE_DURATION and BASE_DURATION + DURATION_SPREAD.
    function getEasedDuration({
        i,
        totalNumberOfItems,
        easing = circOut,
    }: {
        i: number;
        totalNumberOfItems: number;
        easing?: EasingFunction;
    }) {
        const t = i / (totalNumberOfItems - 1);
        return BASE_DURATION + easing(t) * DURATION_SPREAD;
    }
</script>

<div class="navigation-wrapper">
    <Navigation
        translateFn={$i18n.t}
        items={makeNavLinks(webNavigation.tabs, {
            shouldShowSearchTab: $sidebarIsHidden,
        })}
        personalizedItemsHeader={$i18n.t(
            'ASE.Web.AppStore.Navigation.Categories.Title',
        )}
        personalizedItems={makeNavLinks(categoryTabLinks, {
            shouldShowSearchTab: $sidebarIsHidden,
        })}
        currentTab={currentTabStore}
        libraryItems={[]}
        on:menuItemClick={handleMenuItemClick}
    >
        <div slot="logo" class="platform-selector-container">
            <span
                id="app-store-icon-contianer"
                class="app-store-icon-container"
                role="img"
                aria-label={$i18n.t(
                    'ASE.Web.AppStore.Navigation.AX.AppStoreLogo',
                )}
            >
                <AppStoreLogo focusable={false} />
            </span>

            {#if !$sidebarIsHidden && !isXSmallViewport}
                <PlatformSelectorDropdown
                    platformSelectors={webNavigation.platforms}
                />
            {/if}
        </div>

        <svelte:fragment slot="search">
            <div class="search-input-container">
                <SearchInput {searchAction} />
            </div>
        </svelte:fragment>

        <div slot="after-navigation-items" class="platform-selector-inline">
            {#if isXSmallViewport}
                <h3 in:fade out:fade={{ delay: 250, duration: BASE_DURATION }}>
                    {$i18n.t('ASE.Web.AppStore.Navigation.PlatformHeading')}
                </h3>
            {/if}

            <ul>
                {#each inlinePlatformItems as platformSelector, i (platformSelector.action.title)}
                    {@const { action, isActive } = platformSelector}
                    {@const artwork = action.artwork}
                    {@const totalNumberOfItems = inlinePlatformItems.length}
                    <li
                        in:flyAndBlur={{
                            y: -50,
                            delay: i * BASE_DELAY,
                            duration: getEasedDuration({
                                i,
                                totalNumberOfItems,
                            }),
                        }}
                        out:flyAndBlur={{
                            y: i * -5,
                            delay:
                                // This delay is calculated in a negative/backwards manner,
                                // which makes it so the items build out from the bottom to the top.
                                (totalNumberOfItems - i - 1) * (BASE_DELAY / 2),
                            duration: BASE_DURATION,
                        }}
                    >
                        <FlowAction destination={action}>
                            <span class="platform" class:is-active={isActive}>
                                {#if isSome(artwork) && isSystemImageArtwork(artwork)}
                                    <div
                                        class="icon-container"
                                        aria-hidden="true"
                                    >
                                        <SystemImage {artwork} />
                                    </div>
                                {/if}

                                <span class="platform-title">
                                    {action.title}
                                </span>

                                {#if action.destination && isSearchResultsPageIntent(action.destination)}
                                    <span
                                        aria-hidden={true}
                                        class="search-icon-container"
                                    >
                                        <SFSymbol name="magnifyingglass" />
                                    </span>
                                {/if}
                            </span>
                        </FlowAction>
                    </li>
                {/each}
            </ul>
        </div>
    </Navigation>
</div>

<style lang="scss">
    .navigation-wrapper {
        display: contents;
    }

    .platform-selector-container {
        --header-gap: 3px;
        --platform-selector-trigger-gap: var(--header-gap);
        display: flex;
        gap: var(--header-gap);
        position: relative;

        @media (--sidebar-visible) {
            padding: 19px 25px 14px;
        }
    }

    // Japanese and Catalonian both require scaling down the platform selector in order to make it
    // fit cleanly in the sidebar, due to their longer character lengths.
    .platform-selector-container:lang(ja),
    .platform-selector-container:lang(ca) {
        --scale-factor: 0.1;
        z-index: 3;
        transform: scale(calc(1 - var(--scale-factor)));
        transform-origin: center left;

        & :global(dialog) {
            top: 60px;
            // Since the `dialog` is a child of `platform-selector-container, we re-scale it back
            // to it's original size by applying the inverse scale transformation.
            transform: scale(calc(1 + var(--scale-factor)));
            transform-origin: center left;
        }
    }

    .app-store-icon-container {
        display: flex;
        align-items: center;
        gap: var(--header-gap);
        font: var(--title-1);
        font-weight: 600;
    }

    .app-store-icon-container :global(svg) {
        height: 18px;
        position: relative;
        top: 0.33px;
        width: auto;

        @media (--sidebar-visible) and (--range-xsmall-only) {
            height: 22px;
            width: auto;
        }
    }

    .search-input-container {
        margin: 0 25px;
    }

    .navigation-wrapper :global(.navigation__header) {
        @media (--sidebar-visible) {
            display: flex;
            flex-direction: column;
        }
    }

    .navigation-wrapper :global(.navigation-item__link) {
        height: 100%;
        display: flex;
    }

    .navigation-wrapper :global(.navigation-item__icon) {
        --navigation-item-icon-size: 32px;
        width: var(--navigation-item-icon-size);
        height: var(--navigation-item-icon-size);
        display: flex;
        justify-content: center;

        @media (--sidebar-visible) {
            --navigation-item-icon-size: 24px;
        }
    }

    // Our SVG icons for the landing pages are sized differently than other Onyx apps,
    // so we have to reach into the navigation component and style them so they look
    // visually similar to the other Onyx apps
    .navigation-wrapper :global(.navigation-item__icon svg) {
        color: var(--keyColor);
        width: 20px;

        @media (--sidebar-visible) {
            width: 18px;
        }
    }

    // Below is styling for the "inline" version of the Platform Selector
    .platform-selector-inline {
        margin: 8px 32px;
    }

    ul {
        display: flex;
        flex-direction: column;
        gap: 5px;
    }

    h3 {
        color: var(--systemTertiary);
        font: var(--body-emphasized);
        margin: 0 0 10px;
        padding-top: 20px;

        @media (--sidebar-visible) {
            font: var(--footnote-emphasized);
            margin: 0 0 6px;
            padding-top: 7px;
        }
    }

    .platform {
        display: flex;
        gap: 10px;
        padding: 8px 0;
        color: var(--systemTertiary);

        @media (prefers-color-scheme: dark) {
            color: var(--systemSecondary);
        }
    }

    .platform,
    .platform :global(svg) {
        transition: color 210ms ease-out;
    }

    .platform:not(.is-active):hover,
    .platform:not(.is-active):hover :global(svg) {
        color: var(--systemPrimary);
    }

    .platform.is-active {
        color: var(--systemPrimary);
        font: var(--body-emphasized);
    }

    .platform.is-active :global(svg) {
        color: currentColor;
    }

    .icon-container {
        display: flex;
    }

    .icon-container :global(svg) {
        color: var(--systemTertiary);
        width: 18px;
        max-height: 16px;

        @media (prefers-color-scheme: dark) {
            color: var(--systemSecondary);
        }
    }

    .search-icon-container {
        display: flex;
    }

    .search-icon-container :global(svg) {
        fill: var(--systemSecondary);
        width: 16px;
    }

    .platform-title {
        font: var(--body);
        flex-grow: 1;
    }
</style>
