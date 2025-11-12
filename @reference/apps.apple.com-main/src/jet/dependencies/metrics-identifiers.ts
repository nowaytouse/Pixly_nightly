export class WebMetricsIdentifiers implements MetricsIdentifiers {
    async getIdentifierForContext(
        _metricsIdentifierKeyContext: MetricsIdentifierKeyContext,
    ): Promise<string | undefined> {
        return undefined;
    }

    async getMetricsFieldsForContexts(
        _metricsIdentifierKeyContexts: MetricsIdentifierKeyContext[],
    ): Promise<JSONData | undefined> {
        return undefined;
    }
}
