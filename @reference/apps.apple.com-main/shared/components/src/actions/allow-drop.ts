import type { ActionReturn } from 'svelte/action';
import { get } from 'svelte/store';
import { activeDragHandler } from '@amp/web-app-components/src/actions/allow-drag';

/*
    FOLLOW-UP WORK:
    - it now adds and destroys the handler, but doesn't have a update method.
      We might want to keep track of any DropHandler that got created for an element and just update the existing instance.
      rdar://98074771 (Onyx: DnD: Update allow-drag and allow-drop actions to support updates)
*/

const DROP_AREA_DATA_ATTR = 'data-drop-area';
const DRAG_OVER_CLASS = 'is-drag-over';

export type DropOptions = {
    dropEnabled: boolean;
    onDrop: (details: DropData) => void;
    targets?:
        | [DropTarget]
        | [DropTarget.Top, DropTarget.Bottom]
        | [DropTarget.Left, DropTarget.Right];
    dropEffect?: DataTransfer['dropEffect'];
};

export type DropData = {
    data: unknown;
    dropTarget?: DropTarget;
};

export enum DropTarget {
    Top = 'top',
    Bottom = 'bottom',
    Left = 'left',
    Right = 'right',
}

const DRAG_OVER_CLASSES = {
    default: DRAG_OVER_CLASS,
    [DropTarget.Top]: `${DRAG_OVER_CLASS}-${DropTarget.Top}`,
    [DropTarget.Bottom]: `${DRAG_OVER_CLASS}-${DropTarget.Bottom}`,
    [DropTarget.Left]: `${DRAG_OVER_CLASS}-${DropTarget.Left}`,
    [DropTarget.Right]: `${DRAG_OVER_CLASS}-${DropTarget.Right}`,
};

class DropHandler {
    private readonly element: HTMLElement;
    private readonly options: DropOptions;
    private enterTarget: HTMLElement;
    private target: DropTarget;
    private lastPosition: number;

    constructor(element: HTMLElement, options: DropOptions) {
        this.element = element;
        this.options = options;

        this.addEventListeners();
    }

    private addEventListeners = (): void => {
        this.element.setAttribute(DROP_AREA_DATA_ATTR, '');
        this.element.addEventListener('dragenter', this.onDragEnter);
        this.element.addEventListener('dragover', this.onDragOver);
        this.element.addEventListener('dragleave', this.onDragLeave);
        this.element.addEventListener('drop', this.onDrop);
    };

    private removeEventListeners = (): void => {
        this.element.removeEventListener('dragenter', this.onDragEnter);
        this.element.removeEventListener('dragover', this.onDragOver);
        this.element.removeEventListener('dragleave', this.onDragLeave);
        this.element.removeEventListener('drop', this.onDrop);
    };

    public destroy = (): void => {
        this.resetState();
        this.element.removeAttribute(DROP_AREA_DATA_ATTR);
        this.removeEventListeners();
    };

    private resetState = (): void => {
        this.enterTarget = null;
        this.target = null;
        this.lastPosition = null;
        this.removeDragOverClasses();
    };

    private removeDragOverClasses = (): void => {
        Object.keys(DRAG_OVER_CLASSES).forEach((key) => {
            this.element.classList.remove(DRAG_OVER_CLASSES[key]);
        });
    };

    private setDragOverClass = (targetName: DropTarget): void => {
        const target = targetName || this.target;
        const dragOverClass =
            DRAG_OVER_CLASSES[target] || DRAG_OVER_CLASSES.default;
        // add right target class if not yet present
        if (!this.element.classList.contains(dragOverClass)) {
            this.removeDragOverClasses(); // clear all target classes before switching target
            this.element.classList.add(dragOverClass);
        }
    };

