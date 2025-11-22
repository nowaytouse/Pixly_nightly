import type { IntentController } from '@jet/environment/dispatching';
import type { RouteProvider } from '@jet/environment/routing';
import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import { withActiveIntent } from '@jet-app/app-store/foundation/dependencies/active-intent';
import { generateRoutes } from '@jet-app/app-store/common/util/generate-routes';
import { injectWebNavigation } from '@jet-app/app-store/common/web-navigation/inject-web-navigation';

import { StaticMessagePage } from '~/jet/models/static-message-page';

const { routes, makeCanonicalUrl } = generateRoutes(
    (opts) => ({
        ...opts,
        $kind: 'WinBackPageIntent',
    }),
    '/win-back/{offerId}',
    [],
    {
        extraRules: [
            {
                regex: [/(?:\/[a-z]{2})?\/win-back/],
            },
        ],
    },
);

export const WinBackPageIntentController: IntentController<any> &
    RouteProvider = {
    $intentKind: 'WinBackPageIntent',

    routes,

    async perform(intent, objectGraphWithoutActiveIntent: AppStoreObjectGraph) {
        return await withActiveIntent(
            objectGraphWithoutActiveIntent,
            intent,
            async (objectGraph) => {
                const page = new StaticMessagePage({
                    titleLocKey: 'ASE.Web.AppStore.WinBack.Title',
                    contentType: 'win-back',
                });

                page.canonicalURL = makeCanonicalUrl(objectGraph, intent);

                injectWebNavigation(objectGraph, page, intent.platform);
                return page;
            },
        );
    },
};
