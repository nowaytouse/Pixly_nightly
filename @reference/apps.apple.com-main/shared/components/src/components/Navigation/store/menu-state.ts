import { writable } from 'svelte/store';

export const menuIsExpanded = writable(false);
export const menuIsTransitioning = writable(false);
