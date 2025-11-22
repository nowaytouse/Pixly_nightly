import type { Logger, LoggerFactory } from '@amp/web-apps-logger';
import { getPWADisplayMode, PWADisplayMode } from '@amp/web-apps-utils/src';
import type {
    LintedMetricsEvent,
    MetricsData,
    MetricsFields,
} from '@jet/environment/types/metrics';
import type { PageMetrics } from '@jet/environment/types/metrics';

import type { Opt } from '@jet/environment';

import {
    MetricsFieldsAggregator,
    type MetricsFieldsContext,
    type MetricsFieldsProvider,
    MetricsPipeline,
    PageMetricsPresenter,
    type MetricsEventRecorder,
} from '@jet/engine';

import {
    CompositeEventRecorder,
    type FunnelKitConfig,
    FunnelKitRecorder,
    LoggingEventRecorder,
    type MetricKitConfig,
    MetricsKitRecorder,
    VoidEventRecorder,
} from './recorder';

import type {
    MetricsEnterEventType,
    MetricsExitEventType,
    SystemLoggerLevel,
} from './types';

import type {
    EnvironmentDelegates,
    WebDelegates as WebDelegatesInstance,
} from '@amp-metrics/mt-metricskit-delegates-web';
import type { ClickstreamProcessor as ClickstreamProcessorInstance } from '@amp-metrics/mt-metricskit-processor-clickstream';
import { Impressions } from './impressions';
import { buildMakeAjaxRequest } from './utils/metrics-dev-console/metrics-dev-network';
import { ImpressionFieldProvider } from './impression-provider';
import { ImpressionSnapshotFieldProvider } from './impression-snapshot-provider';
import type { ImpressionSettings } from './impressions/types';

const CONTEXT_NAME = 'metrics';

export type MetricsProvider = {
    provider: MetricsFieldsProvider;
    request: string;
};

export interface MetricSettings {
    shouldEnableImpressions?: () => boolean;
    shouldEnableFunnelKit: () => boolean;
    getConsumerId: () => Promise<string>;
    suppressMetricsKit?: boolean;
    impressions?: ImpressionSettings;
}

interface InitializedMetrics {
    clickstream: ClickstreamProcessorInstance;
    webDelegate: WebDelegatesInstance;
}

interface Config {
    baseFields: {
        appName: string;
        delegateApp: string;
        appVersion: string;
        resourceRevNum: string;
        storageObject?: 'sessionStorage' | 'localStorage';
    };
    clickstream: MetricKitConfig;

    /**
     * `FunnelKit` configuration
     *
     * Can be `undefined` to disable the `FunnelKit` recorder entirely
     */
    funnel?: FunnelKitConfig;

    initialURL?: string | null;
}

type ClickstreamProcessorClass = typeof ClickstreamProcessorInstance;
type WebDelegatesClass = typeof WebDelegatesInstance;

export class Metrics {
    private readonly log: Logger;
    private impressions: InstanceType<typeof Impressions> | undefined;

    // Properties asynchronously set in the `init` function
    private ClickstreamProcessor!: ClickstreamProcessorClass;
    private WebDelegates!: WebDelegatesClass;

    private readonly metricsKitRecorder?: MetricsKitRecorder;
    private readonly funnelKitRecorder?: FunnelKitRecorder;
    private firstEnterRecorded: boolean = false;
    private funnelKit: ClickstreamProcessorInstance | undefined;
    private config: Config;

    public readonly metricsPipeline: MetricsPipeline;
    public currentPageMetrics: Opt<PageMetricsPresenter>;

