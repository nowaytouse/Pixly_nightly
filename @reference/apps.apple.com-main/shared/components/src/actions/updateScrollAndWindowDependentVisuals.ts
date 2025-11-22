import { debounce } from '@amp/web-app-components/src/utils/debounce';
import { throttle } from '@amp/web-app-components/src/utils/throttle';
/**
 * Dynamically change header and bottom gradient style when scrolling within a modal, and on window resize
 */
export function updateScrollAndWindowDependentVisuals(node) {
    let animationRequest;
    const handleScroll = () => {
        // Get scroll details
        const { scrollHeight, scrollTop, offsetHeight } = node;
        const maxScroll = scrollHeight - offsetHeight;

        // Calculate whether content is scrolled
        const contentIsScrolling = scrollTop > 1;

        // Calculate if bottom gradient should be hidden
        const scrollingNotPossible = maxScroll === 0;
        const pastMaxScroll = scrollTop >= maxScroll;
        const hideGradient = scrollingNotPossible || pastMaxScroll;

        if (animationRequest) {
            window.cancelAnimationFrame(animationRequest);
        }

        animationRequest = window.requestAnimationFrame(() =>
            node.dispatchEvent(
                new CustomEvent('scrollStatus', {
                    detail: { contentIsScrolling, hideGradient },
                }),
            ),
        );
    };

    const onResize = throttle(handleScroll, 250);
    const onScroll = debounce(handleScroll, 50);
    node.addEventListener('scroll', onScroll, { capture: true, passive: true });
    window.addEventListener('resize', onResize);

    return {
        destroy() {
            node.removeEventListener('scroll', onScroll, { capture: true });
            window.removeEventListener('resize', onResize);
            if (animationRequest) {
                window.cancelAnimationFrame(animationRequest);
            }
        },
    };
}
