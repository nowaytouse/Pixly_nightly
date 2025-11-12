/**
 * Determine if {@linkcode arg} is a Plain Old JavaScript Object.
 *
 * @see https://masteringjs.io/tutorials/fundamentals/pojo
 *
 * @param arg to test
 * @returns true if {@linkcode arg} is a POJO
 */
export function isPOJO(arg: unknown): arg is Record<string, unknown> {
    if (!arg || typeof arg !== 'object') {
        return false;
    }

    const proto = Object.getPrototypeOf(arg);
    if (!proto) {
        return true; // `Object.create(null)`
    }

    return proto === Object.prototype;
}
