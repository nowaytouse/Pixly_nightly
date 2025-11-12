import type { LoggerFactory as AppLoggerFactory } from '@amp/web-apps-logger';

import { Random } from '@amp/web-apps-common/src/jet/dependencies/random';
import { Host } from '@amp/web-apps-common/src/jet/dependencies/host';
import { WebBag } from './bag';
import { WebClient } from './client';
import { WebConsole } from './console';
import { Locale } from './locale';
import { WebLocalization } from './localization';
import { makeProperties } from './properties';
import { WebMetricsIdentifiers } from './metrics-identifiers';
import { Net, type FeaturesCallbacks } from './net';
import { WebStorage } from './storage';
import { makeUnauthenticatedUser } from './user';
import { SEO } from './seo';

export type Dependencies = ReturnType<typeof makeDependencies>;

export function makeDependencies(
    loggerFactory: AppLoggerFactory,
    fetch: typeof window.fetch,
    featuresCallbacks?: FeaturesCallbacks,
) {
    const locale = new Locale(loggerFactory);
    return {
        bag: new WebBag(loggerFactory, locale),
        client: new WebClient(
            // TODO: set the right `BuildType` based on the environment where the app is running
            'production',
            locale,
        ),
        console: new WebConsole(loggerFactory),
        host: new Host(),
        localization: new WebLocalization(locale, loggerFactory),
        locale,
        metricsIdentifiers: new WebMetricsIdentifiers(),
        net: new Net(fetch, featuresCallbacks),
        properties: makeProperties(),
        random: new Random(),
        seo: new SEO(locale),
        storage: new WebStorage(),
        user: makeUnauthenticatedUser(),
        URL,
    };
}
