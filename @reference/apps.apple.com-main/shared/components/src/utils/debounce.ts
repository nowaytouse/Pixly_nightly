/* eslint-disable import/prefer-default-export */

/**
 * @name debounce
 * @description
 * Creates a debounced function that delays invoking func until
 * after delayMs milliseconds have elapsed since the last time the
 * debounced function was invoked.
 *
 * @param delayMs - delay in milliseconds
 * @param immediate - Specify invoking on the leading edge of the timeout
 * (Defaults to trailing)
 *
 *(f: F): (...args: Parameters<F>) => void
 */
export function debounce<F extends (...args: any[]) => any>(
    fn: F,
    delayMs: number,
    immediate = false,
): (...args: Parameters<F>) => void {
    let timerId;

    return function debounced(...args) {
        const shouldCallNow = immediate && !timerId;
        clearTimeout(timerId);

        if (shouldCallNow) {
            fn.apply(this, args);
        }

        timerId = setTimeout(() => {
            timerId = null;
            if (!immediate) {
                fn.apply(this, args);
            }
        }, delayMs);
    };
}

export const DEFAULT_MOUSE_OVER_DELAY = 300;
