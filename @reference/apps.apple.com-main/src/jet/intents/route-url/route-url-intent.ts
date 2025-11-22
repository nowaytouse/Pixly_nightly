import type { Optional } from '@jet/environment/types/optional';
import type { Intent } from '@jet/environment/dispatching';
import type { FlowAction } from '@jet-app/app-store/api/models';

import type {
    NormalizedStorefront,
    NormalizedLanguage,
} from '@jet-app/app-store/api/locale';

/**
 * A response from the router given an incoming (deeplink) URL.
 */
export interface RouterResponse {
    /**
     * The intent to dispatch to get the view model for this URL.
     */
    intent: Intent<unknown>;

    /**
     * action to navigate to a new page of the app.
     */
    action: FlowAction;

    storefront: NormalizedStorefront;

    language: NormalizedLanguage;
}

export interface RouteUrlIntent extends Intent<Optional<RouterResponse>> {
    $kind: 'RouteUrlIntent';

    /**
     * The URL to route (ex. "https://podcasts.apple.com/us/show/serial/id123").
     */
    url: string;
}

export function isRouteUrlIntent(
    intent: Intent<unknown>,
): intent is RouteUrlIntent {
    return intent.$kind === 'RouteUrlIntent';
}

export function makeRouteUrlIntent(
    options: Omit<RouteUrlIntent, '$kind'>,
): RouteUrlIntent {
    return { $kind: 'RouteUrlIntent', ...options };
}
