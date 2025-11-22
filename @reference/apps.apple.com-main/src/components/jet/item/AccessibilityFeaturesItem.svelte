<script lang="ts">
    import type { AccessibilityFeatures } from '@jet-app/app-store/api/models';

    import SystemImage, {
        isSystemImageArtwork,
    } from '~/components/SystemImage.svelte';

    export let item: AccessibilityFeatures;
    export let isDetailView: boolean = false;
</script>

<article
    class:is-detail-view={isDetailView}
    role={isDetailView ? 'presentation' : 'article'}
>
    {#if !isDetailView}
        {#if item.artwork && isSystemImageArtwork(item.artwork)}
            <span class="icon-container" aria-hidden="true">
                <SystemImage artwork={item.artwork} />
            </span>
        {/if}
        <h2>{item.title}</h2>
    {/if}

    <ul class:grid={item.features.length > 1 && !isDetailView}>
        {#each item.features as feature}
            <li>
                {#if isSystemImageArtwork(feature.artwork)}
                    <span class="feature-icon-container" aria-hidden="true">
                        <SystemImage artwork={feature.artwork} />
                    </span>
                {/if}
                <div class="feature-content">
                    <h3 class="feature-title">{feature.title}</h3>
                    {#if feature.description}
                        <span class="feature-description">
                            {feature.description}
                        </span>
                    {/if}
                </div>
            </li>
        {/each}
    </ul>
</article>

<style lang="scss">
    @use 'amp/stylekit/core/border-radiuses' as *;
    @use '@amp/web-shared-styles/app/core/globalvars' as *;

    article {
        display: flex;
        flex-direction: column;
        height: 100%;
        padding: 30px;
        gap: 8px;
        text-align: center;
        font: var(--body-tall);
        border-radius: $global-border-radius-rounded-large;
        background-color: var(--systemQuinary);

        &.is-detail-view {
            padding: 0;
            text-align: start;
            background-color: transparent;
        }
    }

    .icon-container {
        width: 30px;
        margin: 0 auto;
    }

    .icon-container :global(svg) {
        width: 100%;
        fill: var(--keyColor);
    }

    h2 {
        font: var(--title-3-emphasized);
        margin-bottom: 8px;
    }

    ul {
        display: flex;
        flex-direction: column;
        gap: 25px;
    }

    ul.grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 10px;
    }

    li {
        display: flex;
        align-items: center;
        justify-content: center;
        text-align: start;
        padding: 4px 0;
        gap: 8px;

        .is-detail-view & {
            gap: 10px;
            justify-content: start;
            align-items: flex-start;
        }
    }

    .grid li {
        justify-content: start;
    }

    .feature-icon-container {
        display: inline-flex;

        @media (prefers-color-scheme: dark) {
            filter: invert(1);
        }

        .is-detail-view & {
            display: flex;
            align-items: center;

            @media (prefers-color-scheme: dark) {
                filter: none;
            }
        }
    }

    .feature-icon-container :global(svg) {
        width: 20px;

        .is-detail-view & {
            width: 30px;
            fill: var(--keyColor);
        }
    }

    .feature-content {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .feature-title {
        font: var(--body-tall);

        .is-detail-view & {
            color: var(--systemPrimary);
            font: var(--title-2-emphasized);
        }
    }

    .feature-description {
        color: var(--systemSecondary);
        font: var(--body);
    }
</style>
