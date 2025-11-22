import type I18N from '@amp/web-apps-localization';
import he from 'he';

export function isString(string: unknown): string is string {
    return typeof string === 'string';
}

export function concatWithMiddot(pieces: string[], i18n: I18N): string {
    if (!pieces.length) {
        return '';
    }

    return (
        pieces.reduce((memo, current) => {
            return i18n.t('ASE.Web.AppStore.ContentA.Middot.ContentB', {
                contentA: memo,
                contentB: current,
            });
        }) || ''
    );
}

/**
 * Truncates a block of text to fit within a character limit, with a bias towards ending on a
 * full sentence. If no complete sentence fits within the limit, it falls back to a word-based
 * truncation with an ellipsis.
 *
 * @param {string} text - The text to truncate.
 * @param {number} limit - The maximum number of characters allowed before truncation.
 * @param {string} [locale=en_US] - The locale to use when breaking the text into segments.
 * @returns {string} Truncated text clipped to the limit, ideally ending on a natural stopping point.
 */
export function truncateAroundLimit(
    text: string,
    limit: number,
    locale: string = 'en-US',
): string {
    // If the text is shorter than the limit, return all the text, unaltered.
    if (text.length <= limit) {
        return text;
    }

    const decodedText = he.decode(text);

    const isSegemnterSupported = typeof Intl.Segmenter === 'function';
    const terminatingPunctuation = '…';

    // A very naive fallback if the browser doesn't support `Segementer`,
    // which just truncates the text to the last space before the `limit`.
    if (!isSegemnterSupported) {
        const truncatedText = decodedText.slice(0, limit);
        const indexOfLastSpace = truncatedText.lastIndexOf(' ');
        if (indexOfLastSpace) {
            return (
                truncatedText.slice(0, indexOfLastSpace).trim() +
                terminatingPunctuation
            );
        } else {
            // If the text is an _exteremly_ long word or block of text, like a URL
            return truncatedText.trim() + terminatingPunctuation;
        }
    }

    const sentences = Array.from(
        new Intl.Segmenter(locale, { granularity: 'sentence' }).segment(text),
        (s) => s.segment,
    );

    let result = '';
    for (const sentence of sentences) {
        // If there is still room to add another sentence without going over the limit, add it.
        if (result.length + sentence.length <= limit) {
            result += sentence;
        } else {
            break;
        }
    }

    result = result.trim();

    // If the result we built based on full sentences is close-enough to the desired limit
    // (e.g. within the threshold of 75% of 160), we can use it.
    if (result.length >= limit * 0.75) {
        return result;
    }

    // Otherwise, fallback to building up single words until we approach the limit.
    const segments = Array.from(
        new Intl.Segmenter(locale, { granularity: 'word' }).segment(
            decodedText,
        ),
    );

    result = '';
    for (const { segment } of segments) {
        if (result.length + segment.length <= limit) {
            result += segment;
        } else {
            break;
        }
    }

    return result.trim() + terminatingPunctuation;
}

export function escapeHtml(text: string): string {
    return text
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;');
}

export function commaSeparatedList(items: Array<string>, locale = 'en') {
    return new Intl.ListFormat(locale, {
        style: 'long',
        type: 'conjunction',
    }).format(items);
}

export function stripTags(text: string) {
    return text.replace(/(<([^>]+)>)/gi, '');
}

export function stripUnicodeWhitespace(text: string) {
    return text.replace(/[\u0000-\u001F]/g, '');
}
