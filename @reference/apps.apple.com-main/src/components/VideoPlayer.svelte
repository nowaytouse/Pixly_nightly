<script lang="ts">
    import { intersectionObserver } from '@amp/web-app-components/src/actions/intersection-observer';
    import MotionArtwork from '~/components/MotionArtwork.svelte';
    import { getJet } from '~/jet';
    import { getI18n } from '~/stores/i18n';
    import type { Video } from '@jet-app/app-store/api/models';
    import {
        MetricsActionDetails,
        MetricsActionType,
        type MetricsActionDetailItem,
        type MetricsActionTypeItem,
    } from '~/constants/media-metrics';

    /** HTML `id` attribute for the <video /> element */
    export let id: string;

    /** Source URL for the video, an HLS playlist ending in .m3u8 */
    export let src: string;

    /** Poster image to show while the video is loading */
    export let poster: string | undefined;

    /** If the video should play automatically when in view */
    export let autoplay: boolean = false;

    /* The whole-number percentage amount of the video needs to be in view before autoplay kicks in */
    export let autoplayVisibilityThreshold: number = 0;

    /** If the video should loop from end to start. */
    export let loop: boolean = false;

    /** If the audio should be muted on the video. */
    export let muted: boolean = true;

    /** If our controls should be shown in the video player. */
    export let useControls: boolean = true;

    /** The constructor to use for creating an Hls playback session. */
    export let HLS: Window['Hls'] = window.Hls;

    /**
     * If we should bypass the `poster` attribute on the `video` tag, in favor of having the poster
     * image overlaid as it's own DOM element, which covers an HLS playback bug in Safari, wherein
     * the video is seeked to the first frame once the metadata is loaded, thus removing the poster.
     */
    export let shouldSuperimposePosterImage: boolean = false;

    /** an optional metric template provided by jet */
    export let metricsTemplate:
        | Record<string, unknown>
        | Video['templateMediaEvent'] = {};

    export function play(isAutoPlay = true) {
        videoRef?.play();
        recordMediaEvent(
            MetricsActionType.PLAY,
            isAutoPlay
                ? MetricsActionDetails.AUTOPLAY
                : MetricsActionDetails.PLAY,
        );
    }

    export function pause(isAutoPause = true) {
        recordMediaEvent(
            MetricsActionType.STOP,
            isAutoPause
                ? MetricsActionDetails.AUTOPAUSE
                : MetricsActionDetails.PAUSE,
        );

        videoRef?.pause();
    }

    let isPaused: boolean = !autoplay;
    let isMuted: boolean = muted;
    let shouldShowReplayControl: boolean = false;
    let shouldShowPlaybackControls: boolean = true;
    let hasPlaybackBeenInitiated: boolean = false;
    let videoRef: HTMLVideoElement | null = null;

    const i18n = getI18n();
    const jet = getJet();

    const handleFullScreenButtonClick = () => {
        videoRef?.requestFullscreen();
    };

    const handleReplayButtonClick = () => {
        if (videoRef) {
            videoRef.currentTime = 0;
            videoRef.play();
            shouldShowPlaybackControls = true;
        }
    };

    const handlePlayButtonClick = () => {
        if (isPaused) {
            play(false);
        } else {
            pause(false);
        }
    };

    const handleMuteButtonClick = () => {
        isMuted = !isMuted;
    };

    const handleVideoEnded = () => {
        if (!loop) {
            shouldShowPlaybackControls = true;

            if (videoRef) {
                videoRef.currentTime = 1;
                videoRef.pause();
            }

            recordMediaEvent(
                MetricsActionType.STOP,
                MetricsActionDetails.COMPLETE,
            );
        }
    };

    const handleVideoPlay = () => {
        // Display the replay button after the first play
        shouldShowReplayControl = true;
        hasPlaybackBeenInitiated = true;
    };

    // metric events that are waiting for loadMetadata from video element
    let queuedMetricEvents: Array<() => void> = [];

    // flush any metric events once load metadata has been called
    const flushMetricEvents = () => {
        queuedMetricEvents.forEach((recordFn) => recordFn());

        queuedMetricEvents = [];
    };

    const recordMediaEvent = (
        actionType: MetricsActionTypeItem,
        actionDetail: MetricsActionDetailItem,
    ) => {
        if (!metricsTemplate?.fields) {
            return;
        }

        const recordEvent = () => {
            const duration = Math.floor(videoRef?.duration ?? 0) * 1000;
            const position = Math.min(
                Math.floor((videoRef?.currentTime ?? 0) * 1000),
                duration,
            );
            jet.recordCustomMetricsEvent({
                ...(metricsTemplate?.fields ?? {}),
                actionType: actionType,
                actionDetails: actionDetail,
                url: src,
                duration,
                position,
                topic: metricsTemplate?.topic ?? '',
            });
        };

        if (Number.isNaN(videoRef?.duration)) {
            queuedMetricEvents.push(() => recordEvent());
        } else {
            recordEvent();
        }
    };

    const isVideoPlaying = (video: HTMLVideoElement | null) => {
        if (!video) {
            return false;
        }
        return !!(
            video.currentTime > 0 &&
            !video.paused &&
            !video.ended &&
            video.readyState > 2
        );
    };

    const intersectionObserverConfig = {
        threshold: autoplayVisibilityThreshold,
        callback: (isIntersectingViewport: boolean) => {
            if (isIntersectingViewport) {
                play();
            } else if (isVideoPlaying(videoRef)) {
                pause();
            }
        },
    };
</script>

<div
    class="video-container"
    use:intersectionObserver={autoplay ? intersectionObserverConfig : undefined}
