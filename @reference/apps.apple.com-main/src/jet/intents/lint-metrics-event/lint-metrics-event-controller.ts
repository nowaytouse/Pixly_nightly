import type { IntentController } from '@jet/environment/dispatching/base/intent-controller';
import type { LintedMetricsEvent } from '@jet/environment/types/metrics';

import {
    type LintMetricsEventIntent,
    LintMetricsEventIntentKind,
} from './lint-metrics-event-intent';

export const LintMetricsEventIntentController: IntentController<LintMetricsEventIntent> =
    {
        $intentKind: LintMetricsEventIntentKind.Name,

        async perform(
            intent: LintMetricsEventIntent,
        ): Promise<LintedMetricsEvent> {
            return { fields: intent.fields };
        },
    };
