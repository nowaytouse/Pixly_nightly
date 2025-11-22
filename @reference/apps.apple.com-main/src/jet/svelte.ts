import { getContext } from 'svelte';
import type { Opt } from '@jet/environment';
import type { ActionOutcome } from '@jet/engine';

import type { ActionModel } from '~/jet/models';
import type { Jet } from '~/jet/jet';

export const CONTEXT_NAME = 'jet';

/**
 * Gets the current Jet instance from the Svelte context.
 *
 * @return jet The current instance of Jet
 */
export function getJet(): Jet {
    const jet = getContext<Opt<Jet>>(CONTEXT_NAME);

    if (!jet) {
        throw new Error('getJet called before Jet.load');
    }

    return jet;
}

/**
 * Jet helper to expose jet.perform in single location
 *
 * @return Promise<ActionOutcome>
 */
type ActionUndefined = 'noActionProvided';

export function getJetPerform(): (
    action: ActionModel,
) => Promise<ActionOutcome | ActionUndefined> {
    const jet = getJet();

    return (action: ActionModel) => {
        if (!action) {
            //TODO: rdar://73165545 (Error Handling Across App)
            return Promise.resolve('noActionProvided');
        }

        return jet.perform(action);
    };
}
