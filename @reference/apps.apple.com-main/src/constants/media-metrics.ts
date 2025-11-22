type ValueOf<T> = T[keyof T];

export const MetricsActionType = {
    PLAY: 'play',
    STOP: 'stop',
} as const;

export type MetricsActionTypeItem = ValueOf<typeof MetricsActionType>;

export const MetricsActionDetails = {
    AUTOPLAY: 'autoplay',
    AUTOPAUSE: 'autopause',
    PLAY: 'play',
    COMPLETE: 'complete',
    PAUSE: 'pause',
} as const;

export type MetricsActionDetailItem = ValueOf<typeof MetricsActionDetails>;
