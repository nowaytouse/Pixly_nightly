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
export function serializeJSONData(data: object): string {
    try {
        return JSON.stringify(data).replace(
            pattern,
            (match) => replacements[match],
        );
    } catch (e) {
        // Don't let recursive data (or other non-serializable things) throw.
        // We'd rather just let the serialize no-op to avoid breaking consumers.
        return '';
    }
}
