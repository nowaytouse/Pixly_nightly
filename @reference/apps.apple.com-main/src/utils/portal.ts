/**
 * Svelte action to move an element to a different part of the DOM (as specified by the `targetId`
 * provided), effectively creating a "portal."
 *
 * @param {HTMLElement} node - The element to be moved (provided by Svelte's `use:action` syntax).
 * @param {string} targetId - The ID of the target element where `node` should be moved.
 * @returns {{ destroy(): void } | void} - An object with a `destroy` method to remove `node` from the target when unmounted.
 *
 * @example
 * ```svelte
 * <div use:portal={'target-container'}>
 *   This content will be moved to the element with ID "target-container".
 * </div>
 * ```
 */
export default function portal(node: HTMLElement, targetId: string) {
    if (typeof document === 'undefined') {
        return;
    }

    let targetElement: HTMLElement | null = document.getElementById(targetId);

    if (!targetElement) {
        return;
    }

    targetElement.appendChild(node);

    return {
        destroy() {
            targetElement.removeChild(node);
        },
    };
}
