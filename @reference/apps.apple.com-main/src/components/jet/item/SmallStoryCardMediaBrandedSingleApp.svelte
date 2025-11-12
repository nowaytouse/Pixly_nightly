<script lang="ts" context="module">
    import type {
        TodayCard,
        TodayCardMediaBrandedSingleApp,
    } from '@jet-app/app-store/api/models';

    export interface SmallStoryCardMediaBrandedSingleApp extends TodayCard {
        media: TodayCardMediaBrandedSingleApp;
    }

    export function isSmallStoryCardMediaBrandedSingleApp(
        item: TodayCard,
    ): item is SmallStoryCardMediaBrandedSingleApp {
        return !!item.media && item.media.kind === 'brandedSingleApp';
    }
</script>

<script lang="ts">
    import Artwork from '~/components/Artwork.svelte';
    import HoverWrapper from '~/components/HoverWrapper.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';

    export let item: SmallStoryCardMediaBrandedSingleApp;

    $: artwork = item.media.artworks?.[0] || item.media.icon;
</script>

<article>
    <LinkWrapper action={item.clickAction}>
        <HoverWrapper element="div">
            <Artwork {artwork} profile="brick" useCropCodeFromArtwork={false} />
        </HoverWrapper>

        <div class="text-container">
            <h4>{item.heading}</h4>
            <h3>{item.title}</h3>
            <p>{item.inlineDescription}</p>
        </div>
    </LinkWrapper>
</article>

<style>
    article {
        aspect-ratio: 16/9;
    }

    .text-container {
        gap: 4px;
        display: flex;
        flex-direction: column;
        margin-top: 8px;
    }

    h3 {
        font: var(--title-3);
    }

    h4 {
        margin-bottom: 2px;
        font: var(--callout-emphasized);
        color: var(--systemSecondary);
    }

    p {
        font: var(--body-tall);
        color: var(--systemSecondary);
        text-wrap: pretty;
    }
</style>
