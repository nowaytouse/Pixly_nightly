<script lang="ts">
    import type { PrivacyType } from '@jet-app/app-store/api/models';

    import SystemImage, {
        isSystemImageArtwork,
    } from '~/components/SystemImage.svelte';

    export let item: PrivacyType;
    export let isDetailView: boolean = false;
</script>

<article class:is-detail-view={isDetailView}>
    {#if item.artwork && isSystemImageArtwork(item.artwork)}
        <span class="icon-container" aria-hidden="true">
            <SystemImage artwork={item.artwork} />
        </span>
    {/if}

    <h2>{item.title}</h2>
    <p>{item.detail}</p>

    <ul class:grid={item.categories.length > 1 && !isDetailView}>
        {#each item.categories as category}
            <li>
                {#if isSystemImageArtwork(category.artwork)}
                    <span aria-hidden="true" class="category-icon-container">
                        <SystemImage artwork={category.artwork} />
                    </span>
                {/if}
                {category.title}
            </li>
        {/each}
    </ul>

    {#each item.purposes as purpose}
        <section class="purpose-section">
            <h3>{purpose.title}</h3>

            {#each purpose.categories as category}
                <li class="purpose-category">
                    {#if isSystemImageArtwork(category.artwork)}
                        <span
                            aria-hidden="true"
                            class="category-icon-container"
                        >
                            <SystemImage artwork={category.artwork} />
                        </span>
                    {/if}

                    <span class="category-title">{category.title}</span>

                    <ul class="privacy-data-types">
                        {#each category.dataTypes as type}
                            <li>{type}</li>
                        {/each}
                    </ul>
                </li>
            {/each}
        </section>
    {/each}
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
            padding: 20px 0 0;
            margin-top: 20px;
            text-align: left;
            border-radius: 0;
            background-color: transparent;
            border-top: 1px solid var(--defaultLine);
        }
    }

    .icon-container {
        width: 30px;
        margin: 0 auto;

        .is-detail-view & {
            display: block;
            width: 32px;
            margin: 0;
        }
    }

    .icon-container :global(svg) {
        width: 100%;
        fill: var(--keyColor);
    }

    h2 {
        font: var(--title-3-emphasized);

        .is-detail-view & {
            font: var(--title-2-emphasized);
        }
    }

    p {
        text-wrap: pretty;
        font: var(--body-tall);
        color: var(--systemSecondary);
    }

    .grid {
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
            justify-content: start;
        }
    }

    .category-title {
        font: var(--title-3);
    }

    .grid li {
        justify-content: start;
    }

    .category-icon-container {
        display: inline-flex;

        @media (prefers-color-scheme: dark) {
            filter: invert(1);
        }

        .is-detail-view & {
            display: flex;
            align-items: center;
        }
    }

    .category-icon-container :global(svg) {
        width: 20px;

        .is-detail-view & {
            width: 20px;
            height: 18px;
        }
    }

    .purpose-section {
        border-top: 1px solid var(--defaultLine);
        padding-top: 16px;
    }

    .purpose-section + .purpose-section {
        margin-top: 4px;
    }

    .purpose-section h3 {
        margin-bottom: 8px;
    }

    .purpose-category {
        display: grid;
        grid-template-areas:
            'icon title'
            '. types';
        align-items: center;
    }

    .privacy-data-types {
        grid-area: types;
        color: var(--systemSecondary);
        font: var(--body);
    }
</style>
