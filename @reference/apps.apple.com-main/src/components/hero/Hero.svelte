<!--
@component
Component for rendering an item in a "Hero Carousel" without coupling to any specific data model
-->
<script lang="ts">
    import type { Opt } from '@jet/environment/types/optional';
    import type {
        Action,
        Artwork as ArtworkModel,
        Color,
        Video as VideoModel,
    } from '@jet-app/app-store/api/models';

    import mediaQueries from '~/utils/media-queries';
    import { prefersReducedMotion } from '@amp/web-app-components/src/stores/prefers-reduced-motion';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';

    import AppIcon from '~/components/AppIcon.svelte';
    import Artwork from '~/components/Artwork.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import Video from '~/components/jet/Video.svelte';
    import type { NamedProfile } from '~/config/components/artwork';
    import {
        colorAsString,
        getBackgroundGradientCSSVarsFromArtworks,
        getLuminanceForRGB,
    } from '~/utils/color';
    import { isRtl } from '~/utils/locale';

    /**
     * The main text for the carousel item
     */
    export let title: Opt<string> = undefined;

    /**
     * Additional text above the title.
     * Note: If a slot is defined with the name `eyebrow`, the slot takes precedence.
     */
    export let eyebrow: Opt<string> = undefined;

    /**
     * Additional text below the title
     */
    export let subtitle: Opt<string> = undefined;

    /**
     * Primary accent color for the carousel item
     */
    export let backgroundColor: Opt<Color> = undefined;

    /**
     * Static artwork to display in the carousel item
     */
    export let artwork: Opt<ArtworkModel> = undefined;

    /**
     * Video to display in the carousel item
     *
     * Takes precedence over `artwork`
     */
    export let video: Opt<VideoModel> = undefined;

    /**
     * Action to perform when clicking on the carousel item
     */
    export let action: Opt<Action> = undefined;

    /**
     * Whether the artwork should be aligned to the end (e.g. the right edge in LTR) of the container
     */
    export let pinArtworkToHorizontalEnd: boolean = false;

    /**
     * Whether the artwork should be pinned to the vertical middle of the container (it's pinned to the top by default)
     */
    export let pinArtworkToVerticalMiddle: boolean = false;

    /**
     * Whether the text (e.g. title, description, etc) should be pinned to the top of the container
     */
    export let pinTextToVerticalStart: boolean = false;

    /**
     * Allows for the absolute overriding of the profile used for the Hero artwork
     */
    export let profileOverride: Opt<NamedProfile> = null;

    export let isMediaDark: boolean = true;

    export let collectionIcons: ArtworkModel[] | undefined = undefined;

    let isPortraitLayout: boolean;
    let profile: NamedProfile;
    let collectionIconsBackgroundGradientCssVars: string | undefined =
        undefined;

    $: isPortraitLayout = $mediaQueries === 'xsmall';

    $: {
        if (profileOverride) {
            profile = profileOverride;
        } else if (isPortraitLayout) {
            profile = 'large-hero-portrait';
        } else if (pinArtworkToHorizontalEnd && isRtl()) {
            profile = 'large-hero-east';
        } else if (pinArtworkToHorizontalEnd) {
            profile = 'large-hero-west';
        } else {
            profile = 'large-hero';
        }
    }

    const color: string = backgroundColor
        ? colorAsString(backgroundColor)
        : '#000';

    if (collectionIcons && collectionIcons.length > 1) {
        // If there are multiple app icons, we build a string of CSS variables from the icons
        // background colors to fill as many of the lockups quadrants as possible.
        collectionIconsBackgroundGradientCssVars =
            getBackgroundGradientCSSVarsFromArtworks(collectionIcons, {
                // sorts from darkest to lightest
                sortFn: (a, b) => getLuminanceForRGB(a) - getLuminanceForRGB(b),
                shouldRemoveGreys: true,
            });
    }
</script>

