<script lang="ts">
    import type { ImageLockup } from '@jet-app/app-store/api/models';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import AppIcon from '~/components/AppIcon.svelte';
    import Artwork from '~/components/Artwork.svelte';
    import GradientOverlay from '~/components/GradientOverlay.svelte';
    import HoverWrapper from '~/components/HoverWrapper.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import { colorAsString } from '~/utils/color';

    export let item: ImageLockup;

    const color: string = item.artwork.backgroundColor
        ? colorAsString(item.artwork.backgroundColor)
        : '#000';
</script>

<LinkWrapper action={item.lockup.clickAction}>
    <HoverWrapper>
        <div class="container">
            <div class="artwork-container">
                <Artwork artwork={item.artwork} profile="large-image-lockup" />
            </div>

            {#if item.lockup}
                <div
                    class="lockup-container"
                    class:on-dark={item.isDark}
                    class:on-light={!item.isDark}
                >
                    {#if item.lockup.icon}
                        <div class="app-icon-container">
                            <AppIcon icon={item.lockup.icon} />
                        </div>
                    {/if}

                    <div class="metadata-container">
                        {#if item.lockup.heading}
                            <LineClamp clamp={1}>
                                <p>{item.lockup.heading}</p>
                            </LineClamp>
                        {/if}

                        {#if item.lockup.title}
                            <LineClamp clamp={2}>
                                <h3>{item.lockup.title}</h3>
                            </LineClamp>
                        {/if}

                        {#if item.lockup.subtitle}
                            <LineClamp clamp={1}>
                                <p>{item.lockup.subtitle}</p>
                            </LineClamp>
                        {/if}
                    </div>
                </div>
            {/if}

            <div class="gradient-container">
                <GradientOverlay --color={color} --height="85%" />
            </div>
        </div>
    </HoverWrapper>
</LinkWrapper>

<style>
    .artwork-container {
        position: absolute;
        z-index: -1;
        width: 100%;
    }

    .container {
        width: 100%;
        aspect-ratio: 16/9;
        container-type: inline-size;
        container-name: container;
    }

    .gradient-container {
        position: absolute;
        z-index: -1;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
    }

    .lockup-container {
        display: flex;
        align-items: flex-end;
        width: 100%;
        height: 100%;
        padding: 0 20px 20px;
    }

    .lockup-container.on-dark {
        color: var(--systemPrimary-onDark);
    }

    .lockup-container.on-light {
        color: var(--systemPrimary-onLight);
    }

    @container container (max-width: 260px) {
        .lockup-container {
            padding: 0 10px 10px;
        }
    }

    .app-icon-container {
        flex-shrink: 0;
        width: 48px;
        margin-inline-end: 8px;
    }

    h3 {
        margin: 2px 0;
        font: var(--title-1-emphasized);
    }

    p {
        font: var(--callout-emphasized);
    }

    .lockup-container.on-dark p {
        mix-blend-mode: plus-lighter;
    }
</style>
