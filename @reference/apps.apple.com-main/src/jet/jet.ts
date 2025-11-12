import type I18N from '@amp/web-apps-localization';
import type { Logger, LoggerFactory } from '@amp/web-apps-logger';

import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import type { AppStoreRuntime } from '@jet-app/app-store/foundation/runtime/runtime';
import type {
    NormalizedStorefront,
    NormalizedLanguage,
} from '@jet-app/app-store/api/locale';

import type {
    LintedMetricsEvent,
    MetricsFields,
    PageMetrics,
} from '@jet/environment/types/metrics';
import { type Opt } from '@jet/environment/types/optional';
import type { Intent, IntentReturnType } from '@jet/environment/dispatching';
import {
    type ActionImplementation,
    ActionDispatcher,
    type ActionOutcome,
    type MetricsBehavior,
} from '@jet/engine';

import { Metrics } from '@amp/web-apps-metrics-8';
import { makeMetricsSettings } from '~/jet/metrics/settings';
import { makeMetricsProviders } from '~/jet/metrics/providers';
import { config as metricsConfig } from '~/config/metrics';

import { bootstrap } from '~/jet/bootstrap';
import { makeDependencies } from '~/jet/dependencies';
import type { Locale } from '~/jet/dependencies/locale';
import type { WebLocalization } from '~/jet/dependencies/localization';
import {
    type RouterResponse,
    type RouteUrlIntent,
    makeRouteUrlIntent,
    makeLintMetricsEventIntent,
} from '~/jet/intents';
import type { Page, ActionModel } from '~/jet/models';
import { PrefetchedIntents } from '@amp/web-apps-common/src/jet/prefetched-intents';
import { CONTEXT_NAME } from '~/jet/svelte';
import type { FeaturesCallbacks } from './dependencies/net';

/**
 * The entry point for interacting with the Jet shared business logic.
 */
export class Jet {
    private readonly log: Logger;
    private readonly runtime: AppStoreRuntime;
    private readonly actionDispatcher: ActionDispatcher;
    private readonly metrics: Metrics;
    private readonly locale: Locale;

    /**
     * Intents (and their resolved data) that have yet to be dispatched that
     * were recently dispatched. These are consulted before dispatching
     * intents. If a prefetched intent exists for an ongoing dispatch, it will
     * be used as the return value instead of actually dispatching.
     *
     * This can be used, for example, for intents that are dispatched during
     * SSR. The server can serialize the intents it dispatches and then the
     * client can populate this, to avoid re-dispatching the intents.
     */
    private readonly prefetchedIntents: PrefetchedIntents;

    /**
     * A set of the action types that already have registered implementations to catch
     * double registers.
     */
    private readonly wiredActions: Set<string>;

    readonly objectGraph: AppStoreObjectGraph;
    readonly localization: WebLocalization;

    static load({
        loggerFactory,
        context,
        fetch,
        prefetchedIntents = PrefetchedIntents.empty(),
        featuresCallbacks,
    }: {
        loggerFactory: LoggerFactory;
        context: Map<string, unknown>;
        fetch: typeof window.fetch;
        prefetchedIntents?: PrefetchedIntents;
        featuresCallbacks?: FeaturesCallbacks;
    }): Jet {
        const dependencies = makeDependencies(
            loggerFactory,
            fetch,
            featuresCallbacks,
        );
        const { runtime, objectGraph } = bootstrap(dependencies);
        let jet: Jet;

        const processEvent = async (
            fields: MetricsFields,
        ): Promise<LintedMetricsEvent> => {
            const intent = makeLintMetricsEventIntent({ fields });
            return jet.dispatch(intent);
        };
        const metrics = Metrics.load(
            loggerFactory,
            context,
            processEvent,
            metricsConfig,
            makeMetricsProviders(objectGraph),
            makeMetricsSettings(context),
        );
        const actionDispatcher = new ActionDispatcher(
            // `@amp/web-apps-metrics` depends on a different version of `@jet/engine` with a different
            // type definition for `MetricsPipeline`
            // @ts-expect-error
            metrics.metricsPipeline,
        );

        jet = new Jet(
            loggerFactory.loggerFor('Jet'),
            runtime,
            objectGraph,
            actionDispatcher,
            metrics,
            dependencies.locale,
            prefetchedIntents,
            dependencies.localization,
        );

        context.set(CONTEXT_NAME, jet);

        return jet;
    }

    private constructor(
        log: Logger,
        runtime: AppStoreRuntime,
        objectGraph: AppStoreObjectGraph,
        actionDispatcher: ActionDispatcher,
        metrics: Metrics,
        locale: Locale,
        prefetchedIntents: PrefetchedIntents,
        localization: WebLocalization,
    ) {
        this.log = log;
        this.runtime = runtime;
        this.objectGraph = objectGraph;
        this.actionDispatcher = actionDispatcher;

        this.metrics = metrics;
        this.locale = locale;
        this.localization = localization;

        this.prefetchedIntents = prefetchedIntents;

        this.wiredActions = new Set();
    }

