<script lang="ts">
    import { onMount } from 'svelte';
    import Nav from '@amp/web-app-components/src/components/Shelf/Nav.svelte';
    import { getGridVars } from '@amp/web-app-components/src/components/Shelf/utils/getGridVars';
    import { checkItemPositionInShelf } from '@amp/web-app-components/src/components/Shelf/utils/observerCallback';
    import { ShelfWindow } from '@amp/web-app-components/src/components/Shelf/utils/shelf-window';
    import { throttle } from '@amp/web-app-components/src/utils/throttle';
    import { GRID_COLUMN_GAP_DEFAULT } from '@amp/web-app-components/src/components/Shelf/constants';
    import scrollByPolyfill from '@amp/web-app-components/src/utils/scrollByPolyfill';
    import { TEXT_DIRECTION } from '@amp/web-app-components/src/constants';
    import type {
        GridType,
        ArrowOffset,
        AspectRatioOverrideConfig,
    } from '@amp/web-app-components/src/components/Shelf/types';
    import { observe } from '@amp/web-app-components/src/components/Shelf/actions/observe';
    import ShelfItem from '@amp/web-app-components/src/components/Shelf/ShelfItem.svelte';
    import { createVisibleIndexStore } from '@amp/web-app-components/src/components/Shelf/store/visibleStore';
    import { getMaxVisibleItems } from '@amp/web-app-components/src/components/Shelf/utils/getMaxVisibleItems';
    import { createShelfAspectRatioContext } from '@amp/web-app-components/src/utils/shelfAspectRatio';
    import type { Readable } from 'svelte/store';

    type T = $$Generic;

    export let translateFn: (
        str: string,
        values?: Record<string, string | number>,
    ) => string;
    // eslint-disable-next-line no-undef-init
    export let id: string | undefined = undefined;
    export let items: T[];
    export let gridType: GridType;
    export let gridRows = 1;
    export let arrowOffset: ArrowOffset | null = null;
    // TODO: rdar://112908912 (Update `alignItems` prop in Shelf component and config to better match its actual function)
    export let alignItems = false;
    export let stackXSItems = false;
    export let overflowBleedBottom: string = null;
    export let aspectRatioOverride: AspectRatioOverrideConfig = null;
    export let getItemIdentifier:
        | ((item: unknown, index?: number) => string)
        | null = null;
    export let pageScrollMultiplier: number = null;

    /**
     * On shelf scroll this handler returns the first and last indexes
     * of the items currently visible in the shelf viewport.
     */
    export let onIntersectionUpdate: (
        itemIndexsInViewport: [number, number],
    ) => void | null = null;
    /**
     * Determines the first index in the items[] that should be visible on load.
     * Defaults to the start of the items[].
     */
    export let firstItemIndex: number = 0;

    // Exporting a function to scroll to a specific page number
    export function scrollToPage(pageNumber: number): void {
        pageScroll(pageMultiplier * pageNumber);
    }

    // This makes the let:item of type T
    function cast(x: T): T {
        return x as T;
    }

    const shelfItemIdentifier = (
        item: unknown,
        index: number,
    ): unknown | string => {
        let id: string;
        if (typeof getItemIdentifier === 'function') {
            id = getItemIdentifier(item, index);
            if (typeof id !== 'string') {
                // TODO: rdar://92459555 (Shared Components: integrate app logger in to shared components)
                console.debug(
                    'Could not get unique id, falling back to default',
                    item,
                );
            }
        } else if (isObjectWithId(item)) {
            id = item.id;
        }
        return id || item;
    };

    interface WithID {
        id: string;
    }
    function isObjectWithId(o: unknown): o is WithID {
        return typeof o === 'object' && 'id' in o;
    }

    // used to center arrows
    let headerHeight = 0;

    // Corresponds to `$global-container-shadow-offset` in `_globavars.scss`
    const STANDARD_LOCKUP_SHADOW_OFFSET = 15;

    let shelfAspectRatioStore: Readable<string> | null = null;
    if (aspectRatioOverride !== null) {
        const { shelfAspectRatio } =
            createShelfAspectRatioContext(aspectRatioOverride);
        shelfAspectRatioStore = shelfAspectRatio;
    }

    $: style = (() => {
        // TODO: possibly move this to app level rdar://74522896
        let customStyles = `
            ${getGridVars(gridType)}
            --grid-type: ${gridType};
            --grid-rows: ${gridRows};
            --standard-lockup-shadow-offset: ${STANDARD_LOCKUP_SHADOW_OFFSET}px;
            ${
                aspectRatioOverride !== null && $shelfAspectRatioStore !== null
                    ? `--shelf-aspect-ratio: ${$shelfAspectRatioStore};`
                    : ''
            }
        `;

        if (overflowBleedBottom) {
            customStyles += `--overflowBleedBottom: ${overflowBleedBottom};`;
        }

        return customStyles;
    })();

    let scrollableContainer: HTMLUListElement = null;

    let hasPreviousPage = false;
    let hasNextPage = true;
    let shelfBodyBoundingRect: HTMLDivElement = null;

    let observer: IntersectionObserver = null;
    let viewport: [number, number] | null = null;
    $: isRTL = false;

    const visibleStore = createVisibleIndexStore();
    const initalVisibleGridItems =
        getMaxVisibleItems(gridType) * (gridRows || 1);
    visibleStore.updateEndIndex(initalVisibleGridItems);

    const createObserver = (shelfBody: HTMLElement) => {
        const options = {
            root: shelfBody,
            rootMargin: '0px',
            threshold: 0.5,
        };

        const shelfWindow = new ShelfWindow();
        const callback = (entries: IntersectionObserverEntry[]) => {
            const LAST_ITEM = items.length - 1;
            entries.forEach((entry) => {
                const item = entry.target as HTMLUListElement;
                const currentIndex = parseInt(item.dataset.index, 10);

                // to prevent user seeing items loading
                // load a few items off screen
                const EXTRA_ITEMS = 2 * gridRows || 2;
                const [isFirstItemAndInView, isLastItemAndInView] =
                    checkItemPositionInShelf(entry, LAST_ITEM);
                if (entry.isIntersecting) {
                    shelfWindow.enterValue(currentIndex);

                    const nextIndex = currentIndex + 1;
                    if (nextIndex >= $visibleStore.endIndex) {
                        const lastIndex = currentIndex + EXTRA_ITEMS;
                        visibleStore.updateEndIndex(lastIndex);
                    }
                    setShelfItemInteractivity(entry.target, true);
                } else {
                    shelfWindow.exitValue(currentIndex);
                    setShelfItemInteractivity(entry.target, false);
                }

                if (isFirstItemAndInView !== null) {
                    hasPreviousPage = !isFirstItemAndInView;
                }

                if (isLastItemAndInView !== null) {
                    hasNextPage = !isLastItemAndInView;
                }
            });

            viewport = shelfWindow.getViewport();

            if (viewport && onIntersectionUpdate) {
                onIntersectionUpdate(viewport);
            }
        };
        return new IntersectionObserver(callback, options);
    };

    onMount(() => {
        scrollByPolyfill();
        // rdar://81757000 (TLF: Make storefront / language updates happen in-place with JS instead of hard-refreshes)
        isRTL = document.dir === TEXT_DIRECTION.RTL;
        observer = createObserver(shelfBodyBoundingRect);
        if (firstItemIndex !== 0) {
            scrollToIndex(firstItemIndex);
        }

        return () => {
            observer.disconnect();
        };
    });

    export function scrollToIndex(index: number) {
        const shelfItems = scrollableContainer.getElementsByClassName(
            'shelf-grid__list-item',
        );
        if (!shelfItems) {
            return;
        }
        const firstItem = shelfItems[0] as HTMLDivElement;
        const itemWidth = firstItem.getBoundingClientRect().width;

        let scrollAmount: number;
        if (index === 0) {
            scrollAmount = 0;
        } else {
            scrollAmount =
                (itemWidth +
                    GRID_COLUMN_GAP_DEFAULT -
                    STANDARD_LOCKUP_SHADOW_OFFSET * 2) *
                index;
        }

        let offset = isRTL ? -scrollAmount : scrollAmount;
        scrollableContainer.scrollTo({ left: offset, behavior: 'instant' });
    }

    const pageScroll = (pageCount = 1) => {
        const containerWidth =
            scrollableContainer.getBoundingClientRect().width;
        const scrollAmount =
            (containerWidth +
                GRID_COLUMN_GAP_DEFAULT -
                STANDARD_LOCKUP_SHADOW_OFFSET * 2) *
            pageCount;
        scrollableContainer.scrollBy(scrollAmount, 0);
    };
    const THROTTLE_LIMIT = 300;

    const pageMultiplierNumber = pageScrollMultiplier || 1;
    $: pageMultiplier = isRTL ? -pageMultiplierNumber : pageMultiplierNumber;
    $: handleNextPage = throttle(
        pageScroll.bind(null, pageMultiplier),
        THROTTLE_LIMIT,
    );
    $: handlePreviousPage = throttle(
        pageScroll.bind(null, -pageMultiplier),
        THROTTLE_LIMIT,
    );

    let firstKnownItem: WithID;
    let initialScroll = 0;
    function restoreScroll(node: HTMLElement, items: T[]) {
        if (!isObjectWithId(items[0])) {
            return {};
        }
        firstKnownItem = items[0];
        return {
            update(items: T[]) {
                if (
                    isObjectWithId(items[0]) &&
                    items[0].id !== firstKnownItem.id &&
                    initialScroll === 0 &&
                    node.scrollLeft > 0
                ) {
                    node.scrollLeft = 0;
                }
            },
        };
    }

    function trackScrollPosition(e: UIEvent) {
        initialScroll = (e.target as HTMLElement).scrollLeft;
    }

    function setShelfItemInteractivity(
        shelfItemElement: Element,
        isShelfItemVisible: boolean,
    ) {
        const interactiveContent: NodeListOf<
            HTMLAnchorElement | HTMLButtonElement
        > = shelfItemElement.querySelectorAll('a, button');
        interactiveContent.forEach((interactiveElement) => {
            if (interactiveElement.nodeName === 'A') {
                if (isShelfItemVisible) {
                    interactiveElement.removeAttribute('tabindex');
                } else {
                    interactiveElement.setAttribute('tabindex', '-1');
                }
            } else {
                // if this is a <button>
                if (isShelfItemVisible) {
                    interactiveElement.removeAttribute('disabled');
                } else {
                    interactiveElement.setAttribute('disabled', 'true');
                }
            }
        });
    }
