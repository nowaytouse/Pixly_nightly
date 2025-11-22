import type { Intent } from '@jet/environment/dispatching';
import { FlowAction } from '@jet-app/app-store/api/models';

export const FLOW_ACTION_KIND: FlowAction['$kind'] = 'flowAction';

/**
 * Creates a FlowAction For a given destination.
 *
 * Note: this is only here temporarily as a convenience for the "web" client, to be used
 * while the upstream `FlowAction` is represented as a class that needs to be constructed,
 * so those details are abstracted away from our codebase. Once `FlowAction` has been
 * migrated to a POJO, there should be a factory-function provided that we should leverage
 * instead
 *
 * @param destination Destination of the `FlowAction`
 */
export function makeFlowAction(destination: Intent<unknown>): FlowAction {
    const action = new FlowAction(
        // This data is only used by the Jet app's `PageRouter` architecture, which is not
        // relevant for us. We should safely be able to pass an arbitrary value here.
        'page',
    );

    // The important part for the "web" client router: setting the `destination`
    action.destination = destination;

    return action;
}
