import type { Opt } from '@jet/environment/types/optional';
import type {
    ArcadeSeeAllGamesPage,
    ArticlePage,
    ChartsHubPage,
    GenericPage,
    ReviewsPage,
    SearchLandingPage,
    SearchResultsPage,
    SeeAllPage,
    ShelfBasedProductPage,
    TodayPage,
    TopChartsPage,
} from '@jet-app/app-store/api/models';
import type { WebRenderablePage } from '@jet-app/app-store/api/models/web-renderable-page';
import type { SEO as SEODependency } from '@jet-app/app-store/foundation/dependencies/seo';
import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import type { DataContainer } from '@jet-app/app-store/foundation/media/data-structure';

import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';

import type { Locale } from './locale';

import { seoDataForAnyPage, updateCanonicalURL } from '~/utils/seo/common';
import { seoDataForArticlePage } from '~/utils/seo/article-page';
import { seoDataForChartsPage } from '~/utils/seo/charts-page';
import { seoDataForChartsHubPage } from '~/utils/seo/charts-hub-page';
import { seoDataForDeveloperPage } from '~/utils/seo/developer-page';
import { seoDataForProductPage } from '~/utils/seo/product-page';
import { seoDataForAppEventDetailPage } from '~/utils/seo/app-event-detail-page';
import { seoDataForReviewsPage } from '~/utils/seo/reviews-page';
import { seoDataForSearchLandingPage } from '~/utils/seo/search-landing-page';
import { seoDataForSearchResultsPage } from '~/utils/seo/search-results-page';
import { seoDataForEditorialShelfCollectionPage } from '~/utils/seo/editorial-shelf-collection-page';
import { seoDataForArcadeSeeAllPage } from '~/utils/seo/arcade-see-all-page';
import { seoDataForSeeAllPage } from '~/utils/seo/see-all-page';

export class SEO implements SEODependency {
    private locale: Locale;

    constructor(locale: Locale) {
        this.locale = locale;
    }

    private get i18n() {
        if (this.locale.i18n) {
            return this.locale.i18n;
        }

        throw new Error('`i18n` not yet configured ');
    }

    private getSEODataForGenericPage(page: GenericPage): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
        };
    }

    updateCanonicalURL(page: WebRenderablePage, canonicalURL: string): void {
        updateCanonicalURL(page, canonicalURL);
    }

    /// MARK: Page SEO Data Hooks

    getSEODataForAppEventPage(
        objectGraph: AppStoreObjectGraph,
        page: GenericPage,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
            ...seoDataForAppEventDetailPage(
                page,
                this.i18n,
                objectGraph.locale.activeLanguage,
            ),
        };
    }

    getSEODataForArcadeSeeAllPage(
        _objectGraph: AppStoreObjectGraph,
        page: ArcadeSeeAllGamesPage,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
            ...seoDataForArcadeSeeAllPage(page, this.i18n),
        };
    }

    getSEODataForArticlePage(
        objectGraph: AppStoreObjectGraph,
        page: ArticlePage,
        response: Opt<DataContainer>,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
            ...seoDataForArticlePage(
                objectGraph,
                this.i18n,
                page,
                response,
                objectGraph.locale.activeLanguage,
            ),
        };
    }

    getSEODataForBundlePage(
        objectGraph: AppStoreObjectGraph,
        page: ShelfBasedProductPage,
        data: Opt<DataContainer>,
    ): Opt<SeoData> {
        return this.getSEODataForProductPage(objectGraph, page, data);
    }

    getSEODataForChartsPage(
        objectGraph: AppStoreObjectGraph,
        page: TopChartsPage,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
            ...seoDataForChartsPage(
                page,
                this.i18n,
                objectGraph.locale.activeLanguage,
            ),
        };
    }

    getSEODataForChartsHubPage(
        objectGraph: AppStoreObjectGraph,
        page: ChartsHubPage,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
            ...seoDataForChartsHubPage(
                page,
                this.i18n,
                objectGraph.locale.activeLanguage,
            ),
        };
    }

    getSEODataForDeveloperPage(
        objectGraph: AppStoreObjectGraph,
        page: GenericPage,
        response: Opt<DataContainer>,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
            ...seoDataForDeveloperPage(objectGraph, response, this.i18n),
        };
    }

    getSEODataForEditorialPage(
        _objectGraph: AppStoreObjectGraph,
        page: GenericPage,
    ): Opt<SeoData> {
        return this.getSEODataForGenericPage(page);
    }

    getSEODataForEditorialShelfCollectionPage(
        _objectGraph: AppStoreObjectGraph,
        page: GenericPage,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
            ...seoDataForEditorialShelfCollectionPage(page, this.i18n),
        };
    }

    getSEODataForGroupingPage(
        _objectGraph: AppStoreObjectGraph,
        page: GenericPage,
    ): Opt<SeoData> {
        return this.getSEODataForGenericPage(page);
    }

    getSEODataForProductPage(
        objectGraph: AppStoreObjectGraph,
        page: ShelfBasedProductPage,
        data: Opt<DataContainer>,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
            ...seoDataForProductPage(
                objectGraph,
                page,
                data,
                this.i18n,
                objectGraph.locale.activeLanguage,
            ),
        };
    }

    getSEODataForReviewsPage(
        objectGraph: AppStoreObjectGraph,
        page: ReviewsPage,
        productPage: ShelfBasedProductPage,
    ): Opt<SeoData> {
        return {
            ...this.getSEODataForGenericPage(page),
            ...seoDataForReviewsPage(this.i18n, page, productPage, objectGraph),
        };
    }

    getSEODataForRoomPage(
        _objectGraph: AppStoreObjectGraph,
        page: GenericPage,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
        };
    }

    getSEODataForSearchLandingPage(
        _objectGraph: AppStoreObjectGraph,
        page: SearchLandingPage,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
            ...seoDataForSearchLandingPage(page, this.i18n),
        };
    }

    getSEODataForSearchResultsPage(
        objectGraph: AppStoreObjectGraph,
        page: SearchResultsPage,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
            ...seoDataForSearchResultsPage(
                page,
                this.i18n,
                objectGraph.locale.activeLanguage,
            ),
        };
    }

    getSEODataForTodayPage(
        _objectGraph: AppStoreObjectGraph,
        page: TodayPage,
    ): Opt<SeoData> {
        return seoDataForAnyPage(page, this.i18n);
    }

    getSEODataForSeeAllPage(
        _objectGraph: AppStoreObjectGraph,
        page: SeeAllPage,
    ): Opt<SeoData> {
        return {
            ...seoDataForAnyPage(page, this.i18n),
            ...seoDataForSeeAllPage(page, this.i18n),
        };
    }
}