    /**
     * getLocationTarget: this function determines in what target region the user currently is
     *
     * @param e DragEvent
     * @param threshold threshold for the target location switch zone
     * @returns DropTarget
     */
    private getLocationTarget = (e: DragEvent, threshold = 0): DropTarget => {
        const { targets } = this.options;

        // Do not check on drag over region when it has no or one target
        if (!targets || targets.length === 1) {
            this.target = targets?.[0];
            return this.target;
        }

        let position, size;

        // When using top - bottom targets
        if (targets.join('-') === `${DropTarget.Top}-${DropTarget.Bottom}`) {
            // offset to drop area, instead of target (which could be a child)
            position = e.clientY - this.element.getBoundingClientRect().top;
            size = this.element.offsetHeight;
        }
        // When using left - right targets
        else if (
            targets.join('-') === `${DropTarget.Left}-${DropTarget.Right}`
        ) {
            // offset to drop area, instead of target (which could be a child)
            position = e.clientX - this.element.getBoundingClientRect().left;
            size = this.element.offsetWidth;
        }

        if (position && size) {
            if (
                !this.lastPosition ||
                Math.abs(position - this.lastPosition) > threshold
            ) {
                this.lastPosition = position;
                this.target = position <= size / 2 ? targets[0] : targets[1];
            }
        }

        return this.target;
    };

    private isCompatibleDropEffect(e: DragEvent) {
        // Workaround for https://bugs.webkit.org/show_bug.cgi?id=178058
        // There is a longstanding WebKit bug where any value set by the user
        // on `dataTransfer.effectAllowed` in the dragstart event is ignored
        // and always returns "all". This means that we cannot trust the value
        // that is set in the DragEvent. As a workaround, we store and check
        // the active drag handler for the effectAllowed specified in the options.
        //
        // const { dropEffect, effectAllowed } = e.dataTransfer;
        const { dropEffect } = e.dataTransfer;
        const effectAllowed = get(activeDragHandler)?.getEffectAllowed();

        return (
            effectAllowed === 'all' ||
            effectAllowed.toLowerCase().includes(dropEffect)
        );
    }

    private onDragEnter = (e: DragEvent): void => {
        e.dataTransfer.dropEffect = this.options.dropEffect || 'copy';

        if (!this.isCompatibleDropEffect(e)) {
            return;
        }

        e.stopPropagation();

        // Set enterTarget to cover entering child elements
        this.enterTarget = e.target as HTMLElement;
        this.setDragOverClass(this.getLocationTarget(e));
    };

    private onDragOver = (e: DragEvent): void => {
        e.dataTransfer.dropEffect = this.options.dropEffect || 'copy';

        if (!this.isCompatibleDropEffect(e)) {
            return;
        }

        e.preventDefault(); // prevent the browser from default handling of the data to allow drop
        e.stopPropagation(); // prevent setting classes on parent drop areas
        this.setDragOverClass(this.getLocationTarget(e, 10));
    };

    private onDragLeave = (e: Event): void => {
        // Only set drag-over to false when it leaves the drop area. Not on entering childs
        if (e.target === this.enterTarget) {
            this.resetState();
        }
    };

    private onDrop = (e: DragEvent): void => {
        e.preventDefault();
        e.stopPropagation(); // Prevent drop action on parent elements

        const data = JSON.parse(e.dataTransfer.getData('text/plain'));
        const draggedData: DropData = { data };

        if (this.target) {
            draggedData.dropTarget = this.target;
        }

        this.resetState();
        this.options.onDrop(draggedData);
    };
}

/**
 * Allow Drop action
 *
 * Usage:
 * <div use:allow-drop={{ dropEnabled: true, onDrop: dropAction }}></div>
 */
export function allowDrop(
    target: HTMLElement,
    options: DropOptions,
): ActionReturn<DropOptions> {
    let dropHandler;

    if (options?.dropEnabled && options?.onDrop) {
        dropHandler = new DropHandler(target, options);
    }

    return {
        destroy: () => {
            dropHandler?.destroy();
        },
        update: (updatedOptions: DropOptions) => {
            // Hotfix for updated properties. Remove handlers with data and add new ones.
            // TODO: rdar://98074771 (Onyx: DnD: Update allow-drag and allow-drop actions to support updates)
            dropHandler?.destroy();

            if (updatedOptions?.dropEnabled && updatedOptions?.onDrop) {
                dropHandler = new DropHandler(target, updatedOptions);
            }
        },
    };
}

export default allowDrop;
