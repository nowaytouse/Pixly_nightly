import type { MetricsProvider } from '@amp/web-apps-metrics-8';
import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';

import { StorefrontFieldsProvider } from './StorefrontFieldsProvider';

export function makeMetricsProviders(
    objectGraph: AppStoreObjectGraph,
): MetricsProvider[] {
    return [
        {
            provider: new StorefrontFieldsProvider(objectGraph),
            request: 'storeFrontCountryCode',
        },
    ];
}
