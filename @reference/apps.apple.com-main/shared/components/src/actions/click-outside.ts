export default function clickOutside(
    node: HTMLElement,
    handler: (event: any) => void,
) {
    const handleClick = (event) => {
        if (!node.contains(event.target)) {
            handler(event);
        }
    };

    document.addEventListener('click', handleClick);

    return {
        destroy() {
            document.removeEventListener('click', handleClick);
        },
    };
}
