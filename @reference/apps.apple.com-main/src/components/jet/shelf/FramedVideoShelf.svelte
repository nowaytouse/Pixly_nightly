<script lang="ts" context="module">
    import type { FramedVideo, Shelf } from '@jet-app/app-store/api/models';

    interface FramedVideoShelf extends Shelf {
        contentType: 'framedArtwork';
        items: [FramedVideo];
    }

    export function isFramedVideoShelf(
        shelf: Shelf,
    ): shelf is FramedVideoShelf {
        return (
            shelf.contentType === 'framedVideo' && Array.isArray(shelf.items)
        );
    }
</script>

<script lang="ts">
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import { getNaturalProfile } from '~/components/Artwork.svelte';
    import Video from '~/components/jet/Video.svelte';

    export let shelf: FramedVideoShelf;

    $: ({ caption, video } = shelf.items[0]);
    $: aspectRatio = video.preview.width / video.preview.height;
    $: profile = getNaturalProfile(video.preview, [608, 528, 608, 928, 298]);
    $: isPortrait = aspectRatio < 1;
</script>

<ShelfWrapper {shelf} withBottomPadding={false}>
    <figure class="framed-artwork-item" class:is-portrait={isPortrait}>
        <div class="artwork-container" style:--aspect-ratio={aspectRatio}>
            <Video {video} {profile} autoplay />
        </div>

        {#if caption}
            <figcaption class="caption">
                {@html sanitizeHtml(caption)}
            </figcaption>
        {/if}
    </figure>
</ShelfWrapper>

<style>
    .framed-artwork-item {
        border-radius: var(--global-border-radius-medium);
        padding: 0 20px;
        overflow: hidden;

        @media (--sidebar-visible) {
            padding: 0 20px;
        }

        @media (--range-small-only) {
            padding: 0 var(--bodyGutter);
        }
    }

    .artwork-container {
        aspect-ratio: var(--aspect-ratio);
        overflow: hidden;
        line-height: 0;
        border-radius: var(--global-border-radius-medium);
        background-color: var(--systemQuaternary);
        max-height: 560px;
        max-width: 100%;
        margin: 0 auto;
    }

    .caption {
        border-bottom-left-radius: var(--global-border-radius-medium);
        border-bottom-right-radius: var(--global-border-radius-medium);
        color: var(--systemSecondary);
        padding: 8px var(--article-page-padding) 0;
    }
</style>
