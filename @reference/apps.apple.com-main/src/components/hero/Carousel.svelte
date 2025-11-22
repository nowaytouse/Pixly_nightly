<!--
@component
Component for rendering a carousel of `Hero.svelte` components in a way taht's decoupled from
any particular data model
-->
<script lang="ts" generics="Item">
    import type { Opt } from '@jet/environment/types/optional';
    import type { Artwork, Shelf } from '@jet-app/app-store/api/models';

    import HorizontalShelf from '~/components/jet/shelf/HorizontalShelf.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import { intersectionObserver } from '@amp/web-app-components/src/actions/intersection-observer';
    import mediaQueries from '~/utils/media-queries';
    import { sidebarIsHidden } from '@amp/web-app-components/src/stores/sidebar-hidden';
    import HeroCarouselBackgroundPortal, {
        id as portalId,
    } from '~/components/hero/CarouselBackgroundPortal.svelte';
    import AmbientBackgroundArtwork from '~/components/AmbientBackgroundArtwork.svelte';
    import portal from '~/utils/portal';
    import { carouselMediaStyle } from '~/stores/carousel-media-style';

    interface $$Slots {
        default: {
            /**
             * The `Item` to render as a `Hero` in the carousel
             */
            item: Item;
        };
    }

    /**
     * The shelf being rendered
     *
     * Used to derrive any shelf-specific presentation
     */
    export let shelf: Shelf;

    /**
     * The items to render in the hero carousel
     *
     * This is decoupled from `shelf` to avoid assuming that `shelf.items` is, itself,
     * the set of items that we need to present; some shelves model their items as chilren
     * of the first shelf item.
     */
    export let items: Item[];

    /**
     * Callback that determines the "background artwork" to use behind the
     * active `Hero` for the given `Item`
     */
    export let deriveBackgroundArtworkFromItem: (item: Item) => Opt<Artwork>;

    $: gridRows = shelf.rowsPerColumn ?? undefined;
    $: isXSmallViewport = $mediaQueries === 'xsmall';

    let activeIndex: number | undefined = 0;

    function createIntersectionObserverCallback(index: number) {
        return (isIntersectingViewport: boolean) => {
            if (isIntersectingViewport) {
                // Many different types of `item`s can be rendered in this Carousel, and all those
                // different items have different ways of determining whether or not the background
                // is dark or light, so we are running through all the options here.
                const { style, mediaOverlayStyle, isMediaDark } = items[
                    index
                ] as any;
                const fallbackStyle = 'dark';
                let derivedStyle;

                if (typeof isMediaDark !== 'undefined') {
                    derivedStyle = isMediaDark ? 'dark' : 'light';
                }

                carouselMediaStyle.set(
                    style || mediaOverlayStyle || derivedStyle || fallbackStyle,
                );

                activeIndex = index;
            }
        };
    }
</script>

<HeroCarouselBackgroundPortal />

<ShelfWrapper {shelf} --shelfGridGutterWidth="0">
    <HorizontalShelf
        {gridRows}
        {items}
        --shelfScrollPaddingInline="0"
        --grid-max-content-xsmall={!$sidebarIsHidden
            ? 'calc(100% + 50px)'
            : '100vw'}
        gridType="Spotlight"
        let:item
        let:index
    >
        {#if isXSmallViewport}
            <div
                use:intersectionObserver={{
                    callback: createIntersectionObserverCallback(index),
                    threshold: 0.5,
                }}
            >
                <slot {item} />
            </div>
        {:else}
            <div
                use:intersectionObserver={{
                    callback: createIntersectionObserverCallback(index),
                    threshold: 0,
                }}
            >
                {#if !import.meta.env.SSR}
                    {@const backgroundArtwork =
                        deriveBackgroundArtworkFromItem(item)}

                    {#if backgroundArtwork}
                        <div use:portal={portalId}>
                            <AmbientBackgroundArtwork
                                artwork={backgroundArtwork}
                                active={activeIndex === index}
                            />
                        </div>
                    {/if}
                {/if}

                <slot {item} />
            </div>
        {/if}
    </HorizontalShelf>
</ShelfWrapper>
