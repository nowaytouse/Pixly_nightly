/**
 * COPIED FROM: https://github.pie.apple.com/amp-ui/ember-ui-media-artwork/blob/957fc3e586d4ff710b2263a45d8950d4ee65616a/addon/utils/srcset.js
 * and converted to TypeScript
 */
import { replaceQualityParam } from '@amp/web-app-components/src/components/Artwork/utils/replaceQualityParam';
import {
    DEFAULT_FILE_TYPE,
    DEFAULT_QUALITY,
    PIXEL_DENSITIES,
    EMBEDDED_CROP_CODE_REGEX,
    EFFECT_ID_REGEX,
    FILE_TYPE_REGEX,
} from '@amp/web-app-components/src/components/Artwork/constants';
import { ArtworkConfig } from '@amp/web-app-components/config/components/artwork';
import { memoize } from '@amp/web-app-components/src/utils/memoize';
import { getDataFromProfile } from '@amp/web-app-components/src/components/Artwork/utils/artProfile';
import type { MediaConditions } from '@amp/web-app-components/src/utils/getMediaConditions';
import { getMediaConditions } from '@amp/web-app-components/src/utils/getMediaConditions';
import type {
    FileExtension,
    Artwork,
    ArtworkMaxSizes,
    ImageSettings,
    ImageURLParams,
    Profile,
    CropCode,
    ChinConfig,
} from '@amp/web-app-components/src/components/Artwork/types';
import type { Size } from '@amp/web-app-components/src/types';

type ProfileConfig = {
    width: number;
    height: number;
    crop: CropCode;
};
type SizeMap = {
    [key in Size]?: ProfileConfig;
};

const isAFillCropCode = (crop: CropCode) => crop === 'bf';

const getSmallestProfileSize = (sizeMap: SizeMap) => {
    const { xlarge, large, medium, small, xsmall } = sizeMap;
    return xsmall || small || medium || large || xlarge;
};

const filterSizeConfig = (
    config: ProfileConfig,
    maxWidth: number | null,
): boolean => (maxWidth ? config.width <= maxWidth : true);

const getSizesAndBreakpoints = (
    profile: Profile | string,
): [SizeMap, MediaConditions] => {
    const { BREAKPOINTS } = ArtworkConfig.get();
    const profileSize = profile ? getDataFromProfile(profile) : {};

    const mediaConditions = getMediaConditions(BREAKPOINTS);
    const SIZES = Object.keys(mediaConditions);
    // TODO: rdar://76402413 (Convert imperative reduce pattern
    // to functionalwith Object.fromEntries once on Node 12)
    const sizeMap: SizeMap = SIZES.reduce((accumulator, sizeName) => {
        // only add to size map if
        // profile exists for mediaCondition

        if (profileSize[sizeName]) {
            const imageWidth = profileSize[sizeName].width;
            const imageHeight = profileSize[sizeName].height;
            const imageCrop = profileSize[sizeName].crop;

            accumulator[sizeName] = {
                width: imageWidth,
                height: imageHeight,
                crop: imageCrop,
            };
        }

        return accumulator;
    }, {});

    return [sizeMap, mediaConditions];
};

function deriveUrlParamsArray(
    urlParams: Partial<ImageURLParams>,
    profile: Profile | string,
    maxWidth: number,
): ImageURLParams[] {
    const [profileBySize] = getSizesAndBreakpoints(profile);

    let filteredSizes = Object.values(profileBySize).filter((config) =>
        filterSizeConfig(config, maxWidth),
    );

    // if image is smaller than all profile sizes
    // use the smallest profile size available
    if (filteredSizes.length === 0) {
        const smallestProfile = getSmallestProfileSize(profileBySize);
        filteredSizes = [smallestProfile];
    }

    return filteredSizes.map((viewportProfile) => ({
        crop: viewportProfile.crop,
        width: viewportProfile.width,
        height: viewportProfile.height,
        quality: urlParams.quality,
        fileType: urlParams.fileType,
    }));
}

/**
 * Converts Artwork object to expected input for image src functions.
 * @param artwork Artwork object
 * @param quality image quality value
 * @param fileType file type
 * @param chinConfig chin configuration object
 */
function deriveDataFromArtwork(
    artwork: Artwork,
    quality?: number,
    fileType?: FileExtension,
    chinConfig?: ChinConfig,
): [string, Partial<ImageURLParams>, ArtworkMaxSizes] {
    const { width, height, template } = artwork;
    const chinHeight = chinConfig?.height ?? 0;

    const urlParams: Partial<ImageURLParams> = {
        fileType,
        quality,
    };

    const ogImageSizes: ArtworkMaxSizes = {
        maxHeight: height + chinHeight,
        maxWidth: width,
    };

    return [template, urlParams, ogImageSizes];
}

