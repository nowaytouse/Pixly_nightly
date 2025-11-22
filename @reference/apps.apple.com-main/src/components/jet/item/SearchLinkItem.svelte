<script lang="ts">
    import {
        isFlowAction,
        type SearchLink,
    } from '@jet-app/app-store/api/models';

    import FlowAction from '~/components/jet/action/FlowAction.svelte';
    import MagnifyingGlass from '~/sf-symbols/magnifyingglass.svg';

    export let item: SearchLink;
</script>

{#if isFlowAction(item.clickAction)}
    <div class="link-container">
        <FlowAction destination={item.clickAction}>
            <MagnifyingGlass class="icon" />
            {item.title}
        </FlowAction>
    </div>
{/if}

<style>
    .link-container {
        display: contents;
    }

    .link-container :global(a) {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 6px;
        padding: 12px;
        font: var(--title-2);
        border-radius: var(--global-border-radius-large);
        background: var(--systemQuinary);
    }

    .link-container :global(a:hover) {
        text-decoration: none;
    }

    .link-container :global(a) :global(.icon) {
        overflow: visible;
        width: 20px;
        fill: currentColor;
    }
</style>
