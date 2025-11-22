import type { MetricsEventRecorder } from '@jet/engine';
import type { LintedMetricsEvent } from '@jet/environment/types/metrics';
import type { Opt } from '@jet/environment/types/optional';
import type { Logger, LoggerFactory } from '@amp/web-apps-logger';

import { METRICS_EVENT_TYPES } from '../constants';

import type { WebDelegates as WebDelegatesInstance } from '@amp-metrics/mt-metricskit-delegates-web';
import type { ClickstreamProcessor as ClickstreamProcessorInstance } from '@amp-metrics/mt-metricskit-processor-clickstream';
import type { Impressions } from '../impressions';
import { sendToMetricsDevConsole } from '../utils/metrics-dev-console/setup-metrics-dev';
import { getEventFieldsWithTopic } from '../utils/get-event-field-topic';
import { eventType } from '../utils/metrics-dev-console/constants';

interface DeferredEvent {
    event: LintedMetricsEvent;
    topic: Opt<string>;
}

type EventRecorder = WebDelegatesInstance['eventRecorder'];

type MetricEventType = (typeof METRICS_EVENT_TYPES)[number];

export interface MetricKitConfig {
    constraintProfiles: string[];
    topic: string;
}

export class MetricsKitRecorder implements MetricsEventRecorder {
    private readonly log: Logger;
    private eventRecorder: EventRecorder | undefined;
    private mtkit: ClickstreamProcessorInstance | undefined;
    private recordedEventsCount: number;
    private config: MetricKitConfig;
    private readonly impressions: InstanceType<typeof Impressions> | undefined;
    private enabled: boolean = true;

    /**
     * Queues events prior to the mt-event-queue recorder being available
     */
    private readonly deferredEvents: DeferredEvent[];

    constructor(
        loggerFactory: LoggerFactory,
        config: MetricKitConfig,
        impressions: InstanceType<typeof Impressions> | undefined,
    ) {
        this.log = loggerFactory.loggerFor('MetricsKitRecorder');
        this.deferredEvents = [];
        this.recordedEventsCount = 0;
        this.config = config;
        this.impressions = impressions;
    }

    record(event: LintedMetricsEvent, topic: Opt<string>): void {
        topic = topic ?? this.config.topic;
        if (this.isDisabled()) {
            this.log.info(
                `topic ${this.config.topic} is disabled following event not captured:`,
                event,
            );
            return;
        }

        if (this.eventRecorder) {
            const eventHandler = event.fields.eventType as MetricEventType;
            const { pageId, pageType, pageContext } = event.fields;
            if (!eventHandler) {
                this.log.warn('No `eventType` found on event', event, topic);
                return;
            } else if (!METRICS_EVENT_TYPES.includes(eventHandler)) {
                this.log.warn(
                    'Invalid `eventType` found on event',
                    event,
                    topic,
                );
                return;
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

                const metricsData = this.mtkit?.eventHandlers[
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
                            topic ?? '',
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

            let impressionsData = {};
            // snapshot impressions to include in click events
            if (
                (this.impressions?.isEnabled('click') &&
                    eventHandler === 'click') ||
                (this.impressions?.isEnabled('impressions') &&
                    eventHandler === 'impressions')
            ) {
                const snapshotImpressions =
                    this.impressions.captureSnapshotImpression();
                impressionsData = {
                    impressions: snapshotImpressions,
                };
            }

            const eventFields = getEventFieldsWithTopic(event, topic);
            // click events are the only ones with different method signature
            // https://github.pie.apple.com/amp-metrics/mt-metricskit/blob/7.3.5/src/metrics/event_handlers/click.js#L133
            const metricsDataArgs =
                eventHandler === 'click' // TODO rdar://102438307 (JMOTW Clickstream – Pass targetElement to click events)
                    ? [
                          pageId,
                          pageType,
                          pageContext,
                          null,
                          { ...eventFields, ...impressionsData },
                      ]
                    : [pageId, pageType, pageContext, eventFields];

            if (eventHandler === 'impressions') {
                metricsDataArgs.push(impressionsData);
            }

            let metricsData = this.mtkit?.eventHandlers[
                eventHandler
            ]?.metricsData(
                // @ts-expect-error TypeScript doesn't handle spreading the argument array well
                ...metricsDataArgs,
            );

            metricsData
                ?.recordEvent(topic)
                .then((data) => {
                    this.log.info('MetricsKit event data', data, topic);
                    sendToMetricsDevConsole(
                        data as { [key: string]: unknown },
                        topic ?? '',
                    );
                })
                .catch((e) => {
                    this.log.error(
                        'MetricsKit failed to capture metric',
                        e,
                        topic,
                    );
                });

            this.recordedEventsCount++;

            // on exit events we should flush all metrics
            if (eventHandler === 'exit') {
                this.eventRecorder?.flushUnreportedEvents?.(true);
                sendToMetricsDevConsole(
                    { metricsDevType: eventType.FLUSH, status: 'SUCCESS' },
                    topic,
                );
            }
        } else {
            this.deferredEvents.push({ event, topic });
        }
    }

    async flush(): Promise<number> {
        await this.eventRecorder?.flushUnreportedEvents?.(false);
        const count = this.recordedEventsCount;
        this.recordedEventsCount = 0;
        return count;
    }

    setupEventRecorder(
        eventRecorder: EventRecorder,
        mtkit: ClickstreamProcessorInstance,
    ): void {
        this.eventRecorder = eventRecorder;
        this.mtkit = mtkit;
        this.deferredEvents.forEach(({ event, topic }) =>
            this.record(event, topic),
        );
        this.deferredEvents.length = 0;
    }

    isDisabled(): boolean {
        return !this.enabled;
    }

    enable(): void {
        if (this.enabled) {
            this.log.info(
                `Clickstream topic ${this.config.topic} already enabled`,
            );
            return;
        }

        this.log.info(`Enabling clickstream topic ${this.config.topic}`);
        this.enabled = true;
    }

    disable(): void {
        if (this.isDisabled()) {
            return;
        }

        this.log.info(`Disabling clickstream topic ${this.config.topic}`);
        this.enabled = false;
    }
}
