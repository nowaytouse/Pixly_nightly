import type { IntentController } from '@jet/environment/dispatching';
import type { RouteProvider } from '@jet/environment/routing';
import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import { withActiveIntent } from '@jet-app/app-store/foundation/dependencies/active-intent';
import { generateRoutes } from '@jet-app/app-store/common/util/generate-routes';
import { injectWebNavigation } from '@jet-app/app-store/common/web-navigation/inject-web-navigation';
import { makeChartsPageURL } from '@jet-app/app-store/common/charts/charts-page-url';
import { makeChartsPageIntent } from '@jet-app/app-store/api/intents/charts-page-intent';
import { GenericPage } from '@jet-app/app-store/api/models';
import { isPreviewPlatform } from '@jet-app/app-store/api/models/preview-platform';
import { notFoundError } from '@jet-app/app-store/foundation/media/network';

const makeIntent = (opts) => ({
    ...opts,
    $kind: 'ChartsPageRedirect',
});

// This will catch URLs like `/charts/iphone`
const { routes: routesWithoutGenreId } = generateRoutes(
    makeIntent,
    '/charts/{platform}',
);

// This will catch URLs like `/charts/iphone/utilities-apps/6002`
const { routes: routesWithGenreId } = generateRoutes(
    makeIntent,
    '/charts/{platform}/{slug}/{genreId}',
);

function chartsPageRedirectRoutes(objectGraph: AppStoreObjectGraph) {
    return [
        ...routesWithoutGenreId(objectGraph),
        ...routesWithGenreId(objectGraph),
    ];
}

export const ChartsPageRedirectIntentController: IntentController<any> &
    RouteProvider = {
    $intentKind: 'ChartsPageRedirect',

    routes: chartsPageRedirectRoutes,

    async perform(intent, objectGraphWithoutActiveIntent: AppStoreObjectGraph) {
        return await withActiveIntent(
            objectGraphWithoutActiveIntent,
            intent,
            async (objectGraph) => {
                const page = new GenericPage([]);
                const chartPageIntent = makeChartsPageIntent(intent);

                if (!isPreviewPlatform(intent.platform)) {
                    throw notFoundError();
                }

                // Setting the `canonicalUrl` on the page to normal Charts Page URL (e.g. /{platform}/charts)
                // will trigger a 301 redirect to the that page.
                page.canonicalURL = makeChartsPageURL(
                    objectGraph,
                    chartPageIntent,
                );

                injectWebNavigation(objectGraph, page, intent.platform);

                return page;
            },
        );
    },
};
