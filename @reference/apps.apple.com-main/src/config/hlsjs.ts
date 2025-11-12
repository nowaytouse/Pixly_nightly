declare global {
    interface Window {
        Hls?: any;
    }
}

/**
 * Base URL for CDN hosting HLS.js files
 */
export const HLSJS_CDN = 'https://js-cdn.music.apple.com/hls.js';

/**
 * HLS.js version to load.
 */
export const HLSJS_VERSION = '2.820.0';

/**
 * Generate a URL for loading HLS.js.
 */
export function generateHLSJSURL(version?: string): URL {
    // FIXME: Add a local storage override for the HLS.js version
    version = version ?? HLSJS_VERSION;

    return new URL(`${HLSJS_CDN}/${version}/hls.js/hls.js`);
}