>
    <div class="video">
        <MotionArtwork
            {id}
            {HLS}
            {src}
            {loop}
            poster={!shouldSuperimposePosterImage ? poster : undefined}
            bind:muted={isMuted}
            bind:paused={isPaused}
            bind:videoElement={videoRef}
            on:play={handleVideoPlay}
            on:ended={handleVideoEnded}
            on:loadedmetadata={flushMetricEvents}
        />
    </div>

    {#if shouldSuperimposePosterImage && !hasPlaybackBeenInitiated}
        <img
            src={poster}
            class="fake-poster"
            aria-hidden="true"
            loading="lazy"
            alt=""
        />
    {/if}

    {#if useControls}
        <div class="video-control">
            {#if shouldShowReplayControl}
                <button
                    class="video-control-replay"
                    aria-label={$i18n.t(
                        'ASE.Web.AppStore.VideoPlayer.AX.Replay',
                    )}
                    on:click={handleReplayButtonClick}
                >
                    <img
                        class="btn-img"
                        src="/assets/images/video-control/video-control-replay.png"
                        alt={$i18n.t('ASE.Web.AppStore.VideoPlayer.AX.Replay')}
                        aria-hidden="true"
                    />
                </button>
            {/if}

            {#if shouldShowPlaybackControls}
                <div class="video-control-playback">
                    <button
                        class="video-control-play"
                        aria-label={$i18n.t(
                            isPaused
                                ? 'ASE.Web.AppStore.VideoPlayer.AX.Play'
                                : 'ASE.Web.AppStore.VideoPlayer.AX.Pause',
                        )}
                        on:click={handlePlayButtonClick}
                    >
                        {#if isPaused}
                            <img
                                class="btn-img"
                                src="/assets/images/video-control/video-control-play.png"
                                alt={$i18n.t(
                                    'ASE.Web.AppStore.VideoPlayer.AX.Play',
                                )}
                                aria-hidden="true"
                            />
                        {:else}
                            <img
                                class="btn-img"
                                src="/assets/images/video-control/video-control-pause.png"
                                alt={$i18n.t(
                                    'ASE.Web.AppStore.VideoPlayer.AX.Pause',
                                )}
                                aria-hidden="true"
                            />
                        {/if}
                    </button>

                    <button
                        class="video-control-unmute"
                        aria-label={$i18n.t(
                            isMuted
                                ? 'ASE.Web.AppStore.VideoPlayer.AX.Unmute'
                                : 'ASE.Web.AppStore.VideoPlayer.AX.Mute',
                        )}
                        on:click={handleMuteButtonClick}
                    >
                        {#if isMuted}
                            <img
                                class="btn-img"
                                src="/assets/images/video-control/video-control-volume-muted.png"
                                alt={$i18n.t(
                                    'ASE.Web.AppStore.VideoPlayer.AX.Mute',
                                )}
                                aria-hidden="true"
                            />
                        {:else}
                            <img
                                class="btn-img"
                                src="/assets/images/video-control/video-control-volume.png"
                                alt={$i18n.t(
                                    'ASE.Web.AppStore.VideoPlayer.AX.Unmute',
                                )}
                                aria-hidden="true"
                            />
                        {/if}
                    </button>

                    <button
                        class="video-control-fullscreen"
                        aria-label={$i18n.t(
                            'ASE.Web.AppStore.VideoPlayer.AX.Fullscreen',
                        )}
                        on:click={handleFullScreenButtonClick}
                    >
                        <img
                            class="btn-img"
                            src="/assets/images/video-control/video-control-fullscreen.png"
                            alt={$i18n.t(
                                'ASE.Web.AppStore.VideoPlayer.AX.Fullscreen',
                            )}
                            aria-hidden="true"
                        />
                    </button>
                </div>
            {/if}
        </div>
    {/if}
</div>

<style>
    .video-container {
        --button-size: 32px;
        display: grid;
        position: relative;
        container-type: inline-size;
        container-name: video-container;
        width: 100%;
        height: 100%;
        background-color: var(--systemQuaternary);
    }

    .video {
        width: 100%;
        height: 100%;
        grid-column: 1;
        grid-row: 1;
        line-height: 0;
    }

    .video-control {
        grid-column: 1;
        grid-row: 1;
        display: inline-flex;
        justify-content: space-between;
        z-index: 1;
        align-self: end;
        color: white;
        margin: 0 12px 12px;
    }

    .video-control::after {
        position: absolute;
        content: '';
        z-index: -1;
        bottom: 0;
        left: 0;
        display: block;
        box-sizing: border-box;
        width: 100%;
        height: calc(var(--button-size) * 2);
        background: linear-gradient(
            0deg,
            rgb(0, 0, 0, 0.68),
            rgb(0, 0, 0, 0.2),
            transparent
        );
        mask-image: linear-gradient(360deg, #000 47%, transparent);
    }

    .video-control-playback {
        display: inline-flex;
        margin-inline-start: auto;
        gap: 6px;
    }

    .btn-img {
        height: var(--button-size);
        width: var(--button-size);
        border-radius: 50%;
        border: 1px solid var(--systemQuaternary-onDark);
        background: rgba(0, 0, 0, 0.11);
        backdrop-filter: blur(20px);
        object-fit: cover;
        transition: background 105ms ease-out;
    }

    .btn-img:hover {
        background: rgba(0, 0, 0, 0.05);
    }

    @container video-container (max-width: 500px) {
        .btn-img {
            --button-size: 24px;
        }
    }

    .fake-poster {
        width: 100%;
        position: absolute;
        top: 0;
        left: 0;
    }
</style>
