<script lang="ts">
    import type { Video } from '@jet-app/app-store/api/models';
    import VideoPlayer from '../VideoPlayer.svelte';
    import HlsJsDecorator from '../decorators/HlsJSDecorator.svelte';
    import { buildPoster } from '~/utils/video-poster';
    import { generateUuid } from '@amp/web-apps-utils/src';
    import type { NamedProfile } from 'src/config/components/artwork';
    import type { Profile } from '@amp/web-app-components/src/components/Artwork/types';
    import mediaQueries from '~/utils/media-queries';
    import { colorAsString } from '~/utils/color';

    export let video: Video;
    export let autoplay: boolean = false;
    export let loop: boolean = false;
    export let muted: boolean = true;
    export let useControls: boolean = true;
    export let profile: NamedProfile | Profile;
    export let autoplayVisibilityThreshold: number = 0;
    export let videoPlayerRef: InstanceType<typeof VideoPlayer> | null = null;
    export let shouldSuperimposePosterImage: boolean = false;

    $: poster =
        video.preview && buildPoster(video.preview, profile, $mediaQueries);
    $: backgroundColor = video.preview.backgroundColor
        ? colorAsString(video.preview.backgroundColor)
        : '#f1f1f1';

    $: metricsTemplate = video?.templateMediaEvent ?? {};
    const uuid = generateUuid();
</script>

<HlsJsDecorator let:HLS>
    <VideoPlayer
        {HLS}
        {loop}
        {muted}
        {autoplay}
        {useControls}
        {autoplayVisibilityThreshold}
        {metricsTemplate}
        {shouldSuperimposePosterImage}
        id={uuid}
        src={video.videoUrl}
        poster={poster ?? undefined}
        --aspect-ratio={video.preview.width / video.preview.height}
        bind:this={videoPlayerRef}
    />

    <div
        class="loader"
        slot="loading-component"
        style:--aspect-ratio={video.preview.width / video.preview.height}
        style:--background-image={`url(${poster})`}
        style:--background-color={backgroundColor}
    />
</HlsJsDecorator>

<style>
    .loader {
        aspect-ratio: var(--aspect-ratio);
        width: 100%;
        background-image: var(--background-image);
        background-color: var(--background-color);
        background-size: cover;
    }
</style>
