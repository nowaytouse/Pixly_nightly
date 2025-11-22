import type {
    MetricsFieldsBuilder,
    MetricsFieldsContext,
    MetricsFieldsProvider,
} from '@jet/engine';
import { IMPRESSION_CONTEXT_NAME } from './impressions/constants';
import type { Impressions } from './impressions';

export class ImpressionFieldProvider implements MetricsFieldsProvider {
    constructor(private readonly appContext: Map<string, unknown>) {
        this.appContext = appContext;
    }

    addMetricsFields(
        builder: MetricsFieldsBuilder,
        _metricsContext: MetricsFieldsContext,
    ) {
        const impressionInstance = this.appContext.get(
            IMPRESSION_CONTEXT_NAME,
        ) as Impressions;

        if (impressionInstance?.settings?.captureType === 'jet') {
            let impressions = impressionInstance.consumeImpressions();
            builder.addValue(impressions, 'impressions');
        }
    }
}
