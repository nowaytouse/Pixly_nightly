import { isNothing, unwrapOptional } from '@jet/environment';
import type { Intent } from '@jet/environment/dispatching';
import type { LoggerFactory } from '@amp/web-apps-logger';
import { History } from '@amp/web-apps-utils';

import type { FlowAction } from '@jet-app/app-store/api/models';
import { isSearchResultsPageIntent } from '@jet-app/app-store/api/intents/search-results-page-intent';
import { isChartsPageIntent } from '@jet-app/app-store/api/intents/charts-page-intent';

import type { Jet } from '~/jet';
import { type Page, assertIsPage, FLOW_ACTION_KIND } from '~/jet/models';
import { mapException } from '~/utils/error';
import { stripHost } from '~/utils/url';

import type { ComponentProps } from 'svelte';
import type AppComponent from '~/App.svelte';
import { handleModalPresentation } from '~/jet/utils/handle-modal-presentation';
import { addRejectedIntent } from '../utils/error-metadata';

type AppComponentProps = Partial<ComponentProps<AppComponent>>;

// This action handler is responsible for all routing and related state
// management.
//
// Take care when making modifications here. There are many subtle invariants
// that must be maintained. They should be documented in comments throughout.
// It might be best to read the whole file to understand this full context
// before attempting even a small fix.
//
// High level overview:
//
// There are two ways for routing state changes to arise in the app:
//
//   1. Direct user interaction with the app (a FlowAction)
//   2. Indirect user interaction via browser back/forward buttons (popstate)
//
// FlowAction is the bedrock of navigation in the app. Anytime the user interacts
// with a button, link, etc. a FlowAction is performed (Jet.perform). When that
// happens, the Jet runtime eventually invokes the handler in this file
// (see jet.onAction below) to change the state of the app.
//
// This file manages the browser history and thus has the dual responsibility
// of handling state changes that come from the back and forward buttons. The
// state stored off when handling a FlowAction is later used by the popstate
// handler to navigate backwards without needing to re-fetch the previous page.
//
// Take note that these two processes are coupled fairly tightly due to the
// popstate needing data from the previous navigation. This is stored in the
// State interface. Take care when updating one flow that a modification is
// likely needed in the other.
//
// At the end of both of these processes, a call to updateApp is made. This
// changes the view model passed down to the top level <App> component. As a
// result of Svelte's reactivity, this could result in the entire page changing
// or just a part of it being amended to or removed. Additionally, the `page`
// passed in (the view model) can also be a promise. In which case, <App> will
// await it and display a loading spinner until it resolves or rejects.
//
// Notable specifics:
//
// Handling a FlowAction roughly has the following steps:
//
//   1. Extract a "destination" intent from the FlowAction. Recall that Jet
//      actions communicate a user interaction, but return no value. Jet
//      intents can be contained within an action and return data. In this case,
//      the intent derived from a FlowAction is used to retrieve the data for
//      the new page to which the FlowAction sends the user.
//
//   2. Dispatch the "destination" intent. Here, we resolve the Promise when
//      the page is ready, but we'll resolve early with an unresolve page
//      promise after 500ms. We take advantage of that the fact that passing a
//      Promise to updateApp will show a loading spinner. We wait 500ms,
//      because we don't want to immediately show a loading spinner or change
//      the page.
//
//   3. Update current page state in the history (ex. scroll position) and then
//      push a new history state for the page we're about to display. Note that
//      this must be done after the page Promise resolves, because we need to
//      store the page view model itself and we only know the canonicalURL of
//      it once it resolves. This state is used by popstate to return to the
//      page should the user ever leave and then come back to it.
//
//   4. Call updateApp to change the UI presented. At this point, it could be a
//      completed page (in which case step 3 will have already happened). The
//      app will display the new page immediately. Or, it could still be a
//      Promise (in which case step 3 will happen once it resolves and then the
//      page will resolve). The <App> will display a loading spinner until this
//      resolution happens.
//
// Handling a popstate event follows a similar pattern, but has some additional
// complexity.
//
// The simple case is that the state that we stored off above in step 3 is
// available. In which case, returning to the old page only involves calling
// updateApp with the view model we stored.
//
// But, we don't want to store an infinite history as these view models are
// sizable. We limit history to an arbitrary depth. After the user has
// navigated beyond that depth, we forget the oldest states. If a user ever
// were to back button all the way back to them, there would be no view model
// to restore. But, we do have the URL, so we use that and pretend like we're
// deeplinking into the app again for the first time. Care must be taken here
// to not perform a FlowAction, since that would modify the history. popstate
// events have already modified the browser history to point to the desired
// new state. So, we manually dispatch the page intent and perform other
// actions (such as switching the selected tab) ourselves. We then use the page
// promise as above to call updateApp.
export type Dependencies = {
    jet: Jet;
    logger: LoggerFactory;
    updateApp: (props: AppComponentProps) => void;
};

