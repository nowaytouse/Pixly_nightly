import { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import { ObjectGraphType } from '@jet-app/app-store/gameservicesui/src/foundation/object-graph-types';

import type { Dependencies } from './make-dependencies';
import { WebFeatureFlags } from './feature-flags';
import { WebMediaTokenService } from './media-token-service';

export { ObjectGraphType };

class AppStoreWebObjectGraph extends AppStoreObjectGraph {
    /**
     * Configures the ObjectGraph from our `Dependencies` definition
     *
     * @param dependencies
     * @returns
     */
    configureWithDependencies(dependencies: Dependencies) {
        const {
            bag,
            client,
            console,
            host,
            locale,
            localization,
            metricsIdentifiers,
            net,
            properties,
            random,
            seo,
            storage,
            user,
        } = dependencies;

        return this.addingClient(client)
            .addingNetwork(net)
            .addingHost(host)
            .addingBag(bag)
            .addingLoc(localization)
            .addingMediaToken(new WebMediaTokenService())
            .addingConsole(console)
            .addingAppleSilicon(undefined)
            .addingProperties(properties)
            .addingLocale(locale)
            .addingUser(user)
            .addingFeatureFlags(new WebFeatureFlags())
            .addingMetricsIdentifiers(metricsIdentifiers)
            .addingSEO(seo)
            .addingStorage(storage)
            .addingRandom(random);
    }
}

export function makeObjectGraph(
    dependencies: Dependencies,
): AppStoreObjectGraph {
    const objectGraph = new AppStoreWebObjectGraph('app-store');

    return objectGraph.configureWithDependencies(dependencies);
}
