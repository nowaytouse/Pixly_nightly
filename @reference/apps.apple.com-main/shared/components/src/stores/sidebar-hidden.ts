import { derived } from 'svelte/store';
import { buildMediaQueryStore } from '@amp/web-app-components/src/stores/media-query';

export const sidebarHiddenQuery = buildMediaQueryStore('visible', {
    hidden: '(max-width: 483px)',
    visible: '(min-width: 484px)',
});

export const sidebarIsHidden = derived(
    sidebarHiddenQuery,
    ($sidebarHiddenQuery) => $sidebarHiddenQuery === 'hidden',
);
