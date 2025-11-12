/* eslint-disable object-curly-newline */
import type { Size } from '@amp/web-app-components/src/types';
import type { GridType } from '@amp/web-app-components/src/components/Shelf/types';

/**
 * Used to customize the shared shelf
 *
 * @param GRID_MAX_CONTENT - Sets the max content size of the column for each viewport
 * @param GRID_ROW_GAP -  Sets the row gap for a shelf in each viewport
 * @param GRID_COL_GAP - Sets the column gap for a shelf in each viewport
 * @param GRID_VALUES - Sets the number of items to show in a column of the grid for each viewport
 *
 * @example
 * const ShelvesConfig = {
 *   GRID_MAX_CONTENT: {
 *     FooShelf: { xsmall: '298px' },
 *   },
 *   GRID_COL_GAP: {
 *     FooShelf:  { xsmall: '10px', small:'20px', medium:'20px', large:'20px', xlarge: '30px' }
 *   },
 *   GRID_ROW_GAP: {
 *     FooShelf:  { xsmall: '10px', small:'20px', medium:'20px', large:'20px', xlarge: '30px' }
 *   },
 *   GRID_VALUES: {
 *     FooShelf:  { xsmall: 1, small: 3, medium: 5, large: 6, xlarge: 10 }
 *   }
 * }
 */
export interface ShelfConfigOptions {
    /**
     * Sets the max size of the column for each viewport
     * (NOTE: these values will override GRID_VALUES)
     */
    GRID_MAX_CONTENT: {
        [key in GridType]: { [value in Size]?: string };
    };
    /**
     * Sets the row gap for a shelf in each viewport
     * - Default for all shelves is { xsmall: '24px', small: '24px', medium: '24px', large: '24px', xlarge: '24px' }
     */
    GRID_ROW_GAP: {
        [key in GridType]?: { [value in Size]?: number | null };
    };
    /**
     * Sets the column gap for a shelf in each viewport
     * - Default for all shelves is { xsmall: '10px', small: '20px', medium: '20px', large: '20px', xlarge: '20px' }
     */
    GRID_COL_GAP: {
        [key in GridType]?: { [value in Size]?: string | null };
    };
    /**
     * Sets the number of columns in the grid for each viewport
     * (NOTE: this value will be overridden by values in GRID_MAX_CONTENT)
     */
    GRID_VALUES: {
        [key in GridType]: { [value in Size]: number | null };
    };
}

// Grid values correspond with dynamic-grids.scss
function ShelfConfigInit() {
    let config: ShelfConfigOptions = {
        GRID_MAX_CONTENT: {
            A: { xsmall: '298px' },
            B: { xsmall: '298px' },
            C: { xsmall: '200px' },
            D: { xsmall: '144px' },
            E: { xsmall: '144px' },
            F: { xsmall: '270px' },
            G: { xsmall: '144px' },
            H: { xsmall: '94px' },
            I: { xsmall: '144px' },
            EllipseA: {},
            Spotlight: {},
            Single: {},
            '1-1-2-3': {},
            '2-2-3-4': { xsmall: '270px' },
            '1-2-2-2': {},
        },
        GRID_COL_GAP: {},
        GRID_ROW_GAP: {
            None: { xsmall: 0, small: 0, medium: 0, large: 0, xlarge: 0 },
            '1-2-2-2': { xsmall: 0, small: 0, medium: 0, large: 0, xlarge: 0 },
        },
        GRID_VALUES: {
            A: { xsmall: null, small: 2, medium: 2, large: 3, xlarge: 3 },
            B: { xsmall: null, small: 2, medium: 3, large: 4, xlarge: 4 },
            C: { xsmall: null, small: 3, medium: 4, large: 5, xlarge: 5 },
            D: { xsmall: null, small: 4, medium: 5, large: 8, xlarge: 8 },
            E: { xsmall: null, small: 5, medium: 9, large: 10, xlarge: 10 },
            F: { xsmall: null, small: 2, medium: 3, large: 3, xlarge: 3 },
            G: { xsmall: null, small: 4, medium: 5, large: 6, xlarge: 6 },
            H: { xsmall: null, small: 6, medium: 8, large: 10, xlarge: 10 },
            I: { xsmall: null, small: 5, medium: 6, large: 8, xlarge: 8 },
            Single: { xsmall: 1, small: 1, medium: 1, large: 1, xlarge: 1 },
            EllipseA: { xsmall: 2, small: 4, medium: 6, large: 6, xlarge: 6 },
            Spotlight: { xsmall: 1, small: 1, medium: 1, large: 1, xlarge: 1 },
            '1-1-2-3': { xsmall: 1, small: 1, medium: 2, large: 3, xlarge: 3 },
            '2-2-3-4': { xsmall: 2, small: 2, medium: 3, large: 4, xlarge: 4 },
            '1-2-2-2': { xsmall: 1, small: 2, medium: 2, large: 2, xlarge: 2 },
        },
    };

    const get = () => config;

    const set = (obj: ShelfConfigOptions) => {
        config = { ...config, ...obj };
    };

    return {
        set,
        get,
    };
}

export const ShelfConfig = ShelfConfigInit();
