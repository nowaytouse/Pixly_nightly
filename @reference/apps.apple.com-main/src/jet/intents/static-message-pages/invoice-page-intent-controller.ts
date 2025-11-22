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
        $kind: 'InvoicePageIntent',
    }),
    '/invoice',
);

export const InvoicePageIntentController: IntentController<any> &
    RouteProvider = {
    $intentKind: 'InvoicePageIntent',

    routes,

    async perform(intent, objectGraphWithoutActiveIntent: AppStoreObjectGraph) {
        return await withActiveIntent(
            objectGraphWithoutActiveIntent,
            intent,
            async (objectGraph) => {
                const page = new StaticMessagePage({
                    titleLocKey: 'ASE.Web.AppStore.Invoice.Title',
                    contentType: 'invoice',
                });

                page.canonicalURL = makeCanonicalUrl(objectGraph, intent);

                injectWebNavigation(objectGraph, page, intent.platform);
                return page;
            },
        );
    },
};
