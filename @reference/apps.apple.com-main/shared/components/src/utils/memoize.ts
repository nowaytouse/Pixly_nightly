// eslint-disable-next-line import/prefer-default-export
export function memoize<T extends unknown[], S>(
    fn: (...args: T) => S,
    hashFn: (...args: unknown[]) => string = JSON.stringify,
    entryLimit = 5,
): (...args: T) => S {
    const cache: Map<string, S> = new Map();

    return (...args: T) => {
        const value = hashFn(args);
        if (cache.has(value)) {
            return cache.get(value);
        }

        const returnedValue: S = fn.apply(this, args);

        if (cache.size >= entryLimit) {
            const iterator = cache.keys();
            const firstValue = iterator.next().value;
            // remove oldest value
            cache.delete(firstValue);
        }
        cache.set(value, returnedValue);
        return returnedValue;
    };
}
