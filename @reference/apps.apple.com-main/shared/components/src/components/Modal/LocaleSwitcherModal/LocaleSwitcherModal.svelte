<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import ChevronIcon from '@amp/web-app-components/assets/icons/chevron.svg';
    import CloseIcon from '@amp/web-app-components/assets/icons/close.svg';
    import { focusNodeOnMount } from '@amp/web-app-components/src/actions/focus-node-on-mount';
    import type { Region } from '@amp/web-app-components/src/components/buttons/LocaleSwitcherButton/types';
    import { updateScrollAndWindowDependentVisuals } from '@amp/web-app-components/src/actions/updateScrollAndWindowDependentVisuals';
    import LocaleSwitcherRegionList from './LocaleSwitcherRegionList.svelte';
    import LocaleSwitcherRegion from './LocaleSwitcherRegion.svelte';

    const DEFAULT_LIST_MINIMUM_LENGTH = 6;
    /**
     * translate function provided by the parent app.
     */
    export let translateFn: (
        str: string,
        values?: Record<string, string | number>,
    ) => string;
    export let regions: Region[];
    export let defaultRoute: string;
    export let dialogTitleId: string | null = null;

    let contentIsScrolling = false;
    let showDefaultList = true;
    let seeAllRegion: Region;
    let contentContainerElement: HTMLElement;

    // the default list for each region is what shows when you first open the modal
    // this consists of each storefront in the default language, with no duplicate storefronts
    const regionsDefaultList = regions.map(({ name, locales }) => {
        return {
            name,
            locales: locales.filter((locale) => locale.isDefault),
        };
    });

    const dispatch = createEventDispatcher();

    const getExpandedRegion = (region: Region) =>
        regions.find((expandedRegion) => expandedRegion.name === region.name);

    const handleSeeAll = (region: Region) => {
        seeAllRegion = getExpandedRegion(region);
        showDefaultList = false;
        contentContainerElement.scroll(0, 0);
    };

    const handleCloseButton = () => {
        dispatch('close');
    };

    const handleBack = () => {
        showDefaultList = true;
    };
</script>

<div
    data-testid="locale-switcher-modal-container"
    class="locale-switcher-modal-container"
