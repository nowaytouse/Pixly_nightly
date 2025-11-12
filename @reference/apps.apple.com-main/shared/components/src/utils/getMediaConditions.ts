import type { Breakpoints, Size } from '@amp/web-app-components/src/types';

export type MediaConditions<T extends string | number | symbol = Size> = {
    [key in T]?: string;
};

type BasicBreapoints<T extends string | number | symbol> = Record<T, number>;

type BreakpointOptions = { offset?: number };

// eslint-disable-next-line import/prefer-default-export
export function getMediaConditions<T extends string | number | symbol = Size>(
    breakpoints: Breakpoints<T>,
    options?: BreakpointOptions,
): MediaConditions<T> {
    const viewportOrder = {
        xsmall: 0,
        small: 1,
        medium: 2,
        large: 3,
        xlarge: 4,
    };

    const offset = options?.offset ?? 0;
    const viewportSizes = Object.keys(breakpoints).sort(
        (a, b) => viewportOrder[a] - viewportOrder[b],
    ) as T[];

    return viewportSizeToMediaConditions<T>(breakpoints, viewportSizes, offset);
}

function viewportSizeToMediaConditions<T extends string | number | symbol>(
    breakpoints: Breakpoints<T>,
    viewportSizes?: T[],
    offset?: number,
): MediaConditions<T> {
    viewportSizes ||= Object.keys(breakpoints) as T[];
    const queries: MediaConditions<T> = {};
    viewportSizes.reduce((acc, viewport) => {
        const { min, max } = {
            min: undefined,
            max: undefined,
            ...breakpoints[viewport],
        };

        if (min && !max) {
            acc[viewport] = `(min-width:${min + offset}px)`;
        } else if (!min && max) {
            acc[viewport] = `(max-width:${max + offset}px)`;
        } else if (min && max) {
            acc[viewport] = `(min-width:${min + offset}px) and (max-width:${
                max + offset
            }px)`;
        }
        return acc;
    }, queries);
    return queries;
}

/**
 * Transforms a breakpoints object into media queries that match ranges between each breakpoint and the next.
 *
 * @param breakpoints - Object with breakpoint names as keys and pixel values as values
 * @returns Object with breakpoint names as keys and media query strings as values
 *
 * @example
 * const breakpoints = { XSM: 0, SM: 350, MD: 484, LG: 1000 };
 * const mediaQueries = breakpointsToMediaQueries(breakpoints);
 * // Returns:
 * // {
 * //   XSM: '(max-width: 349px)',
 * //   SM: '(min-width: 350px) and (max-width: 483px)',
 * //   MD: '(min-width: 484px) and (max-width: 999px)',
 * //   LG: '(min-width: 1000px)'
 * // }
 */
export function breakpointsToMediaQueries<T extends string>(
    breakpoints: BasicBreapoints<T>,
): MediaConditions<T> {
    const entries = Object.entries(breakpoints) as [T, number][];
    entries.sort(([, a], [_, b]) => a - b);
    const transformedBreakpoints: Breakpoints<T> = {};

    entries.forEach(([breakpointName, minWidth], index) => {
        const isFirst = index === 0;
        const isLast = index === entries.length - 1;
        const nextBreakpointWidth = isLast ? null : entries[index + 1][1];

        if (isFirst && minWidth === 0) {
            // First breakpoint starting at 0: only max-width
            if (nextBreakpointWidth !== null) {
                transformedBreakpoints[breakpointName] = {
                    max: nextBreakpointWidth - 1,
                };
            } else {
                // Edge case: only one breakpoint starting at 0
                transformedBreakpoints[breakpointName] = { min: 0 };
            }
        } else if (isLast) {
            // Last breakpoint: only min-width
            transformedBreakpoints[breakpointName] = { min: minWidth };
        } else {
            // Middle breakpoints: min-width and max-width range
            transformedBreakpoints[breakpointName] = {
                min: minWidth,
                max: nextBreakpointWidth! - 1,
            };
        }
    });

    const viewportSizes = entries.map(([breakpointName]) => breakpointName);
    return viewportSizeToMediaConditions<T>(
        transformedBreakpoints,
        viewportSizes,
        0,
    );
}
