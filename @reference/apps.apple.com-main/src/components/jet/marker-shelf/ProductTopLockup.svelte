<script lang="ts" context="module">
    import type {
        AppPlatform,
        ShelfBasedProductPage,
    } from '@jet-app/app-store/api/models';

    /**
     * The parts of {@linkcode ShelfBasedProductPage} that are required to render
     * the `MarkerShelf` component
     */
    export type MarkerShelfPageRequirements = Pick<
        ShelfBasedProductPage,
        | 'badges'
        | 'banner'
        | 'developerAction'
        | 'lockup'
        | 'shelfMapping'
        | 'titleOfferDisplayProperties'
        | 'canonicalURL'
        | 'appPlatforms'
    >;
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import { buildSrc } from '@amp/web-app-components/src/components/Artwork/utils/srcset';
    import { platform } from '@amp/web-apps-utils';
    import AppIcon, {
        doesAppIconNeedBorder,
    } from '~/components/AppIcon.svelte';
    import Banner from '~/components/jet/item/BannerItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ProductPageArcadeBanner from '~/components/ProductPageArcadeBanner.svelte';
    import { getI18n } from '~/stores/i18n';
    import { colorAsString, isNamedColor, isRGBColor } from '~/utils/color';
    import { concatWithMiddot, isString } from '~/utils/string-formatting';
    import {
        isPlatformExclusivelySupported,
        isPlatformSupported,
        PlatformToExclusivityText,
    } from '~/utils/app-platforms';
    import AppleArcadeLogo from '~/components/icons/AppleArcadeLogo.svg';
    import ShareArrowButton, {
        isShareSupported,
    } from '~/components/ShareArrowButton.svelte';
    import LaunchNativeButton from '~/components/LaunchNativeButton.svelte';

    export let page: MarkerShelfPageRequirements;

    $: banner = page.banner;
    $: lockup = page.lockup;
    $: appPlatforms = page.appPlatforms;
    $: offerDisplayProperties = lockup.offerDisplayProperties || {};
    $: ({ expectedReleaseDate } = offerDisplayProperties?.subtitles || {});

    const i18n = getI18n();

    // TODO: replace with `supportsArcade` from Jet
    // rdar://143706610 (Support `supportsArcade` attribute)
    $: supportsArcade = offerDisplayProperties.offerType === 'arcadeApp';

    $: backgroundColor = isRGBColor(lockup.icon?.backgroundColor)
        ? colorAsString(lockup.icon.backgroundColor)
        : '#fff';

    $: backgroundImage = lockup.icon
        ? buildSrc(
              lockup.icon.template,
              {
                  crop: 'bb',
                  width: 400,
                  height: 400,
                  fileType: 'webp',
              },
              {},
          )
        : undefined;

    $: attributes = concatWithMiddot(
        [
            expectedReleaseDate && $i18n.t('ASE.Web.AppStore.App.ComingSoon'),
            expectedReleaseDate && expectedReleaseDate,
            // Attributes that are not relevant for Arcade Apps:
            ...(!supportsArcade
                ? [
                      page.titleOfferDisplayProperties?.isFree &&
                          $i18n.t('ASE.Web.AppStore.Free'),
                      offerDisplayProperties.priceFormatted,
                      offerDisplayProperties.subtitles?.standard,
                      lockup.tertiaryTitle,
                  ]
                : []),
        ].filter(isString),
        $i18n,
    );

    $: exclusivePlatform = (
        Object.keys(PlatformToExclusivityText) as AppPlatform[]
    ).find((platform: AppPlatform) =>
        isPlatformExclusivelySupported(platform, appPlatforms),
    );
    $: exclusivityText = exclusivePlatform
        ? PlatformToExclusivityText[exclusivePlatform]
        : null;

    $: shouldShowLaunchNativeButton =
        platform.ismacOS() &&
        (lockup.isIOSBinaryMacOSCompatible ||
            isPlatformSupported('mac', appPlatforms));

    let shouldShowShareButton: boolean = true;

    onMount(() => {
        shouldShowShareButton = isShareSupported();
    });
