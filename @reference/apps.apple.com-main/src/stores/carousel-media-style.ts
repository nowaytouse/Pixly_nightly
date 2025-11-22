import { writable } from 'svelte/store';

type Style = 'light' | 'dark' | 'white';

export const carouselMediaStyle = writable<Style>('light');
