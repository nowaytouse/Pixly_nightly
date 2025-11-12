// eslint-disable-next-line import/no-extraneous-dependencies
import { ShelfConfig } from '@amp/web-app-components/config/components/shelf';
import type { GridType } from '@amp/web-app-components/src/components/Shelf/types';

/**
 * Find the max amount of rendered items for a grid type.
 */
// eslint-disable-next-line import/prefer-default-export
export const getMaxVisibleItems = (type: GridType): number => {
    const { GRID_VALUES } = ShelfConfig.get();

    const gridValues = GRID_VALUES[type];

    const arrayOfgridValues = [...Object.values(gridValues)].filter(
        (item) => typeof item === 'number',
    );

    return Math.max(...arrayOfgridValues);
};
