<script lang="ts" context="module">
    import type {
        AppSearchResult,
        AppEventSearchResult,
        SearchResult,
        Trailers,
        Screenshots,
        FlowAction,
        Artwork as ArtworkType,
        Video as VideoType,
    } from '@jet-app/app-store/api/models';

    export function isAppSearchResult(
        result: SearchResult,
    ): result is AppSearchResult {
        return result.resultType === 'content';
    }

    export function isAppEventSearchResult(
        result: SearchResult,
    ): result is AppEventSearchResult {
        return result.resultType === 'appEvent';
    }
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import type {
        ImageSizes,
        Profile,
    } from '@amp/web-app-components/src/components/Artwork/types';
    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import { sidebarIsHidden } from '@amp/web-app-components/src/stores/sidebar-hidden';

    import type { NamedProfile } from '~/config/components/artwork';
    import { getI18n } from '~/stores/i18n';
    import AppIcon, {
        doesAppIconNeedBorder,
    } from '~/components/AppIcon.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import StarRating from '~/components/StarRating.svelte';
    import Artwork, { getNaturalProfile } from '~/components/Artwork.svelte';
    import Video from '~/components/jet/Video.svelte';
    import SFSymbol from '~/components/SFSymbol.svelte';
    import { isNamedColor } from '~/utils/color';
    import mediaQueries from '~/utils/media-queries';
    import VideoPlayer from '~/components/VideoPlayer.svelte';

    const i18n = getI18n();

    export let item: AppSearchResult;

    $: ({
        clickAction,
        heading,
        isEditorsChoice,
        rating,
        ratingCount,
        screenshots,
        subtitle,
        title,
        trailers,
    } = item.lockup);
    let video: VideoType | undefined;
    let media: (ArtworkType | VideoType)[];
    let mediaAspectRatio: number;
    let numberOfMediaToShow: number;
    let profile: NamedProfile | Profile;
    let mediaSizes: ImageSizes;
    let videoPlayerInstance: InstanceType<typeof VideoPlayer> | null = null;
    let shouldAutoplayVideo: boolean = false;

    const currentPlatform =
        (item.lockup.clickAction as FlowAction).destination?.platform ?? '';

    function isForCurrentPlatform(media: Trailers | Screenshots) {
        return media.mediaPlatform.appPlatform === currentPlatform;
    }

    $: {
        const selectedTrailer =
            trailers?.find(isForCurrentPlatform) ?? trailers?.[0];
        video = selectedTrailer?.videos?.[0];

        const selectedScreenshot =
            screenshots.find(isForCurrentPlatform) ?? screenshots[0];

        const firstMedia = video
            ? video.preview
            : selectedScreenshot.artwork[0];
        const hasPortraitMedia = firstMedia.width < firstMedia.height;
        const isMobile = $mediaQueries === 'xsmall' && $sidebarIsHidden;

        mediaAspectRatio = firstMedia.width / firstMedia.height;

        if (!hasPortraitMedia) {
            numberOfMediaToShow = 1;
            mediaSizes = isMobile ? [308] : [648, 417, 417, 656];
        } else if (currentPlatform !== 'iphone') {
            numberOfMediaToShow = 2;
            mediaSizes = isMobile ? [150] : [238, 203, 203, 320];
        } else {
            numberOfMediaToShow = 3;
            mediaSizes = isMobile ? [98] : [156, 133, 133, 210];
        }

        profile = getNaturalProfile(firstMedia, mediaSizes);
        media = [video, ...selectedScreenshot.artwork]
            .filter(Boolean)
            .slice(0, numberOfMediaToShow) as (ArtworkType | VideoType)[];
    }

    function handleMouseEnter() {
        videoPlayerInstance?.play();
    }

    function handleMouseLeave() {
        videoPlayerInstance?.pause();
    }

    onMount(() => {
        shouldAutoplayVideo = navigator.maxTouchPoints > 0;
    });
</script>

<LinkWrapper
    action={clickAction}
    label={`${$i18n.t('ASE.Web.AppStore.View')} ${clickAction.title}`}
