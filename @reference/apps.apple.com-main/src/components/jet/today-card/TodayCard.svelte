<script lang="ts">
    import type { TodayCard } from '@jet-app/app-store/api/models';

    import Artwork, {
        type Profile,
        getNaturalProfile,
    } from '~/components/Artwork.svelte';
    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';
    import TodayCardMedia from '~/components/jet/today-card/TodayCardMedia.svelte';
    import TodayCardOverlay from '~/components/jet/today-card/TodayCardOverlay.svelte';
    import { isTodayCardMediaList } from '~/components/jet/today-card/media/TodayCardMediaList.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';

    import { colorAsString } from '~/utils/color';
    import { bestBackgroundColor } from './background-color-utils';

    export let card: TodayCard;

    /**
     * When set to `true`, this component will not enable the `clickAction` provided by the
     * `card`
     *
     * This can be useful on the "story" page, where the card will link back to the page
     * currently being viewed
     */
    export let suppressClickAction: boolean = false;

    /**
     * A `Profile` to override the default for the card's media
     */
    export let artworkProfile: Profile | undefined = undefined;

    let useProtectionLayer: boolean;
    let useBlurryProtectionLayer: boolean;
    let useGradientProtectionLayer: boolean;
    let useListStyle: boolean;
    let accentColor: string;

    $: ({
        heading,
        title,
        inlineDescription,
        titleArtwork,
        overlay,
        media,
        editorialDisplayOptions,
        style = 'light',
        clickAction,
    } = card);
    $: action = suppressClickAction ? undefined : clickAction;

    $: {
        const isAppEvent = media?.kind === 'appEvent';
        const isList = !!media && isTodayCardMediaList(media);

        useListStyle = isList;
        useProtectionLayer =
            editorialDisplayOptions?.useTextProtectionColor ||
            editorialDisplayOptions?.useMaterialBlur ||
            false;
        useBlurryProtectionLayer = useProtectionLayer && !isAppEvent && !isList;
        useGradientProtectionLayer = useProtectionLayer && isAppEvent;
        accentColor = colorAsString(bestBackgroundColor(card.media));
    }
</script>

<!--
    We don't wrap the entire card with an action if there is an `overlay`, since the overlay has
    it's own link / action (and we don't want nesting `a` tags, of course).
