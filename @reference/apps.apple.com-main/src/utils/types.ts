/**
 * Determine if {@linkcode input} matches the `"object"` type
 */
export function isObject(input: unknown): input is object {
    return typeof input === 'object' && !!input;
}

type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never };

/**
 * Helper type for creating an exclusive union between two types
 *
 * @see {@link https://stackoverflow.com/a/53229567/2250435 | StackOverflow Post}
 */
export type XOR<T, U> = T | U extends object
    ? (Without<T, U> & U) | (Without<U, T> & T)
    : T | U;
