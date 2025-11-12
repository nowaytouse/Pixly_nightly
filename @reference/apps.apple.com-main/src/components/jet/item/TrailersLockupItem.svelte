<script lang="ts">
    import type { TrailersLockup } from '@jet-app/app-store/api/models';
    import SmallLockup from '~/components/jet/item/SmallLockupItem.svelte';
    import Video from '~/components/jet/Video.svelte';

    export let item: TrailersLockup;

    $: video = item.trailers.videos[0];
</script>

<article>
    {#if video}
        <div class="video-container">
            <Video
                {video}
                shouldSuperimposePosterImage
                loop={true}
                useControls={true}
                profile="app-trailer-lockup-video"
            />
        </div>
    {/if}

    <SmallLockup {item} />
</article>

<style>
    /*
        The video container is explicitly not 16/9 aspect ratio, because a lot trailers have
        pillarboxing (black bars on the sides), so expand the height of their container which
        causes those black bars to overflow outside the container, thus cropping them.
        This follows the iOS pattern. 
    */
    .video-container {
        --app-trailer-lockup-video-aspect-ratio: 16/10;
        aspect-ratio: var(--app-trailer-lockup--video-aspect-ratio);
        margin-bottom: 16px;
        overflow: hidden;
        border-radius: var(--global-border-radius-large);
    }

    /*
        Not all trailers are in a landscape aspect ratio (many iPhone trailers are portrait),
        so for those cases we force them to fit inside a landscape container, centered vertically,
        by using `object-fit: cover;`. 
    */
    .video-container :global(video) {
        aspect-ratio: var(--app-trailer-lockup-video-aspect-ratio);
        object-fit: cover;
    }
</style>