</script>

<ShelfWrapper withBottomPadding={false} withPaddingTop={false}>
    <div
        class="container"
        style:--background-color={backgroundColor}
        style:--background-image={`url(${backgroundImage})`}
    >
        <div class="rotate" />
        <div class="blur" />

        <div class="content-container">
            {#if lockup.icon}
                <div
                    class="app-icon-contianer"
                    class:without-border={!doesAppIconNeedBorder(lockup.icon)}
                    aria-hidden="true"
                >
                    <AppIcon
                        icon={lockup.icon}
                        profile="app-icon-large"
                        fixedWidth={false}
                    />

                    <div class="glow">
                        <AppIcon
                            icon={lockup.icon}
                            profile="app-icon-large"
                            fixedWidth={false}
                        />
                    </div>
                </div>
            {/if}

            <section>
                {#if supportsArcade}
                    <span
                        class="arcade-logo"
                        aria-label={$i18n.t(
                            'ASE.Web.AppStore.ArcadeLogo.AccessibilityValue',
                        )}
                    >
                        <AppleArcadeLogo />
                    </span>
                {:else if lockup.editorialTagline}
                    <h3>{lockup.editorialTagline}</h3>
                {/if}

                <h1>
                    {lockup.title}
                </h1>

                <h2 class="subtitle">
                    {lockup.subtitle}
                </h2>

                {#if exclusivityText}
                    <p class="attributes">
                        {$i18n.t(exclusivityText)}
                    </p>
                {/if}

                {#if attributes.length > 0}
                    <p class="attributes">
                        {attributes}
                    </p>
                {/if}

                {#if page.canonicalURL && (shouldShowLaunchNativeButton || shouldShowShareButton)}
                    <div class="buttons-container">
                        {#if shouldShowLaunchNativeButton}
                            <span class="launch-native-button-container">
                                <LaunchNativeButton url={page.canonicalURL} />
                            </span>
                        {/if}

                        {#if shouldShowShareButton}
                            <!--
                                If there is no launch native button, then we show a label for
                                the share button, which helps to visually fill out the space.
                            -->
                            <ShareArrowButton
                                url={page.canonicalURL}
                                withLabel={!shouldShowLaunchNativeButton}
                            />
                        {/if}
                    </div>
                {/if}
            </section>
        </div>
    </div>
</ShelfWrapper>

{#if banner}
    <ShelfWrapper withBottomPadding={false} withTopMargin={false}>
        <Banner item={banner} />
    </ShelfWrapper>
{/if}

{#if supportsArcade}
    <ShelfWrapper
        withBottomPadding={false}
        withTopMargin={true}
        centered={false}
    >
        <ProductPageArcadeBanner />
    </ShelfWrapper>
{/if}

<style>
    .container {
        --blend-mode: plus-lighter;
        position: relative;
        display: flex;
        overflow: hidden;
        align-items: center;
        height: 200px;
        color: var(--systemPrimary-onDark);
        border-bottom: 1px solid var(--systemQuaternary-vibrant);
        border-bottom-right-radius: 2px;
        border-bottom-left-radius: 2px;
        background: linear-gradient(
                to bottom,
                transparent 20%,
                rgba(0, 0, 0, 0.8) 100%
            ),
            var(--background-image), var(--background-color, #000);
        background-size: cover;
        background-position: center;
        transition: border-bottom-left-radius 210ms ease-out,
            border-bottom-right-radius 210ms ease-out;
        transform: translate(0);

        @media (--range-small-up) {
            height: 286px;
        }

        @media (--range-xlarge-up) {
            border: 1px solid var(--systemQuaternary-vibrant);
            border-top: none;
            border-bottom-right-radius: 30px;
            border-bottom-left-radius: 30px;
        }
    }

    .glow {
        position: absolute;
        z-index: -1;
        top: 0;
        width: 100%;
        transform: scale(1.5);
        filter: blur(60px);
    }

    .blur {
        position: absolute;
        z-index: 1;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        backdrop-filter: blur(100px) saturate(1.5);
    }

    .rotate {
        position: absolute;
        z-index: 2;
        top: 0;
        left: 0;
        width: 100%;
        filter: brightness(1.3) saturate(0) blur(50px);
        mix-blend-mode: overlay;
        height: 500%;
        background-image: var(--background-image);
        background-repeat: repeat;
        opacity: 0;
        transform-origin: top center;
        animation: shift-background 60s infinite linear 10s;
    }

    .content-container {
        display: flex;
        flex-direction: row;
        max-width: 840px;
        gap: 1em;
        margin: 0 var(--bodyGutter);

        @media (--range-small-up) {
            gap: 1.5em;
        }

        @media (--range-medium-up) {
            gap: 2em;
        }
    }

    .app-icon-contianer {
        position: relative;
        z-index: 2;
        display: flex;
        align-items: center;
        width: 128px;
        flex-shrink: 0;

        @media (--range-small-up) {
            width: 194px;
        }
    }

    .app-icon-contianer:not(.without-border) :global(> .app-icon) {
        box-shadow: 0 0 30px rgba(0, 0, 0, 0.33);
        border: 2px solid var(--systemQuaternary-onDark);
    }

    section {
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    .subtitle,
    .attributes {
        position: relative;
        z-index: 2;
        margin-bottom: 4px;
        font: var(--body);
        color: rgba(245.973, 245.973, 245.973, 0.6);
        text-wrap: pretty;
        mix-blend-mode: var(--blend-mode);

        @media (--range-small-up) {
            margin-bottom: 8px;
            font: var(--title-2-emphasized);
        }
    }

    .attributes {
        margin-bottom: 0;
        font: var(--body);
    }

    .buttons-container {
        --share-arrow-size: 27px;
        --launch-native-button-arrow-size: 7px;
        --get-button-font: var(--footnote-bold);
        margin-top: 10px;
        display: flex;
        align-items: center;
        gap: 10px;

        @media (--range-small-up) {
            --share-arrow-size: unset;
            --launch-native-button-arrow-size: unset;
            --get-button-font: unset;
        }
    }

    h1 {
        position: relative;
        z-index: 2;
        display: flex;
        align-items: center;
        margin-bottom: 2px;
        font: var(--title-2-emphasized);
        color: white;
        text-shadow: 0 0 5px rgba(0, 0, 0, 0.2);
        text-wrap: pretty;

        @media (--sidebar-visible) {
            font: var(--title-1-emphasized);
        }

        @media (--range-small-up) {
            font: var(--header-emphasized);
            letter-spacing: -0.02em;
        }
    }

    h3 {
        margin-bottom: 0;
        position: relative;
        z-index: 2;
        mix-blend-mode: plus-lighter;
        font: var(--body-emphasized);

        @media (--range-small-up) {
            font: var(--title-3-emphasized);
        }
    }

    .arcade-logo {
        display: flex;
        height: 10px;
        margin-bottom: 4px;
        position: relative;
        z-index: 2;
        mix-blend-mode: plus-lighter;

        @media (--range-small-up) {
            height: 14px;
        }
    }

    .launch-native-button-container {
        position: relative;
        z-index: 2;
    }

    @keyframes shift-background {
        0% {
            background-position: 50% 50%;
            background-size: 100%;
            transform: rotate(0deg);
            opacity: 0;
        }

        10% {
            opacity: 0.5;
        }

        20% {
            background-position: 65% 25%;
            background-size: 160%;
            transform: rotate(45deg);
        }

        45% {
            background-position: 90% 60%;
            background-size: 250%;
            transform: rotate(160deg);
            opacity: 0.5;
        }

        70% {
            background-position: 70% 40%;
            background-size: 200%;
            transform: rotate(250deg);
            opacity: 0.5;
        }

        100% {
            background-position: 50% 50%;
            background-size: 100%;
            transform: rotate(360deg);
            opacity: 0;
        }
    }
</style>
