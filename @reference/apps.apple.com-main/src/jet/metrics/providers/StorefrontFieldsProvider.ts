import type {
    MetricsFieldsBuilder,
    MetricsFieldsContext,
    MetricsFieldsProvider,
} from '@jet/engine';
import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import { getLocale } from '@jet-app/app-store/common/locale';

export class StorefrontFieldsProvider implements MetricsFieldsProvider {
    constructor(private readonly objectGraph: AppStoreObjectGraph) {}

    addMetricsFields(
        builder: MetricsFieldsBuilder,
        _context: MetricsFieldsContext,
    ) {
        const { storefront } = getLocale(this.objectGraph);
        builder.addValue(storefront, 'storeFrontCountryCode');
    }
}
