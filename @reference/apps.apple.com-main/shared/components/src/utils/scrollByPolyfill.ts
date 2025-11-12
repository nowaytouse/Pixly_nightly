// COPIED FROM
// https://github.pie.apple.com/amp-ui/ember-ui-media-shelf/blob/580ff07a546771bce8b3d85494c6268860e97215/addon/-private/scroll-by-polyfill.js

const SCROLL_TIME = 468;
const Element =
    typeof window !== 'undefined' ? window.HTMLElement || window.Element : null;

let originalScrollBy;

/**
 * returns result of applying ease math function to a number
 * @method ease
 * @param {Number} k
 * @returns {Number}
 */
function ease(k: number): number {
    return 0.5 * (1 - Math.cos(Math.PI * k));
}

// define timing method
const now: () => number =
    typeof window !== 'undefined' && window?.performance?.now
        ? window.performance.now.bind(window.performance)
        : Date.now;

/**
 * changes scroll position inside an element
 * @method scrollElement
 * @param {Number} x
 * @returns {undefined}
 */
function scrollElement(x: number): void {
    this.scrollLeft = x;
}

/**
 * self invoked function that, given a context, steps through scrolling
 * @method step
 * @param {Object} context
 * @returns {undefined}
 */
type Context = {
    startTime: number;
    startX: number;
    x: number;
    method: (x: number) => void;
    scrollable: HTMLElement;
};
function step(context: Context): void {
    const time = now();
    let elapsed = (time - context.startTime) / SCROLL_TIME;

    // avoid elapsed times higher than one
    elapsed = Math.min(1, elapsed);

    // apply easing to elapsed time
    const value = ease(elapsed);

    const currentX = context.startX + (context.x - context.startX) * value;

    context.method.call(context.scrollable, currentX);

    // scroll more if we have not reached our destination
    if (currentX !== context.x) {
        window.requestAnimationFrame(step.bind(window, context));
    }
}

/**
 * scrolls window or element with a smooth behavior
 * @method smoothScroll
 * @param {Object|Node} el
 * @param {Number} x
 * @returns {undefined}
 */
function smoothScroll(el: HTMLElement, x: number): void {
    const startTime = now();
    // define scroll context
    const startX = el.scrollLeft;
    const method = scrollElement;

    // scroll looping over a frame
    step({
        scrollable: el,
        method,
        startTime,
        startX,
        x,
    });
}

let polyfillHasRun = false;
/**
 * ripped partially from https://github.com/iamdustan/smoothscroll/blob/master/src/smoothscroll.js
 * Only polyfill horizontal scroll space to avoid unexpected behaviour in parent apps
 *
 * @method scrollByPolyfill
 */
export default function scrollByPolyfill(): void {
    // return if scroll behavior is supported
    if ('scrollBehavior' in document.documentElement.style || polyfillHasRun) {
        return;
    }

    // if prefers-reduce-motion && need polyfill, navigate shelf immediately without easing
    const motionMediaQuery = window.matchMedia(
        '(prefers-reduced-motion: reduce)',
    );
    function addScrollByToProto() {
        if (motionMediaQuery.matches) {
            if (originalScrollBy) {
                Element.prototype.scrollBy = originalScrollBy;
            }
            return;
        }

        function scrollByPoly(options: ScrollToOptions): void;
        function scrollByPoly(x: number, _y: number): void;
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        function scrollByPoly(
            paramOne: number | ScrollToOptions,
            _paramTwo?: number,
        ): void {
            let xValue = 0;
            if (typeof paramOne === 'number') {
                xValue = paramOne;
            } else if (typeof paramOne === 'object') {
                xValue = paramOne.left || 0;
            }

            const moveByX = this.scrollLeft + xValue;
            smoothScroll(this, moveByX);
        }

        originalScrollBy = Element.prototype.scrollBy;
        Element.prototype.scrollBy = scrollByPoly;
    }

    motionMediaQuery.addListener(addScrollByToProto);

    addScrollByToProto();
    polyfillHasRun = true;
}
