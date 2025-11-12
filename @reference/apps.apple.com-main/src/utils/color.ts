import { isSome } from '@jet/environment/types/optional';
import type {
    Artwork,
    Color,
    RGBColor,
    NamedColor,
} from '@jet-app/app-store/api/models';

export type RGB = [number, number, number];

/**
 * Represents a valid RGB color string, in the format "rgb(r, g, b)" or "rgb(r,g,b)".
 * @example
 * "rgb(255, 0, 128)"
 * "rgb(255,0,128)"
 */
type RGBString =
    | `rgb(${number},${number},${number})`
    | `rgb(${number}, ${number}, ${number})`;

export const isRGBColor = (value: Color): value is RGBColor =>
    value.type === 'rgb';

export const isNamedColor = (value: Color): value is NamedColor =>
    value.type === 'named';

const rgbColorAsString = ({ red, green, blue }: RGBColor): string =>
    `rgb(${[red, green, blue].map((color) => Math.floor(255 * color)).join()})`;

export const colorAsString = (color: Color): string => {
    switch (color.type) {
        case 'named':
            // `ios-appstore-app` makes use of the this `placeholderBackground` named color,
            // which it leaves up to the client to manage. Ideally, we could define a CSS property
            // named `--placeholderBackground`, but the media-apps shared logic to determine Artwork
            // background color doesn't respect CSS properties, so we are specifying the hex value.
            // https://github.pie.apple.com/amp-web/media-apps/blame/main/shared/components/src/components/Artwork/utils/validateBackground.ts
            if (color.name === 'placeholderBackground') {
                return '#f1f1f1';
            }

            return `var(--${color.name})`;
        case 'rgb':
            return rgbColorAsString(color);
        case 'dynamic':
            return colorAsString(color.lightColor);
    }
};

/**
 * Parses an RGB string and returns an array of red, green, and blue values.
 *
 * This function extracts the numeric values from an RGB string (e.g., "rgb(255, 0, 128)")
 * and returns them as an array of numbers.
 *
 * @param {RGBString} rgbString - The RGB string to parse.
 * @returns {RGB} An array of three numbers representing the red, green, and blue values, each between 0 and 255.
 *
 * @example
 * getRGBFromString("rgb(255, 0, 128)")  = [255, 0, 128]
 */
export const getRGBFromString = (rgbString: RGBString): RGB => {
    const rgbValues = rgbString.match(/\d+/g) ?? [];
    const rgb: RGB = [0, 0, 0];

    for (const [index] of rgb.entries()) {
        rgb[index] = parseInt(rgbValues[index]);
    }

    return rgb;
};

/**
 * Calculates the relative luminance for an RGB color.
 *
 * This function uses a standardized formula for luminance, which weights the red, green, and blue
 * channels differently to account for human perception.
 * @see {@link https://en.wikipedia.org/wiki/Relative_luminance|Wikipedia: Relative Luminance}
 *
 * @param {RGB} rgb - An array containing red, green, and blue values, each between 0 and 255.
 * @returns {number} The calculated luminance value, a number between 0 (darkest) and 255 (lightest).
 */
export const getLuminanceForRGB = ([r, g, b]: RGB): number => {
    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
};

export function isRGBDarkerThanThreshold([r, g, b]: RGB, threshold = 10) {
    return r <= threshold && g <= threshold && b <= threshold;
}

export function isDark(rgbColor: RGBColor): boolean {
    const { red, green, blue } = rgbColor;
    const rgbValues = [red, green, blue].map((channel) =>
        Math.floor(channel * 255),
    ) as RGB;

    return isRGBDarkerThanThreshold(rgbValues, 127);
}

/**
 * Determines whether an RGB color is approximately grey based on channel similarity.
 *
 * @param {RGB} rgb - An array containing red, green, and blue values, each between 0 and 255.
 * @param {number} [threshold=10] - Maximum allowed difference between color channels to still be considered grey-ish.
 * @returns {boolean} True if the RGB values are close enough to be considered grey.
 */
function isKindOfGrey([r, g, b]: RGB, threshold = 10) {
    return (
        Math.abs(r - g) <= threshold &&
        Math.abs(r - b) <= threshold &&
        Math.abs(g - b) <= threshold
    );
}

/**
 * Generates CSS variables (custom properties) for a background gradient based on the background
 * colors in the specified list of artworks.
 *
 * @param {Artwork[]} artworks - An array of Artwork, each containing a `backgroundColor` property.
 * @param {Object} [options={}] - Optional configuration options.
 * @param {string[]} [options.variableNames=['bottom-left', 'top-right', 'bottom-right', 'top-left']] -
 *        The names of the CSS variables to assign to the extracted colors. The number of colors
 *        used will match the length of this array.
 * @param {(a: RGB, b: RGB) => number} [options.sortFn=() => 0] -
 *        A sorting function for ordering the colors (e.g., by luminance). Defaults to no sorting,
 *        which preserves input order.
 *
 * @returns {string} A CSS string containing custom properties, e.g.,
 *                   "--bottom-left: rgb(255, 0, 0); --top-right: rgb(0, 255, 0);".
 */
export const getBackgroundGradientCSSVarsFromArtworks = (
    artworks: Artwork[],
    {
        variableNames = [
            'bottom-left',
            'top-right',
            'bottom-right',
            'top-left',
        ],
        sortFn = () => 0,
        shouldRemoveGreys = false,
    }: {
        variableNames?: string[];
        sortFn?: (a: RGB, b: RGB) => number;
        shouldRemoveGreys?: boolean;
    } = {},
): string => {
    return artworks
        .map(({ backgroundColor }) => backgroundColor)
        .filter(isSome)
        .filter(isRGBColor)
        .map(
            ({ red, green, blue }): RGB => [
                Math.floor(255 * red),
                Math.floor(255 * green),
                Math.floor(255 * blue),
            ],
        )
        .filter((rgb) => !isRGBDarkerThanThreshold(rgb, 33))
        .filter((rgb) => (shouldRemoveGreys ? !isKindOfGrey(rgb, 10) : true))
        .sort(sortFn)
        .slice(0, variableNames.length)
        .map(
            ([red, green, blue], index) =>
                `--${variableNames[index]}: rgb(${red}, ${green}, ${blue})`,
        )
        .join('; ');
};