    static load(
        loggerFactory: LoggerFactory,
        context: Map<string, unknown>,
        processEvent: (fields: MetricsFields) => Promise<LintedMetricsEvent>,
        config: Config,
        listofMetricProviders: MetricsProvider[],
        settings: MetricSettings,
    ): Metrics {
        const {
            getConsumerId,
            shouldEnableFunnelKit,
            suppressMetricsKit = false,
        } = settings;

        const log = loggerFactory.loggerFor('Metrics');

        // server
        if (typeof window === 'undefined' || suppressMetricsKit) {
            const recorder = new VoidEventRecorder();
            const metricsPipeline = new MetricsPipeline({
                aggregator: new MetricsFieldsAggregator(),
                linter: {
                    async processEvent(
                        fields: MetricsFields,
                    ): Promise<LintedMetricsEvent> {
                        return { fields };
                    },
                },
                recorder,
            });

            return new Metrics(log, metricsPipeline, config);
        }

        config.initialURL = window.location.href;

        const aggregator = setupAggregators(listofMetricProviders, context);

        let impressions: InstanceType<typeof Impressions> | undefined =
            undefined;
        if (settings.shouldEnableImpressions?.() ?? false) {
            impressions = new Impressions(
                loggerFactory,
                context,
                settings?.impressions,
            );
        }

        const metricsKitRecorder = new MetricsKitRecorder(
            loggerFactory,
            config.clickstream,
            impressions,
        );

        const recorders: MetricsEventRecorder[] = [
            new LoggingEventRecorder(loggerFactory),
            metricsKitRecorder,
        ];

        const funnelKitRecorder = config.funnel
            ? new FunnelKitRecorder(loggerFactory, config.funnel, impressions)
            : undefined;
        if (funnelKitRecorder) {
            recorders.push(funnelKitRecorder);
        }

        let recorder = new CompositeEventRecorder(recorders);

        const metricsPipeline = new MetricsPipeline({
            aggregator,
            linter: {
                processEvent: async (fields: MetricsFields) => {
                    const lintedEvent = await processEvent(fields);

                    // `dsId` is added by the LintMetricsEventIntentController in music-ui-js, but is not needed and erroneous for web
                    // https://github.pie.apple.com/music/music-ui-js/blob/50cbae83deccffad37e5b617394ea30b7e082660/src/metrics/LintMetricsEventIntentController.ts#L19-L22
                    if (lintedEvent.fields?.dsId) {
                        delete lintedEvent.fields.dsId;
                    }

                    // Consumer ID needs to be added at the time of processEvent because the ConsumerID is available after Sign In and not before sign In
                    // Using it through the delegates does not have ability to fetch it dynamically
                    const consumerId = await getConsumerId();
                    if (consumerId) {
                        lintedEvent.fields.consumerId = consumerId;
                    }

                    return lintedEvent;
                },
            },
            recorder,
        });

        const metricsInstance = new Metrics(
            log,
            metricsPipeline,
            config,
            metricsKitRecorder,
            funnelKitRecorder,
            impressions,
        );
        metricsInstance.watchEnterAndExit();

        (async () => {
            try {
                const metricsDependencies = [
                    import('@amp-metrics/mt-metricskit-processor-clickstream'),
                    import('@amp-metrics/mt-metricskit-delegates-web'),
                    impressions
                        ? import('@amp-metrics/mt-impressions-observer')
                        : undefined,
                ] as const;

                const [
                    { ClickstreamProcessor },
                    { WebDelegates },
                    impressionsDependency,
                ] = await Promise.all(metricsDependencies);

                metricsInstance.onDependenciesLoaded(
                    ClickstreamProcessor,
                    WebDelegates,
                );

                const { clickstream, webDelegate } = setupMtkit(
                    ClickstreamProcessor,
                    WebDelegates,
                    config,
                );

                if (impressions && impressionsDependency) {
                    const { newInstanceWithMetricsConfig } =
                        impressionsDependency;
                    impressions.init(newInstanceWithMetricsConfig, clickstream);
                }

                const eventRecorder = webDelegate.eventRecorder;
                metricsKitRecorder.setupEventRecorder(
                    eventRecorder,
                    clickstream,
                );

                if (shouldEnableFunnelKit()) {
                    metricsInstance.enableFunnelKit();
                }
                log.info('Metricskit loaded');
            } catch (e) {
                log.warn('Metricskit failed to load', e);
            }
        })();

        // Save Metrics Instance on Context before Returning
        context.set(CONTEXT_NAME, metricsInstance);

        return metricsInstance;
    }

    private constructor(
        log: Logger,
        metricsPipeline: MetricsPipeline,
        config: Config,
        metricsKitRecorder?: MetricsKitRecorder,
        funnelKitRecorder?: FunnelKitRecorder,
        impressions?: InstanceType<typeof Impressions>,
    ) {
        this.log = log;
        this.metricsPipeline = metricsPipeline;
        this.metricsKitRecorder = metricsKitRecorder;
        this.funnelKitRecorder = funnelKitRecorder;
        this.config = config;
        this.impressions = impressions;
    }

