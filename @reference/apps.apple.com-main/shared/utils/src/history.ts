import type { Logger, LoggerFactory } from '@amp/web-apps-logger';
import { LruMap } from './lru-map';
import type { ScrollableElement } from './try-scroll';
import { tryScroll } from './try-scroll';
import { removeHost } from './url';
import { generateUuid } from './uuid';

export interface Options {
    getScrollablePageElement(): ScrollableElement | null;
}

type Id = string;
const HISTORY_SIZE_LIMIT = 10;

interface WithScrollPosition<State> {
    scrollY: number;
    state: State;
}
/**
 * We are using a currentStateId on this class to always store the state id instead of saving
 * it on the window.history.state because there seems to be a bug in Safari where it is mutating
 * the window.history.state to null after our Sign In flow which includes multiple iframes
 * and multiple internal state changes inside the iframes. We can move back to window.history.state storing the id
 * if the Safari Issue is fixed in future.
 */
export class History<State> {
    private readonly log: Logger;
    private readonly states: LruMap<Id, WithScrollPosition<State>>;
    private readonly getScrollablePageElement: () => ScrollableElement | null;
    private currentStateId: string | undefined;

    constructor(
        loggerFactory: LoggerFactory,
        options: Options,
        sizeLimit: number = HISTORY_SIZE_LIMIT,
    ) {
        this.log = loggerFactory.loggerFor('History');
        this.states = new LruMap(sizeLimit);
        this.getScrollablePageElement = options.getScrollablePageElement;
    }

    // Update page data but keep scroll position
    updateState(update: (state?: State) => State): void {
        if (!this.currentStateId) {
            this.log.warn(
                'failed: encountered a null currentStateId inside updateState',
            );
            return;
        }

        const currentState = this.states.get(this.currentStateId);
        const newState = update(currentState?.state);
        this.log.info('updateState', newState, this.currentStateId);
        this.states.set(this.currentStateId, {
            ...(currentState as WithScrollPosition<State>),
            state: newState,
        });
    }

    replaceState(state: State, url: string | null): void {
        const id = generateId();
        this.log.info('replaceState', state, url, id);
        window.history.replaceState({ id }, '', this.removeHost(url));
        this.currentStateId = id;
        this.states.set(id, { state, scrollY: 0 });
        this.scrollTop = 0;
    }

    pushState(state: State, url: string | null): void {
        const id = generateId();
        this.log.info('pushState', state, url, id);
        window.history.pushState({ id }, '', this.removeHost(url));
        this.currentStateId = id;
        this.states.set(id, { state, scrollY: 0 });
        this.scrollTop = 0;
    }

    beforeTransition(): void {
        const { state } = window.history;

        if (!state) {
            return;
        }

        const oldState = this.states.get(state.id);
        if (!oldState) {
            this.log.info(
                'current history state evicted from LRU, not saving scroll position',
            );
            return;
        }

        const { scrollTop } = this;

        this.states.set(state.id, {
            ...oldState,
            scrollY: scrollTop,
        });

        this.log.info('saving scroll position', scrollTop);
    }

    private removeHost(url: string | null): string | undefined {
        if (!url) {
            this.log.warn('received null URL');
            return;
        }

        // TODO: rdar://77982655 (Investigate router improvements): host mismatch?
        return removeHost(url);
    }

    onPopState(
        listener: (url: string, state: State | undefined) => void,
    ): void {
        window.addEventListener('popstate', (event: PopStateEvent): void => {
            this.currentStateId = event.state?.id;

            if (!this.currentStateId) {
                this.log.warn(
                    'encountered a null event.state.id in onPopState event: ',
                    window.location.href,
                );
            }

            this.log.info('popstate', this.states, this.currentStateId);
            const state = this.currentStateId
                ? this.states.get(this.currentStateId)
                : undefined;
            listener(window.location.href, state?.state);

            if (!state) {
                return;
            }

            const { scrollY } = state;

            this.log.info('restoring scroll to', scrollY);

            tryScroll(this.log, () => this.getScrollablePageElement(), scrollY);
        });
    }

    private get scrollTop(): number {
        return this.getScrollablePageElement()?.scrollTop || 0;
    }

    private set scrollTop(scrollTop: number) {
        const element = this.getScrollablePageElement();
        if (element) {
            element.scrollTop = scrollTop;
        }
    }

    // TODO: rdar://77982655 (Investigate router improvements): offPopState?
}

/**
 * Generate a (unique) id for storing in window.history.state.
 *
 * @return the generated ID
 */
function generateId(): Id {
    // The use of something random (and not say, an incrementing counter) is important
    // here. These states can survive refreshes so the IDs used must be globally unique
    // (and not just unique to the current page load).
    return generateUuid();
}
