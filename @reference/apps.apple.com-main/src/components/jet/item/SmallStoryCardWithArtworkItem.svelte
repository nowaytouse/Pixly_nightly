<script lang="ts" context="module">
    import type {
        Artwork as ArtworkModel,
        TodayCard,
    } from '@jet-app/app-store/api/models';

    export interface SmallStoryCardWithArtwork extends TodayCard {
        artwork: ArtworkModel;
        badge: any;
    }

    export function isSmallStoryCardWithArtworkItem(
        item: TodayCard,
    ): item is SmallStoryCardWithArtwork {
        return !('media' in item) && 'artwork' in item;
    }
</script>

<script lang="ts">
    import Artwork from '~/components/Artwork.svelte';
    import HoverWrapper from '~/components/HoverWrapper.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import GradientOverlay from '~/components/GradientOverlay.svelte';
    import { colorAsString } from '~/utils/color';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';

    export let item: SmallStoryCardWithArtwork;

    $: artwork = item.heroMedia?.artworks?.[0] || item.artwork;

    $: gradientColor = artwork.backgroundColor
        ? colorAsString(artwork.backgroundColor)
        : 'rgb(0 0 0 / 62%)';
</script>

<article>
    <LinkWrapper action={item.clickAction}>
        <HoverWrapper element="div">
            <Artwork {artwork} profile="small-story-card-portrait" />

            <GradientOverlay --color={gradientColor} />

            <div class="text-container">
                {#if item.badge?.title}
                    <h4>{item.badge.title}</h4>
                {/if}

                {#if item.title}
                    <h3>{@html sanitizeHtml(item.title)}</h3>
                {/if}
            </div>
        </HoverWrapper>
    </LinkWrapper>
</article>

<style>
    article {
        aspect-ratio: 3/4;
    }

    .text-container {
        position: absolute;
        display: flex;
        flex-direction: column;
        justify-content: end;
        height: 100%;
        margin-top: 8px;
        padding: 16px;
        color: var(--systemPrimary);
    }

    h3 {
        z-index: 1;
        text-wrap: pretty;
        font: var(--body-bold);
        color: var(--systemPrimary-onDark);
    }

    h4 {
        position: relative;
        z-index: 1;
        margin-bottom: 2px;
        font: var(--caption-2-emphasized);
        color: var(--systemSecondary-onDark);
        mix-blend-mode: plus-lighter;
    }
</style>
