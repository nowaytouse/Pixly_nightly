import type { Logger, LoggerFactory } from '@amp/web-apps-logger/src/types';
import { IMPRESSION_CONTEXT_NAME } from './constants';
import { createSvelteImpressionAction } from './utils/svelte/impressions-svelte-action';
import type {
    AppImpressionModel,
    ImpressionSettings,
    ImpressionsInstance,
} from './types';
import type {
    ImpressionObserver,
    newInstanceWithMetricsConfig,
} from '@amp-metrics/mt-impressions-observer';
import type { ClickstreamProcessor } from '@amp-metrics/mt-metricskit-processor-clickstream';

/**
 * Adapter class to handle interactions with
 * metricsKit impression observer.
 */
export class Impressions implements ImpressionsInstance {
    private readonly logger: Logger;
    private impressionObserverInstance: ImpressionObserver | undefined;
    private hasInitialized: boolean = false;
    private impressionDataMap: Map<HTMLElement, any> = new Map();
    private currentSnapshot: Record<string, unknown>[] = [];
    private readonly impressionSettings: ImpressionSettings | undefined;

    constructor(
        loggerFactory: LoggerFactory,
        context: Map<string, unknown>,
        settings?: ImpressionSettings,
    ) {
        this.logger = loggerFactory.loggerFor(IMPRESSION_CONTEXT_NAME);
        this.impressionSettings = settings;

        context.set(IMPRESSION_CONTEXT_NAME, this);
    }

    async init(
        makeImpressionObserver: typeof newInstanceWithMetricsConfig,
        clickStreamInstance: ClickstreamProcessor,
    ) {
        if (this.hasInitialized) {
            this.logger.warn(
                'Ignoring, Impressions.init() can only be called once',
            );
            return;
        }

        const options = { root: document, rootMargin: '0px' };
        const config = clickStreamInstance.config;
        const impressionObserver: ImpressionObserver =
            await makeImpressionObserver(config, options);

        impressionObserver.setDelegate({
            extractImpressionInfo: (domNode: HTMLElement) => {
                const dataMap = this.impressionDataMap;
                const nodeMetricData = dataMap.get(domNode);
                if (nodeMetricData) {
                    const impressionData = nodeMetricData.impressionMetrics;
                    impressionData.location =
                        clickStreamInstance.utils.eventFields.buildLocationStructure(
                            domNode,
                            (node: HTMLElement) => {
                                const metrics = dataMap.get(node);
                                if (metrics?.location) {
                                    return metrics.location;
                                }
                                return;
                            },
                        );
                    return impressionData;
                } else {
                    this.logger.warn('no impression data found for', domNode);
                }
            },
        });

        this.impressionObserverInstance = impressionObserver;
        this.impressionDataMap.forEach((_value, node) => {
            this.logger.debug('observing deffered node', node);
            this.impressionObserverInstance?.observe(node);
        });
        this.hasInitialized = true;

        this.logger.debug('impressions initialized');
    }

    get settings() {
        return this.impressionSettings;
    }

    isEnabled(event: 'click' | 'exit' | 'impressions'): boolean {
        if (this.impressionSettings?.captureType === 'jet') {
            return (
                this.impressionSettings?.metricsKitEvents?.includes(event) ??
                false
            );
        }
        return true;
    }

    consumeImpressions(): Record<string, unknown>[] | undefined {
        if (this.hasInitialized) {
            this.logger.debug('consuming impression metrics');
            return this.impressionObserverInstance?.consumeImpressions();
        }
        this.logger.warn('impressions not avaiable yet');
        return;
    }

    captureSnapshotImpression(): Record<string, unknown>[] | undefined {
        const snapshot =
            this.impressionObserverInstance?.snapshotImpressions() ?? [];

        // if the current page already transitioned. fallback to the snapshot we captured before transition
        if (snapshot.length === 0) {
            return this.getSnapshotImpression();
        }

        return snapshot;
    }

    getSnapshotImpression(): Record<string, unknown>[] | undefined {
        if (this.hasInitialized) {
            return this.currentSnapshot;
        }
        this.logger.warn('impressions not avaiable yet');
        return;
    }

    setCurrentSnapshot(): void {
        if (this.hasInitialized) {
            this.logger.debug('capturing impression snapshot');
            this.currentSnapshot =
                this.impressionObserverInstance?.snapshotImpressions() ?? [];
        } else {
            this.logger.warn('impressions not avaiable yet');
        }
    }

    get nodeList() {
        const impressionClass = this;

        return new Proxy(impressionClass.impressionDataMap, {
            get(target, prop, receiver) {
                const orginalFn = Reflect.get(target, prop, receiver);

                // overriding 'set' to also be able to observe
                if (prop === 'set') {
                    return (
                        node: HTMLElement,
                        value: Record<string, unknown>,
                    ) => {
                        if (impressionClass.hasInitialized) {
                            impressionClass.logger.debug(
                                'observing',
                                node,
                                value,
                            );

                            impressionClass.impressionObserverInstance?.observe(
                                node,
                            );
                        }

                        return orginalFn.bind(target)(node, value);
                    };
                }

                // overriding 'delete' to also be able to unobserve
                if (prop === 'delete') {
                    return (node: HTMLElement) => {
                        if (impressionClass.hasInitialized) {
                            impressionClass.logger.debug('unobserve', node);
                            impressionClass.impressionObserverInstance?.unobserve(
                                node,
                            );
                        }

                        return orginalFn.bind(target)(node);
                    };
                }

                return orginalFn.bind(target);
            },
            set(target, prop, value) {
                return Reflect.set(target, prop, value);
            },
        });
    }
}

/**
 * Server Noop for above
 */
class ServerNoopImpressions implements ImpressionsInstance {
    readonly nodeList: WeakMap<any, any>;
    constructor() {
        this.nodeList = new WeakMap();
    }
    setCurrentSnapshot(): void {}
}

/**
 * Gets the current Impression instance from the Svelte context.
 *
 * @return The current instance of Impression
 */
export function generateBrowserImpressionsContextGetter(
    getContext: (context: string) => unknown,
): () => AppImpressionModel {
    return function getImpressions(): AppImpressionModel {
        const impressions = getContext(IMPRESSION_CONTEXT_NAME) as
            | Impressions
            | undefined;

        if (!impressions) {
            const noopImpressions = new ServerNoopImpressions();
            return {
                captureImpressions: (_node: any, _impressionsData: any) => {
                    return {
                        destroy() {},
                    };
                },
                impressions: noopImpressions,
            };
        }

        return {
            captureImpressions: createSvelteImpressionAction(impressions),
            impressions,
        };
    };
}

/**
 * Server No-op for generateImpressionsContextGetter
 *
 */
export function generateServerImpressionsContextGetter(
    _getContext: (context: string) => unknown,
): () => AppImpressionModel {
    const impressions = new ServerNoopImpressions();
    return () => ({
        captureImpressions: (_node: any, _impressionsData: any) => {
            return {
                destroy() {},
            };
        },
        impressions,
    });
}
