<script lang="ts">
    import type { SvelteComponent } from 'svelte';
    import { onMount } from 'svelte';
    import { makeSafeTick } from '@amp/web-app-components/src/utils/makeSafeTick';
    import type { Readable } from 'svelte/store';
    import LoaderSelector, {
        LOADER_TYPE,
    } from '@amp/web-app-components/src/components/Artwork/loaders/LoaderSelector.svelte';
    import {
        getShelfAspectRatioContext,
        hasShelfAspectRatioContext,
    } from '@amp/web-app-components/src/utils/shelfAspectRatio';
    import { FILE_TO_MIME_TYPE, DEFAULT_FILE_TYPE } from './constants';
    import type { Artwork, ImageSettings, Profile, ChinConfig } from './types';
    import { getAspectRatio, getImageTagWidthHeight } from './utils/artProfile';
    import { getPreconnectTracker } from './utils/preconnect';
    import { buildSourceSet, getImageSizes } from './utils/srcset';
    import { deriveBackgroundColor } from './utils/validateBackground';

    const preconnectTracker = getPreconnectTracker();

    /**
     * artwork object
     * @type {{ template: string, width: number, height: number, backgroundColor: string }} Artwork
     */
    export let artwork: Artwork;
    /**
     * alt tag to use on image.
     */
    export let alt: string = '';
    /**
     * id to use on image.
     * @type {string}
     */
    export let id: string | undefined = undefined;
    /**
     * Profiles are required to determine the optimal image to render for given viewports.
     * @type {Profile | string}
     */
    export let profile: Profile | string;
    /**
     * k/v map of settings that don't depend on viewport size.
     * @type {ImageSettings}
     */
    export let imageSettings: ImageSettings = {};
    /**
     * Apply rounded secondary corner styles to top of artwork image
     * @type {boolean}
     */
    export let topRoundedSecondary: boolean = false;
    /**
     * Whether to lazy load the image.
     * Set this to false if this image is expected to be the LCP.
     */
    export let lazyLoad: boolean = true;
    /**
     * Sets the `fetchpriority` attribute on the image.
     * Set this to 'high' if this image is expected to be the LCP.
     */
    export let fetchPriority: 'high' | 'auto' | 'low' = 'auto';
    /**
     * Turning off container styles allows for a custom wrapper to be used to provide different
     * styling when an artwork is used outside of a lockup or in a different context.
     * @type {boolean}
     */
    export let useContainerStyle: boolean = true;
    /**
     * Option to disable CSS anchoring for shelf chevron.
     * Useful to isolate anchor when there are multiple images in a single lockup.
     * @type {boolean}
     */
    export let noShelfChevronAnchor: boolean = false;

    /**
     * Configuration object for chin effects including height and style.
     * Used primarily by TV app for adding visual effects below the main artwork.
     * @type {ChinConfig}
     */
    export let chinConfig: ChinConfig | undefined = undefined;

    export let forceFullWidth: boolean = true;

    /**
     * Option to disable image from being auto-centered
     * in its container. Only relevant for non-square
     * images.
     */
    export let disableAutoCenter = false;

    /**
     * `isDecorative` indicates if an image is decoration.
     * Decoaration images should be attributed a presentation role (role=presentation) to avoid an oververbose auditory user experience.
     * By default, it is set to false if an alt attribute is provided.
     * See https://www.w3.org/WAI/tutorials/images/decorative/
     * @type {boolean}
     */
    export let isDecorative: boolean = !!!alt;

    /**
     * Allows artwork to be rendered without a border, regardless of it's background color or transparency.
     */
    export let withoutBorder: boolean = false;

    let localShelfAspectRatioStore: Readable<string> | null = null;

    if (hasShelfAspectRatioContext()) {
        const { addProfile, shelfAspectRatio } = getShelfAspectRatioContext();
        addProfile(profile);
        localShelfAspectRatioStore = shelfAspectRatio;
    }

    $: template = artwork && artwork.template;

    $: imageIsLoading = !!template; // start in loading state when template is available
    $: thereWasAnError = !artwork; // start in clean error state unless there's no artwork passed

    $: backgroundColor = artwork?.backgroundColor;

    $: ({ fileType = DEFAULT_FILE_TYPE } = imageSettings);

    $: isBackgroundTransparent =
        imageSettings?.hasTransparentBackground ?? false;

    $: validBackgroundColor = isBackgroundTransparent
        ? 'transparent'
        : deriveBackgroundColor(backgroundColor);

    $: srcset =
        artwork && buildSourceSet(artwork, imageSettings, profile, chinConfig);
    $: webpSourceSet =
        artwork &&
        buildSourceSet(
            artwork,
            Object.assign({}, imageSettings, { fileType: 'webp' }),
            profile,
            chinConfig,
        );
    $: aspectRatio = getAspectRatio(profile);
    $: imageTagSizeObj = getImageTagWidthHeight(profile);

    // Calculate effective aspect ratio accounting for chin height
    $: effectiveAspectRatio = (() => {
        const chinHeightValue = chinConfig?.height ?? 0;
        if (chinHeightValue === 0 || aspectRatio === null) {
            return aspectRatio;
        }

        // Get the base dimensions from the profile
        const baseHeight = imageTagSizeObj.height;
        const baseWidth = imageTagSizeObj.width;

        // Calculate new aspect ratio with chin height added
        const newHeight = baseHeight + chinHeightValue;
        return baseWidth / newHeight;
    })();

    // NOTE: We intentionally set opacity to 1 in SSR so that images will load
    //       in before the JS loads.
    $: opacity = `${imageIsLoading && typeof window !== 'undefined' ? 0 : 1}`;
    // And similarly, we force <NoLoader> so that the image markup is emitted
    $: loaderType =
        lazyLoad && typeof window !== 'undefined'
            ? LOADER_TYPE.LAZY
            : LOADER_TYPE.NONE;

    $: sizes = getImageSizes(profile, artwork?.width);

    $: wrapperStyle = (() => {
        // remove the joe color background to prevent
        // parts of it from bleeding through artwork
        const background =
            ($$slots['placeholder-component'] && thereWasAnError) ||
            hasTransitionInEnded ||
            isBackgroundTransparent
                ? 'transparent'
                : `${validBackgroundColor}`;

        // if backgroundColor data is unavailable, do not insert inline background styles
        // (--artwork-bg-color & --placeholder-bg-color) - to allow joe color fallback
        const artworkBGColor = validBackgroundColor
            ? `--artwork-bg-color: ${validBackgroundColor};`
            : '';
        const placeholderBGColor = background
            ? `--placeholder-bg-color: ${background};`
            : '';

        return `
            ${artworkBGColor}
            --aspect-ratio: ${
                effectiveAspectRatio !== null ? effectiveAspectRatio : 1
            };
            ${placeholderBGColor}
       `;
    })();

    $: {
        preconnectTracker?.trackUrl(template);
    }

    /**
     * false if image natural aspect ratio is not equal to profile
     *
     * @see {onImageLoad}
     */
    let aspectRatioMatchesProfile = true;

    $: hasDominantShelfAspectRatio =
        localShelfAspectRatioStore !== null &&
        $localShelfAspectRatioStore !== null;

    // Should apply joe color BG if image natural aspect ratio doesn't match shelfAspectRatio
    $: shouldOverrideBG = (() => {
        let overrideBG = false;
        if (localShelfAspectRatioStore !== null) {
            const shelfAspectRatio = parseFloat($localShelfAspectRatioStore);
            if (!isNaN(shelfAspectRatio)) {
                const roundedShelfAspectRatio =
                    Math.round(shelfAspectRatio * 100) / 100;
                const roundedAspectRatio =
                    Math.round(effectiveAspectRatio * 100) / 100;
                if (roundedShelfAspectRatio !== roundedAspectRatio) {
                    overrideBG = true;
                }
            }
        } else if (!aspectRatioMatchesProfile) {
            overrideBG = true;
        }
        return overrideBG;
    })();

    const onImageLoad = (e: Event) => {
        const img = e.target as HTMLImageElement;

        if (img.naturalHeight !== 0 && img.naturalWidth !== 0) {
            const actualAspectRatio =
                Math.round((img.naturalWidth / img.naturalHeight) * 100) / 100;
            const roundedEstimate =
                Math.round(effectiveAspectRatio * 100) / 100;

            if (
                actualAspectRatio !== roundedEstimate &&
                Math.abs(
                    (actualAspectRatio - roundedEstimate) /
                        ((actualAspectRatio + roundedEstimate) / 2),
                ) > 0.1
            ) {
                aspectRatioMatchesProfile = false;
            }
        }
        imageIsLoading = false;
    };

    let hasTransitionInEnded = false;
    const onTransitionEnd = (e: TransitionEvent) => {
        const img = e.target as HTMLElement;
        const opacityValue = parseFloat(img.style.opacity);

        if (opacityValue === 1) {
            hasTransitionInEnded = true;
        } else {
            hasTransitionInEnded = false;
        }
    };

    const onImageError = () => {
        thereWasAnError = true;
        imageIsLoading = false;
    };

    let loaderComponent: SvelteComponent;
    let artworkComponent: HTMLElement;

    const safeTick = makeSafeTick();

    onMount(async () => {
        await safeTick(async (tick) => {
            await tick();
            loaderComponent.onSlotMount(artworkComponent);
        });
    });

    const getImageOrientation = (aspectRatio: number) => {
        let orientation: 'square' | 'landscape' | 'portrait';
        if (aspectRatio === 1) {
            orientation = 'square';
        } else if (aspectRatio > 1) {
            orientation = 'landscape';
        } else {
            orientation = 'portrait';
        }
        return orientation;
    };
