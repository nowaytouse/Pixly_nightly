// eslint-disable-next-line import/no-extraneous-dependencies
import type { ShelfConfigOptions } from '@amp/web-app-components/config/components/shelf';
import { ShelfConfig } from '@amp/web-app-components/config/components/shelf';
import {
    GRID_COLUMN_GAP_DEFAULT,
    GRID_COLUMN_GAP_DEFAULT_XSMALL,
    GRID_ROW_GAP_DEFAULT,
    // eslint-disable-next-line import/no-extraneous-dependencies
} from '@amp/web-app-components/src/components/Shelf/constants';
import type { GridType } from '@amp/web-app-components/src/components/Shelf/types';
import type { Sizes, Size } from '@amp/web-app-components/src/types';

const generateGridColSizeVars = (
    viewport: Size,
    gridValues: ShelfConfigOptions['GRID_VALUES'][string],
    maxContents: ShelfConfigOptions['GRID_MAX_CONTENT'][string],
): string[] => {
    const value = gridValues[viewport];
    const maxContent = maxContents[viewport];
    const gridVars = [];

    if (maxContent) {
        // create CSS variable for px values in grid
        gridVars.push(`--grid-max-content-${viewport}: ${maxContent};`);
    } else if (value) {
        // create CSS variable for grid unit
        gridVars.push(`--grid-${viewport}: ${value};`);
    }

    return gridVars;
};

const generateGridGapSizeVars = (
    viewport: Size,
    gridColumnGap: Partial<ShelfConfigOptions['GRID_COL_GAP'][string]>,
    gridRowGap: Partial<ShelfConfigOptions['GRID_ROW_GAP'][string]>,
): string[] => {
    const gridVars = [];
    const defaultColGap =
        viewport === 'xsmall'
            ? GRID_COLUMN_GAP_DEFAULT_XSMALL
            : GRID_COLUMN_GAP_DEFAULT;

    // check if gap override for certain viewport
    gridVars.push(
        `--grid-column-gap-${viewport}: ${
            gridColumnGap[viewport] ?? defaultColGap
        }px;`,
    );
    gridVars.push(
        `--grid-row-gap-${viewport}: ${
            gridRowGap[viewport] ?? GRID_ROW_GAP_DEFAULT
        }px;`,
    );

    return gridVars;
};

/**
 * converts the JS configs to CSS variables.
 *
 * variables created:
 * --grid-{viewport} - grid value to use for columns widths
 * --grid-max-content-{viewport} - px value to use for column width
 * --grid-column-gap-{viewport} - grid gap size // default is 20px
 * */

// eslint-disable-next-line import/prefer-default-export
export const getGridVars = (type: GridType): string => {
    const { GRID_VALUES, GRID_MAX_CONTENT, GRID_COL_GAP, GRID_ROW_GAP } =
        ShelfConfig.get();

    const gridValues = GRID_VALUES[type];
    const maxContent = GRID_MAX_CONTENT[type];
    const gridRowGap = GRID_ROW_GAP[type] || {};
    const gridColumnGap = GRID_COL_GAP[type] || {};
    const gridKeys = Object.keys(gridValues) as unknown as Sizes;

    let gridVars: string[] = [];

    gridKeys.forEach((viewport) => {
        // generate variables for each viewport
        const gridColumnSizeVars = generateGridColSizeVars(
            viewport,
            gridValues,
            maxContent,
        );
        const gridGapSizeVars = generateGridGapSizeVars(
            viewport,
            gridColumnGap,
            gridRowGap,
        );

        gridVars = [...gridVars, ...gridColumnSizeVars, ...gridGapSizeVars];
    });

    return gridVars.join(' ');
};