    /**
     * Metrics code that should get called before a page changes.
     */
    willPageTransition(): void {
        this.impressions?.setCurrentSnapshot();
    }

    async didEnterPage<
        T extends { pageMetrics: PageMetrics; canonicalURL: string },
    >(page: T | null): Promise<void> {
        if (this.currentPageMetrics) {
            await this.currentPageMetrics.didLeavePage();
            this.currentPageMetrics = null;
        }

        if (page?.pageMetrics) {
            this.currentPageMetrics = new PageMetricsPresenter(
                this.metricsPipeline,
            );
            this.currentPageMetrics.pageMetrics = page.pageMetrics;
            await this.currentPageMetrics.didEnterPage();
        } else {
            this.log.warn('No pageMetrics', page);
        }

        if (!this.firstEnterRecorded) {
            const event = document.referrer?.length > 0 ? 'link' : 'launch';
            this.enter(event, { openUrl: page?.canonicalURL });
            this.firstEnterRecorded = true;
        }
    }

    async enter(type: MetricsEnterEventType, fields?: Opt<MetricsFields>) {
        let openUrl: string = window.location.href;
        let pwaDisplayMode: PWADisplayMode | null = null;

        if (fields?.openUrl) {
            openUrl = fields?.openUrl as string;
        }

        if (type === 'launch' && this.config.initialURL) {
            openUrl = this.config.initialURL;
            // Clearing the initial URL as we don't need this post launch event
            this.config.initialURL = null;
            pwaDisplayMode = getPWADisplayMode();
        }

        this.recordCustomEvent({
            eventType: 'enter',
            extRefUrl: document.referrer ?? '',
            refUrl: document.referrer ?? '',
            openUrl,
            type,
            // only add buildFlavor property if coming from the PWA (represented by 'standalone' in the manifest.json) or android app
            ...(pwaDisplayMode === PWADisplayMode.STANDALONE ||
            pwaDisplayMode === PWADisplayMode.TWA
                ? { buildFlavor: pwaDisplayMode }
                : {}),
        });
    }

    async exit(type: MetricsExitEventType, _fields?: Opt<MetricsFields>) {
        this.recordCustomEvent({
            eventType: 'exit',
            type,
        });
    }

    async pageTransition() {
        this.log.info('triggered metrics for page transition');
        if (this.impressions) {
            this.impressions.setCurrentSnapshot();
        }
    }

    private watchEnterAndExit() {
        document.addEventListener(
            'visibilitychange',
            this.onVisibilityChange.bind(this),
        );
    }

    async onVisibilityChange() {
        if (document.visibilityState === 'visible') {
            this.enter('taskSwitch');
        } else {
            this.exit('taskSwitch');
        }
    }

    async processEvent(metricsFields: MetricsFields) {
        const metricsData: MetricsData = {
            excludingFields: [],
            includingFields: [],
            shouldFlush: false,
            fields: metricsFields,
        };
        const context: MetricsFieldsContext = {};
        await this.metricsPipeline.process(metricsData, context);
    }

    async recordCustomEvent(fields?: Opt<MetricsFields>) {
        await this.processEvent({
            ...this.currentPageMetrics?.pageMetrics?.pageFields,
            ...fields,
        });
    }

    /**
     * Sets up FunnelKit for clickstream events
     */
    private setupFunnelKit(): void {
        if (!this.config.funnel) {
            this.log.warn(
                'Tried to set up `FunnelKit` but no config was provided',
            );
            return;
        }

        const { topic } = this.config.funnel;
        const { clickstream, webDelegate } = setupStarkit(
            this.ClickstreamProcessor,
            this.WebDelegates,
            this.config.funnel,
            this.config.baseFields,
        );
        clickstream.config.setDebugSource(null);

        // Disable PII fields and cookies for the funnel topic
        webDelegate.eventRecorder.setProperties?.(topic, {
            anonymous: true,
        });

        this.funnelKitRecorder?.setupEventRecorder(clickstream);
        this.funnelKit = clickstream;
    }

    private onDependenciesLoaded(
        ClickstreamProcessor: ClickstreamProcessorClass,
        webDelegate: WebDelegatesClass,
    ): void {
        this.ClickstreamProcessor = ClickstreamProcessor;
        this.WebDelegates = webDelegate;
    }

