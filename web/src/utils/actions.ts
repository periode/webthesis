export function clickOutside(node: HTMLElement) {
    const handleClick = (event: MouseEvent) => {
        if (!node.contains(event.target as Node)) {
            node.dispatchEvent(new CustomEvent('outclick'))
        }
    };

    document.addEventListener("click", handleClick)

    return {
        destroy() {
            document.removeEventListener("click", handleClick, true)
        }
    }
}