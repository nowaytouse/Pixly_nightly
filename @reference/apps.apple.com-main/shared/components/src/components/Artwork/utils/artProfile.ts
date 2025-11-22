import type {
    Profile,
    ImageURLParams,
    CropCode,
} from '@amp/web-app-components/src/components/Artwork/types';
import { ArtworkConfig } from '@amp/web-app-components/config/components/artwork';

const ARTWORK_IDENTIFIERS = [
    'xlarge',
    'large',
    'medium',
    'small',
    'xsmall',
] as const;

function getArtworkProfile(profile: Profile | string): Profile {
    const { PROFILES } = ArtworkConfig.get();
    const selectedProfile: Profile =
        typeof profile === 'string' ? PROFILES.get(profile) : profile;
    // TODO: add validation + warning / error handling for profiles
    // rdar://76365525 (Artwork Component: add validation + warning / error handling for profiles)
    return selectedProfile;
}

function buildImgDimensions(
    width: number,
    aspectRatio: number,
    crop: CropCode,
): Partial<ImageURLParams> {
    const dimensions = {
        width,
        height: Math.round(width * (1 / aspectRatio)),
        crop,
    };

    return dimensions;
}

export type ConvertedProfile = {
    [key in (typeof ARTWORK_IDENTIFIERS)[number]]?: ImageURLParams;
};

export const getAspectRatio = (profile: Profile | string): number => {
    const [, aspectRatio] = getArtworkProfile(profile);
    return aspectRatio === null ? null : aspectRatio;
};

type ImageTagWidthHeight = { width: number; height: number };
export const getImageTagWidthHeight = (
    profile: Profile | string,
): ImageTagWidthHeight => {
    const [imageSize, aspectRatio] = getArtworkProfile(profile);
    const width = imageSize[0];
    return {
        width,
        height: Math.floor(width / aspectRatio),
    };
};

export const getDataFromProfile = (
    profile: Profile | string,
): ConvertedProfile => {
    const selectedProfile = getArtworkProfile(profile);

    const [widths, aspectRatio, crop] = selectedProfile;

    const imgDimensions = widths.reduce((acc, w, indx) => {
        acc[ARTWORK_IDENTIFIERS[indx]] = buildImgDimensions(
            w,
            aspectRatio,
            crop,
        );
        return acc;
    }, {});

    return imgDimensions;
};