interface State {
    page: Page;
}

export function registerHandler(dependencies: Dependencies) {
    const { jet, logger, updateApp } = dependencies;

    const history = new History<State>(logger, {
        getScrollablePageElement() {
            return (
                document.getElementById('scrollable-page-override') ||
                document.getElementById('scrollable-page') ||
                // If we haven't defined a specific scrollable element,
                // scroll the whole page
                document.getElementsByTagName('html')?.[0]
            );
        },
    });

    const log = logger.loggerFor('jet/action-handlers/flow-action');

    let isFirstPage = true;

    jet.onAction(FLOW_ACTION_KIND, async (action: FlowAction) => {
        log.info('received FlowAction:', action);
        // timer for request time start
        // TODO: fix perfkit - rdar://111465791 ([Onyx] Foundation - PerfKit)
        // const pageSpeedMetric = perfkit.makeNewPageSpeedMetric();
        // pageSpeedMetric.capturePageRequestTime();

        let intent: Intent<unknown>;
        try {
            intent = unwrapOptional(action.destination);
        } catch (e) {
            log.info(
                '`FlowAction` received without a destination `Intent`: update the Jet app to attach an `Intent` to this `FlowAction`',
            );

            return;
        }

        // If the destination `Intent` must be performed server-side, determine
        // the destination URL and perform full browser navigation to that location
        if (!isFirstPage && mustPerformServerSide(intent)) {
            const { pageUrl } = action;

            if (isNothing(pageUrl)) {
                log.error(
                    `\`${intent.$kind}\` must be performed server-side, but the action lacks a \`pageUrl\` to navigate to`,
                );
                return 'performed';
            }

            window.location.href = stripHost(pageUrl);
            return 'performed';
        }

        // We capture this variable since below it is used asynchronously, but
        // we updated it at the end of this handler (so it could change before
        // it's used below).
        const shouldReplace = isFirstPage;

        // Resolves either when the page is ready or 800ms have elapsed
        // (we want to show a loading spinner after 800ms)
        const page = await getPage(intent, action);

        // If the action requires the page to be rendered in a modal.
        if (action.presentationContext === 'presentModal') {
            handleModalPresentation(page, log, action.page);
            return 'performed';
        }

        // This must happen before history.replaceState/pushState
        // We call this now, because the next line updates <App> which changes
        // the DOM. After that point we can't do things like record scroll
        // position, etc.
        history.beforeTransition();

        updateApp({
            page: page.promise.then((page: Page): Page => {
                const state = {
                    page,
                };

                const canonicalURL = mapException(
                    () => unwrapOptional(page.canonicalURL),
                    '`page` resolved without a `canonicalURL`, which is required for navigation',
                );

                // TODO: fix perfkit - rdar://111465791 ([Onyx] Foundation - PerfKit)
                // perfkit.setPageType(page.pageMetrics?.pageFields?.pageType as string | undefined || 'unknown');

                if (shouldReplace) {
                    history.replaceState(state, canonicalURL);
                } else {
                    history.pushState(state, canonicalURL);
                }

                didEnterPage(page);
                return page;
            }),
            isFirstPage,
        });

        // Future updates won't be for the first page
        isFirstPage = false;

        return 'performed';
    });

    history.onPopState(
        async (url: string, state: State | undefined): Promise<void> => {
            // NOTE: We don't call history.beforeTransition() anywhere here,
            // because we don't expect to save any state from the previous page
            // on back.

            if (state) {
                const { page } = state;

                log.info('received popstate, so resetting page:', page);
                didEnterPage(page);
                updateApp({ page, isFirstPage });

                return;
            }

            // If the state is missing page data, we have to recompute the view model
            const routing = await jet.routeUrl(url);

            if (!routing) {
                log.error(
                    'received popstate without data, but URL was unroutable:',
                    url,
                );

                // This probably shouldn't happen (since we only ever push valid
                // URLs to the history), but if it does, the best we can do is show
                // an error.
                didEnterPage(null); // to exit the current page
                updateApp({
                    page: Promise.reject(new Error('404')),
                    isFirstPage,
                });
                return;
            }

            log.info(
                'received popstate without data, so routing URL to:',
                routing,
            );

            // We can't perform the FlowAction here, as that would cause a new
            // history state to be pushed. Since we're in the context of a
            // popState, that would cause an infinite history loop where the back
            // button goes back but then immediately pushes again to the history
            // (so the user doesn't actually go back in history).
            // See: rdar://92621382 (Navigating more than 10 pages and then going back breaks back button)
            //
            // Careful reading will note that this promise will not reject.
            // Only the page.promise can reject (and we'll hand that to updateApp,
            // which will display the appropriate error for this case).
            //
            // Like in the handling of FlowAction (above), this blocks for at
            // most 800ms before resolving. Either the page is ready, or we
            // want to display a loading spinner. updateApp() will show a
            // spinner if page.promise is not ready.
            const page = await getPage(routing.intent, routing.action);

            updateApp({
                page: page.promise.then((page: Page): Page => {
                    // No history.replaceState/pushState like in handling FlowAction
                    // (above) since this is in the context of a popstate. The
                    // history stack, URL bar, etc. have already been updated.

                    didEnterPage(page);
                    return page;
                }),
                isFirstPage,
            });
        },
    );

    /**
     * Get a Page by dispatching its intent. Returns a promise that resolves
     * when the page is ready or after 800ms, whichever is first.
     *
     * The promise-inside-an-object-inside-a-promise return type is
     * intentional. If we just returned Promise<Page>, then this function
     * would not resolve until the page was ready. But we want it to resolve
     * after 800ms, even if the page isn't ready.
     */
    async function getPage(
        intent: Intent<unknown>,
        sourceAction: FlowAction | undefined,
    ): Promise<{ promise: Promise<Page> }> {
        const page = (async (): Promise<Page> => {
            try {
                let page = await jet.dispatch(intent);
                log.info('FlowAction destination resolved to:', page);

                assertIsPage(page);

                return page;
            } catch (e: any) {
                log.error('FlowAction destination rejected:', e);

                // Provide a way to retry the flow action from <ErrorPage>
                if (!e.userInfo || e.userInfo.status !== 404) {
                    e.retryFlowAction = sourceAction;
                }

                e.isFirstPage = isFirstPage;
                addRejectedIntent(e, intent);
                throw e;
            }
        })();

        // Wait until the page loads (or up to 500ms, then show loading spinner)
        await Promise.race([
            page,
            // Note that this has interplay with <PageResolver>
            new Promise((resolve) => setTimeout(resolve, 500)),
            // TODO: rdar://78166703 Add test to ensure catch no-ops
            //
            // NOTE: This catch is important. If the page promise rejects, we
            // want that to flow down into updateApp, where the appropriate
            // error page will be displayed. If we don't no-op here, we'll
            // cause the FlowAction to not finish handling (and updateApp will
            // never be called).
        ]).catch(() => {});

        // Wrapping in an object to prevent this function's promise from
        // not resolving until the page is ready. We want to resolve
        // immediately if it's already been 800ms
        return { promise: page };
    }

    function didEnterPage(page: Page | null): void {
        // Wrapped in an IIFE to avoid blocking anything (or breaking anything
        // if this fails)
        (async (): Promise<void> => {
            try {
                await jet.didEnterPage(page);
            } catch (e) {
                log.error('didEnterPage error:', e);
            }
        })();
    }
}

/**
 * Determines if an `Intent` must be performed server-side
 */
function mustPerformServerSide(intent: Intent<unknown>): boolean {
    return isSearchResultsPageIntent(intent) || isChartsPageIntent(intent);
}
