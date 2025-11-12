import type { MetricsEventRecorder } from '@jet/engine';
import type { LintedMetricsEvent } from '@jet/environment/types/metrics';
import type { Opt } from '@jet/environment/types/optional';

export class VoidEventRecorder implements MetricsEventRecorder {
    private recorded: number = 0;

    record(_event: LintedMetricsEvent, _topic: Opt<string>): void {
        this.recorded++;
    }

    async flush(): Promise<number> {
        const { recorded } = this;
        this.recorded = 0;
        return recorded;
    }
}