>
    <button
        data-testid="locale-switcher-modal-close-button"
        class="close-button"
        type="button"
        on:click={handleCloseButton}
        aria-label={translateFn('AMP.Shared.AX.Close')}
        use:focusNodeOnMount
    >
        <CloseIcon data-testid="locale-switcher-modal-close-button-svg" />
    </button>
    <div
        class="header-container"
        class:content-is-scrolling={contentIsScrolling}
    >
        <span
            id={dialogTitleId}
            data-testid="locale-switcher-modal-title"
            class="title"
        >
            {translateFn('AMP.Shared.LocaleSwitcher.Heading')}
        </span>
    </div>
    <div
        class="region-container"
        bind:this={contentContainerElement}
        use:updateScrollAndWindowDependentVisuals
        on:scrollStatus={(e) =>
            (contentIsScrolling = e.detail.contentIsScrolling)}
    >
        {#if showDefaultList}
            {#each regionsDefaultList as region (region.name)}
                <LocaleSwitcherRegion regionName={translateFn(region.name)}>
                    <button
                        slot="button"
                        class="see-all-button"
                        class:see-all-button-hidden={region.locales.length <=
                            DEFAULT_LIST_MINIMUM_LENGTH}
                        on:click={() => handleSeeAll(region)}
                        >{translateFn('AMP.Shared.LocaleSwitcher.SeeAll')}
                    </button>
                    <!-- If the default list is less than or equal to 6, pass in see all list instead for the default view -->
                    <LocaleSwitcherRegionList
                        slot="list"
                        regionList={region.locales.length <=
                        DEFAULT_LIST_MINIMUM_LENGTH
                            ? getExpandedRegion(region)?.locales
                            : region.locales}
                        {defaultRoute}
                    />
                </LocaleSwitcherRegion>
            {/each}
        {:else}
            <button class="back-button" on:click={handleBack}>
                <ChevronIcon class="back-chevron" aria-hidden="true" />
                {translateFn('AMP.Shared.LocaleSwitcher.Back')}
            </button>

            <LocaleSwitcherRegion regionName={translateFn(seeAllRegion.name)}>
                <LocaleSwitcherRegionList
                    slot="list"
                    regionList={seeAllRegion.locales}
                    {defaultRoute}
                />
            </LocaleSwitcherRegion>
        {/if}
    </div>
</div>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;
    @use 'amp/stylekit/core/fonts' as *;
    @use 'amp/stylekit/modules/fontsubsets/core' as *;
    @use '@amp/web-shared-styles/app/core/globalvars' as *;

    .locale-switcher-modal-container {
        position: relative;
        min-height: 230px;
        height: calc(100vh - 160px);
        display: flex;
        flex-direction: column;
        align-items: center;
        overflow: hidden;
        background-color: var(--pageBG);
        max-width: calc(100vw - 50px);
        border-radius: $modal-border-radius;

        // Font subsets for Geos prevents `SF Pro` Web Font from being downloaded
        // after `BlinkMacSystemFont` fails in Chrome.
        font-family: font-family-locale(en-WW, geos);

        @each $lang, $font in font-family(geos) {
            @if $lang != en-WW {
                :global([lang]:lang(#{$lang})) & {
                    font-family: $font;
                }
            }
        }

        @media (--small) {
            width: 990px;
        }

        @media (--xlarge) {
            width: 1250px;
        }

        &::after {
            position: absolute;
            bottom: 0;
            height: 64px;
            opacity: 1;
            pointer-events: none;
            transition-delay: 0s;
            transition-duration: 300ms;
            transition-property: height, width, background;
            width: calc(100% - 40px);
            content: '';
            background: linear-gradient(
                to top,
                var(--pageBG) 0%,
                rgba(var(--pageBG-rgb), 0) 100%
            );
            z-index: var(--z-default);

            @media (--small) {
                width: calc(100% - 60px);
            }
        }
    }

    .header-container {
        pointer-events: none;
        position: sticky;
        transition-delay: 0s;
        transition-duration: 500ms;
        transition-property: height, width;
        width: 100%;
        padding-top: 54px;
        padding-bottom: 32px;
        max-height: 120px;
        z-index: var(--z-default);
    }

    .content-is-scrolling {
        box-shadow: 0 3px 5px var(--systemQuaternary);
        transition: box-shadow 0.2s ease-in-out;
    }

    .close-button {
        position: absolute;
        top: 0;
        margin: 16px 20px 10px;
        width: 18px;
        height: 18px;
        align-self: flex-start;
        fill: var(--systemSecondary);
    }

    .title {
        color: var(--systemPrimary);
        text-align: center;
        width: 100%;
        display: block;
        padding-inline-start: 20px;
        padding-inline-end: 20px;
        font: var(--title-1-emphasized);

        @media (--medium) {
            font: var(--large-title-emphasized);
        }
    }

    .region-container {
        position: relative;
        height: calc(100% - 120px);
        padding-bottom: 42px;
        overflow-y: auto;
        padding-inline-start: 20px;
        padding-inline-end: 20px;

        @media (width >= 600px) {
            padding-inline-start: 50px;
            padding-inline-end: 50px;
        }
    }

    .back-button {
        color: var(--keyColor);
        margin-bottom: 16px;
        display: flex;
        align-items: center;

        :global(.back-chevron) {
            height: 12px;
            fill: var(--keyColor);
            transform: rotate(180deg);
            margin-inline-end: 5px;

            @include rtl {
                transform: rotate(0deg);
            }
        }
    }

    // shadow-DOM RTL styles
    :global(:host([dir='rtl'])) {
        :global(.back-chevron) {
            transform: rotate(0deg);
        }
    }

    .see-all-button {
        min-width: 42px;
        color: var(--keyColor);
    }

    .see-all-button-hidden {
        display: none;
    }
</style>
