export default function focusNode(
    node: HTMLElement,
    focusedIndex: number | null,
) {
    const nodeIndex = Number(node.getAttribute('data-index'));

    // Handle the initial focus when applicable
    if (nodeIndex === focusedIndex) {
        node.focus();
    }

    return {
        update(newFocusedIndex) {
            if (nodeIndex === newFocusedIndex) {
                node.focus();
            }
        },
    };
}
