import { makeRouterUsingRegisteredControllers } from '@jet/environment/routing';

import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import { AppStoreIntentDispatcher } from '@jet-app/app-store/foundation/runtime/app-store-intent-dispatcher';
import { AppStoreRuntime } from '@jet-app/app-store/foundation/runtime/runtime';

import {
    type Dependencies,
    ObjectGraphType,
    makeObjectGraph,
} from '~/jet/dependencies';

import { AppEventPageIntentController } from '@jet-app/app-store/controllers/app-events/app-event-page-intent-controller';
import { BundlePageIntentController } from '@jet-app/app-store/controllers/product-page/bundle-page-intent-controller';
import { EditorialPageIntentController } from '@jet-app/app-store/controllers/editorial-pages/editorial-page-intent-controller';
import { EditorialShelfCollectionPageIntentController } from '@jet-app/app-store/controllers/editorial-pages/editorial-shelf-collection-page-intent-controller';
import { GroupingPageIntentController } from '@jet-app/app-store/controllers/grouping/grouping-page-intent-controller';
import { ProductPageIntentController } from '@jet-app/app-store/controllers/product-page/product-page-intent-controller';
import { SearchLandingPageIntentController } from '@jet-app/app-store/controllers/search/search-landing-page-intent-controller';
import { SearchResultsPageIntentController } from '@jet-app/app-store/controllers/search/search-results-controller';
import { RoutableArticlePageIntentController } from '@jet-app/app-store/controllers/today/routable-article-page-intent-controller';
import { ArcadeGroupingPageIntentController } from '@jet-app/app-store/controllers/arcade/arcade-grouping-page-intent-controller';
import { DeveloperPageIntentController } from '@jet-app/app-store/controllers/developer/developer-page-intent-controller';
import { ChartsPageIntentController } from '@jet-app/app-store/controllers/top-charts/charts-page-intent-controller';
import { ChartsHubPageIntentController } from '@jet-app/app-store/controllers/top-charts/charts-hub-page-intent-controller';
import { SeeAllPageIntentController } from '@jet-app/app-store/controllers/product-page/see-all-intent-controller';
import { RoutableTodayPageIntentController } from '@jet-app/app-store/controllers/today/routable-today-page-intent-controller';
import { RoomPageIntentController } from '@jet-app/app-store/controllers/room/room-page-intent-controller';
import { RoutableArcadeSeeAllPageController } from '@jet-app/app-store/controllers/arcade/routable-arcade-see-all-page-controller';
import * as landingPageNavigationControllers from '@jet-app/app-store/common/web-navigation/platform-landing-page-intent-controllers';
import { RootRedirectController } from '@jet-app/app-store/common/web-navigation/platform-landing-page-intent-controllers';
import { EulaPageIntentController } from '@jet-app/app-store/controllers/product-page/eula-page-intent-controller';
import { CategoryTabsIntentController } from '@jet-app/app-store/controllers/web-navigation/category-tabs-intent-controller';

import { ErrorPageIntentController } from '~/jet/intents/error-page-intent-controller';
import { ChartsPageRedirectIntentController } from '~/jet/intents/charts-page-redirect-intent-controller';

import {
    RouteUrlIntentController,
    LintMetricsEventIntentController,
} from '~/jet/intents';
import * as staticMessagePageControllers from '~/jet/intents/static-message-pages';

function makeIntentDispatcher(): AppStoreIntentDispatcher {
    const intentDispatcher = new AppStoreIntentDispatcher();

    intentDispatcher.register(RouteUrlIntentController);
    intentDispatcher.register(LintMetricsEventIntentController);

    // Route Providers
    for (const Controller of Object.values(landingPageNavigationControllers)) {
        // `RootRedirectController` needs to be registered last, due to it's path match of `/{sf}`,
        // it could inadvertently match a landing page route like `/vision`, so we are skipping it here
        // and registering it at the bottom of this function.
        if (Controller !== RootRedirectController) {
            intentDispatcher.register(Controller);
        }
    }

    for (const StaticMessagePageController of Object.values(
        staticMessagePageControllers,
    )) {
        intentDispatcher.register(StaticMessagePageController);
    }

    intentDispatcher.register(ArcadeGroupingPageIntentController);
    intentDispatcher.register(BundlePageIntentController);
    intentDispatcher.register(EditorialPageIntentController);
    intentDispatcher.register(EditorialShelfCollectionPageIntentController);
    intentDispatcher.register(GroupingPageIntentController);
    intentDispatcher.register(new SearchResultsPageIntentController());
    intentDispatcher.register(SearchLandingPageIntentController);
    intentDispatcher.register(DeveloperPageIntentController);
    intentDispatcher.register(RoutableArticlePageIntentController);
    intentDispatcher.register(RoutableTodayPageIntentController);
    intentDispatcher.register(RoomPageIntentController);
    intentDispatcher.register(RoutableArcadeSeeAllPageController);
    intentDispatcher.register(EulaPageIntentController);
    intentDispatcher.register(ChartsPageRedirectIntentController);
    intentDispatcher.register(ErrorPageIntentController);

    // "Charts" Pages; "hub" must come first since so it's URL matches before the "detail" page
    intentDispatcher.register(ChartsHubPageIntentController);
    intentDispatcher.register(ChartsPageIntentController);

    // Product Page Routes; order is important due to overlapping URL patterns
    // The product page itself must come last or it will match the more-specific patterns
    intentDispatcher.register(AppEventPageIntentController);
    intentDispatcher.register(SeeAllPageIntentController);
    intentDispatcher.register(ProductPageIntentController);

    intentDispatcher.register(new CategoryTabsIntentController());

    // We register the root redirect controller last so more specific path patterns can be matched first
    intentDispatcher.register(RootRedirectController);

    return intentDispatcher;
}

/**
 * Bootstraps the Jet runtime for Apps
 *
 * @param dependencies dependencies to initialize the Object Graph with
 */
export function bootstrap(dependencies: Dependencies): {
    runtime: AppStoreRuntime;
    objectGraph: AppStoreObjectGraph;
} {
    const intentDispatcher = makeIntentDispatcher();

    const baseObjectGraph = makeObjectGraph(dependencies);

    const router = makeRouterUsingRegisteredControllers(
        intentDispatcher,
        baseObjectGraph,
    );
    const appObjectGraph = baseObjectGraph
        .adding(ObjectGraphType.router, router)
        .adding(ObjectGraphType.dispatcher, intentDispatcher);

    return {
        runtime: new AppStoreRuntime(intentDispatcher, appObjectGraph),
        objectGraph: appObjectGraph,
    };
}
