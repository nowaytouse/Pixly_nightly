import type { Action } from '@amp/web-app-components/src/types';

// eslint-disable-next-line import/prefer-default-export
export function observe(
    node: HTMLElement,
    observer: IntersectionObserver,
): Action {
    let oldObserver: IntersectionObserver | undefined;

    function update(observerInstance: IntersectionObserver): void {
        if (oldObserver === observerInstance || !observerInstance) {
            return;
        }

        if (oldObserver) {
            oldObserver.unobserve(node);
        }

        observerInstance.observe(node);
        oldObserver = observerInstance;
    }

    update(observer);

    return {
        update,
        destroy() {
            oldObserver?.unobserve(node);
        },
    };
}
