/**
 * LRU Map implementation storing key/values up to a provided size limit. Beyond that
 * size limit, the least recently used entry is evicted.
 *
 * @see https://github.pie.apple.com/isao/lru-map
 */
export class LruMap<K, V> extends Map<K, V> {
    private sizeLimit: number;

    constructor(sizeLimit: number) {
        super();
        this.setSizeLimit(sizeLimit);
        // Needed to convince TS that this is set (it's actually handled by setSizeLimit)
        this.sizeLimit = sizeLimit;
    }

    /**
     * Retrieve a value from the map with a given key.
     * @param key The key for the entry
     * @return value The entry's value (or undefined if non existent)
     */
    get(key: K): V | undefined {
        let value: V | undefined;

        if (this.has(key)) {
            value = super.get(key);

            // Map entries are always in insertion order. So
            // readding, pushes this entry to the top of the LRU.
            this.delete(key);
            super.set(key, value!);
        }

        return value;
    }

    set(key: K, value: V): this {
        super.set(key, value);
        this.prune();
        return this;
    }

    setSizeLimit(newSizeLimit: number): void {
        if (newSizeLimit < 0 || !isFinite(newSizeLimit)) {
            throw new Error(
                `setSizeLimit expects finite positive number, got: ${newSizeLimit}`,
            );
        }

        this.sizeLimit = newSizeLimit;
        this.prune();
    }

    private prune(): void {
        while (this.size > this.sizeLimit) {
            const leastRecentlyUsedKey = this.keys().next().value;
            this.delete(leastRecentlyUsedKey);
        }
    }
}
