import { getRafQueue } from '@amp/web-app-components/src/utils/rafQueue';
// TODO: rdar://91082022 (JMOTW: Performance - Refactor IntersectionObserver Admin Locally)
import IntersectionObserverAdmin from 'intersection-observer-admin';

// Threshold is how much of the target element is currently visible within the
// root's intersection ratio, as a value between 0.0 and 1.0.
// https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRatio
//
// Examples:
// 0 = a single visible pixel counts as the target being "visible"
// 1 = a single non-visible pixel counts as the target being "not visible""
const DEFAULT_VIEWPORT_THRESHOLD = 0.6;

// https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver#properties
// Adding `callback` to the type since you can only pass an array or object into actions
type configObject = {
    root?: Element | null;
    rootMargin?: string;
    threshold?: number;
    callback?: Function;
};

let intersectionObserverAdmin;

/**
 * IntersectionObserver action to track when an element comes in to/goes out of the visible viewport.
 * Useful for stopping animations of elements no longer visible, starting animations when
 * they appear/reappear, applying/removing styles, etc.
 *
 * Callbacks will be called with a boolean depending on if the item is intersecting (true) or not (false).
 *
 * Utilizes Intersection Observer Admin (https://github.com/snewcomer/intersection-observer-admin) to allow
 * the setup of a single Intersection Observer queue that handles observations in a way that allows each
 * element to have it's own callback and IntersectionObserver configuration.
 *
 * @function intersectionObserver
 * @param {Element} target Element to track (DOM element, Document, or null for top-level document viewport)
 * @param {configObject} options callback function for handling viewport visiblity changes
 *
 * @example `<div use:intersectionObserver={{ callback: handleIntersectionUpdate }}></div>`
 * @example `<div use:intersectionObserver={{
 *              callback: handleIntersectionUpdate,
 *              root: document.querySelector('some-element')
 *          }}></div>`
 * @example `<div use:intersectionObserver={{
 *              callback: handleIntersectionUpdate,
 *              root: document.querySelector('some-element'),
 *              threshold: 1
 *          }}></div>`
 * @example `<div use:intersectionObserver={{
 *              callback: handleIntersectionUpdate,
 *              root: document.querySelector('some-element'),
 *              rootMargin: '0px 0px 0px 0px',
 *              threshold: 1
 *          }}></div>`
 */
export function intersectionObserver(
    target: Element,
    options: configObject = {},
): { destroy: () => void } {
    if (!('IntersectionObserver' in window)) return;

    if (!options.callback) {
        console.warn(
            'Use of intersectionObserver action requires passing in a callback function',
        );
        return;
    }

    const rafQueue = getRafQueue();
    const customCallback = options.callback;

    // Clone options to manipulate object without side effects
    // Assign initial default threshold, overridden by any settings in `options`
    const optionsObj = Object.assign(
        { threshold: DEFAULT_VIEWPORT_THRESHOLD },
        options,
    );
    delete optionsObj.callback;

    const callback = (ioEntry) => {
        rafQueue.add(() => customCallback(ioEntry.isIntersecting));
    };

    if (!intersectionObserverAdmin) {
        intersectionObserverAdmin = new IntersectionObserverAdmin();
    }

    // Add callbacks that will be called when observer detects entering and leaving viewport
    intersectionObserverAdmin.addEnterCallback(target, callback);
    intersectionObserverAdmin.addExitCallback(target, callback);

    intersectionObserverAdmin.observe(target, optionsObj);

    return {
        destroy() {
            intersectionObserverAdmin.unobserve(target, optionsObj);
        },
    };
}
