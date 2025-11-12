<script lang="ts">
    import type { AppEvent } from '@jet-app/app-store/api/models';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import Artwork from '~/components/Artwork.svelte';
    import GradientOverlay from '~/components/GradientOverlay.svelte';
    import HoverWrapper from '~/components/HoverWrapper.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import Video from '~/components/jet/Video.svelte';
    import AppEventDate from '~/components/AppEventDate.svelte';
    import SmallLockupItem from './SmallLockupItem.svelte';

    export let item: AppEvent;
    export let isArticleContext: boolean = false;

    $: artwork = item.moduleArtwork;
    $: video = item.moduleVideo;
    $: hasLightArtwork = item.mediaOverlayStyle === 'light';
    $: gradientColor = hasLightArtwork
        ? 'rgb(240 240 240 / 48%)'
        : 'rgb(83 83 83 / 48%)';
    $: shouldShowLockup = !!item.lockup && !item.hideLockupWhenNotInstalled;
</script>

<div
    class="app-event-item"
    class:with-lockup={!!item.lockup && !item.hideLockupWhenNotInstalled}
>
    <span class="time-indicator">
        <AppEventDate appEvent={item} />
    </span>

    <div class="lockup-container">
        <HoverWrapper hasChin={shouldShowLockup} --display="block">
            <LinkWrapper action={item.clickAction}>
                <div class="text-over-artwork">
                    {#if video}
                        <div class="video-container">
                            <Video
                                {video}
                                autoplay
                                loop={true}
                                useControls={false}
                                profile="app-promotion"
                            />
                        </div>
                    {:else if artwork}
                        <div class="artwork-container">
                            <Artwork
                                {artwork}
                                profile={isArticleContext
                                    ? 'app-promotion-in-article'
                                    : 'app-promotion'}
                            />
                        </div>
                    {/if}

                    <div class="gradient-container">
                        <GradientOverlay
                            --border-radius={0}
                            --color={gradientColor}
                            --height="80%"
                            shouldDarken={!hasLightArtwork}
                        />
                    </div>

                    <div class="text-container" class:dark={hasLightArtwork}>
                        <h4>{item.kind}</h4>

                        <h3>{item.title}</h3>

                        <LineClamp clamp={1}>
                            <p>{item.detail}</p>
                        </LineClamp>
                    </div>
                </div>
            </LinkWrapper>
        </HoverWrapper>

        {#if item.lockup && shouldShowLockup}
            <div class="small-lockup-container">
                <SmallLockupItem item={item.lockup} appIconProfile="app-icon" />
            </div>
        {/if}
    </div>
</div>

<style>
    .app-event-item {
        height: 100%;
        display: grid;
        grid-template-areas:
            'time-indicator'
            'lockup';
        grid-template-rows: 1rem 1fr;
        gap: 4px;
    }

    .time-indicator {
        grid-area: time-indicator;
        color: var(--keyColor);
        font-weight: bold;
    }

    .lockup-container {
        grid-area: lockup;
    }

    .text-over-artwork {
        /* Allow artwork, overlay and text containers to overlap by targeting the same grid area */
        display: grid;
        grid-template-areas: 'content';
    }

    .artwork-container {
        grid-area: content;
        border-radius: var(--global-border-radius-large);
    }

    .video-container {
        grid-area: content;
        border-radius: var(--global-border-radius-large);
        line-height: 0;
    }

    .app-event-item.with-lockup .artwork-container,
    .app-event-item.with-lockup .video-container {
        border-radius: 0;
    }

    .gradient-container {
        grid-area: content;
        z-index: 1;
        position: relative;
    }

    .text-container {
        color: var(--systemPrimary-onDark);
        padding: 12px 16px;
        grid-area: content;
        z-index: 2;

        /* Float text to the bottom of the lockup */
        display: flex;
        flex-direction: column;
        justify-content: flex-end;
    }

    .text-container.dark {
        color: var(--systemPrimary-onLight);
    }

    .small-lockup-container {
        background: var(--systemPrimary-onDark);
        border-radius: 0 0 var(--global-border-radius-large)
            var(--global-border-radius-large);
        box-shadow: var(--shadow-small);
        padding: 12px;

        @media (prefers-color-scheme: dark) {
            background: var(--systemQuinary-onDark);
        }
    }

    h3 {
        font: var(--title-2-tall);
    }

    h4 {
        font: var(--callout-emphasized-tall);
    }

    p {
        font: var(--callout-emphasized);
    }
</style>
