<script lang="ts" context="module">
    import { isSome } from '@jet/environment/types/optional';
    import type {
        TodayCard,
        TodayCardMediaWithArtwork,
    } from '@jet-app/app-store/api/models';

    import { isTodayCardMediaWithArtwork } from '~/components/jet/today-card/media/TodayCardMediaWithArtwork.svelte';

    export interface SmallStoryCardWithMedia extends TodayCard {
        media: TodayCardMediaWithArtwork;
        heroMedia: TodayCardMediaWithArtwork;
    }

    export function isSmallStoryCardWithMediaItem(
        item: TodayCard,
    ): item is SmallStoryCardWithMedia {
        return isSome(item.media);
    }
</script>

<script lang="ts">
    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import Artwork from '~/components/Artwork.svelte';
    import HoverWrapper from '~/components/HoverWrapper.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';

    export let item: SmallStoryCardWithMedia;

    $: artwork = (() => {
        if (item.heroMedia) {
            return item.heroMedia?.artworks?.[0];
        }

        if (isTodayCardMediaWithArtwork(item.media)) {
            return item.media.artworks?.[0];
        }

        return null;
    })();
</script>

<article>
    <LinkWrapper action={item.clickAction}>
        <HoverWrapper element="div">
            {#if artwork}
                <div class="artwork-container">
                    <Artwork
                        {artwork}
                        profile={item.heroMedia
                            ? 'small-story-card'
                            : 'small-story-card-legacy'}
                        useCropCodeFromArtwork={!item.heroMedia}
                    />
                </div>
            {/if}
        </HoverWrapper>

        <div class="text-container">
            <h4>{item.heading}</h4>
            <LineClamp clamp={1}>
                <h3>{item.title}</h3>
            </LineClamp>

            {#if item.inlineDescription}
                <LineClamp clamp={1}>
                    <p>{item.inlineDescription}</p>
                </LineClamp>
            {/if}
        </div>
    </LinkWrapper>
</article>

<style>
    .artwork-container {
        width: 100%;
        aspect-ratio: 16 / 9;
        background-color: var(--color);
        border-radius: 8px;
    }

    .text-container {
        display: flex;
        margin-top: 8px;
        gap: 4px;
        color: var(--systemPrimary);
        flex-direction: column;
    }

    h3 {
        font: var(--title-3);
    }

    h4 {
        font: var(--callout-emphasized);
        color: var(--systemTertiary);
    }

    p {
        font: var(--body-tall);
        color: var(--systemSecondary);
        text-wrap: pretty;
    }
</style>
