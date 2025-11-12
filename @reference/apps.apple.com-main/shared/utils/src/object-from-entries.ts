// TODO: rdar://78109780 (Update to Node 16)
/**
 * Create an object from an iterable of key/value pairs.
 *
 * @param entries The key value pairs (ex. [['a', 1], ['b', 2]])
 * @return        The created object
 */
export function fromEntries<V>(entries: Iterable<readonly [PropertyKey, V]>): {
    [k: string]: V;
} {
    const result: Record<PropertyKey, V> = {};

    for (const [key, value] of entries) {
        result[key] = value;
    }

    return result;
}
