<script lang="ts">
    import { isFlowAction, type Banner } from '@jet-app/app-store/api/models';
    import { isSome } from '@jet/environment';
    import LinkWrapper from '~/components/LinkWrapper.svelte';

    export let item: Banner;
</script>

<div class="banner">
    <p>
        {item.message}
        {#if isSome(item.action) && isFlowAction(item.action)}
            &nbsp;<LinkWrapper action={item.action}>
                {item.action.title}
            </LinkWrapper>
        {/if}
    </p>
</div>

<style>
    .banner {
        background: rgba(var(--keyColor-rgb), 0.07);
        padding: 8px 16px;
        margin: 0 var(--bodyGutter);
        text-align: center;
        border-radius: var(--global-border-radius-small);
    }

    .banner :global(a) {
        color: var(--keyColor);
        text-decoration: none;
    }

    .banner :global(a:hover) {
        text-decoration: underline;
    }
</style>
