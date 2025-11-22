<script lang="ts">
    import { getI18n } from '~/stores/i18n';
    import { isSome } from '@jet/environment/types/optional';
    import type { TopChartsPage } from '@jet-app/app-store/api/models';

    import DefaultPage from '~/components/pages/DefaultPage.svelte';
    import Shelf from '~/components/Shelf/Wrapper.svelte';
    import Artwork, { getNaturalProfile } from '~/components/Artwork.svelte';
    import Menu from '~/components/Menu.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import SFSymbol from '~/components/SFSymbol.svelte';

    export let page: TopChartsPage;

    const i18n = getI18n();

    $: ({ categories, categoriesButtonTitle, segments, initialSegmentIndex } =
        page);
    $: segment = segments[initialSegmentIndex];
</script>

<DefaultPage page={{ shelves: segment.shelves, title: page.title }}>
    <Shelf slot="before-shelves" centered>
        <header>
            <div class="dropdown-container">
                {#if categoriesButtonTitle}
                    <Menu options={categories}>
                        <svelte:fragment slot="trigger">
                            <span class="menu-trigger-contents">
                                {categoriesButtonTitle}

                                <SFSymbol name="chevron.down" />
                            </span>
                        </svelte:fragment>

                        <svelte:fragment slot="option" let:option>
                            {@const { artwork, chartSelectAction, name } =
                                option}

                            <LinkWrapper action={chartSelectAction}>
                                <div
                                    class="category-menu-item"
                                    class:active={name ===
                                        categoriesButtonTitle}
                                >
                                    {#if isSome(artwork)}
                                        <div class="artwork-container">
                                            <Artwork
                                                {artwork}
                                                profile={getNaturalProfile(
                                                    artwork,
                                                    [24],
                                                )}
                                            />
                                        </div>
                                    {/if}

                                    <span>{name}</span>
                                </div>
                            </LinkWrapper>
                        </svelte:fragment>
                    </Menu>
                {/if}
            </div>

            <div class="segment-selector" aria-label={categoriesButtonTitle}>
                {#each segments as segment, index}
                    {@const { segmentSelectAction } = segment}
                    {@const isSelected = initialSegmentIndex === index}
                    {@const filterLabel = $i18n.t(
                        isSelected
                            ? 'ASE.Web.AppStore.SelectedFilterApps.AX.Label'
                            : 'ASE.Web.AppStore.FilterApps.AX.Label',
                        { filterName: segment.shortName },
                    )}

                    <LinkWrapper
                        action={segmentSelectAction}
                        label={filterLabel}
                    >
                        <span class="segment" class:selected={isSelected}>
                            {segment.shortName}
                        </span>
                    </LinkWrapper>
                {/each}
            </div>
        </header>
    </Shelf>
</DefaultPage>

<style>
    header {
        --pill-button-border-radius: 1000px; /* Arbitrary large value for "pill-style" rounded sides */
        --menu-item-padding: 0;
        --menu-item-margin: 0 0 8px 0;
        --menu-popover-padding: 12px 16px;
        --menu-common-padding: 0;
        --menu-trigger-border-radius: var(--pill-button-border-radius);
        --menu-trigger-background-color: var(--systemPrimary-onDark);
        --menu-trigger-padding: 6px 16px;
        --menu-trigger-font: var(--body-semibold-tall);
        --menu-popover-background-color: white;
        --menu-popover-box-shadow: 10px 10px 10px 0
            var(--systemQuaternary-onLight);
        --menu-popover-border-radius: 14px;
        --menu-popover-border: 1px solid var(--systemQuaternary);
        --menu-popover-z-index: calc(var(--z-web-chrome) + 1);
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 20px;

        @media (--range-small-up) {
            display: grid;
            align-items: center;
            justify-items: start;
            grid-template-columns: 1fr max-content 1fr;
        }

        @media (prefers-color-scheme: dark) {
            --menu-trigger-background-color: var(--systemQuaternary);
            --menu-popover-background-color: var(--systemQuaternary-vibrant);
        }
    }

    .segment-selector {
        display: flex;
        justify-self: end;
        gap: 4px;
        padding: 2px;
        background: var(--systemQuaternary);
        border-radius: var(--pill-button-border-radius);

        @media (--range-small-up) {
            align-items: center;
            justify-self: center;
            grid-column: 2;
        }
    }

    .segment-selector :global(a) {
        display: contents;
    }

    .segment {
        border-radius: var(--pill-button-border-radius);
        font: var(--body-semibold-tall);
        padding: 6px 16px;
    }

    .segment.selected {
        background-color: var(--systemPrimary-onDark);
        color: var(--systemPrimary);

        @media (prefers-color-scheme: dark) {
            background-color: var(--systemQuaternary);
        }
    }

    .dropdown-container {
        justify-self: start;

        @media (--range-small-up) {
            grid-column: 1;
        }
    }

    .menu-trigger-contents {
        display: flex;
        align-items: center;
        gap: 4px;
    }

    .menu-trigger-contents :global(svg) {
        height: 0.7em;
    }

    .menu-trigger-contents :global(path:not([fill='none'])) {
        fill: currentColor;
    }

    .category-menu-item {
        display: flex;
        align-items: center;
        padding: 8px;
        border-radius: 10px;
        height: 40px;
        transition: background 150ms ease-in;
    }

    .category-menu-item.active {
        background: var(--systemQuinary);
    }

    .category-menu-item:not(.active):hover {
        background: rgba(0, 0, 0, 0.035);
    }

    .artwork-container {
        width: 24px;
        margin-inline-end: 8px;
        flex-shrink: 0;
    }

    .category-menu-item span {
        text-overflow: ellipsis;
        overflow: hidden;
    }

    .dropdown-container :global(.menu-popover) {
        max-width: 600px;
        width: 100%;
        column-count: 2;

        @media (--range-medium-up) {
            column-count: 3;
        }
    }
</style>
