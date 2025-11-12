import { getContext } from 'svelte';

const CONTEXT_NAME = 'shared-components:preconnect-tracker';

/**
 * Setup a PreconnectTracker used by <Artwork> and <MotionVideo>.
 * This keeps track of the origins of rendered assets to generate the
 * appropriate <link rel="preconnect"> tags.
 *
 * Preconnect tags should be rendered by placing a <Preconnects /> at the
 * bottom of the top level <App> component.
 */
export class PreconnectTracker {
    private readonly originsSet: Set<string>;

    /**
     * Add a new PreconnectTracker to the Svelte context.
     * This should only be called on the server. The components will no-op when
     * run clientside (if this isn't called).
     */
    static setup(context: Map<string, unknown>): PreconnectTracker {
        const tracker = new PreconnectTracker();
        context.set(CONTEXT_NAME, tracker);
        return tracker;
    }

    private constructor() {
        this.originsSet = new Set();
    }

    /**
     * Track a URL of an asset for preconnect origin aggregation.
     * This should only be called from `<Artwork>` and `<MotionVideo>`.
     */
    trackUrl(url: string): void {
        try {
            const { origin } = new URL(url);
            this.originsSet.add(origin);
        } catch (_) {
            // Just in case the URL parsing fails
            // Worst case this misses a preconnect. We'd rather it not take
            // down the whole component.
        }
    }

    /**
     * The current list of origins of all rendered <Artwork> and <MotionVideo>
     * components.
     */
    get origins(): string[] {
        return [...this.originsSet];
    }
}

/**
 * Gets the current PreconnectTracker instance from the Svelte context.
 *
 * @return locale The current instance of Locale
 */
export function getPreconnectTracker(): PreconnectTracker | undefined {
    // We intentionally allow this to be missing. In the browse, we want this
    // since preconnects are only needed for SSR.
    return getContext(CONTEXT_NAME) as PreconnectTracker | undefined;
}