-->
<LinkWrapper action={overlay || useListStyle ? null : action}>
    <div
        class="today-card"
        class:light={style === 'light'}
        class:dark={style === 'dark'}
        class:white={style === 'white'}
        class:list={useListStyle}
        class:with-overlay={overlay}
        style:--today-card-accent-color={accentColor}
    >
        {#if media && !useListStyle}
            <TodayCardMedia {media} {artworkProfile} />
        {/if}

        <div class="wrapper">
            <div
                class="information-layer"
                class:with-gradient={useGradientProtectionLayer}
                class:with-action={!!action}
            >
                <LinkWrapper action={useListStyle ? null : action}>
                    <div class="content-container">
                        {#if useBlurryProtectionLayer}
                            <div class="protection-layer" />
                        {/if}

                        <div class="title-container">
                            {#if heading && !titleArtwork}
                                <p class="badge">
                                    <LineClamp clamp={1}>
                                        {heading}
                                    </LineClamp>
                                </p>
                            {/if}

                            {#if titleArtwork}
                                <div class="title-artwork-container">
                                    <Artwork
                                        artwork={titleArtwork}
                                        profile={getNaturalProfile(
                                            titleArtwork,
                                        )}
                                    />
                                </div>
                            {/if}

                            {#if title && !titleArtwork}
                                <h3 class="title">
                                    <LinkWrapper
                                        action={useListStyle ? action : null}
                                    >
                                        {@html sanitizeHtml(title)}
                                    </LinkWrapper>
                                </h3>
                            {/if}

                            {#if inlineDescription}
                                <LineClamp clamp={2}>
                                    <p class="description">
                                        {@html sanitizeHtml(inlineDescription)}
                                    </p>
                                </LineClamp>
                            {/if}
                        </div>
                    </div>
                </LinkWrapper>

                {#if overlay}
                    <div
                        class="overlay"
                        class:blur-only={!useProtectionLayer}
                        class:dark={useProtectionLayer && style !== 'dark'}
                        class:light={useProtectionLayer && style === 'dark'}
                    >
                        <TodayCardOverlay
                            {overlay}
                            buttonVariant={useProtectionLayer
                                ? 'transparent'
                                : 'dark-gray'}
                            --text-color="var(--today-card-text-color)"
                            --text-accent-color="var(--today-card-text-accent-color)"
                            --text-accent-blend-mode="var(--today-card-text-accent-blend-mode)"
                        />
                    </div>
                {/if}
            </div>
        </div>

        {#if media && useListStyle}
            <TodayCardMedia {media} {artworkProfile} />
        {/if}
    </div>
</LinkWrapper>

<style lang="scss">
    @property --gradient-color {
        syntax: '<color>';
        inherits: true;
        initial-value: #000;
    }

    .today-card {
        --today-card-gutter: 16px;
        --today-card-border-radius: var(
            --border-radius,
            var(--global-border-radius-large)
        );
        --protection-layer-bottom-offset: 0px;
        --gradient-color: var(--today-card-accent-color);
        background-color: var(--today-card-accent-color);
        position: relative;
        display: flex;
        align-items: end;
        height: 100%;
        overflow: hidden;
        color: var(--today-card-text-color);
        container-type: size;
        container-name: today-card;
        border-radius: var(--today-card-border-radius);
        box-shadow: var(--shadow-small);
    }

    .today-card.with-overlay {
        --protection-layer-bottom-offset: 80px;
    }

    .today-card.light,
    .today-card.dark {
        --today-card-text-color: rgb(255, 255, 255);
        --today-card-text-accent-color: rgba(255, 255, 255, 0.56);
        --today-card-text-accent-blend-mode: plus-lighter;
        --today-card-background-tint-color: rgba(0, 0, 0, 0.18);
    }

    .today-card.white {
        --today-card-text-color: var(--systemPrimary-onLight);
        --today-card-text-accent-color: rgba(0, 0, 0, 0.56);
        --today-card-background-tint-color: rgba(255, 255, 255, 0.33);
        --today-card-text-accent-blend-mode: revert;
    }

    .today-card :global(.artwork-component) {
        z-index: unset;
    }

    .wrapper {
        position: absolute;
        display: flex;
        width: 100%;
        height: 100%;
    }

    .content-container {
        position: relative;
    }

    .information-layer {
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: end;
        align-self: flex-end;
        width: 100%;
        height: 100%;
        border-radius: var(--today-card-border-radius);
        overflow: hidden;
    }

    .information-layer > :global(a) {
        display: flex;
        flex-grow: 1;
        flex-direction: column;
        justify-content: end;
    }

    .information-layer.with-gradient {
        // A smooth bottom-to-top gradient with an intermediate stop at 60% of the accent color's
        // opacity to ease the hard transition.
        --gradient-color-end-position: 22%;
        --gradient-fade-end-position: 50%;
        background: linear-gradient(
            0deg,
            var(--gradient-color) var(--gradient-color-end-position),
            color-mix(in srgb, var(--gradient-color) 60%, transparent)
                calc(
                    (
                            var(--gradient-color-end-position) +
                                var(--gradient-fade-end-position)
                        ) / 2
                ),
            transparent var(--gradient-fade-end-position)
        );
        transition: --accent-color-end 500ms ease-out, --fade-end 350ms ease-out,
            --gradient-color 350ms ease-out;
    }

    .information-layer.with-gradient.with-action:has(> a:hover) {
        // Darkens the color used in the gradient on hover
        --gradient-color: color-mix(
            in srgb,
            var(--today-card-accent-color) 93%,
            black
        );
    }

    @container today-card (aspect-ratio >= 16/9) {
        .information-layer.with-gradient {
            --accent-color-end: 30%;
        }
    }

    .protection-layer {
        --brightness: 0.95;
        position: absolute;
        width: 100%;
        // On cards with overlays (app lockups at the bottom), we increase the height of the
        // protection layer and shift it downward the same amount, so it is aligned to bottom
        // of the overlay.
        height: calc(100% + var(--protection-layer-bottom-offset) + 60px);
        bottom: calc(-1 * var(--protection-layer-bottom-offset));
        background: var(--today-card-background-tint-color);
        backdrop-filter: blur(34px) brightness(var(--brightness)) saturate(1.6)
            contrast(1.1);
        mask-image: linear-gradient(
            to top,
            black 30%,
            rgba(0, 0, 0, 0.75) 70%,
            rgba(0, 0, 0, 0.4) 86%,
            transparent 100%
        );
        transition: backdrop-filter 210ms ease-in;
    }

    .information-layer:has(> a:hover) .protection-layer {
        --brightness: 0.88;
    }

    .badge {
        font: var(--callout-emphasized);
        margin-bottom: 4px;
        mix-blend-mode: var(--today-card-text-accent-blend-mode);
        color: var(--today-card-text-accent-color);
    }

    .title-container {
        width: auto;
        position: relative;
        padding: 0 var(--today-card-gutter) var(--today-card-gutter);
    }

    @container today-card (orientation: landscape) {
        .title-artwork-container {
            width: 33%;
            min-width: 200px;
            max-width: 300px;
            padding-bottom: 8px;
        }
    }

    @container today-card (orientation: portrait) {
        .title-artwork-container {
            max-width: 75%;
            padding-bottom: 8px;
        }
    }

    .title {
        font: var(--header-emphasized);
        color: var(--today-card-text-color);
        text-wrap: pretty;
    }

    .description {
        font: var(--body);
        padding-top: calc(var(--today-card-gutter) / 2);
        mix-blend-mode: var(--today-card-text-accent-blend-mode);
        color: var(--today-card-text-accent-color);
        text-wrap: pretty;
        z-index: 1;
        position: relative;
    }

    .overlay {
        z-index: 1;
        position: relative;
        padding: var(--today-card-gutter);
    }

    .overlay.blur-only {
        backdrop-filter: blur(50px);
    }

    .overlay.light {
        background-image: linear-gradient(rgba(225, 225, 225, 0.15) 0 0);
    }

    .overlay.dark {
        background-image: linear-gradient(rgba(0, 0, 0, 0.15) 0 0);
    }

    .list {
        background: var(--systemPrimary-onDark);
        padding: var(--today-card-gutter) 0;
        width: 100%;
        flex-direction: column;

        @media (prefers-color-scheme: dark) {
            --title-color: var(--systemPrimary);
            background: var(--systemQuaternary);

            .title {
                --today-card-text-color: var(--systemPrimary);
            }

            .badge {
                --today-card-text-accent-color: var(--systemSecondary);
            }
        }
    }

    .list .wrapper {
        position: relative;
        height: auto;
        width: 100%;
    }

    .list .information-layer {
        padding-top: 0;
    }
</style>
