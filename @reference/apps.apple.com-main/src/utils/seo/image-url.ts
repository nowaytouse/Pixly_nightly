import type { URL } from 'schema-dts';
import type { Opt } from '@jet/environment/types/optional';

import type { CropCode } from '@amp/web-app-components/src/components/Artwork/types';
import { buildSrcSeo } from '@amp/web-app-components/src/components/Artwork/utils/srcset';

const RECOMMENDED_OPEN_GRAPH_IMAGE_WIDTH = 1200;
const RECOMMENDED_OPEN_GRAPH_IMAGE_HEIGHT = 630;

const DEFAULT_OPEN_GRAPH_IMAGE_CROP = 'bb';
const DEFAULT_OPEN_GRAPH_IMAGE_FILE_TYPE = 'png';

/**
 * Generate an OpenGraph image URL from a Media API artwork definition
 *
 * This overrides the default size of the image with the recommendations
 * from the Open Graph documentation
 */
export function buildOpenGraphImageURL(
    artworkDefinition: Opt<MapLike<JSONValue>>,
    crop: CropCode = DEFAULT_OPEN_GRAPH_IMAGE_CROP,
): URL | undefined {
    if (!artworkDefinition) {
        return undefined;
    }

    const { url } = artworkDefinition;

    if (typeof url !== 'string') {
        return undefined;
    }

    return (
        buildSrcSeo(url, {
            crop,
            width: RECOMMENDED_OPEN_GRAPH_IMAGE_WIDTH,
            height: RECOMMENDED_OPEN_GRAPH_IMAGE_HEIGHT,
            fileType: DEFAULT_OPEN_GRAPH_IMAGE_FILE_TYPE,
        }) ?? undefined
    );
}

/**
 * Construct a metadata-friendly URL for some Media API-provided artwork
 */
export function buildImageURL(
    artworkDefinition: Opt<MapLike<JSONValue>>,
): URL | undefined {
    if (!artworkDefinition) {
        return undefined;
    }

    const { url, width, height } = artworkDefinition;

    if (
        typeof url !== 'string' ||
        typeof width !== 'number' ||
        typeof height !== 'number'
    ) {
        return undefined;
    }

    return (
        buildSrcSeo(url, {
            crop: DEFAULT_OPEN_GRAPH_IMAGE_CROP,
            width,
            height,
            fileType: DEFAULT_OPEN_GRAPH_IMAGE_FILE_TYPE,
        }) ?? undefined
    );
}
