import type { RatingCountsList } from './types';

// eslint-disable-next-line import/prefer-default-export
export const calculatePercentages = (
    ratingValues: RatingCountsList,
    totalCount: number,
): RatingCountsList =>
    ratingValues?.map((value: number) =>
        Math.round((value / totalCount) * 100),
    ) || [];
