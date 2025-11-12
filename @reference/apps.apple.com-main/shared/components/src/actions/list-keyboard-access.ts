const NAVIGATION_KEY_NAMES = ['ArrowDown', 'ArrowUp'];
const INTERACTABLE_NODE_NAMES = ['A', 'BUTTON'];
export type configObject = {
    listItemClassNames: string;
    isRoving?: boolean;
    listGroupElement?: HTMLElement;
    syncInteractivityWithVisibility?: boolean;
};

type configParams = configObject & { targetElement: HTMLElement };

/**
 * A construct that manages keyboard navigation as it relates to lists.
 * @class
 */

class ListKeyboardAccess {
    private listItemClassNames: Array<string>;
    private listParentElement: HTMLElement;
    private boundFocusInHandler: EventListener;
    private boundKeyDownHandler: EventListener;
    private listGroupElement: HTMLElement | undefined;
    // a current index based on an ancestor parent i.e. `listGroupElement`.
    private currentRootIndex: number = -1;
    // a current index based on an immediate list parent i.e. `listParentElement`.
    private currentIndex: number = -1;
    private isRoving: boolean = false;
    private syncInteractivityWithVisibility: boolean | undefined;
    private intersectionObserver: IntersectionObserver | undefined;

    static isWindowEventBound: boolean = false;

    constructor(options: configParams) {
        const {
            listGroupElement,
            targetElement,
            syncInteractivityWithVisibility,
        } = options;
        this.listParentElement = targetElement;
        this.listGroupElement = listGroupElement;
        this.isRoving = (options.isRoving ?? false) && !!this.listGroupElement;
        this.syncInteractivityWithVisibility = syncInteractivityWithVisibility;

        // converting a string list into an array of CSS class names (note: not selectors).
        this.listItemClassNames = options.listItemClassNames
            ?.split(',')
            .map((className) => className.trim());
        // Attempting to only bind this event once for the purpose of list navigation.
        if (!ListKeyboardAccess.isWindowEventBound) {
            window.addEventListener(
                'keydown',
                ListKeyboardAccess.windowKeyUpHandler,
            );
            ListKeyboardAccess.isWindowEventBound = true;
        }

        if (this.listItemClassNames?.join('').length) {
            this.boundFocusInHandler = this.focusInHandler.bind(this);
            this.boundKeyDownHandler = this.keyDownHandler.bind(this);

            this.listParentElement.addEventListener(
                'focusin',
                this.boundFocusInHandler,
                {
                    capture: true,
                },
            );
            this.listParentElement.addEventListener(
                'keydown',
                this.boundKeyDownHandler,
            );
        } else {
            throw Error('ListKeyboardAccess requires listItemClassNames');
        }

        if (this.syncInteractivityWithVisibility) {
            // Create the observer
            this.intersectionObserver = new IntersectionObserver(
                (entries) => {
                    entries.forEach((entry) => {
                        setItemInteractivity(
                            entry.target as HTMLElement,
                            entry.isIntersecting,
                        );
                    });
                },
                {
                    root: targetElement,
                    rootMargin: '0px',
                    threshold: 0.5,
                },
            );

            const listItems = this.getListItems();
            for (let i = 0; i < listItems.length; i++) {
                this.intersectionObserver.observe(listItems[i]);
            }
        }
    }

    destroy() {
        if (ListKeyboardAccess.isWindowEventBound) {
            window.removeEventListener(
                'keydown',
                ListKeyboardAccess.windowKeyUpHandler,
            );
            ListKeyboardAccess.isWindowEventBound = false;
        }

        this.listParentElement?.removeEventListener(
            'focusin',
            this.boundFocusInHandler,
            {
                capture: true,
            },
        );

        this.listParentElement?.removeEventListener(
            'keydown',
            this.boundKeyDownHandler,
        );

        this.intersectionObserver?.disconnect();
    }

    private getListItems(
        fromlistGroupElement: boolean = false,
    ): Array<HTMLElement> {
        const { listGroupElement, listParentElement } = this;
        const root =
            fromlistGroupElement && listGroupElement
                ? listGroupElement
                : listParentElement;
        const selectors = getSelectorsFromCSSClassNames(
            this.listItemClassNames.join(','),
        );
        return Array.from(root.querySelectorAll(selectors));
    }

    private focusInHandler(event: any) {
        const currentListItem = this.findListItem(event.target);
        const listItems = this.getListItems();
        // bail if no list items or currentListItem
        if (!listItems.length || !currentListItem) return;
        this.currentIndex = listItems.indexOf(currentListItem);

        this.currentRootIndex = this.getListItems(this.isRoving)?.indexOf(
            currentListItem,
        );

        if (this.currentIndex >= 0 && this.isRoving) {
            for (let i = 0; i < listItems.length; i++) {
                setTabFocusable(listItems[i], i === this.currentIndex);
            }
        }
    }

    private keyDownHandler(event: any) {
        if (
            !NAVIGATION_KEY_NAMES.includes(event.key) ||
            this.currentIndex < 0
        ) {
            return;
        }
        const currentIndex = this.isRoving
            ? this.currentRootIndex
            : this.currentIndex;
        const listItems = this.getListItems(this.isRoving);

        let nextIndex =
            event.key === 'ArrowUp'
                ? Math.max(0, currentIndex - 1)
                : Math.min(currentIndex + 1, listItems.length - 1);

        focusVisibleItemByIndex(nextIndex, currentIndex, listItems);
    }

