<script lang="ts" context="module">
    import type {
        TodayCardMedia,
        TodayCardMediaVideo,
        Video as VideoModel,
    } from '@jet-app/app-store/api/models';

    export function isTodayCardMediaVideo(
        media: TodayCardMedia,
    ): media is TodayCardMediaVideo {
        return (
            media.kind === 'video' ||
            (media.kind === 'artwork' && 'videos' in media)
        );
    }
</script>

<script lang="ts">
    import mediaQueries from '~/utils/media-queries';
    import type { Profile as ArtworkProfile } from '~/components/Artwork.svelte';
    import Video from '~/components/jet/Video.svelte';
    import { colorAsString } from '~/utils/color';

    export let media: TodayCardMediaVideo;

    /**
     * A `Profile` to override the default for the card's media
     */
    export let artworkProfile: ArtworkProfile | undefined = undefined;

    let videoToDisplay: VideoModel | undefined;
    $: videoToDisplay = media.videos[0];

    let profile: ArtworkProfile;
    $: profile =
        artworkProfile ??
        ($mediaQueries === 'small' ? 'card' : 'card-horizontal');
    $: backgroundColor = videoToDisplay?.preview.backgroundColor
        ? colorAsString(videoToDisplay?.preview.backgroundColor)
        : null;
</script>

{#if videoToDisplay}
    <div class="video-wrapper" style:--background-color={backgroundColor}>
        <Video
            autoplay
            loop
            {profile}
            useControls={false}
            video={videoToDisplay}
        />
    </div>
{/if}

<style>
    .video-wrapper {
        background: black;
        aspect-ratio: 3/4;
        width: 100%;
        position: relative;
        overflow: hidden;
        border-radius: var(--today-card-border-radius);
        background-color: var(--background-color);
    }

    @container (orientation: landscape) {
        .video-wrapper {
            aspect-ratio: 16/9;
            height: 100%;
        }
    }
</style>
