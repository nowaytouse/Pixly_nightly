import type { MetricsEventRecorder } from '@jet/engine';
import type { LintedMetricsEvent } from '@jet/environment/types/metrics';
import type { Opt } from '@jet/environment/types/optional';
import type { Logger, LoggerFactory } from '@amp/web-apps-logger';
import type { ClickstreamProcessor as ClickstreamProcessorInstance } from '@amp-metrics/mt-metricskit-processor-clickstream';
import type { Impressions } from '../impressions';
import { sendToMetricsDevConsole } from '../utils/metrics-dev-console/setup-metrics-dev';
import { getEventFieldsWithTopic } from '../utils/get-event-field-topic';
import { eventType } from '../utils/metrics-dev-console/constants';

interface DeferredEvent {
    event: LintedMetricsEvent;
    topic: Opt<string>;
}

export interface FunnelKitConfig {
    constraintProfiles: string[];
    topic: string;
}

/**
 * These fields are considered PII and should be ignored by FunnelKit.
 * `consumerId` is added via the `processEvent` based on when it is available (see jet/metrics/index.ts)
 * However it should be ignored when sent to the FunnelKit topic.
 */
const IGNORED_FIELDS = ['consumerId'];

export class FunnelKitRecorder implements MetricsEventRecorder {
    private readonly log: Logger;
    private funnelKit: ClickstreamProcessorInstance | undefined;
    private funnelKitEnabled: boolean = false;
    private recordedEventsCount: number;
    private config: FunnelKitConfig;
    private readonly impressions: InstanceType<typeof Impressions> | undefined;

    /**
     * Queues events prior to the mt-event-queue recorder being available
     */
    private readonly deferredEvents: DeferredEvent[];

    constructor(
        loggerFactory: LoggerFactory,
        config: FunnelKitConfig,
        impressions: InstanceType<typeof Impressions> | undefined,
    ) {
        this.log = loggerFactory.loggerFor('FunnelKitRecorder');
        this.deferredEvents = [];
        this.recordedEventsCount = 0;
        this.config = config;
        this.impressions = impressions;
    }

    async record(
        event: LintedMetricsEvent,
        eventTopic: Opt<string>,
    ): Promise<void> {
        let topic = eventTopic ?? this.config.topic;

        // TV always uses the config topic
        // TODO: rdar://151772731 (Align funnel metrics between Music + TV)
        if (this.config.topic === 'xp_amp_tv_unidentified') {
            topic = this.config.topic;
        }

        if (!this.funnelKitEnabled) {
            this.log.info('FunnelKit not enabled', event, topic);
            return;
        }

        if (this.funnelKit) {
            const eventHandler = event.fields.eventType as string;
            const { pageId, pageType, pageContext } = event.fields;
            if (!eventHandler) {
                this.log.warn('No `eventType` found on event', event, topic);
            } else if (!this.impressions && eventHandler === 'impressions') {
                this.log.info(
                    'Supressing impression event. Impressions not enabled',
                );
                return;
            }

            // when the user leaves a page to report the accumulated impressions for that page
            if (
                (this.impressions?.isEnabled('exit') &&
                    eventHandler === 'exit') ||
                (this.impressions?.isEnabled('click') &&
                    event.fields.actionType === 'navigate')
            ) {
                // create + capture impressions
                const accumulatedImpressions =
                    this.impressions.consumeImpressions();
                const metricsData = this.funnelKit?.eventHandlers[
                    'impressions'
                ]?.metricsData(pageId, pageType, pageContext, {
                    impressions: accumulatedImpressions,
                });

                metricsData
                    ?.recordEvent(topic)
                    .then((data) => {
                        this.log.info(
                            'impressions event captured',
                            data,
                            topic,
                        );
                        sendToMetricsDevConsole(
                            data as { [key: string]: unknown },
                            topic,
                        );
                    })
                    .catch((e) => {
                        this.log.warn(
                            'failed to capture impression metrics',
                            e,
                            topic,
                        );
                    });
            }

            let impressionsData: Record<string, unknown> = {};
            // snapshot impressions to include in click events
            if (
                (this.impressions?.isEnabled('click') &&
                    eventHandler === 'click') ||
                (this.impressions?.isEnabled('impressions') &&
                    eventHandler === 'impressions')
            ) {
                const snapshotImpressions =
                    this.impressions.captureSnapshotImpression();
                impressionsData = snapshotImpressions
                    ? {
                          impressions: snapshotImpressions,
                      }
                    : {};
            }

            const eventFields = getEventFieldsWithTopic(event, topic);
            // Handle transaction events differently per Ember implementation
            // https://github.pie.apple.com/amp-ui/ember-metrics/blob/7eb762601db5e37cb428d7a4e6f24e22d0529515/addon/services/metrics.js#L347-L349
            const metricsDataArgs =
                eventHandler === 'transaction'
                    ? [eventFields]
                    : [pageId, pageType, pageContext, eventFields];

            try {
                const baseFields = await this.funnelKit.eventHandlers[
                    eventHandler
                ]
                    ?.metricsData(
                        // @ts-expect-error TypeScript doesn't handle spreading the argument array well
                        ...metricsDataArgs,
                    )
                    .toJSON();

                const metricsData = {
                    ...baseFields,
                    ...eventFields,
                    ...impressionsData,
                };
                IGNORED_FIELDS.forEach(
                    (ignoredField) => delete metricsData[ignoredField],
                );
                this.log.info('FunnelKit event data', metricsData, topic);

                try {
                    const data =
                        await this.funnelKit.system.eventRecorder.recordEvent(
                            topic,
                            metricsData,
                        );
                    sendToMetricsDevConsole(data, topic);
                } catch (e) {
                    this.log.info(
                        'FunnelKit failed to capture',
                        metricsData,
                        topic,
                    );
                }

                // on exit events we should flush all metrics
                if (eventHandler === 'exit') {
                    this.funnelKit?.system.eventRecorder.flushUnreportedEvents?.(
                        true,
                    );

                    sendToMetricsDevConsole(
                        { metricsDevType: eventType.FLUSH, status: 'SUCCESS' },
                        topic,
                    );
                }

                this.recordedEventsCount++;
            } catch (e) {
                this.log.error('FunnelKit failed to capture metric', e, topic);
            }
        } else {
            this.deferredEvents.push({ event, topic });
        }
    }

    async flush(): Promise<number> {
        if (!this.funnelKitEnabled) {
            return 0;
        }

        await this.funnelKit?.system.eventRecorder.flushUnreportedEvents(false);
        const count = this.recordedEventsCount;
        this.recordedEventsCount = 0;
        return count;
    }

    setupEventRecorder(funnelKit: ClickstreamProcessorInstance): void {
        this.funnelKit = funnelKit;
        this.deferredEvents.forEach(({ event, topic }) =>
            this.record(event, topic),
        );
        this.deferredEvents.length = 0;
    }

    enableFunnelKit(): void {
        if (this.funnelKitEnabled) {
            return;
        }

        this.log.info('Enabling FunnelKit');
        this.funnelKitEnabled = true;
    }

    disableFunnelKit(): void {
        if (!this.funnelKitEnabled) {
            return;
        }

        this.log.info('Disabling FunnelKit');
        this.funnelKitEnabled = false;
    }
}