</script>

<div
    data-testid="artwork-component"
    {id}
    class={`artwork-component artwork-component--aspect-ratio artwork-component--orientation-${getImageOrientation(
        effectiveAspectRatio,
    )}`}
    class:container-style={useContainerStyle}
    class:artwork-component--downloaded={!imageIsLoading &&
        hasTransitionInEnded}
    class:artwork-component--error={thereWasAnError}
    class:artwork-component--fullwidth={forceFullWidth}
    class:artwork-component--top-rounded-secondary={topRoundedSecondary}
    class:artwork-component--auto-center={!disableAutoCenter &&
        (hasDominantShelfAspectRatio || !aspectRatioMatchesProfile)}
    class:artwork-component--bg-override={shouldOverrideBG}
    class:artwork-component--has-borders={!isBackgroundTransparent &&
        !withoutBorder}
    class:artwork-component--no-anchor={noShelfChevronAnchor}
    style={wrapperStyle}
    on:transitionend={onTransitionEnd}
    bind:this={artworkComponent}
>
    {#if imageIsLoading && $$slots['loading-component']}
        <div
            class="artwork-component__contents"
            data-testid="artwork-component__loading"
        >
            <slot name="loading-component" />
        </div>
    {:else if thereWasAnError && $$slots['placeholder-component']}
        <div
            class="artwork-component__contents"
            data-testid="artwork-component__placeholder"
        >
            <slot name="placeholder-component" />
        </div>
    {/if}
    <LoaderSelector {loaderType} bind:this={loaderComponent} let:isVisible>
        {#if !thereWasAnError && isVisible}
            <picture>
                {#if webpSourceSet}
                    <source
                        {sizes}
                        srcset={webpSourceSet}
                        type={FILE_TO_MIME_TYPE.webp}
                    />
                {/if}
                <source {sizes} {srcset} type={FILE_TO_MIME_TYPE[fileType]} />
                <img
                    {alt}
                    class="artwork-component__contents artwork-component__image"
                    loading={lazyLoad ? 'lazy' : null}
                    style:opacity
                    src="/assets/artwork/1x1.gif"
                    role={isDecorative ? 'presentation' : null}
                    decoding="async"
                    width={`${imageTagSizeObj.width}`}
                    height={`${
                        imageTagSizeObj.height + (chinConfig?.height ?? 0)
                    }`}
                    fetchpriority={fetchPriority}
                    on:load={onImageLoad}
                    on:error={onImageError}
                />
            </picture>
        {/if}
    </LoaderSelector>
</div>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'amp/stylekit/core/colors' as *;
    @use 'amp/stylekit/core/mixins/browser-targets' as *;
    @use '@amp/web-shared-styles/app/core/globalvars' as *;
    @use '@amp/web-shared-styles/app/core/mixins/after-shadow' as *;
    @use '@amp/web-shared-styles/app/core/colors' as *;
    @use './style/ratio-based-artwork-box.scss' as *;

    // container style design: https://pd-hi.apple.com/viewvc/Common/Modules/macOS/-Cross%20Product/_macOS%20-%20Content%20Container%20Treatment.png?revision=54684&pathrev=57428
    // TODO: rdar://79348133 (Bring in copy + pasted variables into StyleKit)
    .container-style {
        border-radius: var(
            --global-border-radius-medium,
            #{$global-border-radius-medium}
        );

        &::after {
            @include after-shadow;
        }
    }

    .artwork-component {
        width: var(--artwork-override-width, 100%);
        height: var(--artwork-override-height, auto);
        max-width: var(--artwork-override-max-width, none);
        min-width: var(--artwork-override-min-width, 0);
        min-height: var(--artwork-override-min-height, 0);
        max-height: var(--artwork-override-max-height, none);
        border-radius: inherit;
        box-sizing: border-box;
        contain: content;
        overflow: hidden;
        position: relative;
        background-color: var(
            --override-placeholder-bg-color,
            var(--placeholder-bg-color, var(--genericJoeColor))
        );
        z-index: var(--z-default);

        &.artwork-component--has-borders {
            &::after {
                @include after-shadow;
            }
        }

        &.artwork-component--auto-center {
            @include ratio-based-artwork-box;

            &.artwork-component--bg-override {
                background-color: var(--artwork-bg-color);
            }
        }
    }

    // Artwork with rounded-secondary border-radius on top corners
    .artwork-component--top-rounded-secondary {
        // Required to keep lockups/chins aligned with the same height, when 2-line clamps are visible.
        flex-grow: 0;
        // Applying `border-radius` and `overflow: hidden;` to prevent image/chin subpixel width mismatch
        // prettier-ignore
        border-radius: var(--global-border-radius-large, #{$global-border-radius-large}) var(--global-border-radius-large, #{$global-border-radius-large}) 0 0;
        overflow: hidden;

        &,
        &::after {
            // prettier-ignore
            border-radius: var(--global-border-radius-large, #{$global-border-radius-large}) var(--global-border-radius-large, #{$global-border-radius-large}) 0 0;
        }

        @media (--target-desktop) {
            &::after {
                --global-transition-property: background-color;
                transition: var(--global-transition, opacity 0.1s ease-in);

                .horizontal-poster-lockup:hover &,
                .horizontal-poster-lockup:focus &,
                .horizontal-poster-lockup:focus-within & {
                    background-color: var(--lockupHoverBGColor);
                }
            }
        }

        //
        // Webkit Box Reflect chins
        //
        @supports (-webkit-box-reflect: inherit) {
            -webkit-box-reflect: below;
            overflow: visible;

            &::after {
                box-shadow: none;
            }
        }
    }

    //Revisit for potential clean up
    .artwork-component__contents {
        border-radius: inherit;
        transition: var(--global-transition, opacity 0.1s ease-in);
    }

    .artwork-component__image {
        height: var(--artwork-override-height, auto);
        width: var(--artwork-override-width, 100%);
        max-width: var(--artwork-override-max-width, none);
        min-width: var(--artwork-override-min-width, 0);
        min-height: var(--artwork-override-min-height, 0);
        max-height: var(--artwork-override-max-height, none);
        display: block;
        object-fit: var(--artwork-override-object-fit, fill);
        object-position: var(--artwork-override-object-position, center);
    }

    .artwork-component:not(.artwork-component--downloaded),
    // If image doesn't download/render, on error, show JoeColor in placeholders.
    // .artwork-component--feature-recommended,
    .artwork-component--error {
        background-color: var(
            --override-placeholder-bg-color,
            var(--placeholder-bg-color, var(--genericJoeColor))
        );
        // for generic joe color - it provides light/dark mode.
        &[style*='#ebebeb'] {
            @media (prefers-color-scheme: dark) {
                // Force Dark Generic joeColor for dark mode
                background-color: swatch(genericJoeColor, dark);
            }
        }
    }

    // Dynamic aspect ratios
    // Create placeholders with aspect-ratio derived from `artwork-profiles.js`
    // https://github.com/thierryk/aspect-ratio-via-css/tree/master/aspect-ratio-via-class-selector
    //
    // Apply aspect ratio to `1x1` `src` placeholders. Once downloaded, the placeholder aspect ratio is no longer needed.
    //
    .artwork-component--aspect-ratio:not(.artwork-component--downloaded),
    // If image doesn't download/render, on error, show aspect-ratio placeholders instead.
    .artwork-component--error {
        // Placeholder `src` may have different aspect ratio. Hide overflow in that case.
        overflow: hidden;

        &::before,
        &::after {
            content: '';
            display: block;
            // prettier-ignore
            padding-bottom: calc(100% / var(--shelf-aspect-ratio, var(--aspect-ratio)));
            // Prevent distortion of overlaid border from additional padding
            box-sizing: border-box;
        }

        &::after {
            position: absolute;
            // No `min-height: 100%` on border overlay when generating aspect-ratio placeholder.
            min-height: 0;
        }

        // `img` may not always be the first-child. Can be an svg or another container.
        > :global(:first-child),
        > :global(noscript) > :global(:first-child) {
            position: absolute;
            width: var(--artwork-override-width, 100%);
            height: var(--artwork-override-height, 100%);
            max-width: var(--artwork-override-max-width, none);
            min-width: var(--artwork-override-min-width, 0);
            min-height: var(--artwork-override-min-height, 0);
            max-height: var(--artwork-override-max-height, none);
            top: 50%;
            left: 50%; // RTL not needed
            transform: translateY(-50%) translateX(-50%); // RTL not needed
            z-index: var(--z-default);
        }

        > :global(img),
        > :global(noscript) > :global(img) {
            height: auto;
            min-height: var(--artwork-override-min-height, 0);
        }
    }

    // Full width (`forceFullWidth`) sizing is default, since most artwork are in responsive lockups.
    // Avoid using `--artwork-override-width` or `--artwork-override-height` with `forceFullWidth` property enabled.
    .artwork-component--fullwidth {
        &,
        > :global(noscript) {
            width: 100%;
        }

        > :global(noscript > picture .artwork-component__image) {
            width: 100%;
            height: auto;

            &::after {
                width: 100%;
                display: block;
                content: '';
            }
        }
    }
</style>
