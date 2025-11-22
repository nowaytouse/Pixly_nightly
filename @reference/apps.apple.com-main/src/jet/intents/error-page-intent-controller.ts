import type { Intent, IntentController } from '@jet/environment/dispatching';
import type { Opt } from '@jet/environment/types/optional';
import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import { withActiveIntent } from '@jet-app/app-store/foundation/dependencies/active-intent';
import { injectWebNavigation } from '@jet-app/app-store/common/web-navigation/inject-web-navigation';

import { ErrorPage } from '~/jet/models/error-page';
import type { Page } from '~/jet/models/page';
import { getRejectedIntent } from '~/jet/utils/error-metadata';
import { isWithPlatform } from '~/jet/utils/with-platform';

interface ErrorPageIntent extends Intent<Page> {
    $kind: 'ErrorPageIntent';
    error: Opt<Error>;
}

export function makeErrorPageIntent(
    options: Omit<ErrorPageIntent, '$kind'>,
): ErrorPageIntent {
    return {
        ...options,
        $kind: 'ErrorPageIntent',
    };
}

export const ErrorPageIntentController: IntentController<ErrorPageIntent> = {
    $intentKind: 'ErrorPageIntent',

    async perform(
        intent,
        objectGraphWithoutActiveIntent: AppStoreObjectGraph,
    ): Promise<Page> {
        const { error } = intent;
        const rejectedIntent = error ? getRejectedIntent(error) : null;
        const platform =
            (rejectedIntent && isWithPlatform(rejectedIntent)
                ? rejectedIntent.platform
                : null) ?? 'iphone';

        return await withActiveIntent(
            objectGraphWithoutActiveIntent,
            { ...intent, platform },
            async (objectGraph) => {
                const page = new ErrorPage({ error: intent.error });

                injectWebNavigation(objectGraph, page, platform);

                return page;
            },
        );
    },
};
