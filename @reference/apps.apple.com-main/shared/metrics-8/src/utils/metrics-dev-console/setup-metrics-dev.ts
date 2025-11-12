import { isFlushEvent, makeFlushEvent } from './events/flush-event';
import { makeRecordEvent } from './events/record-event';
import type { MetricsOptions, FlushEvent, MetricsObject } from './type';

/**
 * Updates the metrics console by dispatching appropriate events
 */
const updateMetricsConsole = (
    topic: string,
    metricsData: MetricsOptions | FlushEvent,
): void => {
    let event = null;
    const { metricsDevType, ...data } = metricsData ?? ({} as MetricsObject);

    if (isFlushEvent(metricsData)) {
        event = makeFlushEvent(metricsData, topic);
    } else if (metricsData) {
        event = makeRecordEvent(data, topic);
    }

    if (event) {
        try {
            window.dispatchEvent(event);
        } catch (e) {
            console.error('metric console failed', e);
        }
    }
};

const isMetricsDevConsoleEnabled = () => {
    return (
        typeof window !== 'undefined' &&
        window.localStorage?.getItem('metrics-dev') === 'true'
    );
};

/**
 * Sends metrics data to the development console if enabled
 * @param metricsData - The metrics data to send
 * @param topic - The topic/category for the metrics
 */
export const sendToMetricsDevConsole = (
    metricsData: MetricsOptions,
    topic: string,
): void => {
    if (import.meta.env.APP_SCOPE === 'internal') {
        if (isMetricsDevConsoleEnabled()) {
            try {
                updateMetricsConsole(topic, metricsData);
            } catch (error) {
                console.warn('Failed to send metrics to dev console:', error);
            }
        }
    }
};
