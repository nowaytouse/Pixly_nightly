import { QUALITY_PARAM_REGEX } from '@amp/web-app-components/src/components/Artwork/constants';

/**
 * Utility function that handles the replacement of quality value.
 * Does not add any values to the URL string.  Just replaces any hardcoded values
 * with the quality placeholder.
 *
 * @param url image url
 * @param quality quality value
 * @returned url and the defaultQuality from URL
 */
// eslint-disable-next-line import/prefer-default-export
export function replaceQualityParam(
    url: string,
    quality?: number,
): [string, string] {
    const hasQualityPlaceholder = /-\{q\}/.test(url);
    // Convert url string to URL object
    // Some image URLs, like those for radio stations that are formatted with effect codes,
    // may have query params in the path which are used to build out the image with other
    // images/effects.  Ensure we only modify the image path and not the query params.
    const urlObj = new URL(url);

    // Split URL.pathname into parts, so we are only modifying the very last portion of the path
    const lastURLPartIdx = urlObj.pathname.lastIndexOf('/');
    const firstURLpart = urlObj.pathname.substring(0, lastURLPartIdx);
    let lastURLpart = decodeURI(urlObj.pathname.substring(lastURLPartIdx));

    let defaultQuality = '';

    if (quality && !hasQualityPlaceholder) {
        // Find an optional hardcoded quality value (e.g. `-80`)
        // And then find the `.` and fileType placeholder (ext)
        lastURLpart = lastURLpart.replace(
            QUALITY_PARAM_REGEX,
            (_match, defaultQualityVal: string, fileType: string) => {
                // only pass update defaultQuality if it exists in the URL
                defaultQuality = defaultQualityVal
                    ? defaultQualityVal.replace('-', '')
                    : defaultQuality;

                return `-{q}.${fileType}`;
            },
        );
    } else if (!quality && hasQualityPlaceholder) {
        // Strip quality param
        lastURLpart = lastURLpart.replace('-{q}', '');
    }

    // Update urlObj with our modified pathname parts and then combine all
    // parts into a final string.
    urlObj.pathname = `${firstURLpart}${lastURLpart}`;
    let updatedURL = urlObj.toString();

    // Need to decode the URL string conversion to preserve curley braces in URL string.
    // Only decoding the last part of the URL, in the event that there may be intentionally
    // escaped characters in other parts of the URL.
    //
    // With decode: .../mza_4812113047298400850.png/{w}x{h}AM.RSMA01.jpg
    // Without decode: .../mza_4812113047298400850.png/%7Bw%7Dx%7Bh%7DAM.RSMA01.jpg
    updatedURL = `${updatedURL.substring(0, lastURLPartIdx)}${decodeURI(
        updatedURL.substring(lastURLPartIdx),
    )}`;

    return [updatedURL, defaultQuality];
}