    disableMetrics(): void {
        this.metricsKitRecorder?.disable();
    }

    enableMetrics(): void {
        this.metricsKitRecorder?.enable();
    }

    enableFunnelKit(): void {
        if (!this.funnelKit) {
            this.setupFunnelKit();
        }
        this.funnelKitRecorder?.enableFunnelKit();
    }

    disableFunnelKit(): void {
        this.funnelKitRecorder?.disableFunnelKit();
    }
}

/**
 * Shared setup for *kit, namely MetricsKit and FunnelKit
 */
function setupStarkit(
    ClickstreamProcessor: ClickstreamProcessorClass,
    WebDelegates: WebDelegatesClass,
    setupConfig: FunnelKitConfig | MetricKitConfig,
    config: Config['baseFields'],
): InitializedMetrics {
    const { topic } = setupConfig;
    const webDelegate = new WebDelegates(topic);

    if (import.meta.env.APP_SCOPE === 'internal') {
        try {
            // Temporary setup to get Network Dependency
            const networkCopy = {
                ...Object.getPrototypeOf(webDelegate.config.network),
            };

            const makeAjaxRequest = buildMakeAjaxRequest(networkCopy, topic);

            webDelegate.setNetwork({
                makeAjaxRequest,
            });
        } catch (e) {
            console.warn('failed to setup flush logger');
        }
    }

    const clickstream = new ClickstreamProcessor(webDelegate);

    const systemLoggerLevel: SystemLoggerLevel = 'none';
    clickstream.system.logger.setLevel(systemLoggerLevel);
    clickstream.init();

    setupMtkitDelegates(clickstream, setupConfig, config);
    return { clickstream, webDelegate };
}

/**
 * MetricsKit setup for main clickstream events
 */
function setupMtkit(
    ClickstreamProcessor: ClickstreamProcessorClass,
    webDelegates: WebDelegatesClass,
    config: Config,
): InitializedMetrics {
    const mtkit = setupStarkit(
        ClickstreamProcessor,
        webDelegates,
        config.clickstream,
        config.baseFields,
    );
    return mtkit;
}

function setupMtkitDelegates(
    mtkit: ClickstreamProcessorInstance,
    setupConfig: FunnelKitConfig | MetricKitConfig,
    config: Config['baseFields'],
): void {
    const { appName, delegateApp, appVersion, resourceRevNum, storageObject } =
        config;
    const additionalDelegates: EnvironmentDelegates = {
        app: () => appName,
        appVersion: () => appVersion,
        delegateApp: () => delegateApp,
        resourceRevNum: () => resourceRevNum,
    };

    if (storageObject === 'sessionStorage') {
        additionalDelegates['localStorageObject'] = () => {
            return sessionStorage;
        };
    }

    mtkit.system.environment.setDelegate(additionalDelegates);

    if (Array.isArray(setupConfig.constraintProfiles)) {
        mtkit.config.setDelegate({
            constraintProfiles: () => setupConfig.constraintProfiles,
        });
    }
}

function setupAggregators(
    metricsFieldsProviders: MetricsProvider[],
    context: Map<string, unknown>,
): MetricsFieldsAggregator {
    const aggregator = MetricsFieldsAggregator.makeDefaultAggregator();

    aggregator.addOptInProvider(
        new ImpressionFieldProvider(context),
        'impressions',
    );

    aggregator.addOptInProvider(
        new ImpressionSnapshotFieldProvider(context),
        'impressionsSnapshot',
    );

    metricsFieldsProviders.forEach((metricsFields) => {
        aggregator.addOptOutProvider(
            metricsFields.provider,
            metricsFields.request,
        );
    });

    return aggregator;
}

/**
 * Gets the current Metrics instance from the Svelte context.
 *
 * @return metrics The current instance of Metrics
 */

export function generateMetricsContextGetter(
    getContext: (context: string) => unknown,
): () => Metrics {
    return function getMetrics(): Metrics {
        const metrics = getContext(CONTEXT_NAME) as Metrics | undefined;

        if (!metrics) {
            throw new Error('getMetrics called before Metrics.load');
        }

        return metrics;
    };
}

export * from './impressions/index';
export * from './impressions/utils/svelte/impressions-svelte-action';
