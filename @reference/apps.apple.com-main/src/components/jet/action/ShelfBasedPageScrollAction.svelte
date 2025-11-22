<script lang="ts" context="module">
    import type {
        Action,
        ShelfBasedPageScrollAction,
    } from '@jet-app/app-store/api/models';

    export function isShelfBasedPageScrollAction(
        action: Action,
    ): action is ShelfBasedPageScrollAction {
        return (
            action.$kind === 'ShelfBasedPageScrollAction' && 'shelfId' in action
        );
    }
</script>

<script lang="ts">
    import type { HTMLAnchorAttributes } from 'svelte/elements';

    interface $$Props extends HTMLAnchorAttributes {
        destination: ShelfBasedPageScrollAction;
    }

    export let destination: ShelfBasedPageScrollAction;

    function handleLinkClick(e: Event) {
        const anchorElement = e.currentTarget as HTMLAnchorElement;
        const hash = anchorElement.hash;
        const elementToScrollTo = document.querySelector(hash);
        if (!elementToScrollTo) {
            return;
        }
        elementToScrollTo.scrollIntoView({
            behavior: 'smooth',
            block: 'start',
        });
        history.replaceState(null, '', hash);
    }
</script>

{#if destination.shelfId}
    <a
        {...$$restProps}
        data-test-id="scroll-link"
        href={`#${destination.shelfId}`}
        on:click|preventDefault|stopPropagation={handleLinkClick}
    >
        <slot />
    </a>
{:else}
    <slot />
{/if}