</script>

<section
    {id}
    data-testid="shelf-component"
    class="shelf-grid shelf-grid--onhover"
    {style}
>
    {#if $$slots.header}
        <div class="shelf-grid__header" bind:offsetHeight={headerHeight}>
            <slot name="header" />
        </div>
    {/if}
    <div
        class="shelf-grid__body"
        data-testid="shelf-body"
        bind:this={shelfBodyBoundingRect}
    >
        <!--
            Fix for rdar://101154977 (AX: JMOW: Play button in Album lockup is not announced)

            Firefox adds scrollable elements to the tab order, so we need to
            remove the grid list from the tab order with `tabindex="-1"` so
            item announcement works as expected with NVDA.

            Since it has a tabindex set, we also need to prevent the mouse from
            being able to focus the element on mousedown.
        -->
        <!-- TODO: rdar://97308317 (Investigate svelte AX warnings in shared components) -->
        <!--
            In Safari, list semantics are removed from the AX tree when
            CSS property list-style-type: none is used (this does not include nav elements).
            Including role="list" on ul elements will re-add list semantics.
            See https://bugs.webkit.org/show_bug.cgi?id=170179
        -->
        <Nav
            on:next={handleNextPage}
            on:previous={handlePreviousPage}
            {headerHeight}
            {translateFn}
            {arrowOffset}
            {hasNextPage}
            {hasPreviousPage}
            {isRTL}
        >
            <ul
                slot="shelf-content"
                class={`shelf-grid__list shelf-grid__list--grid-type-${gridType} shelf-grid__list--grid-rows-${gridRows}`}
                class:shelf-grid__list--align-items-end={alignItems}
                class:shelf-grid__list--stack-xs-items={stackXSItems}
                role="list"
                tabindex="-1"
                data-testid="shelf-item-list"
                on:scroll={trackScrollPosition}
                bind:this={scrollableContainer}
                use:restoreScroll={items}
            >
                <!--
                    TODO: rdar://77578080
                    (Shared Components: Create a keyed each loop shelf and non-keyed shelf)
                -->
                {#each items as item, index (shelfItemIdentifier(item, index))}
                    {@const isItemInteractable =
                        index >= viewport?.[0] && index <= viewport?.[1]}
                    <ShelfItem {index} {visibleStore} let:isRendered>
                        <!-- TODO: rdar://97308317 (Investigate svelte AX warnings in shared components) -->
                        <li
                            class="shelf-grid__list-item"
                            class:placeholder={!isRendered}
                            class:shelf-grid__list-item--stack-xs-items={stackXSItems}
                            data-index={index}
                            aria-hidden={isItemInteractable ? 'false' : 'true'}
                            use:observe={observer}
                        >
                            {#if isRendered}
                                <div
                                    use:setShelfItemInteractivity={isItemInteractable}
                                >
                                    <slot
                                        name="item"
                                        item={cast(item)}
                                        {index}
                                        numberOfItems={items.length}
                                    />
                                </div>
                            {/if}
                        </li>
                    </ShelfItem>
                {/each}
            </ul>
        </Nav>
    </div>
</section>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/selectors' as *;
    @use 'amp/stylekit/core/viewports' as *;
    @use 'amp/stylekit/core/mixins/overflow-bleed' as *;
    @use '@amp/web-shared-styles/app/core/globalvars' as *;
    @use './style/core.scss' as *;
    @use './style/base.scss' as *;

    @mixin shelf-grid-list-styles($viewport: null) {
        $grid-cols: var(--grid-#{$viewport});
        $grid-offset: calc(
            (#{$grid-cols} - 1) * var(--grid-column-gap-#{$viewport})
        );
        grid-auto-columns: var(
            --grid-max-content-#{$viewport},
            calc((100% - #{$grid-offset}) / #{$grid-cols})
        );
        grid-template-rows: repeat(var(--grid-rows), max-content);
        column-gap: var(--grid-column-gap-#{$viewport});
        row-gap: var(--grid-row-gap-#{$viewport});
    }

    .shelf-grid__list {
        // Standard lockups, of different heights, should align to titles under artwork
        align-items: stretch;

        @include shelf-grid-list-styles(xsmall);

        @each $viewport in ('small', 'medium', 'large', 'xlarge') {
            @media (--range-#{$viewport}-only) {
                @include shelf-grid-list-styles($viewport);
            }

            // Reduce column count by 1 in `medium` and `large` viewports when drawer is open
            @if $viewport == 'medium' or $viewport == 'large' {
                @include feature-detect(is-drawer-open) {
                    @media (--range-#{$viewport}-only) {
                        // No adjustments on Grid Types `A` and `music-radio`, for parity with DMA
                        &:not(
                                .shelf-grid__list--grid-type-A,
                                .shelf-grid__list--grid-type-music-radio,
                                .shelf-grid__list--grid-type-H
                            ) {
                            // Subtract 1 column when drawer is open
                            $grid-cols: calc(var(--grid-#{$viewport}) - 1);
                            $grid-offset: calc(
                                (#{$grid-cols} - 1) *
                                    var(--grid-column-gap-#{$viewport})
                            );
                            grid-auto-columns: var(
                                --grid-max-content-#{$viewport},
                                calc((100% - #{$grid-offset}) / #{$grid-cols})
                            );
                        }

                        &.shelf-grid__list--grid-type-H {
                            // Subtract 2 columns on grid-type "H" only
                            $grid-cols: calc(var(--grid-#{$viewport}) - 2);
                            $grid-offset: calc(
                                (#{$grid-cols} - 2) *
                                    var(--grid-column-gap-#{$viewport})
                            );
                            grid-auto-columns: var(
                                --grid-max-content-#{$viewport},
                                calc((100% - #{$grid-offset}) / #{$grid-cols})
                            );
                        }
                    }
                }
            }
        }

        @media (--small) {
            :first-child {
                // Set anchor for shelf chevron alignment
                // Use `noShelfChevronAnchor={true}` to activate `artwork-component--no-anchor`
                // class and disable chevron anchoring on an `<Artwork>` component. That will help isolate
                // the true anchor when there are multiple `<Artworks>`s are in a single shelf lockup.
                :global(.artwork-component:not(.artwork-component--no-anchor)) {
                    anchor-name: --shelf-first-artwork;
                }
            }
        }
    }

    .shelf-grid--onhover {
        // stylelint-disable-next-line selector-pseudo-class-no-unknown
        :global(.shelf-grid-nav__arrow) {
            opacity: 0;
            will-change: opacity;
            transition: $shelf-grid-nav-transition;

            &:focus {
                opacity: 1;
            }
        }

        &:hover,
        &:focus-within {
            // stylelint-disable-next-line selector-pseudo-class-no-unknown
            :global(.shelf-grid-nav__arrow:not([disabled])) {
                opacity: 1;
            }
        }
    }

    // TODO: rdar://112908912 (Update `alignItems` prop in Shelf component and config to better match its actual function)
    .shelf-grid__list--align-items-end {
        --override-shelf-overflow-bleed-bottom: 35px;
        padding-top: 0;
    }

    // TODO: rdar://88487875 (Revisit accessibility for shelf)
    // allows for accurate count for VO
    // .placeholder::before {
    //     content: '•';
    //     opacity: 0;
    // }

    // Stack Music Radio shelf lockups, for `xs-1` viewport only.
    .shelf-grid__list--stack-xs-items {
        --override-shelf-overflow-bleed-bottom: 35px;
        align-items: stretch;

        @media (--range-grid-layout-xs-1-down) {
            display: block;
            // Add `bodyGutter` back that is intentionally removed for peeking XS shelves.
            padding-inline-end: var(--bodyGutter);

            :not(:first-child) {
                margin-top: $spacerC;
            }
        }
    }
</style>
