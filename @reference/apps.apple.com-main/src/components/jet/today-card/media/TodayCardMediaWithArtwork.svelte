<script lang="ts" context="module">
    import type {
        Artwork as ArtworkModel,
        TodayCardMedia,
        TodayCardMediaWithArtwork,
    } from '@jet-app/app-store/api/models';

    export function isTodayCardMediaWithArtwork(
        media: TodayCardMedia,
    ): media is TodayCardMediaWithArtwork {
        return (
            media.kind === 'artwork' &&
            'artworks' in media &&
            Array.isArray(media.artworks) &&
            media.artworks.length > 0
        );
    }
</script>

<script lang="ts">
    import Artwork, {
        type Profile as ArtworkProfile,
    } from '~/components/Artwork.svelte';

    export let media: TodayCardMediaWithArtwork;

    export let pinArtworkToLeft: boolean = false;

    /**
     * A `Profile` to override the default for the card's media
     */
    export let artworkProfile: ArtworkProfile | undefined = undefined;

    let artworkToDisplay: ArtworkModel;
    // Today Card artwork comes back from Jet with a width of 800px, even though the source artwork
    // is _much_ larger. The shared `Artwork` component doesn't let us render an image beyond the
    // artwork's `width` and `height` properties, and we absolutely need to render these images
    // larger than 800px wide, so we are forcing these new upper bounds for the artworks dimensions.
    // Eventually, we should rethink this and have the proper dimensions come back from Jet:
    // rdar://148730199 (Bigger images for TodayCard)
    $: artworkToDisplay = Object.assign({}, media.artworks[0], {
        width: 3840,
        height: 2160,
    });
</script>

{#if artworkProfile}
    <Artwork profile={artworkProfile} artwork={artworkToDisplay} />
{:else}
    <div class="wrapper">
        <div class="artwork-container portrait">
            <Artwork profile="card" artwork={artworkToDisplay} />
        </div>

        <div
            class="artwork-container landscape"
            class:pinned-to-left={pinArtworkToLeft}
        >
            <Artwork profile="card-horizontal" artwork={artworkToDisplay} />
        </div>
    </div>
{/if}

<style>
    .wrapper,
    .artwork-container {
        height: 100%;
        width: 100%;
    }

    .wrapper .artwork-container :global(.artwork-component),
    .wrapper .artwork-container :global(img) {
        object-fit: cover;
        height: 100%;
    }

    .pinned-to-left {
        --artwork-override-object-position: left;
    }

    @container (orientation: landscape) {
        .artwork-container.landscape {
            display: block;
        }

        .artwork-container.portrait {
            display: none;
        }
    }

    @container (orientation: portrait) {
        .artwork-container.landscape {
            display: none;
        }

        .artwork-container.portrait {
            display: block;
        }
    }
</style>
