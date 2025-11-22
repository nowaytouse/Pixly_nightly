/**
 * COPIED FROM: https://github.pie.apple.com/amp-ui/ember-ui-media-artwork/blob/main/addon/utils/srcset.js
 * and converted public functions to TypeScript
 */

import type { CropCode, FileExtension } from './types';

const baseWidthHeightRegex = '({w}|[0-9]+)x({h}|[0-9]+)';
const baseFileTypeRegex = '{f}|([a-zA-Z]{3,4})';
// ([A-z]{1,6}\\.[\\w]{1,8}) - copy pasta of the regex used on the backend for EffectIds
// https://github.pie.apple.com/amp/ai-imageservice/blob/84abff624a2da5b45bdf91c5bcd87b6708ad12ae/is-foundation/src/main/java/com/apple/imageservice/foundation/program/EffectId.java#L22
const baseEffectCropCode = '[A-z]{1,6}\\.[\\w]{1,8}';

export const EMBEDDED_CROP_CODE_REGEX = new RegExp(
    `^${baseWidthHeightRegex}([a-zA-Z]+)`,
);
export const FILE_TYPE_REGEX = new RegExp(baseFileTypeRegex);
// TODO: rdar://97913309 (JMOTW: Artwork: Quality Param regex injects quality placeholder when no hardcoded quality param exists)
export const QUALITY_PARAM_REGEX = /(-[0-9]+)?\.(\{f\}|[A-z]{2,4})$/;

export const EFFECT_ID_REGEX = new RegExp(
    `^${baseWidthHeightRegex}(${baseEffectCropCode})\\.(${baseFileTypeRegex})`,
);

// non capturing to ignore either effect cc or regular cc
export const REPLACE_CROP_CODE_REGEX = new RegExp(
    `${baseWidthHeightRegex}(?:${baseEffectCropCode}|[a-z]{1,2})\\.(${baseFileTypeRegex})`,
);

export const DEFAULT_QUALITY = 60;

// Specific viewport widths that don't align cleanly with media query breakpoints
export const LN_TALL_BREAKPOINT_WIDTH = 729;
export const ARTIST_VIDEO_TALL_BREAKPOINT_WIDTH = 674;

/**
 * Instead of reading pixel density (which is different in fastboot and browser),
 * we'll bake in support for 1x and 2x pixel densities. This means a larger
 * set of sources, but it means we don't have to recalculate and potentially double
 * download images.
 * @export const PIXEL_DENSITIES
 * @private
 */
export const PIXEL_DENSITIES = [1, 2];

/**
 * default cropcode if none is provided
 */
export const DEFAULT_CROP: CropCode = 'fa';

/**
 * default fileType if none is provided
 */
export const DEFAULT_FILE_TYPE: FileExtension = 'jpg';

export const ASPECT_RATIOS = {
    HD: 16 / 9,
    ONE_THIRD: 3 / 1,
    ONE: 1,
    THREE_QUARTERS: 3 / 4,
    UBER: 4,
    HD_ASPECT_RATIO: 16 / 9,
    VIDEO_LIST: 7 / 4,
    VIDEO_TALL: 9 / 16,
    HERO: 68 / 39,
    SUPER_HERO_WIDE: 22 / 9,
    WELCOME: 466 / 293,
    EDITORIAL_DEFAULT: 68 / 39,
} as const;

export const FILE_EXTENSIONS = ['jpg', 'webp', 'png'] as const;

export const FILE_TO_MIME_TYPE = {
    jpg: 'image/jpeg',
    png: 'image/png',
    webp: 'image/webp',
} as const;

// https://confluence.sd.apple.com/pages/viewpage.action?spaceKey=AMPDSCE&title=Crop+Code+Master+List
export const ALL_CROP_CODES = [
    '{c}',
    'at',
    'ac',
    'bb',
    'bw',
    'bf',
    'br',
    'h',
    'w',
    'cc',
    'cx',
    'ca',
    'cb',
    'cw',
    'cu',
    'cy',
    'cv',
    'rc',
    'rs',
    'sr',
    'ss',
    'fa',
    'fb',
    'fc',
    'fd',
    'fe',
    'ff',
    'fg',
    'fh',
    'fi',
    'fj',
    'fk',
    'fl',
    'fm',
    'fn',
    'fo',
    'fp',
    'fq',
    'fr',
    'fs',
    'ft',
    'fu',
    'fv',
    'fw',
    'fx',
    'fy',
    'ea',
    'eb',
    'ec',
    'ed',
    'ee',
    'ef',
    'eg',
    'eh',
    'ei',
    'ej',
    'ek',
    'el',
    'em',
    'en',
    'eo',
    'ep',
    'eq',
    'er',
    'es',
    'et',
    'eu',
    'ev',
    'ew',
    'ex',
    'ey',
    'ez',
    'ga',
    'gb',
    'gc',
    'lg',
    'lw',
    'lc',
    'ld',
    'la',
    'lb',
    'lt',
    'lh',
    'mv',
    'mw',
    'mf',
    'nr',
    'sy',
    'sx',
    'sz',
    'sa',
    'sb',
    'sc',
    'sd',
    'se',
    'sf',
    'sg',
    'sh',
    'si',
    'sj',
    'sk',
    'va',
    'vb',
    'vc',
    'vd',
    've',
    'vf',
    'vi',
    'vj',
    'vl',
    'wp',
    'wa',
    'wb',
    'wc',
    'wd',
    'we',
    'wf',
    'wg',
    'wv',
    'wx',
    'wy',
    'wz',
    'ta',
    'tb',
    'tc',
    'td',
    'oa',
    'ob',
    'oc',
    'od',
    'oe',
    'of',
    'og',
    'oh',
    'Sports.TVAGPW01',
    'Sports.SS1x101',
    'PH.WSAHS01',
] as const;

const isLoadingAvailable =
    typeof HTMLImageElement !== 'undefined' &&
    'loading' in HTMLImageElement.prototype;

export const shouldUseLazyLoader =
    typeof window !== 'undefined' &&
    window.IntersectionObserver &&
    !isLoadingAvailable;
