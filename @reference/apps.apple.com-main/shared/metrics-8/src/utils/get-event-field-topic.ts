import type { LintedMetricsEvent } from '@jet/environment';
import type { MetricsFields } from '~/types';

export function getEventFieldsWithTopic(
    event: LintedMetricsEvent,
    topic: string,
) {
    return 'topic' in event.fields
        ? event.fields
        : ({ ...event.fields, topic } as MetricsFields);
}
