import { readable } from 'svelte/store';

const DEFAULT_SETTING = false;

export const prefersReducedMotion = readable(DEFAULT_SETTING, (set) => {
    if (typeof window === 'undefined' || typeof matchMedia === 'undefined') {
        set(DEFAULT_SETTING);
        return;
    }

    const motionQuery = matchMedia('(prefers-reduced-motion)');

    /* istanbul ignore next */
    const motionQueryListener = (): void => {
        set(motionQuery.matches);
    };

    // `addListener` is deprecated but should still be used for compatibility with more browsers.
    motionQuery.addListener(motionQueryListener);

    set(motionQuery.matches);

    return function (): void {
        // `removeListener` is deprecated but should still be used for compatibility with more browsers.
        motionQuery.removeListener(motionQueryListener);
    };
});
