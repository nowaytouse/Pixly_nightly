<script lang="ts" context="module">
    import {
        type AppEventDetailShelf,
        isAppEventDetailShelf,
        isFlowAction,
    } from '@jet-app/app-store/api/models';

    export { isAppEventDetailShelf };
</script>

<script lang="ts">
    import type { CropCode } from '@amp/web-app-components/src/components/Artwork/types';
    import { buildSrc } from '@amp/web-app-components/src/components/Artwork/utils/srcset';
    import mediaQueries from '~/utils/media-queries';
    import Artwork from '~/components/Artwork.svelte';
    import Video from '~/components/jet/Video.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import SmallLockupItem from '~/components/jet/item/SmallLockupItem.svelte';
    import GradientOverlay from '~/components/GradientOverlay.svelte';
    import { colorAsString } from '~/utils/color';
    import AppEventDate from '~/components/AppEventDate.svelte';
    import { platform } from '@amp/web-apps-utils';
    import LaunchNativeButton from '~/components/LaunchNativeButton.svelte';

    export let shelf: AppEventDetailShelf;

    $: item = shelf.items[0];
    $: ({ appEvent, artwork: productArtwork, video } = item);
    $: ({ requirements, lockup } = appEvent);
    $: artwork = video ? video.preview : productArtwork;

    $: backgroundImageUrl = artwork
        ? buildSrc(
              artwork.template,
              {
                  crop: artwork.crop as CropCode,
                  width: 200,
                  height: Math.floor(200 / (artwork.width / artwork.height)),
                  fileType: 'webp',
              },
              {},
          )
        : undefined;

    $: backgroundColor = artwork?.backgroundColor
        ? colorAsString(artwork.backgroundColor)
        : '#000';
    $: hasLightArtwork = appEvent.mediaOverlayStyle === 'light';
    $: isXSmallViewport = $mediaQueries === 'xsmall';
    $: clickAction = lockup?.clickAction;
    $: urlToLaunchNatively =
        clickAction && isFlowAction(clickAction) ? clickAction.pageUrl : null;
    $: shouldShowLaunchNativeButton =
        platform.ismacOS() &&
        lockup?.isIOSBinaryMacOSCompatible &&
        !!urlToLaunchNatively;

    function makeCSSURL(url: string | null | undefined): string {
        return url ? `url(${url})` : '';
    }
</script>

<ShelfWrapper {shelf} withBottomPadding={false} centered={false}>
    <div
        class="event-detail"
        style:--background-image-url={makeCSSURL(backgroundImageUrl)}
        style:--background-color={backgroundColor}
    >
        {#if video}
            <div class="video-container">
                <Video
                    {video}
                    autoplay
                    loop
                    useControls={false}
                    profile="app-event-detail"
                />
            </div>
        {:else if artwork}
            <div class="artwork-container">
                <Artwork {artwork} profile="app-event-detail" />
            </div>
        {/if}

        {#if isXSmallViewport}
            <div class="gradient-container">
                <GradientOverlay
                    --color={backgroundColor}
                    --height="70%"
                    shouldDarken={!hasLightArtwork}
                />
            </div>
        {:else}
            <div class="tint-container" />
        {/if}

        <div class="time-container">
            <AppEventDate {appEvent} />
        </div>

        <div
            class="text-container"
            class:dark={hasLightArtwork && isXSmallViewport}
        >
            <div class="event-details-container">
                <p class="app-event-kind">{appEvent.kind}</p>
                <h1 class="app-event-title">{appEvent.title}</h1>
                <p class="app-event-subtitle">
                    {appEvent.detail}
                </p>
                {#if requirements}
                    <span class="requirements">{requirements}</span>
                {/if}
            </div>

            {#if lockup}
                <div class="lockup-container">
                    <SmallLockupItem
                        {shouldShowLaunchNativeButton}
                        item={lockup}
                        buttonVariant="transparent"
                        appIconProfile="app-icon"
                    >
                        <svelte:fragment slot="launch-native-button">
                            {#if urlToLaunchNatively}
                                <LaunchNativeButton url={urlToLaunchNatively} />
                            {/if}
                        </svelte:fragment>
                    </SmallLockupItem>
                </div>
            {/if}
        </div>
    </div>
</ShelfWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;

    .event-detail {
        --event-image-desktop-width: 31.64%;
        --border-radius: 16px;
        --event-gutter: 16px;
        border-radius: var(--border-radius);
        display: grid;
        grid-template-areas:
            'time'
            'text';
        grid-template-rows: 1fr auto;
        aspect-ratio: 9/16;
        max-height: 90vh;
        overflow: hidden;
        position: relative;

        @media (--range-small-up) {
            --event-gutter: 20px;
            aspect-ratio: 16/9;
            background-image: var(--background-image-url);
            background-position-x: 50%;
            background-position-y: 50%;
            background-size: cover;
            grid-template-areas:
                'image time'
                'image text';
            grid-template-columns: var(--event-image-desktop-width) auto;
            grid-template-rows: auto 1fr;
        }
    }

    .artwork-container,
    .video-container {
        z-index: 1;

        /* On "mobile" the artwork should be behind both the time and text */
        grid-row-start: time;
        grid-row-end: text;
        grid-column: 1;

        @media (--range-small-up) {
            /* On large screens, it should be to the right of the text */
            grid-area: image;
        }
    }

    .video-container {
        background: var(--background-color);
        color: transparent;
    }

    .video-container :global(video) {
        width: unset;
        position: absolute;
    }

    .tint-container {
        background: var(--systemTertiary-onLight_IC);
        backdrop-filter: saturate(120%) blur(24px);
        z-index: 1;

        /* One smaller screens, extend behind just the text */
        grid-area: text;

        /* On larger screens, extend behind time and text */
        grid-row-start: time;
        grid-row-end: text;
    }

    .time-container {
        grid-area: time;
        margin-top: var(--event-gutter);
        margin-inline-start: var(--event-gutter);
    }

    .time-container :global(time) {
        color: var(--systemPrimary-onLight);
        font: var(--callout-emphasized);
        padding: 3px 10px;
        background-color: var(--systemSecondary-onDark);
        border-radius: var(--global-border-radius-medium);
        position: relative;
        z-index: 3;

        @media (--range-small-up) {
            position: relative;
            z-index: 3;
            mix-blend-mode: plus-lighter;
        }
    }

    .text-container {
        --blend-mode: plus-lighter;
        --text-color: var(--systemPrimary-onDark);
        padding: var(--event-gutter);

        /* Placement within parent */
        grid-area: text;

        /* Layout of child elements */
        display: flex;
        flex-direction: column;
        gap: 16px;
        justify-content: space-between;
        color: var(--text-color);
    }

    .text-container.dark {
        --blend-mode: normal;
        --text-color: var(--systemPrimary-onLight);
    }

    .event-details-container {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .app-event-kind {
        font: var(--callout-emphasized);
        mix-blend-mode: var(--blend-mode);
        z-index: 1;
    }

    .app-event-title {
        font: var(--large-title-emphasized);
        text-wrap: pretty;
        z-index: 1;
    }

    .app-event-subtitle {
        font: var(--title-3);
        z-index: 1;
    }

    .requirements {
        position: relative;
        z-index: 1;
        font: var(--body-emphasized);
    }

    .lockup-container {
        --title-color: var(--text-color);
        --subtitle-color: var(--text-color);
        --eyebrow-color: var(--text-color);
        --linkColor: var(--text-color);
        --button-blend-mode: var(--blend-mode);
        border-top: 1px solid var(--systemQuaternary-onDark);
        padding-top: 16px;
        z-index: 1;
    }
</style>
