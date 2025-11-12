/* eslint-disable import/prefer-default-export */
/**
 * @name throttle
 * @description
 * Creates a throttled function that only invokes func at most once per every limit time (ms).
 *
 * *NOTE: this does not capture or recall all functions that were triggered.
 * This will drop function calls that happen during the throttle time*
 * @param limit - time to wait between calls in ms
 * @example
 * Normal event
 * event      | |   |   |
 * time     ----------------
 * callback   | |   |   |
 *
 * Throttled event [300ms]
 * event    |   |   |    |
 * time     ----------------
 * callback |      |      |
 *             [300]  [300]
 */

export function throttle<T extends []>(
    func: (..._: T) => unknown,
    limit: number,
): (..._: T) => void {
    let lastTimeoutId;
    let lastCallTime: number;

    return function throttled(...args) {
        const nextCall = () => {
            func.apply(this, args);
            lastCallTime = Date.now();
        };

        if (!lastCallTime) {
            nextCall();
        } else {
            clearTimeout(lastTimeoutId);
            const timeBetweenCalls = Date.now() - lastCallTime;
            const waitTime = Math.max(0, limit - timeBetweenCalls);
            lastTimeoutId = setTimeout(() => {
                if (timeBetweenCalls >= limit) {
                    nextCall();
                }
            }, waitTime);
        }
    };
}
