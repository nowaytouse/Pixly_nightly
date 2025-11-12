import type { ActionReturn } from 'svelte/action';
import type { Readable } from 'svelte/store';
import { writable } from 'svelte/store';

// Duplicate assignment from '~/components/DragImage.svelte'
const PRESET_CLASS = 'preset';
const VISIBLE_CLASS = 'visible';
const CONTAINER_CLASS = 'drag-image--container';
const IMAGE_ATTR = 'data-drag-image-source';
const BADGE_ATTR = 'data-drag-image-badge';

// resize fallback image when artwork is video or landscape
const ASPECT_RATIO_CLASS = 'aspect-landscape';
const IS_DRAGGING_CLASS = 'is-dragging';

// Workaround for WebKit `effectAllowed` bug: https://bugs.webkit.org/show_bug.cgi?id=178058
// This store points to the active drag handler, set on dragstart and unset on dragend.
// Only store subscription is exported to prevent modification outside this file.
const { set: setActiveDragHandler, subscribe } =
    writable<DragHandler<any>>(null);
export const activeDragHandler: Readable<DragHandler<any>> = { subscribe };

/*
    FOLLOW-UP WORK:
    - it now adds and destroys the handler, and destroys and creates a new one on update.
      We might want to keep track of any DragHandler that got created for an element and just update the existing instance.
      rdar://98074771 (Onyx: DnD: Update allow-drag and allow-drop actions to support updates)
    - Have the options dragEnabled be optional. If not passed in, it should be enabled. Just not when it's set to false.
      We can't update that before the above changes are in.
    - Use the logger instead of console.warn directly.
    - Update DragImage clases and badge count from the DragImage component if possible
*/

/**
 * Note: dragData needs to be JSON serializable, and no recursive structure
 */
export type DragOptions = {
    dragEnabled: boolean;
    dragData: unknown; // Needs to be JSON serializable. The DragData type is being set on initiating a new DragHandler<DragData> based on the passed in dragData
    dragImage?: HTMLElement | string;
    usePlainDragImage?: boolean;
    isContainer?: boolean;
    badgeCount?: number;
    effectAllowed?: DataTransfer['effectAllowed'];
};

class DragHandler<DragData> {
    private readonly element: HTMLElement;
    private readonly options: DragOptions;
    private readonly dragData: DragData;
    private readonly dragImageContainer: HTMLElement;
    private readonly fallbackImage: HTMLElement;
    private dragImage: HTMLElement;

    constructor(
        element: HTMLElement,
        options: Omit<DragOptions, 'dragData'> & { dragData: DragData },
    ) {
        this.element = element;
        this.options = options;
        this.dragData = options.dragData;
        this.dragImageContainer = document.querySelector('[data-drag-image]');
        this.fallbackImage = document.querySelector('[data-fallback-image]');

        if (!this.dragImageContainer) {
            console.warn(
                'Use the <DragImage /> component to allow app specific drag images with fallback, badge and styling',
            );
        }

        this.addEventListeners();
        this.setDraggable();
    }

    private setDraggable(): void {
        this.element.draggable = true;
    }

    private setDraggingClass = () => {
        this.element.classList.add(IS_DRAGGING_CLASS);
    };

    private removeDraggingClass = () => {
        this.element.classList.remove(IS_DRAGGING_CLASS);
    };

    private addEventListeners(): void {
        // Create custom drag image before dragStart, because otherwise it might be empty
        this.element.addEventListener('mousedown', this.createDragImage);
        this.element.addEventListener('mouseup', this.resetDragImage);

        this.element.addEventListener('dragstart', this.onDragStart, {
            capture: true,
        });
        this.element.addEventListener('dragend', this.onDragEnd);
    }

    public destroy(): void {
        this.element.draggable = false;
        this.element.style.setProperty('webkitUserDrag', 'auto');
        this.element.removeEventListener('mousedown', this.createDragImage);
        this.element.removeEventListener('mouseup', this.resetDragImage);
        this.element.removeEventListener('dragstart', this.onDragStart, {
            capture: true,
        });
        this.element.removeEventListener('dragend', this.onDragEnd);
    }

    private onDragStart = (e: DragEvent): void => {
        if (!this.dragData) {
            // Interrupt the drag event as dragging should not be enabled on the element
            e.preventDefault();
            return;
        }

        // Prevent drag action on parent elements
        e.stopPropagation();

        if (this.dragImage) {
            if (this.dragImage === this.dragImageContainer) {
                // Make temporary visible to capture snapshot
                this.dragImageContainer.classList.remove(PRESET_CLASS);
                this.dragImageContainer.classList.add(VISIBLE_CLASS);
            }

            const { clientWidth: imgWidth, clientHeight: imgHeight } =
                this.dragImage;
            e.dataTransfer.setDragImage(
                this.dragImage,
                imgWidth / 2,
                imgHeight / 2,
            );

            // Remove the DOM drag image to not show up for the user.
            // It needs a timeout to have it captured before it gets removed.
            setTimeout(() => this.resetDragImage(), 1);
        }

        e.dataTransfer.setData('text/plain', JSON.stringify(this.dragData));

        // "Drop effect" controls what mouse cursor is shown during DnD operations
        // See: https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed
        e.dataTransfer.effectAllowed = this.getEffectAllowed();
        this.setDraggingClass();

        setActiveDragHandler(this);
    };

