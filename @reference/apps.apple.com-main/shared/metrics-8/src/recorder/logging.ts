import type { MetricsEventRecorder } from '@jet/engine';
import type { LintedMetricsEvent } from '@jet/environment/types/metrics';
import type { Opt } from '@jet/environment/types/optional';
import type { Logger, LoggerFactory } from '@amp/web-apps-logger';

export class LoggingEventRecorder implements MetricsEventRecorder {
    private readonly log: Logger;

    constructor(loggerFactory: LoggerFactory) {
        this.log = loggerFactory.loggerFor('LoggingEventRecorder');
    }

    record(event: LintedMetricsEvent, topic: Opt<string>): void {
        this.log.info('logged metrics event:', event, topic);
    }

    async flush(): Promise<number> {
        this.log.info('flushed metrics');
        return 0;
    }
}
