<script lang="ts" context="module">
    import type { Artwork as JetArtworkType } from '@jet-app/app-store/api/models';
    import type {
        Artwork as ComponentArtworkType,
        Profile as ArtworkProfile,
        CropCode,
        ImageSizes,
    } from '@amp/web-app-components/src/components/Artwork/types';

    import type { NamedProfile } from '~/config/components/artwork';

    /**
     * Creates a {@linkcode Profile} on-the-fly based on the properties of
     * the {@linkcode artwork}
     */
    export function getNaturalProfile(
        artwork: JetArtworkType,
        imageSizes: ImageSizes = [artwork.width],
    ): ArtworkProfile {
        const aspectRatio = artwork.width / artwork.height;

        return [imageSizes, aspectRatio, artwork.crop as CropCode];
    }

    export type Profile = NamedProfile | ArtworkProfile;
</script>

<script lang="ts">
    import type { ImageSettings } from '@amp/web-app-components/src/components/Artwork/types';
    import Artwork from '@amp/web-app-components/src/components/Artwork/Artwork.svelte';
    import { colorAsString, isNamedColor } from '~/utils/color';

    import {
        ArtworkConfig,
        type ArtworkProfileMap,
    } from '@amp/web-app-components/config/components/artwork';

    export let artwork: JetArtworkType;
    export let profile: Profile;
    export let alt: string = '';
    export let topRoundedSecondary: boolean = false;
    export let useContainerStyle: boolean = false;
    export let forceFullWidth: boolean = true;
    export let isDecorative: boolean = true;
    export let lazyLoad: boolean = true;
    export let disableAutoCenter: boolean = false;
    export let noShelfChevronAnchor: boolean = false;
    export let forceCropCode: boolean = false;
    export let quality: number | undefined = undefined;
    export let hasTransparentBackground: boolean =
        !!artwork.backgroundColor &&
        isNamedColor(artwork.backgroundColor) &&
        artwork.backgroundColor.name === 'clear';
    export let useCropCodeFromArtwork: boolean = true;
    export let withoutBorder: boolean = false;

    let imageSettings: ImageSettings;
    $: imageSettings = {
        forceCropCode,
        hasTransparentBackground,
        quality,
    };

    let PROFILES: ArtworkProfileMap<string> | undefined;
    let computedProfileAttributes: Profile | undefined;

    $: {
        const config = ArtworkConfig?.get();
        PROFILES = config?.PROFILES;

        const defaultProfileAttributes: Profile | undefined =
            typeof profile === 'string' ? PROFILES?.get(profile) : profile;

        const cropCodeIndex = 2;

        if (
            useCropCodeFromArtwork &&
            artwork?.crop &&
            defaultProfileAttributes
        ) {
            computedProfileAttributes = [...defaultProfileAttributes];
            computedProfileAttributes[cropCodeIndex] =
                artwork?.crop as CropCode;
        }
    }

    $: artworkForComponent = {
        ...artwork,
        backgroundColor: artwork.backgroundColor
            ? colorAsString(artwork.backgroundColor)
            : undefined,
    } satisfies ComponentArtworkType;
</script>

<Artwork
    artwork={artworkForComponent}
    profile={computedProfileAttributes || profile}
    {topRoundedSecondary}
    {useContainerStyle}
    {forceFullWidth}
    {imageSettings}
    {alt}
    {isDecorative}
    {lazyLoad}
    {disableAutoCenter}
    {noShelfChevronAnchor}
    {withoutBorder}
/>

<style>
    /* When a user enables the "Smart Invert" accessibility setting, images should not be inverted,
    so we are re-inverting back to their normal state in this media query, which only currently works for Safari. */
    @media (inverted-colors: inverted) {
        :global(.artwork-component img) {
            filter: invert(1);
        }
    }
</style>
