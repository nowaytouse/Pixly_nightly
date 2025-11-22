<script lang="ts">
    import {
        menuIsExpanded,
        menuIsTransitioning,
    } from '@amp/web-app-components/src/components/Navigation/store/menu-state';
    import { prefersReducedMotion } from '@amp/web-app-components/src/stores/prefers-reduced-motion';
    import { createEventDispatcher } from 'svelte';

    export let translateFn: (
        key: string,
        data?: Record<string | number, string>,
    ) => string;
    export let navigationId = '';

    const OPEN_NAVIGATION_LABEL = translateFn('FUSE.AX.UI.Open.Navigation');
    const CLOSE_NAVIGATION_LABEL = translateFn('FUSE.AX.UI.Close.Navigation');
    const dispatch = createEventDispatcher();

    // Helper vars for refocusing on menu button when the menu closes.
    let menuWasExpanded = false;
    let menuButton: HTMLButtonElement;

    $: ariaExpanded = $menuIsExpanded;
    $: ariaLabel = ariaExpanded
        ? CLOSE_NAVIGATION_LABEL
        : OPEN_NAVIGATION_LABEL;

    $: if ($menuIsExpanded) {
        menuWasExpanded = true;
    }

    // Only focus the menu button if the menu was previously expanded and is now collapsed.
    // This prevents the menu button from focusing on page mount.
    $: if (!$menuIsExpanded && menuWasExpanded) {
        menuButton?.focus();
        menuWasExpanded = false;
    }

    function handleClick(): void {
        // Only allow the menu to be expanded / contracted if a transition is not currently in flight.
        if ($menuIsTransitioning) {
            return;
        }

        // Update the internal nav store
        // Implicitly updates aria-expanded and aria-label
        menuIsExpanded.set(!$menuIsExpanded);

        // dispatch event to parent app
        dispatch('toggleExpansion', {
            isMenuExpanded: ariaExpanded,
        });

        // If reduced motion is not preferred, the flag needs to be set
        // that a transition is currently in flight. When reduced-motion is preferred,
        // no transition occurs.
        if (!$prefersReducedMotion) {
            // Flag that the menu-transition is in flight. This gets unlocked
            // by the <Navigation /> component as it has the longest duration
            menuIsTransitioning.set(true);
        }
    }
</script>

<button
    data-testid="menuicon"
    class="menuicon"
    aria-controls={navigationId}
    aria-label={ariaLabel}
    aria-expanded={ariaExpanded}
    on:click={handleClick}
    bind:this={menuButton}
>
    <span class="menuicon-bread menuicon-bread-top">
        <span class="menuicon-bread-crust menuicon-bread-crust-top" />
    </span>
    <span class="menuicon-bread menuicon-bread-bottom">
        <span class="menuicon-bread-crust menuicon-bread-crust-bottom" />
    </span>
</button>

<style lang="scss">
    @use '@amp/web-shared-styles/app/core/globalvars' as *;

    $shared-transition-delay: 0.1008s;
    $shared-transition-duration: 0.1806s;
    $amp-nav-ease-blue: cubic-bezier(0.04, 0.04, 0.12, 0.96);
    $amp-nav-ease-green: cubic-bezier(0.52, 0.16, 0.52, 0.84);

    .menuicon {
        height: $global-header-mobile-contracted-height;
        width: $global-header-mobile-contracted-height;
        position: relative;
        z-index: var(--z-default);
    }

    .menuicon-bread {
        height: 20px;
        left: 13px;
        pointer-events: none;
        position: absolute;
        top: 12px;
        transition: transform $shared-transition-duration $amp-nav-ease-blue;
        width: 20px;
        z-index: var(--z-default);

        /* Make sure the crust elements are not clickable to ensure correct locking. */
        span {
            pointer-events: none;
        }

        [aria-expanded='true'] & {
            height: 24px;
            left: 10px;
            top: 11px;
            width: 24px;
            // prettier-ignore
            transition: transform 0.3192s $amp-nav-ease-blue $shared-transition-delay;
        }
    }

    [aria-expanded='true'] {
        .menuicon-bread-top {
            transform: rotate(-45deg);
        }

        .menuicon-bread-bottom {
            transform: rotate(45deg);
        }
    }

    .menuicon-bread-crust {
        background: var(--keyColor);
        border-radius: 1px;
        display: block;
        height: 2px;
        position: absolute;
        // prettier-ignore
        transition: transform 0.1596s $amp-nav-ease-green $shared-transition-delay;
        width: 20px;
        z-index: var(--z-default);

        [aria-expanded='true'] & {
            width: 24px;
            transform: translateY(0);
            transition: transform $shared-transition-duration $amp-nav-ease-blue;
        }
    }

    .menuicon-bread-crust-top {
        top: 9px;
        transform: translateY(-4px);

        [aria-expanded='true'] & {
            top: 11px;
        }
    }

    .menuicon-bread-crust-bottom {
        bottom: 9px;
        transform: translateY(4px);

        [aria-expanded='true'] & {
            bottom: 11px;
        }
    }

    // Remove transitions when user prefers reduced motion
    @media (prefers-reduced-motion: reduce) {
        .menuicon-bread,
        .menuicon-bread-crust {
            &,
            [aria-expanded='true'] & {
                transition: none;
            }
        }
    }
</style>
