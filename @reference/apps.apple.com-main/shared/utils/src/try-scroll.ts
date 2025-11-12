import type { Logger } from '@amp/web-apps-logger';
export interface ScrollableElement {
    scrollTop: number;
    scrollHeight: number;
    offsetHeight: number;
}

// Global is okay here as this only runs in the browser
let nextTry: number | null = null;

export function tryScroll(
    log: Logger,
    getScrollablePageElement: Function,
    scrollY: number,
): void {
    let tries = 0;

    if (nextTry !== null) {
        window.cancelAnimationFrame(nextTry);
    }

    nextTry = window.requestAnimationFrame(function doNextTry() {
        // At 16ms per frame, this is 1600ms
        // See: https://github.com/DockYard/ember-router-scroll/blob/2f17728f/addon/services/router-scroll.js#L56
        if (++tries >= 100) {
            log.warn("wasn't able to restore scroll within 100 frames");
            nextTry = null;
            return;
        }

        let element = getScrollablePageElement();
        if (!element) {
            log.warn(
                'could not restore scroll: the scrollable element is missing',
            );
            return;
        }
        const { scrollHeight, offsetHeight } = element;

        // Only scroll once we're able to get a full screen of content when
        // scrollTop is set to scrollY
        //
        // +16 is a bit of a fudge factor to count for imperfections in
        // features like lazy loading. If the scroll position to restore is
        // the very bottom of the page, then scrollY + offsetHeight must be
        // exactly scrollHeight. But if lazy loading components (for example)
        // cause the page to grow by a few pixels, then this will never hold.
        // Thus, we fudge by a few pixels to be more forgiving in this scenario.
        const canScroll = scrollY + offsetHeight <= scrollHeight + 16;

        if (!canScroll) {
            log.info('page is not tall enough for scroll yet', {
                scrollHeight,
                offsetHeight,
            });

            nextTry = window.requestAnimationFrame(doNextTry);
            return;
        }

        element.scrollTop = scrollY;
        log.info('scroll restored to', scrollY);
        nextTry = null;
    });
}
