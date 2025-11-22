import { isPOJO } from '@amp/web-apps-utils';

// NOTE: be careful with imports here. This file is imported by browser code,
// so we expect tree shaking to only keep these functions.

const SERVER_DATA_ID = 'serialized-server-data';

// Take care with < (which has special meaning inside script tags)
// See: https://github.com/sveltejs/kit/blob/ff9a27b4/packages/kit/src/runtime/server/page/serialize_data.js#L4-L28
const replacements = {
    '<': '\\u003C',
    '\u2028': '\\u2028',
    '\u2029': '\\u2029',
};

const pattern = new RegExp(`[${Object.keys(replacements).join('')}]`, 'g');

/**
 * Serializes a POJO into a HTML <script> tag that can be read clientside by
 * `deserializeServerData`.
 *
 * Use this to share data between serverside and clientside. Include the
 * returned HTML in the response to a client to allow it to read this data.
 *
 * @param data data to serialize
 * @returns serialized data (or empty string if serialization fails)
 */
export function serializeServerData(data: object): string {
    try {
        const sanitizedData = JSON.stringify(data).replace(
            pattern,
            (match) => replacements[match as keyof typeof replacements],
        );
        return `<script type="application/json" id="${SERVER_DATA_ID}">${sanitizedData}</script>`;
    } catch (e) {
        // Don't let recursive data (or other non-serializable things) throw.
        // We'd rather just let the serialize no-op to avoid breaking consumers.
        return '';
    }
}

/**
 * Deserializes data serialized on the server by `serializeServerData`.
 *
 * @returns deserialized data (or undefined if it doesn't exist/errors)
 */
export function deserializeServerData(): ReturnType<JSON['parse']> | undefined {
    const script = document.getElementById(SERVER_DATA_ID);
    if (!script) {
        return;
    }

    script.parentNode?.removeChild(script);

    try {
        return JSON.parse(script.textContent || '');
    } catch (e) {
        // If the content is malformed, we want to avoid throwing. This
        // situation should be impossible since we control the serialization
        // above.
        return;
    }
}

/**
 * JSON stringify a POJO value in a stable manner. Specifically, this means that
 * objects which are structurally equal serialize to the same string.
 *
 * This is useful when comparing objects serialized by a server against objects
 * build in browser. With plain JSON.stringify(), property order matters and is
 * not guaranteed to be the same. In other words these two objects would
 * JSON.stringify() differently:
 *
 *   { a: 1, b: 2 }
 *   { b: 2, a: 1 }
 *
 * But these are structurally equal--they have the same keys and values.
 *
 * The expected use case for this function is generating keys for a Map for
 * objects from a server that will be compared against objects from the browser.
 * This function should be used on objects returned from `deserializeServerData`
 * before they are used in such contexts.
 *
 * See: https://stackoverflow.com/a/43049877
 */
export function stableStringify(data: unknown): string {
    if (Array.isArray(data)) {
        const items = data.map(stableStringify).join(',');
        return `[${items}]`;
    }

    // Sort object keys before serializing
    if (isPOJO(data)) {
        const keys = [...Object.keys(data)];
        keys.sort();

        const properties = keys
            // undefined values should not get included in stringification
            .filter((key) => typeof data[key] !== 'undefined')
            .map(
                (key) => `${JSON.stringify(key)}:${stableStringify(data[key])}`,
            )
            .join(',');

        return `{${properties}}`;
    }

    return JSON.stringify(data);
}
