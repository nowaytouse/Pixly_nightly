import { cubicOut } from 'svelte/easing';
import type { EasingFunction, TransitionConfig } from 'svelte/transition';

interface FlyAndBlurParams {
    // Time (ms) before the animation starts.
    delay?: number;
    // Total animation time (ms).
    duration?: number;
    // Easing function (defaults to cubicOut).
    easing?: EasingFunction;
    // Horizontal offset in pixels at start (like `fly`).
    x?: number;
    // Vertical offset in pixels at start (like `fly`).
    y?: number;
    // Initial blur radius in pixels.
    blur?: number;
}

export function flyAndBlur(
    node: Element,
    {
        delay = 0,
        duration = 420,
        easing = cubicOut,
        x = 0,
        y = 0,
        blur = 3,
    }: FlyAndBlurParams = {},
): TransitionConfig {
    const style = getComputedStyle(node);
    const initialOpacity = +style.opacity;

    return {
        delay,
        duration,
        easing,
        css: (t: number, u: number) => {
            return `
                    transform: translate(${x * u}px, ${y * u}px);
                    opacity: ${initialOpacity * t};
                    filter: blur(${blur * u}px);
                `;
        },
    };
}
