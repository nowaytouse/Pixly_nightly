<script lang="ts">
    import { createEventDispatcher, onMount, onDestroy } from 'svelte';
    import { loggerFor } from '@amp/web-apps-logger';

    const logger = loggerFor('components/MotionArtwork');

    type HLSError = {
        type: string;
        message: string;
        details: string;
        fatal: boolean;
        handled: boolean;
    };

    type MotionArtworkError = {
        type: string;
        reason: string;
        fatal: boolean;
        error?: Error;
    };

    /** HTML `id` attribute for the <video /> element */
    export let id: string;

    /** Source URL for the video, an HLS playlist ending in .m3u8 */
    export let src: string;

    /** Poster image to show while the video is loading */
    export let poster: string | undefined;

    /** If the video should loop from end to start. */
    export let loop: boolean = true;

    /** If the audio should be muted on the video. */
    export let muted: boolean = true;

    /** If the video should be paused when initially loaded. */
    export let paused: boolean = true;

    /** The constructor to use for creating an Hls playback session. */
    export let HLS: Window['Hls'] = window.Hls;

    /** RTCReportingAgent instance for RTC reporting on video playback. */
    export let reportingAgent: any = undefined;

    /** HTMLVideoElement used by HLS.js to render the video */
    export let videoElement: HTMLVideoElement | null = null;

    /** Internal error state for the component */
    let errorState: MotionArtworkError | undefined;

    let hlsSession: Window['Hls'] | undefined;

    /** Dispatcher for errors. */
    const dispatch = createEventDispatcher<{ error: MotionArtworkError }>();

    function handleError(details: MotionArtworkError) {
        logger.error(
            `Error playing MotionArtwork with HLS: ${details?.reason}`,
            details?.error,
        );

        errorState = {
            type: details.type,
            reason: details.reason,
            fatal: details.fatal,
            error: details?.error,
        };

        dispatch('error', errorState);
    }

    const hlsSupported = HLS?.isSupported() ?? false;

    onMount(function () {
        if (!hlsSupported) {
            handleError({
                type: 'runtime',
                reason: 'unsupported',
                fatal: true,
            });
            return;
        }

        // Create a new HLS.js playback session
        hlsSession = new HLS({
            debug: false,
            debugLevel: 'error',
            enablePerformanceLogging: false,
            nativeControlsEnabled: false,

            appData: {
                reportingAgent: reportingAgent,
                serviceName: reportingAgent?.ServiceName,
            },
        });

        hlsSession.on(
            HLS.Events.ERROR,
            function (_event: string, error: HLSError) {
                handleError({
                    type: 'hls',
                    reason: error.message,
                    fatal: error.fatal,
                    error: error as unknown as Error,
                });
            },
        );

        // Direct HLS.js to the VideoElement to use and start loading the video source
        hlsSession.attachMedia(videoElement);
        hlsSession.loadSource(src, {
            /* HLS.js loading options go here */
        });
    });

    onDestroy(() => {
        // Stop the video, release resources, and destroy the HLS context
        hlsSession?.destroy();
    });
</script>

{#if errorState !== undefined}
    <slot name="error" error={errorState} {poster} />
{:else}
    <!-- svelte-ignore a11y-media-has-caption -->
    <video
        {id}
        {loop}
        {poster}
        preload="none"
        data-loop={true}
        playsinline={true}
        controls={false}
        bind:this={videoElement}
        bind:muted
        bind:paused
        on:play
        on:ended
        on:loadedmetadata
    />
{/if}

<style>
    video {
        width: 100%;
        height: 100%;
        object-fit: cover;
        object-position: center center;
        aspect-ratio: var(--aspect-ratio);
    }
</style>