    private onDragEnd = (): void => {
        setActiveDragHandler(null);
        this.resetDragImage();
        this.removeDraggingClass();
    };

    private createDragImage = (): HTMLElement | null => {
        this.resetDragImage();

        const argsDragImage = this.options.dragImage;
        let dragImage: HTMLElement;

        if (argsDragImage instanceof HTMLElement) {
            dragImage = argsDragImage;
        } else if (typeof argsDragImage === 'string') {
            // Find the drag image based on the passed selector
            dragImage = this.element.querySelector(argsDragImage);
        } else {
            // Use artwork by default
            dragImage = this.element.querySelector(
                '.artwork-component picture',
            );
        }

        // Do not create a shallow copy inside our drag container with pre-set sizes.
        // Can be used to either use the default browser behavior of using the element as drag image,
        // or use another DOM element inside the draggable object without additional styling.
        if (this.options.usePlainDragImage) {
            // If no drag image set, use element (default browser drag behavior)
            if (!argsDragImage) {
                dragImage = this.element;
            }
            this.dragImage = dragImage;
            return dragImage;
        }

        // When no drag image container found (<DragImage /> component not rendered in the app), don't use a custom drag image
        if (!this.dragImageContainer) return;

        // Container items should have a bigger drag image (albums, playlists)
        if (this.options.isContainer) {
            this.dragImageContainer.classList.add(CONTAINER_CLASS);
        }

        // Clone image and add to drag image container
        if (dragImage) {
            const dragImageClone = dragImage.cloneNode(true);
            this.dragImageContainer
                .querySelector(`[${IMAGE_ATTR}]`)
                .prepend(dragImageClone);

            // Prevents fallback image from overflowing video or landscaped artwork.
            // In the Tracklist. See: .aspect-landscape class via DragImage.svelte
            if (dragImage.offsetWidth / dragImage.offsetHeight !== 1) {
                this.fallbackImage.classList.add(ASPECT_RATIO_CLASS);
            }
        }

        // Add a track count badge. Container items should always have track count, even if it's 1 (like a single-track-album).
        if (
            this.badgeCount > 1 ||
            (this.options.isContainer && this.options.badgeCount > 0)
        ) {
            const badge = this.dragImageContainer.querySelector(
                `[${BADGE_ATTR}]`,
            );
            badge.classList.add(VISIBLE_CLASS);
            badge.textContent = `${this.badgeCount}`;
        }

        // Make visible for loading the image and capturing for drag image
        this.dragImageContainer.classList.add(PRESET_CLASS);
        this.dragImage = this.dragImageContainer;
    };

    /**
     * DragImage is being set from the DragImage component: '@amp/web-app-components/src/components/DragImage.svelte'.
     * We should find a better way of updating that rendered component instead of modifying the elements from here.
     */
    private resetDragImage = (): void => {
        this.dragImage = null;
        const container = this.dragImageContainer;
        container.classList.remove(PRESET_CLASS);
        container.classList.remove(VISIBLE_CLASS);
        container.classList.remove(CONTAINER_CLASS);
        this.fallbackImage.classList.remove(ASPECT_RATIO_CLASS);
        container.querySelector(`[${IMAGE_ATTR}]`).innerHTML = '';
        const badge = container.querySelector(`[${BADGE_ATTR}]`);
        badge.classList.remove(VISIBLE_CLASS);
        badge.innerHTML = '';
    };

    private get badgeCount(): number {
        return (
            this.options.badgeCount ??
            (Array.isArray(this.dragData) && this.dragData.length)
        );
    }

    public getEffectAllowed(): DataTransfer['effectAllowed'] {
        return this.options?.effectAllowed || 'copy';
    }
}

/**
 * Allow Drag action
 *
 * Usage:
 * <div use:allow-drag={{
 *     dragEnabled: true,
 *     dragData: yourDragData,
 *     isContainer: true,
 *     badgeCount: 4
 * }}></div>
 */
export function allowDrag(
    target: HTMLElement,
    options: DragOptions | false,
): ActionReturn<DragOptions> {
    const enabled = options !== false && (options.dragEnabled ?? true);
    let dragHandler;

    if (enabled && options.dragData) {
        dragHandler = new DragHandler(target, options);
    }

    return {
        destroy: () => {
            dragHandler?.destroy();
        },
        update: (updatedOptions: DragOptions) => {
            // Hotfix for updated properties. Remove handlers with data and add new ones.
            // TODO: rdar://98074771 (Onyx: DnD: Update allow-drag and allow-drop actions to support updates)
            dragHandler?.destroy();

            if (updatedOptions?.dragEnabled && updatedOptions?.dragData) {
                dragHandler = new DragHandler(target, updatedOptions);
            }
        },
    };
}

export default allowDrag;
