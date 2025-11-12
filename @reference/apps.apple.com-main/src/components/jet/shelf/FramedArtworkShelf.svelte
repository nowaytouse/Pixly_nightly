<script lang="ts" context="module">
    import type { FramedArtwork, Shelf } from '@jet-app/app-store/api/models';

    interface FramedArtworkShelf extends Shelf {
        contentType: 'framedArtwork';
        items: [FramedArtwork];
    }

    export function isFramedArtworkShelf(
        shelf: Shelf,
    ): shelf is FramedArtworkShelf {
        return (
            shelf.contentType === 'framedArtwork' && Array.isArray(shelf.items)
        );
    }
</script>

<script lang="ts">
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';
    import Artwork, { getNaturalProfile } from '~/components/Artwork.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: FramedArtworkShelf;

    $: item = shelf.items[0];
    $: ({ artwork, caption, hasRoundedCorners } = item);
    $: profile = getNaturalProfile(artwork, [1275, 1185, 825, 500, 690]);
    $: aspectRatio = artwork.width / artwork.height;
    $: isPortrait = aspectRatio < 1;
</script>

<ShelfWrapper {shelf} withBottomPadding={false}>
    <figure
        class="framed-artwork-item"
        class:has-rounded-corners={hasRoundedCorners}
        class:is-portrait={isPortrait}
    >
        <div
            class="artwork-container"
            style:--aspect-ratio={artwork.width / artwork.height}
        >
            <Artwork {artwork} {profile} forceFullWidth={!isPortrait} />
        </div>

        {#if caption}
            <figcaption class="caption">
                {@html sanitizeHtml(caption)}
            </figcaption>
        {/if}
    </figure>
</ShelfWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/app/core/globalvars' as *;

    .framed-artwork-item {
        border-radius: var(--framed-artwork-border-radius);
        padding: 0 20px;
        overflow: hidden;

        @media (--sidebar-visible) {
            padding: 0 20px;
        }

        @media (--range-small-only) {
            padding: 0 var(--bodyGutter);
        }
    }

    .framed-artwork-item.has-rounded-corners {
        --framed-artwork-border-radius: var(--global-border-radius-medium);
    }

    .artwork-container {
        border-radius: inherit;
    }

    .caption {
        border-bottom-left-radius: var(--framed-artwork-border-radius);
        border-bottom-right-radius: var(--framed-artwork-border-radius);
        color: var(--systemSecondary);
        padding: 8px var(--article-page-padding) 0;
    }

    .framed-artwork-item.is-portrait {
        --artwork-override-max-height: 560px;
        --artwork-override-max-width: 100%;
        --artwork-override-width: auto;
    }

    .framed-artwork-item.framed-artwork-item.is-portrait .artwork-container {
        display: flex;
        justify-content: center;
        width: 100%;
        max-height: var(--artwork-override-max-height);
        aspect-ratio: var(--aspect-ratio);
    }
</style>
