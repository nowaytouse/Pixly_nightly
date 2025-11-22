<script lang="ts">
    import {
        type ProductPageLink,
        isFlowAction,
    } from '@jet-app/app-store/api/models';
    import { isExternalUrlAction } from '~/jet/models/';
    import FlowAction from '~/components/jet/action/FlowAction.svelte';
    import ExternalURLAction from '~/components/jet/action/ExternalUrlAction.svelte';

    export let item: ProductPageLink;

    const clickAction = item.clickAction;

    $: canRenderContainer =
        isFlowAction(clickAction) || isExternalUrlAction(clickAction);
</script>

{#if canRenderContainer}
    <div class="product-link-container">
        {#if isFlowAction(clickAction)}
            <FlowAction destination={clickAction}>
                {item.text}
            </FlowAction>
        {:else if isExternalUrlAction(clickAction)}
            <ExternalURLAction destination={clickAction}>
                {item.text}
            </ExternalURLAction>
        {/if}
    </div>
{/if}

<style>
    .product-link-container {
        @media (--range-xsmall-down) {
            padding: 10px 0;
        }
    }

    .product-link-container :global(a) {
        display: inline-flex;
        align-items: center;
        color: var(--keyColor);
        text-decoration: none;
        gap: 6px;

        &:hover {
            text-decoration: underline;
        }

        @media (--range-xsmall-down) {
            font-size: 18px;
            gap: 8px;
        }
    }

    .product-link-container :global(a) :global(.external-link-arrow) {
        width: 7px;
        height: 7px;
        fill: var(--keyColor);
        margin-top: 3px;

        @media (--range-xsmall-down) {
            width: 10px;
            height: 10px;
            margin-top: 2px;
        }
    }
</style>
