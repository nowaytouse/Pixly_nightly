import type { Action, ExternalUrlAction } from '@jet-app/app-store/api/models';

export function isExternalUrlAction(
    action: Action,
): action is ExternalUrlAction {
    return action.$kind === 'ExternalUrlAction';
}