/**
 * Removes embedded crop codes if:
 *  1. a `crop` is passed (i.e. if a user passed a crop code in the invocation of
 *      the outer function)
 *  2. the rawURL has an embedded crop code that is not an Effect ID
 *
 * Exception to #2 is when using an image with an Effect ID that is being used to create
 * a chin blur (i.e. chins in Power Swoosh lockups).  This is a special case so we can
 * have the blur effect visible in Chrome.
 *
 * Under these conditions the fileType is also removed, but it's not clear why.
 *
 * @public
 * @param rawURL
 * @param crop
 * @param replaceEffectCode
 */
export function fixEmbeddedCropCode(
    rawURL: string,
    crop: string,
    replaceEffectCode = false,
): string {
    // Normalize URL in case crop or format are hardcoded
    // Test against only the filename portion
    const stringParts = rawURL.split('/');
    const fileName = stringParts.pop();
    let url = rawURL;

    const cropMatches = fileName.match(EMBEDDED_CROP_CODE_REGEX);

    // The last match will be the hard-coded crop code or the replacement indicator: {c}
    const cropMatch = cropMatches ? cropMatches.pop() : null;

    // EffectIds (e.g. SH.FPESS01) are the new artwork crop codes
    // that should not be replaced in the artwork url excpet when used
    // for chin blurs.
    const isEffectMatch = !replaceEffectCode && EFFECT_ID_REGEX.test(fileName);

    if (crop && cropMatch && !isEffectMatch) {
        // Update the url to include the replacement indicator {c} instead of the hard-coded crop value
        // Also update the URL to include the replacement indicator {f} if the file type is hard-coded
        const updatedFilename = replaceEffectCode
            ? // EFFECT_ID_REGEX also captures file type
              fileName.replace(EFFECT_ID_REGEX, '$1x$2{c}.{f}')
            : fileName
                  .replace(EMBEDDED_CROP_CODE_REGEX, '$1x$2{c}')
                  .replace(FILE_TYPE_REGEX, '{f}');

        url = `${stringParts.join('/')}/${updatedFilename}`;
    }

    return url;
}

/**
 * @private
 * Utility for build src for images
 * @param url template url for an image
 * @param urlParams
 * @param options
 * @param chinConfig optional chin configuration for style parameter
 */
export function buildSrc(
    url: string,
    urlParams: ImageURLParams,
    options: ImageSettings,
    chinConfig?: ChinConfig,
): string | null {
    if (!url) return null;

    let returnedUrl = url;

    const { width, height, quality, crop, fileType } = urlParams;

    if (options?.forceCropCode !== false) {
        returnedUrl = fixEmbeddedCropCode(returnedUrl, crop);
    }
    const [parsedURL, defaultQuality] = replaceQualityParam(
        returnedUrl,
        quality,
    );
    returnedUrl = parsedURL;

    const qualityValue = Number.isInteger(quality)
        ? quality.toString()
        : defaultQuality;

    let finalUrl = returnedUrl
        .replace('{w}', width?.toString())
        .replace('{h}', height?.toString())
        .replace('{c}', crop)
        .replace('{q}', qualityValue)
        .replace('{f}', fileType);

    // Add style query parameter for chin effects if specified
    if (chinConfig?.style) {
        const separator = finalUrl.includes('?') ? '&' : '?';
        finalUrl += `${separator}style=${chinConfig.style}`;
    }

    return finalUrl;
}

/**
 * Wrapper for buildSrc helper
 * - Preserves effect ids in urls used for SEO
 * @param {string} url
 * @param {ImageURLParams} urlParams
 * @return string | null
 */
export function buildSrcSeo(
    url: string,
    urlParams: ImageURLParams,
): string | null {
    const options = { ...urlParams };

    // Preserve effect ids when generating seo image urls
    if (EFFECT_ID_REGEX.test(url)) {
        delete options.crop;
    }

    return buildSrc(url, options, {});
}

/**
 * This function generates a value for the `srcset` attribute
 * based on a URL and image options.
 *
 * @private
 * @param rawURL The raw URL
 * @param urlParams custom image parameters
 * @param pixelDensity pixel density to optimize for
 * @param options k/v map of other constant options that don't depend on viewport size.
 * @return The `srcset` attribute value
 * @public
 */
function buildSingleSrcset(
    rawURL: string,
    urlParams: ImageURLParams,
    artworkSizes: ArtworkMaxSizes,
    pixelDensity: number,
    options: ImageSettings,
    chinConfig?: ChinConfig,
): string {
    const { maxWidth } = artworkSizes;
    const profileHeight = urlParams.height;
    const profileWidth = urlParams.width;
    const chinHeight = chinConfig?.height ?? 0;

    const calculatedWidth = Math.ceil(profileWidth * pixelDensity);
    const { crop } = urlParams;

    // use profile width if maxWidth is null or 0
    // TODO: rdar://92133085 (Add logging to shared components)
    const artworkMaxWidth = maxWidth || calculatedWidth;

    // prevent pixel dense images from being wider
    // than the OG size of the image
    // unless its using a fill
    const width = isAFillCropCode(crop)
        ? calculatedWidth
        : Math.min(calculatedWidth, artworkMaxWidth);
    const height =
        Math.round((width * profileHeight) / profileWidth) +
        Math.round(chinHeight * pixelDensity);

    const passedOptions = options;

    const fixedUrlParams = {
        ...urlParams,
        crop,
        width,
        height,
    };

    const url = buildSrc(rawURL, fixedUrlParams, passedOptions, chinConfig);

    return `${url} ${fixedUrlParams.width}w`;
}

