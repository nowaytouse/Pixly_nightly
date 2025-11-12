<script lang="ts">
    import type { PrivacyHeader } from '@jet-app/app-store/api/models';
    import LinkableTextItem from '~/components/jet/item/LinkableTextItem.svelte';

    export let item: PrivacyHeader;
</script>

<div>
    <p>
        <LinkableTextItem item={item.bodyText} />
    </p>

    {#if item.supplementaryItems.length}
        <div class="supplementary-items-container">
            {#each item.supplementaryItems as supItem}
                <p>
                    <LinkableTextItem item={supItem.bodyText} />
                </p>
            {/each}
        </div>
    {/if}
</div>

<style>
    p {
        font: var(--body-tall);
    }

    p :global(a) {
        color: var(--keyColor);
    }

    .supplementary-items-container {
        display: flex;
        flex-direction: column;
        gap: 10px;
        padding: 20px 0 0;
        margin-top: 20px;
        border-top: 1px solid var(--systemGray4);
    }
</style>
