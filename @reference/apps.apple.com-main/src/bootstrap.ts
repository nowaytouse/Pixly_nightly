// Sets up app specific configurations
import type { Opt } from '@jet/environment';
import type { Intent } from '@jet/environment/dispatching';
import type { ActionModel } from '@jet/environment/types/models';
import { initializeUniqueIdContext } from '@amp/web-app-components/src/utils/uniqueId';
import { setLocale as setSharedLocale } from '@amp/web-app-components/src/utils/locale';

import type {
    NormalizedStorefront,
    NormalizedLanguage,
} from '@jet-app/app-store/api/locale';

import {
    DEFAULT_STOREFRONT_CODE,
    DEFAULT_LANGUAGE_BCP47,
} from '~/constants/storefront';
import { Jet } from '~/jet';
import { setup as setupI18n } from '~/stores/i18n';
import type { PrefetchedIntents } from '@amp/web-apps-common/src/jet/prefetched-intents';
import type { LoggerFactory } from '@amp/web-apps-logger';
import type { Locale as Language } from '@amp/web-apps-localization';
import type I18N from '@amp/web-apps-localization';
import '~/config/components/artwork';
import '~/config/components/shelf';
import type { FeaturesCallbacks } from './jet/dependencies/net';

export type Context = Map<string, unknown>;

export async function bootstrap({
    loggerFactory,
    initialUrl,
    fetch,
    prefetchedIntents,
    featuresCallbacks,
}: {
    loggerFactory: LoggerFactory;
    initialUrl: string;
    fetch: typeof window.fetch;
    prefetchedIntents: PrefetchedIntents;
    featuresCallbacks?: FeaturesCallbacks;
}): Promise<{
    context: Context;
    jet: Jet;
    initialAction: Opt<ActionModel>;
    intent: Opt<Intent<unknown>>;
    storefront: NormalizedStorefront;
    language: NormalizedLanguage;
    i18n: I18N;
}> {
    const log = loggerFactory.loggerFor('bootstrap');

    const context = new Map();

    const jet = Jet.load({
        loggerFactory,
        context,
        fetch,
        prefetchedIntents,
        featuresCallbacks,
    });

    initializeUniqueIdContext(context, loggerFactory);

    const routing = await jet.routeUrl(initialUrl);

    if (routing) {
        log.info('initial URL routed to:', routing);
    } else {
        log.warn('initial URL was unroutable:', initialUrl);
    }

    const {
        intent = null,
        action: initialAction = null,
        storefront = DEFAULT_STOREFRONT_CODE,
        language = DEFAULT_LANGUAGE_BCP47,
    } = routing || {};

    // TODO: rdar://78109398 (i18n Improvements)
    const i18nStore = await setupI18n(
        context,
        loggerFactory,
        language.toLowerCase() as Language,
    );
    jet.setLocale(i18nStore, storefront, language);
    setSharedLocale(context, { storefront, language });

    return {
        context,
        jet,
        initialAction,
        intent,
        storefront,
        language,
        i18n: i18nStore,
    };
}
