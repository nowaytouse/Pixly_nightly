import type { Opt } from '@jet/environment';
import type { Intent } from '@jet/environment/dispatching';

export function addRejectedIntent(error: Error, intent: Intent<unknown>) {
    (error as any).rejectedIntent = intent;
}

export function getRejectedIntent(error: Error): Opt<Intent<unknown>> {
    return hasRejectedIntent(error) ? error.rejectedIntent : null;
}

function hasRejectedIntent(
    error: Error,
): error is Error & { rejectedIntent: Intent<unknown> } {
    return 'rejectedIntent' in error;
}
