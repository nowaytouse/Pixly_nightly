import type { ComponentType } from 'svelte';
import type {
    BaseNavigationItem,
    NavigationId,
} from '@amp/web-app-components/src/types';
import Item from './Item.svelte';

export function isSameTab(
    a: NavigationId | null,
    b: NavigationId | null,
): boolean {
    if (a === null || b === null) {
        return false;
    }

    // Need deep object equality for things like
    // { kind: 'playlist', id: '123' }
    try {
        return JSON.stringify(a) === JSON.stringify(b);
    } catch {
        return false;
    }
}

export function getItemComponent(item: BaseNavigationItem): ComponentType {
    return item.component ?? Item;
}
