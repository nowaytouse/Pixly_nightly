import type { MetricsEventRecorder } from '@jet/engine';
import type { LintedMetricsEvent } from '@jet/environment/types/metrics';
import type { Opt } from '@jet/environment/types/optional';

export class CompositeEventRecorder implements MetricsEventRecorder {
    constructor(private readonly eventRecorders: MetricsEventRecorder[]) {}

    record(event: LintedMetricsEvent, topic: Opt<string>): void {
        for (const eventRecorder of this.eventRecorders) {
            eventRecorder.record(event, topic);
        }
    }

    async flush(): Promise<number> {
        const flushed: number[] = await Promise.all(
            this.eventRecorders.map((recorder) => recorder.flush()),
        );
        return Math.max(...flushed);
    }
}
