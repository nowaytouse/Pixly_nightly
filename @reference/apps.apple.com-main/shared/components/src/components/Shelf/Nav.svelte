<script lang="ts">
    import type { ArrowOffset } from '@amp/web-app-components/src/components/Shelf/types';
    import ChevronCompactLeft from '@amp/web-app-components/assets/shelf/chevron-compact-left.svg';
    import { createEventDispatcher } from 'svelte';

    export let translateFn: (
        str: string,
        values?: Record<string, string | number>,
    ) => string;
    export let headerHeight: number;
    export let arrowOffset: ArrowOffset;
    export let hasNextPage: boolean;
    export let hasPreviousPage: boolean;
    export let isRTL: boolean;

    $: hasNavArrows = hasPreviousPage || hasNextPage;

    // Adjusting arrows to center on content.
    // This is a fallback for browsers that don't support CSS anchor positioning.
    $: addSpaceForHeader = (() => {
        let offsetStyle = '0px';

        // Custom adjustment provided by user
        if (arrowOffset && arrowOffset.length) {
            arrowOffset.forEach(({ direction, offset }) => {
                if (direction == 'top') {
                    offsetStyle = `
                        ${offset}px;
                    `;
                } else {
                    offsetStyle = `
                        calc(${offset}px * -1);
                    `;
                }
            });
        }
        // Adjust for header
        if (headerHeight) {
            // adjust nav height to account for header
            offsetStyle = `
                ${headerHeight}px;
            `;
        }

        return offsetStyle;
    })();

    const NAV = {
        PREVIOUS: 'previous',
        NEXT: 'next',
    } as const;

    const dispatch = createEventDispatcher();
    const handleNextPage = () => dispatch(NAV.NEXT);
    const handlePreviousPage = () => dispatch(NAV.PREVIOUS);

    $: NEXT_ARROW_PROPS = {
        disabled: !hasNextPage,
        'aria-label': translateFn('AMP.Shared.NextPage'),
    };

    $: PREV_ARROW_PROPS = {
        disabled: !hasPreviousPage,
        'aria-label': translateFn('AMP.Shared.PreviousPage'),
    };

    $: rightArrowProps = isRTL ? PREV_ARROW_PROPS : NEXT_ARROW_PROPS;
    $: rightClick = isRTL ? handlePreviousPage : handleNextPage;

    $: leftArrowProps = isRTL ? NEXT_ARROW_PROPS : PREV_ARROW_PROPS;
    $: leftClick = isRTL ? handleNextPage : handlePreviousPage;
</script>

{#if hasNavArrows}
    <button
        {...leftArrowProps}
        type="button"
        class="shelf-grid-nav__arrow shelf-grid-nav__arrow--left"
        data-testId="shelf-button-left"
        on:click={leftClick}
        style="--offset: {addSpaceForHeader};"
    >
        <ChevronCompactLeft />
    </button>
    <slot name="shelf-content" />
    <button
        {...rightArrowProps}
        type="button"
        class="shelf-grid-nav__arrow shelf-grid-nav__arrow--right"
        data-testId="shelf-button-right"
        on:click={rightClick}
        style="--offset: {addSpaceForHeader};"
    >
        <ChevronCompactLeft />
    </button>
{:else}
    <slot name="shelf-content" />
{/if}

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use './style/core.scss' as *;

    .shelf-grid-nav {
        list-style: none;
        margin: 0;

        ul {
            list-style: none;
            margin: 0;
        }
    }

    .shelf-grid-nav__arrow {
        height: $shelf-grid-arrow-height;
        width: $shelf-grid-arrow-width;
        align-items: center;
        border: none;
        cursor: pointer;
        display: flex;
        justify-content: center;
        overflow: hidden;
        position: absolute;
        top: 50%;
        transition: $shelf-grid-nav-transition;
        translate: 0 -50%;
        border-radius: 6px;

        // Non GPU-accelerated layers must be below GPU-accelerated layers.
        z-index: var(--z-default);

        // Fallback for browsers that don't support CSS anchor positioning
        @supports not (top: anchor(--a center)) {
            transform: translateY(calc(-50% + var(--offset)));
            translate: none;
        }

        // CSS Anchor Positioning to vertically center paddles with artwork
        // Powerswoosh intentionally not targeted — doesn't have `shelf` class.
        :global(.shelf:has(.shelf-grid__list--grid-rows-1)) & {
            // Set `top` to align with center of first artwork in 1-row shelf.
            // Targets anchor in `Shelf.svelte`.
            top: anchor(--shelf-first-artwork center, 50%);
        }

        :global(svg) {
            width: 8.5px;
            height: 30.5px;
            fill: var(--systemSecondary);
        }

        &:hover,
        &:focus-visible {
            text-decoration: none;
            background: var(--systemQuinary);

            @media (prefers-color-scheme: dark) {
                background: var(--systemQuaternary);
            }
        }

        &:active {
            background: var(--systemQuaternary);

            @media (prefers-color-scheme: dark) {
                background: var(--systemTertiary);
            }

            :global(svg) {
                fill: var(--systemPrimary);
            }
        }

        &:disabled {
            cursor: default;
            opacity: 0;
        }

        // Paddles not used in xsmall viewport
        @media (--range-xsmall-down) {
            display: none;
        }
    }

    .shelf-grid-nav__arrow--right {
        right: $shelf-grid-arrow-position;
        scale: -1 1; // Flip icon horizontally
    }

    .shelf-grid-nav__arrow--left {
        left: $shelf-grid-arrow-position;
    }

    @media (--range-xsmall-down) {
        .shelf-grid-nav {
            display: none;
        }
    }
</style>
