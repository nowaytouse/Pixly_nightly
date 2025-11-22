<script lang="ts">
    import type { PosterLockup } from '@jet-app/app-store/api/models';
    import Artwork from '~/components/Artwork.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import Video from '~/components/jet/Video.svelte';
    import AppleArcadeLogo from '~/components/icons/AppleArcadeLogo.svg';
    import HoverWrapper from '~/components/HoverWrapper.svelte';

    export let item: PosterLockup;
</script>

<LinkWrapper action={item.clickAction}>
    <HoverWrapper>
        <article>
            <div class="background">
                {#if item.epicHeading}
                    <div class="title-container">
                        <Artwork
                            hasTransparentBackground
                            artwork={item.epicHeading}
                            alt={item.heading}
                            profile="poster-title"
                        />
                    </div>
                {/if}

                {#if item.posterVideo}
                    <div class="video-container">
                        <Video
                            autoplay
                            loop
                            video={item.posterVideo}
                            useControls={false}
                            profile="poster-lockup"
                        />
                    </div>
                {:else if item.posterArtwork}
                    <div class="artwork-container">
                        <Artwork
                            artwork={item.posterArtwork}
                            profile="poster-lockup"
                        />
                    </div>
                {/if}
            </div>

            <div class="content">
                <div class="logo-container">
                    <AppleArcadeLogo aria-label={item.heading} />
                </div>

                <span>
                    {item.footerText}
                    {#if item.tertiaryTitle}
                        | {item.tertiaryTitle}
                    {/if}
                </span>
            </div>
        </article>
    </HoverWrapper>
</LinkWrapper>

<style>
    article {
        position: relative;
        width: 100%;
        aspect-ratio: 16/9;
        overflow: hidden;
        color: var(--systemPrimary-onDark);
        border-radius: var(--global-border-radius-large);
        container-type: inline-size;
        container-name: poster-lockup-item;
    }

    .title-container {
        position: absolute;
        z-index: 2;
        width: 100%;
    }

    .background {
        position: absolute;
        z-index: -1;
        width: 100%;
        line-height: 0;
    }

    .content {
        display: flex;
        align-items: center;
        flex-direction: column;
        justify-content: space-between;
        height: 100%;
        padding: 12px 0;
        font: var(--body);
        background: linear-gradient(
            0deg,
            rgba(0, 0, 0, 0.5) 0%,
            rgba(255, 255, 255, 0) 25%,
            rgba(255, 255, 255, 0) 50%,
            rgba(255, 255, 255, 0) 80%,
            rgba(0, 0, 0, 0.4) 100%
        );
    }

    .logo-container {
        width: 62px;
        margin-bottom: 10px;
        line-height: 0;
    }

    @container poster-lockup-item (min-width: 550px) {
        .logo-container {
            width: 78px;
        }
    }

    .logo-container :global(path) {
        color: var(--systemPrimary-onDark);
    }
</style>
