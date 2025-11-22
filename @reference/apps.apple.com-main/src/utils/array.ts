/**
 * Split an array into two groups based on the result {@linkcode predicate}
 *
 * Items for which {@linkcode predicate} returns `true` will be in the "left"
 * result, and the others in the "right" one
 */
export function partition<T>(
    input: Array<T>,
    predicate: (element: T) => boolean,
): [Array<T>, Array<T>] {
    const left: Array<T> = [];
    const right: Array<T> = [];

    for (const element of input) {
        if (predicate(element)) {
            left.push(element);
        } else {
            right.push(element);
        }
    }

    return [left, right];
}

/**
 * Deduplicate the elements of {@linkcode items} by their `id` property
 */
export function uniqueById<T extends { id: string }>(items: T[]): T[] {
    const entries = items.map((item) => [item.id, item] as const);
    const mapById = new Map<string, T>(entries);

    return Array.from(mapById.values());
}
