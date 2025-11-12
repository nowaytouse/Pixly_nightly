export function focusNodeOnMount(node: HTMLElement) {
    // Wrapping in queueMicrotask ensures the node is attached to the
    // DOM before attempting to focus.
    queueMicrotask(() => node.focus());
}