    async didEnterPage(page: Page | null): Promise<void> {
        // This is a very temporary hacky fix to move the `platformContext` value from
        // `pageRenderFields` to `pageFields`, which will eventually happen in the Jet
        // business logic.
        const pageWithMetrics = { ...page };
        if (pageWithMetrics.pageMetrics?.pageFields) {
            pageWithMetrics.pageMetrics.pageFields.platformContext =
                pageWithMetrics.pageMetrics.pageRenderFields?.platformContext;
        }

        // @ts-expect-error - pageMetrics property not required at runtime
        await this.metrics.didEnterPage(page);
    }

    get pageMetrics(): Opt<PageMetrics> {
        return this.metrics.currentPageMetrics?.pageMetrics;
    }

    /**
     * Dispatch a Jet intent, returning its output.
     *
     * @param intent The intent to dispatch
     * @return output The value returned by the intent's controller
     */
    async dispatch<I extends Intent<unknown>>(
        intent: I,
    ): Promise<IntentReturnType<I>> {
        const data = this.prefetchedIntents.get(intent);
        if (data) {
            this.log.info(
                're-using prefetched intent response for:',
                intent,
                'data:',
                data,
            );
            return data;
        }

        // TODO: rdar://73165545 (Error Handling Across App)
        return this.runtime.dispatch(intent);
    }

    /**
     * Perform a Jet action, returning the outcome.
     *
     * @param action The action to perform
     * @param metricsBehavior Indicates how to handle metrics for this action
     * @return outcome Either 'performed' or 'unsupported'
     */
    async perform(
        action: ActionModel,
        metricsBehavior?: MetricsBehavior,
    ): Promise<ActionOutcome> {
        if (!metricsBehavior) {
            if (this.pageMetrics) {
                metricsBehavior = {
                    behavior: 'fromAction',
                    context: this.pageMetrics || {},
                };
            } else {
                this.log.warn(
                    'No pageMetrics found for jet.perform action:',
                    action,
                );
                metricsBehavior = { behavior: 'notProcessed' };
            }
        }
        // TODO: rdar://73165545 (Error Handling Across App): handle throw
        const outcome = await this.actionDispatcher.perform(
            action,
            metricsBehavior,
        );

        if (outcome === 'unsupported') {
            this.log.error(
                'unable to perform action:',
                action,
                metricsBehavior,
            );
        }

        return outcome;
    }

    /**
     * Register an implementation to handle a Jet action.
     *
     * @param kind The type of the action
     * @param implementation The code to run when that action is performed
     */
    onAction<A extends ActionModel>(
        kind: string,
        implementation: ActionImplementation<A>,
    ): void {
        if (this.wiredActions.has(kind)) {
            throw new Error(
                `onAction called twice with the same action type: ${kind}`,
            );
        }

        this.actionDispatcher.register(kind, implementation);
        this.wiredActions.add(kind);
    }

    /**
     * Route a URL using Jet, returning the routing if the URL could be routed.
     *
     * @param url The URL to route
     * @return routing The routing of the URL or null if unrouteable
     */
    async routeUrl(url: string): Promise<RouterResponse | null> {
        // TODO: rdar://73165545 (Error Handling Across App): what about 404s?
        const routerResponse = await this.dispatch<RouteUrlIntent>(
            makeRouteUrlIntent({ url }),
        );

        if (routerResponse && routerResponse.action) {
            return routerResponse;
        }

        this.log.warn(
            'url did not resolve to a flow action with a discernable intent:',
            url,
            routerResponse,
        );

        return null;
    }

    /**
     * Propagates the routing-derrived localization information through the Jet app
     *
     * The {@link Locale} instance that is configured here is referenced by
     * the rest of our Jet dependencies in order to lazily retreive the locale
     * information.
     *
     * @param localizer
     * @param storefront
     * @param language
     */
    setLocale(
        localizer: I18N,
        storefront: NormalizedStorefront,
        language: NormalizedLanguage,
    ): void {
        this.locale.i18n = localizer;
        this.locale.setActiveLocale({ storefront, language });
    }

    recordCustomMetricsEvent(fields?: Opt<MetricsFields>) {
        this.metrics.recordCustomEvent(fields);
    }

    enableFunnelKit(): void {
        this.metrics.enableFunnelKit();
    }

    disableFunnelKit(): void {
        this.metrics.disableFunnelKit();
    }

    // TODO: rdar://75011660 (Bridge Jet to MetricsKit and PerfKit for reporting)
}