/**
 * Returns a string that can be used as the value for the srcset attribute.
 *
 * @function buildResponsiveSrcset
 * @param urlParams list of `urlOptions`. See `buildSrcset` for details.
 * @param options some other options to opt into behavior. See `buildSrcset` for details.
 * @returns srcset string
 */
export function buildResponsiveSrcset(
    url: string,
    urlParams: Partial<ImageURLParams>,
    profile: Profile | string,
    artworkSizes: ArtworkMaxSizes,
    options: ImageSettings,
    chinConfig?: ChinConfig,
): string {
    const urlParamsArray = deriveUrlParamsArray(
        urlParams,
        profile,
        artworkSizes.maxWidth,
    );
    const DEFAULT_OPTIONS: Partial<ImageSettings> = {
        forceCropCode: false,
    };
    const {
        pixelDensities = PIXEL_DENSITIES,
        ...optionsWithoutPixelDensities
    } = options;

    // merging custom options with defaults
    const finalOptions: ImageSettings = {
        ...DEFAULT_OPTIONS,
        ...optionsWithoutPixelDensities,
    };

    // using a Set to prevent multiple of the same srcs being added.
    const srcSetStrings = new Set();

    // eslint-disable-next-line no-restricted-syntax
    for (const pixelDensity of pixelDensities) {
        // eslint-disable-next-line no-restricted-syntax
        for (const singleURLParam of urlParamsArray) {
            srcSetStrings.add(
                buildSingleSrcset(
                    url,
                    singleURLParam,
                    artworkSizes,
                    pixelDensity,
                    finalOptions,
                    chinConfig,
                ),
            );
        }
    }
    return [...srcSetStrings].join(',');
}

/**
 * get size attributes based on breakpoints.
 * @param width width of image
 * @param height height of image
 * @param imageMultipler custom multipler to use for image sizes
 */

function imageSizes(
    profile?: Profile | string,
    maxWidth: number = null,
): string {
    const [sizeMap, mediaConditions] = getSizesAndBreakpoints(profile);

    const filteredSizes = Object.entries(sizeMap).filter(([, config]) =>
        filterSizeConfig(config, maxWidth),
    );

    const sizes = filteredSizes.map(([sizeName, config], index, arr) => {
        let condition = mediaConditions[sizeName];
        const { width } = config;
        const widthString = `${width}px`;
        const isFirst = index === 0;
        const isLast = index === arr.length - 1;

        // The smallest size in the 'sizes' attribute shouldn't have a min size
        // or it will cause anything below that size to default
        // to the last size (aka the largest image).
        if (isFirst) {
            const conditions = condition.split('and');
            if (conditions.length > 1) {
                const [, maxCondition] = conditions;
                condition = maxCondition;
            }
        }
        if (isLast) {
            // The last size in the `sizes` attr should not contain the media condition
            // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#attr-sizes
            return widthString;
        }

        // Creates an option like this:
        // (min-width: something) 111px;
        return `${condition} ${widthString}`;
    });
    return sizes.length
        ? sizes.join(',')
        : `${getSmallestProfileSize(sizeMap).width}w`;
}

export const getImageSizes = memoize(imageSizes);

export function buildSourceSet(
    artwork: Artwork,
    options: ImageSettings,
    profile: Profile | string,
    chinConfig?: ChinConfig,
): string | null {
    const fileType = options.fileType || DEFAULT_FILE_TYPE;
    let qualityValue = options.quality || DEFAULT_QUALITY;
    let sourceSet = null;

    const isWebp = fileType === 'webp';
    if (isWebp && qualityValue === DEFAULT_QUALITY) {
        qualityValue = null;
    }

    const [url, urlParams, maxSizes] = deriveDataFromArtwork(
        artwork,
        qualityValue,
        fileType,
        chinConfig,
    );

    if (url) {
        // If the url doesn't have a {f} (file type) placeholder, we do not want
        // to force webp sources.
        const isNotWebpException = !(isWebp && !url.includes('{f}'));
        if (isNotWebpException) {
            sourceSet = buildResponsiveSrcset(
                url,
                urlParams,
                profile,
                maxSizes,
                options,
                chinConfig,
            );
        }
    }

    return sourceSet;
}
