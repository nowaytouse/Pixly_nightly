import type {
    ReviewsPage,
    ShelfBasedProductPage,
} from '@jet-app/app-store/api/models';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';
import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import type I18N from '@amp/web-apps-localization';

import { truncateAroundLimit } from '~/utils/string-formatting';
import { MAX_DESCRIPTION_LENGTH } from '~/utils/seo/common';
import { isProductBadgeShelf } from '~/components/jet/shelf/ProductBadgeShelf.svelte';

export function seoDataForReviewsPage(
    i18n: I18N,
    page: ReviewsPage,
    productPage: ShelfBasedProductPage,
    objectGraph: AppStoreObjectGraph,
): SeoData {
    const appName = productPage.lockup.title;
    const artworkUrl = productPage.lockup.icon?.template;
    const badgeShelf = Object.values(productPage.shelfMapping).find(
        isProductBadgeShelf,
    );
    const developerName = badgeShelf?.items.find(
        ({ key }) => key === 'developer',
    )?.caption;

    const title = i18n.t('ASE.Web.AppStore.Meta.TitleWithSiteName', {
        title: i18n.t('ASE.Web.AppStore.Meta.Reviews.Title', {
            appName,
        }),
    });

    const descriptionLocKey = developerName
        ? 'ASE.Web.AppStore.Meta.Product.Description'
        : 'ASE.Web.AppStore.Meta.Product.DescriptionWithoutDeveloperName';

    const description = truncateAroundLimit(
        i18n.t(descriptionLocKey, {
            appName,
            developerName,
        }),
        MAX_DESCRIPTION_LENGTH,
        objectGraph.locale.activeLanguage,
    );

    return {
        artworkUrl,
        pageTitle: title,
        socialTitle: title,
        appleTitle: title,
        description,
        socialDescription: description,
        appleDescription: description,
    };
}
