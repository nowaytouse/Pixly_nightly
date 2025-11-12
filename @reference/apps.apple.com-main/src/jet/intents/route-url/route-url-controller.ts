import { isSome } from '@jet/environment/types/optional';
import type { IntentController } from '@jet/environment/dispatching';
import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import { isRoutableIntent } from '@jet-app/app-store/api/intents/routable-intent';

import type { RouteUrlIntent } from '~/jet/intents';
import { makeFlowAction } from '~/jet/models';

export const RouteUrlIntentController: IntentController<RouteUrlIntent> = {
    $intentKind: 'RouteUrlIntent',

    async perform(intent: RouteUrlIntent, objectGraph: AppStoreObjectGraph) {
        const targetIntent = objectGraph.router.intentFor(intent.url);

        if (isSome(targetIntent) && isRoutableIntent(targetIntent)) {
            return {
                // intent needed for SSR
                intent: targetIntent,
                // only ever used by client; only clients have actions
                action: makeFlowAction(targetIntent),
                storefront: targetIntent.storefront,
                language: targetIntent.language,
            };
        }

        return null;
    },
};