>
    <article on:mouseenter={handleMouseEnter} on:mouseleave={handleMouseLeave}>
        <div class="top-container">
            {#if item.lockup.icon}
                <div class="app-icon-container">
                    <AppIcon
                        icon={item.lockup.icon}
                        profile="app-icon"
                        withBorder={doesAppIconNeedBorder(item.lockup.icon)}
                    />
                </div>
            {/if}

            <div class="metadata-container">
                {#if heading}
                    <LineClamp clamp={1}>
                        <h4>{heading}</h4>
                    </LineClamp>
                {/if}

                <LineClamp clamp={1}>
                    <h3>{title}</h3>
                </LineClamp>

                <LineClamp clamp={1}>
                    <p>{subtitle}</p>
                </LineClamp>

                {#if isEditorsChoice}
                    <div class="editors-choice-badge-container">
                        <SFSymbol name="laurel.leading" ariaHidden={true} />

                        {$i18n.t('ASE.Web.AppStore.Review.EditorsChoice')}

                        <SFSymbol name="laurel.trailing" ariaHidden={true} />
                    </div>
                {:else if ratingCount}
                    <span class="rating-container">
                        <StarRating
                            {rating}
                            --fill-color="var(--systemGray2-onDark_IC)"
                        />
                        {ratingCount}
                    </span>
                {/if}
            </div>

            <div class="button-container">
                <span class="get-button gray">
                    {$i18n.t('ASE.Web.AppStore.View')}
                </span>
            </div>
        </div>

        <div
            class="artwork-container {currentPlatform}"
            style:--media-aspect-ratio={mediaAspectRatio}
        >
            {#each media as mediaItem}
                {#if 'videoUrl' in mediaItem}
                    <div class="video-wrapper">
                        <Video
                            {profile}
                            loop
                            video={mediaItem}
                            autoplay={shouldAutoplayVideo}
                            useControls={false}
                            autoplayVisibilityThreshold={0.75}
                            bind:videoPlayerRef={videoPlayerInstance}
                        />
                    </div>
                {:else}
                    <Artwork
                        {profile}
                        artwork={mediaItem}
                        disableAutoCenter={true}
                        useCropCodeFromArtwork={false}
                    />
                {/if}
            {/each}
        </div>
    </article>
</LinkWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;

    article {
        display: flex;
        align-items: stretch;
        flex-direction: column;
        padding: 16px;
        border-radius: 28px;
        box-shadow: var(--shadow-medium);
        background: #fff;
        transition: box-shadow 210ms ease-out;
        width: 100%;

        @media (prefers-color-scheme: dark) {
            background: var(--systemQuaternary);
        }
    }

    article:hover {
        box-shadow: 0 5px 28px rgba(0, 0, 0, 0.12);
    }

    .top-container {
        align-items: center;
        width: 100%;
        padding-bottom: 16px;
        gap: 8px;
    }

    .top-container,
    .metadata-container {
        display: flex;
    }

    .metadata-container {
        flex-direction: column;
        flex-grow: 1;
    }

    .rating-container {
        display: flex;
        align-items: center;
        font: var(--subhead-emphasized);
        color: var(--systemSecondary);
    }

    .rating-container :global(svg) {
        @media (prefers-contrast: more) and (prefers-color-scheme: dark) {
            --fill-color: #fff;
        }
    }

    .editors-choice-badge-container {
        display: flex;
        align-items: center;
        gap: 4px;
        font: var(--caption-1-emphasized);
        color: var(--systemSecondary);
    }

    .editors-choice-badge-container :global(svg) {
        height: 14px;
        overflow: visible;

        @include rtl {
            transform: rotateY(180deg);
        }
    }

    .editors-choice-badge-container :global(svg path) {
        fill: var(--systemSecondary);
    }

    h3 {
        font: var(--headline);
    }

    h4 {
        color: var(--systemSecondary);
        font: var(--footnote-emphasized);
        text-transform: uppercase;
    }

    p {
        font: var(--callout);
        color: var(--systemSecondary);
    }

    .artwork-container {
        --container-aspect-ratio: 1.333;
        --artwork-override-object-fit: contain;
        --artwork-override-height: auto;
        --artwork-override-width: 100%;
        --artwork-override-max-height: 100%;
        --artwork-override-max-width: 100%;
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(0, 1fr));
        align-items: center;
        justify-content: center;
        gap: 8px;
        width: 100%;
        height: calc(100% * var(--container-aspect-ratio));
        aspect-ratio: var(--container-aspect-ratio);
        border-radius: var(--global-border-radius-medium);

        &.iphone {
            --container-aspect-ratio: 1.444;
        }

        &.ipad {
            --container-aspect-ratio: 1.54;
        }

        &.mac {
            --container-aspect-ratio: 1.6;
        }

        &.watch {
            --container-aspect-ratio: 1.636;
        }

        &.tv,
        &.vision {
            --container-aspect-ratio: 1.77;
        }
    }

    // Centers a single item in the grid
    .artwork-container :global(> :only-child) {
        justify-self: center;
    }

    // Aligns the first of two items to the center edge
    .artwork-container :global(> :nth-child(1):nth-last-child(2)) {
        justify-self: flex-end;
    }

    // Aligns the second of two items to the center edge
    .artwork-container :global(> :nth-child(2):nth-last-child(1)) {
        justify-self: flex-start;
    }

    .video-wrapper {
        display: flex;
        overflow: hidden;
        max-height: 100%;
        width: auto;
        aspect-ratio: var(--media-aspect-ratio, 16/9);
        border: 1px solid var(--systemQuaternary);
        border-radius: 16px;
    }

    .artwork-container :global(.artwork-component) {
        display: flex;
        aspect-ratio: var(--media-aspect-ratio);
        border-radius: 16px;
        justify-content: center;
        align-items: center;
        width: auto;
        height: auto;
        max-width: 100%;
        max-height: 100%;
    }

    .artwork-container :global(.artwork-component img) {
        height: 100%;
    }

    .artwork-container :global(.video-container) {
        container-type: normal;
    }

    .artwork-container :global(video) {
        width: 100%;
        height: 100%;
        object-fit: cover;
    }
</style>
