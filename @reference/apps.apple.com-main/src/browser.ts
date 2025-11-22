// This must be imported first to ensure base styles are imported first
import '~/styles/app-store.scss';

import App from '~/App.svelte';
import { bootstrap } from '~/bootstrap';
import { registerActionHandlers } from '~/jet/action-handlers';
import { PrefetchedIntents } from '@amp/web-apps-common/src/jet/prefetched-intents';
import {
    CompositeLoggerFactory,
    ConsoleLoggerFactory,
    DeferredLoggerFactory,
    setContext,
} from '@amp/web-apps-logger';

import { setHTMLAttributes } from '@amp/web-apps-localization';
import { ERROR_KIT_CONFIG } from '~/config/errorkit';
import {
    ErrorKitLoggerFactory,
    setupErrorKit,
} from '@amp/web-apps-logger/src/errorkit';
import { setupRuntimeFeatures } from '~/utils/features/runtime';

export async function startApplication() {
    const onyxFeatures = await setupRuntimeFeatures(
        new DeferredLoggerFactory(() => logger),
    );
    const consoleLogger = new ConsoleLoggerFactory();
    const errorKit = setupErrorKit(ERROR_KIT_CONFIG, consoleLogger);
    const logger = new CompositeLoggerFactory([
        consoleLogger,
        new ErrorKitLoggerFactory(errorKit),
        ...(onyxFeatures ? [onyxFeatures.recordingLogger] : []),
    ]);

    let url = window.location.href;

    // TODO: this is busted for some reason? rdar://111465791 ([Onyx] Foundation - PerfKit)
    // const perfkit = setupBrowserPerfkit(PERF_KIT_CONFIG, logger);

    // Initialize Jet, and get starting state.
    const { context, jet, initialAction, storefront, language, i18n } =
        await bootstrap({
            loggerFactory: logger,
            initialUrl: url,
            fetch: window.fetch.bind(window),
            prefetchedIntents: PrefetchedIntents.fromDom(logger, {
                evenIfSignedIn: true,
                featureKitItfe: onyxFeatures?.featureKit?.itfe,
            }),
            featuresCallbacks: {
                getITFEValues(): string | undefined {
                    return onyxFeatures?.featureKit?.itfe;
                },
            },
        });

    // TODO: fix perfkit - rdar://111465791 ([Onyx] Foundation - PerfKit)
    // setPageSpeedContext(context, perfkit, logger);
    setContext(context, logger);

    // Add lang + dir tag to HTML node
    setHTMLAttributes(language);

    // Using a container element to avoid svelte hydration
    // "clean up" from removing tags that have
    // been add to the <body> tag in our HTML file.
    const container = document.querySelector('.body-container');

    const app = new App({
        target: container,
        context,
        hydrate: true,
    });

    // Initialize action-handlers.
    registerActionHandlers({
        jet,
        logger,
        updateApp: (props) => app.$set(props),
    });

    if (initialAction) {
        // TODO: rdar://73165545 (Error Handling Across App): handle throw
        await jet.perform(initialAction);
    } else {
        app.$set({
            page: Promise.reject(new Error('404')),
            isFirstPage: true,
        });
    }
}

// If we export default here, this will run during tests when we do
// `import { startApplication } from '~/browser';`. To avoid this, we guard using the
// presence of an ENV var only set by Vitest.

// This is covered by acceptance tests
if (!import.meta.env?.VITEST) {
    startApplication();
}
