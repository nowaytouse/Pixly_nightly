/**
 * A list of event types we use across all onyx apps for metrics.
 */
export const METRICS_EVENT_TYPES = [
    // The following types come from the jet enum `MetricsEventType`
    // https://github.pie.apple.com/app-store/jet-js/blob/505144151e875c1bcbacd898216127fbc14c1562/packages/environment/src/types/metrics.ts#L198-L205
    // and the events could be handled by MetricsKit
    // https://github.pie.apple.com/amp-ae/mt-metricskit/tree/dev/packages/processors/mt-metricskit-processor-clickstream/src/metrics/event_handlers
    'account', // For GDPR
    'click',
    'dialog',
    'enter',
    'exit',
    'impressions',
    'media',
    'page',
    'pageRender',
    'search',
] as const;