    /**
     * A helper method to find the closest focusable list item.
     * @param sourceElement origin of traversal
     * @returns  HTMLElement | null
     */
    private findListItem(source: HTMLElement | null): HTMLElement | null {
        if (!source || !this.listItemClassNames?.length) return null;

        const selector = this.listItemClassNames.map((c) => `.${c}`).join(',');
        const hit = source.closest(selector) as HTMLElement | null;
        if (hit) return hit;

        const parent = source.parentElement;
        if (!parent) return null;

        // BFS over siblings and their descendants
        const q: Element[] = Array.from(parent.children);
        const checked = new Set<Element>([parent]);
        for (let i = 0; i < q.length; i++) {
            const el = q[i] as HTMLElement;
            if (checked.has(el)) continue;
            checked.add(el);

            if (el.matches(selector)) return el;
            // enqueue children
            for (const child of Array.from(el.children)) {
                if (!checked.has(child)) q.push(child);
            }
        }
        return null;
    }

    /**
     * Event handler for the window to stop scrolling the page when users use the arrow keys.
     * @param event
     */
    static windowKeyUpHandler(event: any) {
        if (NAVIGATION_KEY_NAMES.includes(event.key)) {
            event.preventDefault();
        }
    }
}

function focusVisibleItemByIndex(
    index: number,
    targetIndex: number,
    listItems: Array<HTMLElement>,
) {
    const direction = index - targetIndex > 0 ? 1 : -1;
    const listItem = listItems[index];
    if (!listItem) {
        return;
    }
    // Sometimes the list item itself is visible, but the parent
    // is not--like the search button in the nav bar.
    // Check visibility for the element and its parent before assigning focus.
    if (isItemVisible(listItem) && isItemVisible(listItem.parentElement)) {
        listItems[index].focus();
    } else {
        focusVisibleItemByIndex(index + direction, targetIndex, listItems);
    }
}

function isItemVisible(element: HTMLElement | null): boolean {
    if (element === null) return false;
    const { display, visibility, opacity } = window.getComputedStyle(element);
    return display !== 'none' && visibility !== 'hidden' && opacity !== '0';
}

function getSelectorsFromCSSClassNames(classes: string): string {
    if (!classes) return '';
    return classes
        .split(',')
        .map((name) => `.${name.trim()}`)
        .join(',');
}

/**
 * sets tabindex for an element following W3C Web standards.
 * @param element HTMLElement
 * @param isTabFocusable boolean "tab-focusable" refers to whether or not an element is focusable using the Tab key.
 */
export function setTabFocusable(element: HTMLElement, isTabFocusable: boolean) {
    if (INTERACTABLE_NODE_NAMES.includes(element.nodeName)) {
        const isAnchor = element.nodeName === 'A';
        if (isTabFocusable) {
            element.removeAttribute(isAnchor ? 'tabindex' : 'disabled');
        } else {
            const attribtuesToSet: [string, string] = isAnchor
                ? ['tabindex', '-1']
                : ['disabled', 'true'];
            element.setAttribute(...attribtuesToSet);
        }
    } else {
        element.setAttribute('tabindex', isTabFocusable ? '0' : '-1');
    }
}

export function setItemInteractivity(
    shelfItemElement: HTMLElement,
    isShelfItemVisible: boolean,
) {
    if (
        INTERACTABLE_NODE_NAMES.includes(shelfItemElement.nodeName) ||
        shelfItemElement.getAttribute('tabindex')
    ) {
        // Handles the shelf item
        setTabFocusable(shelfItemElement as HTMLElement, isShelfItemVisible);
    }

    if (isShelfItemVisible) {
        shelfItemElement.removeAttribute('aria-hidden');
    } else {
        shelfItemElement.setAttribute('aria-hidden', 'true');
    }

    // handles the children in the item
    const selectors: string = INTERACTABLE_NODE_NAMES.map((nodeName) =>
        nodeName.toLowerCase(),
    ).join(',');
    const interactiveContent: Array<HTMLAnchorElement | HTMLButtonElement> =
        Array.from(shelfItemElement.querySelectorAll(selectors));
    for (let el of interactiveContent) {
        setTabFocusable(el, isShelfItemVisible);
    }
}

/**
 * set up mutation observer to ensure tab-focusablility is set appropriately based on the list item's focusability.
 * @param listItemNode
 * @param interactableTargets
 * @returns
 */
export function initListItemObserver(
    listItemNode: HTMLElement,
    interactableTargets: Array<HTMLElement>,
): MutationObserver {
    const observer = new MutationObserver((mutationsList) => {
        let tabindex: number;
        for (let mutation of mutationsList) {
            if (mutation.type === 'attributes' && interactableTargets.length) {
                for (let i = 0; i < interactableTargets.length; i++) {
                    tabindex = Number(
                        (mutation.target as HTMLElement).getAttribute(
                            'tabindex',
                        ),
                    );
                    setTabFocusable(interactableTargets[i], tabindex >= 0);
                }
            }
        }
    });
    if (listItemNode) {
        observer.observe(listItemNode, { attributes: true });
    }
    return observer;
}

export function listKeyboardAccess(
    targetElement: HTMLElement,
    options: configObject = { listItemClassNames: '' },
) {
    const listKeyboardAXInstance = new ListKeyboardAccess({
        targetElement,
        ...options,
    });
    return {
        destroy() {
            listKeyboardAXInstance.destroy();
        },
    };
}

export default listKeyboardAccess;
