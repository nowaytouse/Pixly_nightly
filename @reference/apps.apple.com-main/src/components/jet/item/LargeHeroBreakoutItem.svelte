<script lang="ts">
    import {
        type Artwork as JetArtworkType,
        type LargeHeroBreakout,
        isFlowAction,
    } from '@jet-app/app-store/api/models';
    import { isSome } from '@jet/environment/types/optional';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import mediaQueries from '~/utils/media-queries';
    import AppIcon from '~/components/AppIcon.svelte';
    import Artwork from '~/components/Artwork.svelte';
    import HoverWrapper from '~/components/HoverWrapper.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import SFSymbol from '~/components/SFSymbol.svelte';
    import Video from '~/components/jet/Video.svelte';
    import type { NamedProfile } from '~/config/components/artwork';
    import { colorAsString, isRGBColor, isDark } from '~/utils/color';
    import { isRtl } from '~/utils/locale';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';

    export let item: LargeHeroBreakout;

    let profile: NamedProfile;
    let artwork: JetArtworkType | undefined;
    let gradientColor: string;

    const {
        collectionIcons = [],
        editorialDisplayOptions,
        rtlArtwork,
        video,
        details: { callToActionButtonAction: action },
    } = item;
    const canUseRTLArtwork = isRtl() && rtlArtwork;
    const shouldShowCollectionIcons =
        collectionIcons?.length > 1 && !editorialDisplayOptions.suppressLockup;

    $: artwork =
        (canUseRTLArtwork ? rtlArtwork : item.artwork) || video?.preview;
    $: doesArtworkHaveDarkBackground =
        artwork?.backgroundColor &&
        isRGBColor(artwork.backgroundColor) &&
        isDark(artwork.backgroundColor);
    $: isBackgroundDark = item.isMediaDark ?? doesArtworkHaveDarkBackground;

    $: profile =
        $mediaQueries === 'xsmall'
            ? 'large-hero-portrait-iphone'
            : canUseRTLArtwork
            ? 'large-hero-breakout-rtl'
            : 'large-hero-breakout';

    $: gradientColor = artwork?.backgroundColor
        ? colorAsString(artwork.backgroundColor)
        : '#000';
</script>

<LinkWrapper {action}>
    <HoverWrapper>
        <div class="artwork-container">
            {#if video && $mediaQueries !== 'xsmall' && !canUseRTLArtwork}
                <Video {video} {profile} autoplay loop useControls={false} />
            {:else if artwork}
                <Artwork {artwork} {profile} />
            {/if}
        </div>

        <div class="gradient" style="--color: {gradientColor};" />

        <div
            class="text-container"
            class:on-dark={isBackgroundDark}
            class:on-light={!isBackgroundDark}
        >
            {#if item.details?.badge}
                <LineClamp clamp={1}>
                    <h4>{item.details.badge}</h4>
                </LineClamp>
            {/if}

            {#if item.details.title}
                <LineClamp clamp={2}>
                    <h3>{@html sanitizeHtml(item.details.title)}</h3>
                </LineClamp>
            {/if}

            {#if item.details.description}
                <LineClamp clamp={3}>
                    <p>{@html sanitizeHtml(item.details.description)}</p>
                </LineClamp>
            {/if}

            {#if isSome(action) && isFlowAction(action)}
                <span class="link-container">
                    {action.title}
                    <span aria-hidden="true">
                        <SFSymbol name="chevron.forward" />
                    </span>
                </span>
            {/if}

            {#if shouldShowCollectionIcons}
                <ul class="collection-icons">
                    {#each collectionIcons.slice(0, 6) as collectionIcon}
                        <li class="app-icon-container">
                            <AppIcon icon={collectionIcon} />
                        </li>
                    {/each}
                </ul>
            {/if}
        </div>
    </HoverWrapper>
</LinkWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/helpers' as *;
    @use 'ac-sasskit/core/locale' as *;

    .artwork-container {
        width: 100%;

        @media (--range-small-up) {
            aspect-ratio: 8 / 3;
        }
    }

    .artwork-container :global(.video-container) {
        display: flex;
    }

    .text-container {
        position: absolute;
        z-index: 2;
        bottom: 0;
        align-items: center;
        width: 100%;
        padding-inline: 20px;
        padding-bottom: 20px;
        text-wrap: pretty;

        @media (--range-small-up) {
            width: 50%;
        }

        @media (--range-large-up) {
            width: 33%;
        }
    }

    .text-container.on-dark {
        color: var(--systemPrimary-onDark);

        h4 {
            color: var(--systemSecondary-onDark);
        }

        :global(svg) {
            fill: var(--systemPrimary-onDark);
        }
    }

    .text-container.on-light {
        color: var(--systemPrimary-onLight);

        h4 {
            color: var(--systemSecondary-onLight);
        }

        :global(svg) {
            fill: var(--systemPrimary-onLight);
        }
    }

    .link-container {
        margin-top: 8px;
        display: flex;
        gap: 4px;
        font: var(--body-emphasized);

        @media (--range-small-up) {
            margin-top: 16px;
            font: var(--title-2-emphasized);
        }
    }

    .link-container :global(svg) {
        width: 8px;
        height: 8px;

        @include rtl {
            transform: rotate(180deg);
        }

        @media (--range-small-up) {
            width: 10px;
            height: 10px;
        }
    }

    h3 {
        text-wrap: balance;
        font: var(--title-1-emphasized);

        @media (--range-small-up) {
            font: var(--large-title-emphasized);
        }
    }

    h4 {
        font: var(--subhead-emphasized);

        @media (--range-small-up) {
            font: var(--callout-emphasized);
        }
    }

    p {
        margin-top: 4px;
        font: var(--body);

        @media (--range-small-up) {
            margin-top: 8px;
            font: var(--title-3);
        }
    }

    .collection-icons {
        display: flex;
        gap: 8px;
        margin-top: 16px;
        padding-top: 16px;
        border-top: 2px solid var(--systemTertiary-onDark);
    }

    .app-icon-container {
        aspect-ratio: 1/1;
    }

    .gradient {
        --rotation: 35deg;
        position: absolute;
        z-index: 1;
        width: 100%;
        height: 100%;
        filter: saturate(1.5) brightness(0.9);
        background: linear-gradient(
            var(--rotation),
            var(--color) 20%,
            transparent 50%
        );

        // In non-XS viewports with an RTL text direction, we flip the legibility gradient to
        // accomodate the right-justified text.
        @include rtl {
            @media (--range-small-up) {
                --rotation: -35deg;
            }
        }

        // In XS viewports, this component is renderd in a 3/4 card layout, so we always want the
        // gradient to be at 0deg rotation, as it goes from botttom to top.
        @media (--range-xsmall-down) {
            --rotation: 0deg;
        }
    }
</style>