<LinkWrapper {action} includeExternalLinkArrowIcon={false}>
    <article
        data-test-id="hero"
        class:with-dark-media={isMediaDark}
        class:with-collection-icons={!artwork && !video && collectionIcons}
        class:text-pinned-to-vertical-start={pinTextToVerticalStart}
    >
        {#if video || artwork}
            <div
                class={`image-container ${profile}`}
                class:pinned-to-horizontal-end={pinArtworkToHorizontalEnd}
                class:pinned-to-vertical-middle={pinArtworkToVerticalMiddle}
                style:--color={color}
            >
                {#if video && !$prefersReducedMotion}
                    <Video
                        loop
                        autoplay
                        useControls={false}
                        {video}
                        {profile}
                    />
                {:else if artwork}
                    <Artwork
                        {artwork}
                        {profile}
                        noShelfChevronAnchor={true}
                        useCropCodeFromArtwork={false}
                        withoutBorder={true}
                    />
                {/if}
            </div>
        {:else if collectionIcons}
            <ul class="app-icons">
                {#each collectionIcons?.slice(0, 5) as collectionIcon}
                    <li class="app-icon-container">
                        <AppIcon
                            icon={collectionIcon}
                            profile="app-icon-large"
                            fixedWidth={false}
                        />
                    </li>
                {/each}
            </ul>

            <div
                class="collection-icons-background-gradient"
                style={collectionIconsBackgroundGradientCssVars}
            />
        {/if}

        <div class="gradient" style="--color: {color};" />

        <slot name="badge" {isPortraitLayout} />

        <div class="metadata-container">
            {#if $$slots.eyebrow}
                <h3><slot name="eyebrow" /></h3>
            {:else if eyebrow}
                <h3>{eyebrow}</h3>
            {/if}

            {#if title}
                <h2>{@html sanitizeHtml(title)}</h2>
            {/if}

            {#if subtitle}
                <p class="subtitle">{@html sanitizeHtml(subtitle)}</p>
            {/if}

            <slot name="details" {isPortraitLayout} />
        </div>
    </article>
</LinkWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/app/core/globalvars' as *;

    article {
        --hero-primary-color: var(--systemPrimary-onLight);
        --hero-secondary-color: var(--systemSecondary-onLight);
        --hero-text-blend-mode: normal;
        --hero-divider-color: var(--systemQuaternary-onLight);
        position: relative;
        display: flex;
        overflow: hidden;
        align-items: end;
        aspect-ratio: 3 / 4;
        container-name: hero-container;
        container-type: size;

        @media (--range-small-up) {
            aspect-ratio: 16 / 9;
            width: 100%;
            height: auto;
            min-height: 360px;
            max-height: min(60vh, 770px);
            border-radius: var(--global-border-radius-large);
            border: 1px solid var(--systemQuaternary);
        }
    }

    article.with-dark-media,
    article.with-collection-icons {
        --hero-primary-color: var(--systemPrimary-onDark);
        --hero-secondary-color: var(--systemSecondary-onDark);
        --hero-divider-color: var(--systemQuaternary-onDark);
        --hero-text-blend-mode: plus-lighter;
    }

    .image-container {
        position: absolute;
        z-index: -1;
        width: 100%;
        height: 100%;
        background-color: var(--color);
    }

    .image-container.pinned-to-vertical-middle {
        display: flex;
        align-items: center;
    }

    .image-container.pinned-to-vertical-middle :global(.video-container),
    .image-container.pinned-to-vertical-middle :global(.artwork-component) {
        width: 100%;
        height: auto;
    }

    .image-container.pinned-to-horizontal-end :global(.artwork-component) {
        height: 100%;
        display: flex;
    }

    .image-container.pinned-to-horizontal-end :global(.artwork-component img) {
        height: 100%;
        width: auto;
        position: absolute;
        inset-inline-end: 0;

        @container hero-container (aspect-ratio >= 279/100) {
            width: 100%;
            height: auto;
        }
    }

    .image-container.pinned-to-horizontal-end.large-hero-story-card-rtl
        :global(.artwork-component img) {
        inset-inline-start: 0;
    }

    // This is terrible but essentially the `large-hero-story-card` profile has an aspect ratio of
    // 2.25:1, so whenever the image container gets expanded past that aspect ratio, we make the
    // artwork full-width rather than full-height. This should eventually be fixed when Editorial
    // can prescribe us only 16x9 (1.77:1) hero images.
    .image-container.pinned-to-horizontal-end.large-hero-story-card,
    .image-container.pinned-to-horizontal-end.large-hero-story-card-rtl {
        @container hero-container (aspect-ratio >= 225/100) {
            :global(.artwork-component img) {
                width: 100%;
                height: auto;
            }
        }
    }

    .metadata-container {
        position: absolute;
        width: 40%;
        padding-bottom: 40px;
        padding-inline-start: 40px;
        text-wrap: pretty;
        color: var(--hero-primary-color);

        @media (--range-small-only) {
            width: 50%;
            padding: 0 20px 20px;
        }

        @media (--range-xsmall-down) {
            width: 100%;
            padding: 0 20px 20px;
            text-align: center;
        }
    }

    .text-pinned-to-vertical-start .metadata-container {
        @media (--range-small-only) {
            top: 20px;
        }

        @media (--range-medium-up) {
            top: 40px;
        }
    }

    h2 {
        position: relative;
        z-index: 1;
        text-wrap: balance;
        font: var(--header-emphasized);

        @media (--range-xsmall-down) {
            font: var(--title-1-emphasized);
        }
    }

    @container hero-container (height < 420px) {
        h2 {
            font: var(--large-title-emphasized);
        }
    }

    h3 {
        margin-bottom: 8px;
        position: relative;
        z-index: 1;
        color: var(--hero-secondary-color);
        font: var(--callout-emphasized-tall);
        mix-blend-mode: var(--hero-text-blend-mode);

        @media (--range-xsmall-down) {
            margin-bottom: 4px;
        }
    }

    p {
        mix-blend-mode: var(--hero-text-blend-mode);
    }

    .subtitle {
        margin-top: 8px;
        position: relative;
        z-index: 1;
        font: var(--body-tall);
        color: var(--hero-secondary-color);
    }

    .gradient {
        --rotation: 55deg;

        &:dir(rtl) {
            --rotation: -55deg;
            mask-image: radial-gradient(
                    ellipse 127% 130% at 95% 100%,
                    rgb(0, 0, 0) 18%,
                    rgb(0, 0, 0.33) 24%,
                    rgba(0, 0, 0, 0.66) 32%,
                    transparent 40%
                ),
                linear-gradient(
                    -129deg,
                    rgb(0, 0, 0) 0%,
                    rgba(255, 255, 255, 0) 55%
                );
        }
        position: absolute;
        z-index: -1;
        width: 100%;
        height: 100%;
        // stylelint-disable color-function-notation
        background: linear-gradient(
            var(--rotation),
            rgb(from var(--color) r g b / 0.25) 0%,
            transparent 50%
        );
        // stylelint-enable color-function-notation
        filter: saturate(1.5) brightness(0.9);
        backdrop-filter: blur(40px);
        mask-image: radial-gradient(
                ellipse 127% 130% at 5% 100%,
                rgb(0, 0, 0) 18%,
                rgb(0, 0, 0.33) 24%,
                rgba(0, 0, 0, 0.66) 32%,
                transparent 40%
            ),
            linear-gradient(51deg, rgb(0, 0, 0) 0%, rgba(255, 255, 255, 0) 55%);

        @media (--range-xsmall-down) {
            --rotation: 0deg;
            mask-image: linear-gradient(
                var(--rotation),
                rgb(0, 0, 0) 28%,
                rgba(0, 0, 0, 0) 56%
            );
        }
    }

    // When the text is pinned to the top of the lockup, we use a different gradient for legibility
    article.text-pinned-to-vertical-start .gradient {
        --rotation: -170deg;
        mask-image: radial-gradient(
            ellipse 118% 121% at 100% 0%,
            rgb(0, 0, 0) 18%,
            rgb(0, 0, 0.33) 22%,
            rgba(0, 0, 0, 0.66) 33%,
            transparent 43%
        );
    }

    .app-icons {
        display: grid;
        align-self: center;
        width: 90%;
        grid-template-rows: auto auto;
        grid-auto-flow: column;
        gap: 24px;
        margin-inline-start: -4%;
        position: absolute;
        inset-inline-end: 24px;

        @media (--range-small-up) {
            width: 44%;
        }
    }

    .app-icons li:nth-child(even) {
        inset-inline-start: 44%;
    }

    .app-icon-container {
        position: relative;
        flex-shrink: 0;
        max-width: 200px;
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

    .collection-icons-background-gradient {
        width: 100%;
        height: 100%;
        position: absolute;
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
        animation: collection-icons-background-gradient-shift 16s infinite
            alternate-reverse;
        animation-play-state: paused;

        @media (--range-small-up) {
            animation-play-state: running;
        }
    }

    @keyframes collection-icons-background-gradient-shift {
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
</style>
