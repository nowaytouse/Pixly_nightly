<script lang="ts" context="module">
    import type {
        PagePresentationOptions,
        Shelf,
    } from '@jet-app/app-store/api/models';
    import type { WebRenderablePage } from '@jet-app/app-store/api/models/web-renderable-page';

    /**
     * Just the `Page` props that we actually need to render this component
     */
    export interface DefaultPageRequirements extends WebRenderablePage {
        shelves: Shelf[];
        presentationOptions?: PagePresentationOptions;
    }
</script>

<script lang="ts">
    import type { MarkerShelf } from '~/components/jet/shelf/MarkerShelf.svelte';
    import { isUberShelf } from '~/components/jet/shelf/UberShelf.svelte';
    import ShelfComponent from '~/components/jet/shelf/Shelf.svelte';
    import { partition } from '~/utils/array';
    import { carouselMediaStyle } from '~/stores/carousel-media-style';
    import mediaQueries from '~/utils/media-queries';
    import { isHeroCarouselShelf } from '../jet/shelf/HeroCarouselShelf.svelte';
    import { isRtl } from '~/utils/locale';

    interface $$Slots {
        'before-shelves': {};

        /**
         * If {@linkcode ShelfComponent}` recognizes a shelf to be a {@linkcode MarkerShelf},
         * this slot will be rendered so that the "page" data can be supplied by a "parent"
         * component
         */
        'marker-shelf': {
            shelf: MarkerShelf;
        };
    }

    export let page: DefaultPageRequirements;

    $: ({ title, presentationOptions = [] } = page);

    // Some shelves are meant to be rendered above the title, rather than below it
    $: [aboveTitleShelves, belowTitleShelves] = partition(
        page.shelves,
        (shelf) => {
            // Some "uber" shelves might be placed above the title
            if (isUberShelf(shelf)) {
                const [uber] = shelf.items;
                return uber.style === 'above';
            }

            // Everything else should be below it
            return false;
        },
    );

    $: prefersHiddenPageTitle = presentationOptions.includes(
        'prefersHiddenPageTitle',
    );
    $: prefersLargeTitle = presentationOptions.includes('prefersLargeTitle');
    $: prefersOverlayedPageHeader =
        $mediaQueries === 'xsmall' &&
        presentationOptions.includes('prefersOverlayedPageHeader');
    $: isOnDarkBackground =
        prefersOverlayedPageHeader && $carouselMediaStyle === 'dark';

    $: isTitleDuplicatedInHero = (() => {
        const firstShelf = page.shelves?.[0];

        if (
            !firstShelf ||
            !isHeroCarouselShelf(firstShelf) ||
            firstShelf.items?.length !== 1
        ) {
            return false;
        }

        const { items: ltrItems, rtlItems } = firstShelf.items?.[0] ?? {};
        const firstItem = isRtl() && rtlItems?.length ? rtlItems : ltrItems;
        const firstTitle = firstItem?.[0]?.overlay?.titleText;

        return title === firstTitle;
    })();
</script>

<div
    class="default-page-container"
    data-testid="default-page-container"
    class:with-overlaid-title={prefersOverlayedPageHeader}
    class:with-title-in-hero={isTitleDuplicatedInHero}
>
    {#each aboveTitleShelves as shelf}
        <ShelfComponent {shelf}>
            <slot name="marker-shelf" slot="marker-shelf" let:shelf {shelf} />
        </ShelfComponent>
    {/each}

    {#if title && !prefersHiddenPageTitle && !isTitleDuplicatedInHero}
        <h1
            data-test-id="page-title"
            class:large-title={prefersLargeTitle}
            class:overlaid={prefersOverlayedPageHeader}
            class:on-dark-background={isOnDarkBackground}
        >
            {title}
        </h1>
    {/if}

    <slot name="before-shelves" />

    {#each belowTitleShelves as shelf}
        {#if !shelf.isHidden}
            <ShelfComponent {shelf}>
                <slot
                    name="marker-shelf"
                    slot="marker-shelf"
                    let:shelf
                    {shelf}
                />
            </ShelfComponent>
        {/if}
    {/each}
</div>

<style lang="scss">
    @use 'ac-sasskit/modules/viewportcontent/core' as *;
    @use 'amp/stylekit/core/viewports' as *;

    .default-page-container {
        flex-grow: 1;
        width: 100%;
        max-width: viewport-content-for(xlarge);
        margin: 0 auto;
    }

    .default-page-container.with-overlaid-title {
        margin-top: -13px;
    }

    .default-page-container.with-title-in-hero {
        @media (--range-small-up) {
            margin-top: 10px;
        }
    }

    h1 {
        padding: 11px var(--bodyGutter);
        font: var(--large-title-emphasized);
        letter-spacing: -0.5px;
        word-wrap: break-word;
        color: var(--systemPrimary, #000);
        position: relative;
        z-index: 1;
        transition: color 210ms ease-in;
    }

    h1.large-title {
        font: var(--large-title-emphasized-tall);
    }

    h1.overlaid {
        position: absolute;
        z-index: 3;
        padding: var(--bodyGutter) var(--bodyGutter) 0;
        color: var(--systemPrimary-onLight, #000);
    }

    h1.on-dark-background {
        color: var(--systemPrimary-onDark);
    }
</style>
