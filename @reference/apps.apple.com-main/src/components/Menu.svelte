<script lang="ts" generics="T">
    import { tick } from 'svelte';
    import type { Opt } from '@jet/environment/types/optional';
    import type { MouseEventHandler } from 'svelte/elements';
    import { onDestroy, onMount } from 'svelte';
    import { generateUuid } from '@amp/web-apps-utils/src';
    import {
        computePosition,
        autoUpdate,
        offset,
        flip,
        shift,
    } from '@floating-ui/dom';

    export let options: T[];
    // Allows the developer the override the floating-ui calculated offset to a fixed number
    export let forcedXPosition: number | null = null;

    export let handleShowMenu: () => void = () => {};

    let isMenuOpen = false;

    /**
     * Display the menu
     *
     * @example
     * <script>
     *   let menu;
     *
     *   function showMenu() {
     *     menu.show();
     *   }
     * <\/script>
     *
     * <Menu bind:this={menu} />
     */
    export async function show() {
        if (!menuEl) return;

        isMenuOpen = true;

        // Menu position should be updated *only* after the dialog has been shown
        updateMenuPosition();

        // Focuses the first link in the dropdown after the DOM updates
        await tick();
        menuEl.querySelector('a')?.focus();

        // When the modal is open, track viewport changes and update the menu position
        floatingUIAutoUpdatePositionCleanupCallback = autoUpdate(
            trigger!,
            menuEl!,
            updateMenuPosition,
        );
    }

    /**
     * Close the menu
     *
     * @example
     * <script>
     *   let menu;
     *
     *   function closeMenu() {
     *     menu.close();
     *   }
     * <\/script>
     *
     * <Menu bind:this={menu} />
     */
    export function close() {
        if (!menuEl) return;

        isMenuOpen = false;
        cleanUpFloatingUIAutoPosition();
    }

    function toggle() {
        if (isMenuOpen) {
            close();
        } else {
            show();
            handleShowMenu?.();
        }
    }

    const menuId = generateUuid();

    let menuEl: HTMLUListElement | undefined;
    let trigger: HTMLButtonElement | undefined;

    function handleKeyUp(event: KeyboardEvent) {
        if (event.key === 'Escape') {
            close();
        }
    }

    /**
     * Dismiss the dialog when clicking anywhere with the dialog open
     */
    const handleBodyClick: MouseEventHandler<HTMLElement> = (event) => {
        const clickedElement = event.target as HTMLElement;

        // Only close the dialog if the click is "outside" of the trigger
        // Otherwise, it will be closed immediately
        if (!trigger?.contains(clickedElement)) {
            close();
        }
    };

    /// MARK: Menu Positioning through `FloatingUI`

    /**
     * Update the position of the menu to align it with the trigger
     */
    async function updateMenuPosition() {
        const { x, y } = await computePosition(trigger!, menuEl!, {
            middleware: [
                offset({
                    mainAxis: 10,
                }),

                flip(),
                shift(),
            ],
            placement: 'bottom-end',
        });

        Object.assign(menuEl!.style, {
            left: `${forcedXPosition || x}px`,
            top: `${y}px`,
        });
    }

    let floatingUIAutoUpdatePositionCleanupCallback: Opt<() => void>;

    /**
     * Cleans up the `FloatingUI` auto-update listener, which should only be "active"
     * while the menu is open
     */
    function cleanUpFloatingUIAutoPosition() {
        floatingUIAutoUpdatePositionCleanupCallback?.();
        floatingUIAutoUpdatePositionCleanupCallback = undefined;
    }

    onMount(() => {
        // Ensures menu is hidden initially
        if (menuEl) isMenuOpen = false;
    });

    onDestroy(function () {
        cleanUpFloatingUIAutoPosition();
    });
</script>

<svelte:body on:keyup={handleKeyUp} on:click={handleBodyClick} />

<button
    class="menu-trigger"
    aria-controls={menuId}
    aria-haspopup="menu"
    aria-expanded={isMenuOpen}
    bind:this={trigger}
    on:click={toggle}
>
    <slot name="trigger" />
</button>

<ul
    id={menuId}
    hidden={!isMenuOpen}
    tabindex="-1"
    class="menu-popover focus-visible"
    bind:this={menuEl}
>
    {#each options as option}
        <li class="menu-item" role="presentation">
            <slot name="option" {option} />
        </li>
    {/each}
</ul>

<style>
    :root {
        --menu-common-padding: 4px 8px;
    }

    .menu-trigger {
        background-color: var(--menu-trigger-background-color);
        border-radius: var(--menu-trigger-border-radius);
        font: var(--menu-trigger-font);
        padding: var(--menu-trigger-padding, var(--menu-common-padding));
    }

    .menu-popover {
        background-color: var(--menu-popover-background-color, var(--pageBg));
        padding: var(--menu-popover-padding, 0);
        border: var(--menu-popover-border, none);
        border-radius: var(
            --menu-popover-border-radius,
            var(--global-border-radius-large)
        );
        box-shadow: var(--menu-popover-box-shadow, var(--shadow-medium));
        position: absolute;
        inset: auto;
        z-index: var(--menu-popover-z-index, 2);
    }

    .menu-popover::backdrop {
        background: var(--menu-popover-backdrop-background, none);
    }

    .menu-item {
        padding: var(--menu-item-padding, var(--menu-common-padding));
        margin: var(--menu-item-margin, 0);
        white-space: nowrap;
    }
</style>
