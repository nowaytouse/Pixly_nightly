import type { MetricSettings } from '@amp/web-apps-metrics-8';

/**
 * Generates a metric settings for Metrics class.
 *
 * @param context - app context map
 * @returns MetricSettings
 */
export function makeMetricsSettings(
    context: Map<string, unknown>,
): MetricSettings {
    return {
        shouldEnableFunnelKit: function (): boolean {
            return false;
        },
        getConsumerId: async function (): Promise<string> {
            return null;
        },
    };
}
