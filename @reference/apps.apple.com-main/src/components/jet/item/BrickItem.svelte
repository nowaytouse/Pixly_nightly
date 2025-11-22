<script lang="ts">
    import type { Brick } from '@jet-app/app-store/api/models';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';
    import AppIcon from '~/components/AppIcon.svelte';
    import Artwork from '~/components/Artwork.svelte';
    import GradientOverlay from '~/components/GradientOverlay.svelte';
    import HoverWrapper from '~/components/HoverWrapper.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import {
        colorAsString,
        getBackgroundGradientCSSVarsFromArtworks,
        getLuminanceForRGB,
    } from '~/utils/color';
    import { isRtl } from '~/utils/locale';

    export let item: Brick;
    export let shouldOverlayDescription: boolean = false;

    const rtlArtwork = item.artworks?.[1] || item.rtlArtwork;
    const artwork = isRtl() && rtlArtwork ? rtlArtwork : item.artworks?.[0];
    const { collectionIcons } = item;

    const gradientColor: string = artwork?.backgroundColor
        ? colorAsString(artwork.backgroundColor)
        : 'rgb(0 0 0 / 62%)';

    let backgroundGradientCssVars: string | undefined = undefined;

    if (collectionIcons && collectionIcons.length > 1) {
        // If there are multiple app icons, we build a string of CSS variables from the icons
        // background colors to fill as many of the lockups quadrants as possible.
        backgroundGradientCssVars = getBackgroundGradientCSSVarsFromArtworks(
            collectionIcons,
            {
                // sorts from darkest to lightest
                sortFn: (a, b) => getLuminanceForRGB(a) - getLuminanceForRGB(b),
                shouldRemoveGreys: true,
            },
        );
    }
</script>

<LinkWrapper
    action={item.clickAction}
    label={item.accessibilityLabel || item.clickAction?.title}
>
    <div class="container">
        <HoverWrapper>
            {#if artwork}
                <Artwork
                    {artwork}
                    profile={shouldOverlayDescription ? 'small-brick' : 'brick'}
                />
            {:else if backgroundGradientCssVars}
                <div
                    class="background-gradient"
                    style={backgroundGradientCssVars}
                />
            {/if}

            {#if item.title}
                <GradientOverlay --color={gradientColor} />
            {/if}

            <div class="text-container">
                <div class="metadata-container">
                    {#if item.caption}
                        <LineClamp clamp={1}>
                            <h4>{item.caption}</h4>
                        </LineClamp>
                    {/if}

                    {#if item.title}
                        <LineClamp clamp={3}>
                            <h3 class="title">
                                {@html sanitizeHtml(item.title)}
                            </h3>
                        </LineClamp>
                    {/if}

                    {#if item.subtitle}
                        <LineClamp clamp={2}>
                            <p>{item.subtitle}</p>
                        </LineClamp>
                    {/if}
                </div>

                {#if !artwork && collectionIcons}
                    <ul class="app-icons">
                        {#each collectionIcons?.slice(0, 8) as collectionIcon}
                            <li class="app-icon-container">
                                <AppIcon
                                    icon={collectionIcon}
                                    profile="brick-app-icon"
                                    fixedWidth={false}
                                />
                            </li>
                        {/each}
                    </ul>
                {/if}
            </div>
        </HoverWrapper>

        {#if item.shortEditorialDescription}
            <h3
                class="editorial-description"
                class:overlaid={shouldOverlayDescription}
            >
                {item.shortEditorialDescription}
            </h3>
        {/if}
    </div>
</LinkWrapper>

<style>
    .container {
        position: relative;
        container-type: inline-size;
        container-name: container;
    }

    .metadata-container {
        width: 100%;
        align-self: end;
    }

    .text-container {
        position: absolute;
        z-index: 2;
        bottom: 0;
        display: flex;
        align-items: flex-end;
        width: 100%;
        height: 100%;
        padding: 20px;
        color: var(--systemPrimary-onDark);
    }

    .app-icon-container {
        position: relative;
        flex-shrink: 0;
        width: 60px;
        margin-inline-end: 5%;
    }

    .title {
        font: var(--title-1-emphasized);
        text-wrap: pretty;
    }

    h4 {
        margin-bottom: 3px;
        font: var(--callout-emphasized);
    }

    p {
        margin-top: 6px;
        font: var(--body-emphasized);
    }

    .editorial-description {
        margin-top: 8px;
        font: var(--title-3);
    }

    .editorial-description.overlaid {
        position: absolute;
        z-index: 1;
        bottom: 9px;
        padding: 0 20px;
        color: white;
        font: var(--title-2-emphasized);
    }

    @property --top-left-stop {
        syntax: '<percentage>';
        inherits: false;
        initial-value: 20%;
    }

    @property --bottom-left-stop {
        syntax: '<percentage>';
        inherits: false;
        initial-value: 40%;
    }

    @property --top-right-stop {
        syntax: '<percentage>';
        inherits: false;
        initial-value: 55%;
    }

    @property --bottom-right-stop {
        syntax: '<percentage>';
        inherits: false;
        initial-value: 50%;
    }

    .container .background-gradient {
        width: 100%;
        aspect-ratio: 16 / 9;
        background: radial-gradient(
                circle at 3% -50%,
                var(--top-left, #000) var(--top-left-stop),
                transparent 70%
            ),
            radial-gradient(
                circle at -50% 120%,
                var(--bottom-left, #000) var(--bottom-left-stop),
                transparent 80%
            ),
            radial-gradient(
                circle at 66% -175%,
                var(--top-right, #000) var(--top-right-stop),
                transparent 80%
            ),
            radial-gradient(
                circle at 62% 100%,
                var(--bottom-right, #000) var(--bottom-right-stop),
                transparent 100%
            );
        animation: gradient-hover 8s infinite alternate-reverse;
        animation-play-state: paused;
    }

    @keyframes gradient-hover {
        0% {
            --top-left-stop: 20%;
            --bottom-left-stop: 40%;
            --top-right-stop: 55%;
            --bottom-right-stop: 50%;
            background-size: 100% 100%;
        }

        50% {
            --top-left-stop: 25%;
            --bottom-left-stop: 15%;
            --top-right-stop: 70%;
            --bottom-right-stop: 30%;
            background-size: 130% 130%;
        }

        100% {
            --top-left-stop: 15%;
            --bottom-left-stop: 20%;
            --top-right-stop: 55%;
            --bottom-right-stop: 20%;
            background-size: 110% 110%;
        }
    }

    .container:hover .background-gradient {
        animation-play-state: running;
    }

    .app-icons {
        display: grid;
        align-self: center;
        flex-direction: row;
        width: 44%;
        grid-template-rows: auto auto;
        grid-auto-flow: column;
        gap: 8px;
    }

    .app-icons li:nth-child(even) {
        inset-inline-start: 40px;
    }

    @container container (max-width: 298px) {
        .title {
            font: var(--title-2-emphasized);
        }

        .text-container {
            padding: 16px;
        }

        .editorial-description.overlaid {
            bottom: 16px;
            padding-inline: 16px;
        }

        .app-icons {
            width: 36%;
        }

        .app-icon-container {
            width: 50px;
        }
    }

    @container container (min-width: 440px) {
        .app-icon-container {
            width: 83px;
        }
    }
</style>
