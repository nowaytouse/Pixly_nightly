<script lang="ts">
    import type { InAppPurchaseLockup } from '@jet-app/app-store/api/models';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import Artwork from '~/components/Artwork.svelte';
    import PlusIcon from '~/sf-symbols/plus.heavy.svg';

    export let item: InAppPurchaseLockup;
</script>

<article>
    <div class="artwork-container">
        <PlusIcon class="plus-icon" aria-hidden="true" />
        <Artwork artwork={item.icon} profile="in-app-purchase" />
    </div>

    <div class="metadata-container">
        {#if item.title}
            <LineClamp clamp={1}>
                <h3>{item.title}</h3>
            </LineClamp>
        {/if}

        {#if item.productDescription}
            <LineClamp clamp={1}>
                <p>{item.productDescription}</p>
            </LineClamp>
        {/if}

        {#if item.offerDisplayProperties.titles}
            <p>
                {item.offerDisplayProperties.titles.discountUnownedParent ||
                    item.offerDisplayProperties.titles.standard}
            </p>
        {/if}
    </div>
</article>

<style>
    .artwork-container {
        position: relative;
        flex-shrink: 0;
        width: 100%;
        margin-bottom: 8px;
        padding: 8%;
        border-radius: var(--global-border-radius-small);
        background: var(--systemQuinary);
    }

    .artwork-container :global(.plus-icon) {
        position: absolute;
        top: 6%;
        width: 9%;
        inset-inline-end: 5%;
    }

    .artwork-container :global(.artwork-component) {
        border-radius: var(--global-border-radius-small) 43%
            var(--global-border-radius-small) var(--global-border-radius-small);
    }

    .metadata-container {
        margin-inline-end: 16px;
    }

    h3 {
        font: var(--body-tall);
    }

    p {
        font: var(--callout-tall);
        color: var(--systemSecondary);
    }
</style>
