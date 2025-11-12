import type { Intent } from '@jet/environment/dispatching';
import type {
    LintedMetricsEvent,
    MetricsFields,
} from '@jet/environment/types/metrics';

export const enum LintMetricsEventIntentKind {
    Name = 'LintMetricsEventIntent',
}

export interface LintMetricsEventIntent extends Intent<LintedMetricsEvent> {
    $kind: LintMetricsEventIntentKind.Name;
    fields: MetricsFields;
}

export function makeLintMetricsEventIntent(
    options: Omit<LintMetricsEventIntent, '$kind'>,
): LintMetricsEventIntent {
    return {
        ...options,
        $kind: LintMetricsEventIntentKind.Name,
    };
}
