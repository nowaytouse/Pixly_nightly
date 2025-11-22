import type {
    ArticlePage,
    ChartsHubPage,
    GenericPage,
    SearchLandingPage,
    SearchResultsPage,
    ShelfBasedProductPage,
    TopChartsPage,
    TodayPage,
    SeeAllPage,
} from '@jet-app/app-store/api/models';
import { StaticMessagePage } from '~/jet/models/static-message-page';
import { isObject } from '~/utils/types';
import { ErrorPage } from './error-page';
import type { WebRenderablePage } from 'node_modules/@jet-app/app-store/src/api/models/web-renderable-page';

/**
 * The union of every type of page that the App Store Onyx app can render
 */
export type Page = (
    | ArticlePage
    | ChartsHubPage
    | GenericPage
    | SearchLandingPage
    | SearchResultsPage
    | ShelfBasedProductPage
    | StaticMessagePage
    | TopChartsPage
    | TodayPage
    | ErrorPage
) &
    // TS needs to be told this explicitly, even though all the above implement this
    WebRenderablePage;

/**
 * Detects if {@linkcode page} is actually an {@linkcode AppEventDetailPage}
 */
export function isAppEventDetailPage(page: Page): page is GenericPage {
    return (
        'shelves' in page &&
        page.shelves.some(({ contentType }) => contentType === 'appEventDetail')
    );
}

/**
 * Detects if {@linkcode page} is actually an {@linkcode ArticlePage}
 */
export function isArticlePage(page: Page): page is ArticlePage {
    return 'card' in page && 'shelves' in page;
}

/**
 * Detects if {@linkcode page} is actually a {@linkcode ChartsHubPage}
 */
export function isChartsHubPage(page: Page): page is ChartsHubPage {
    return 'charts' in page;
}

/**
 * Detects if {@linkcode page} is actually a {@linkcode GenericPage}
 */
export function isGenericPage(page: Page): page is GenericPage {
    return 'shelves' in page;
}

/**
 * Detects if {@linkcode page} is actually a {@linkcode ShelfBasedProductPage}
 */
export function isShelfBasedProductPage(
    page: Page,
): page is ShelfBasedProductPage {
    return 'shelfMapping' in page && !('seeAllType' in page);
}

/**
 * Detects if {@linkcode page} is actually a {@linkcode SeeAllPage}
 */
export function isSeeAllPage(page: Page): page is SeeAllPage {
    return 'seeAllType' in page;
}

/**
 * Detects if {@linkcode page} is actually a {@linkcode SearchLandingPage}
 */
export function isSearchLandingPage(page: Page): page is SearchLandingPage {
    return 'adIncidents' in page;
}

/**
 * Detects if {@linkcode page} is actually a {@linkcode SearchResultsPage}
 */
export function isSearchResultsPage(page: Page): page is SearchResultsPage {
    return 'searchClearAction' in page || 'searchCancelAction' in page;
}

/**
 * Detects if {@linkcode page} is actually a {@linkcode TopChartsPage}
 */
export function isTopChartsPage(page: Page): page is TopChartsPage {
    return 'segments' in page && 'categories' in page;
}

/**
 * Detects if {@linkcode page} is actually a {@linkcode TodayPage}
 */
export function isTodayPage(page: Page): page is TodayPage {
    return 'titleDetail' in page;
}

/**
 * Detects if {@linkcode page} is actually a {@linkcode StaticMessagePage}
 */
export function isStaticMessagePage(
    page: GenericPage,
): page is StaticMessagePage {
    return 'pageType' in page && page.pageType === 'staticMessagePage';
}

export function isErrorPage(page: GenericPage) {
    return 'pageType' in page && page.pageType === 'errorPage';
}

/**
 * Type-guard that determines if the provided {@linkcode page} matches a renderable {@linkcode Page} definition
 */
export function isPage(page: unknown): page is Page {
    if (!isObject(page)) {
        return false;
    }

    return [
        isAppEventDetailPage,
        isArticlePage,
        isChartsHubPage,
        isGenericPage,
        isShelfBasedProductPage,
        isSearchLandingPage,
        isSearchResultsPage,
        isTopChartsPage,
        isTodayPage,
        isErrorPage,
        isSeeAllPage,
    ].some((specificPageTypePredicate) =>
        specificPageTypePredicate(
            // This type-cast reflects the fact that we don't really know if `page` is really a `Page`,
            // but that we're going to use the type-guards of our `Page` members to see if `page` looks
            // like one of them
            page as Page,
        ),
    );
}

/**
 * Type-assertion that determines if the provided {@linkcode page} matches a renderable {@linkcode Page} definition
 */
export function assertIsPage(page: unknown): asserts page is Page {
    if (!isPage(page)) {
        throw new Error(
            'The view-model for the dispatched `Intent` does not match a known renderable shape',
        );
    }
}

/**
 * Detects if {@linkcode page} has the Vision Pro pathname in it's URL.
 */
export function hasVisionProUrl(page: GenericPage) {
    if (!page.canonicalURL) {
        return false;
    }

    const url = new URL(page.canonicalURL);
    return (
        url.pathname.includes('/vision/apps-and-games') ||
        url.pathname.includes('/vision/arcade')
    );
}
