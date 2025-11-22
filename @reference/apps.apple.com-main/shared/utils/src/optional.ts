export type Optional<T> = T | None;
export type None = null | undefined;

/**
 * Determine if an optional value is present.
 *
 * @param optional value
 * @return true if present, false otherwise
 */
export function isSome<T>(optional: Optional<T>): optional is T {
    return optional !== null && optional !== undefined;
}

/**
 * Determine if an optional value is not present.
 *
 * @param optional value
 * @return true if not present, false otherwise
 */
export function isNone<T>(optional: Optional<T>): optional is None {
    return optional === null || optional === undefined;
}
