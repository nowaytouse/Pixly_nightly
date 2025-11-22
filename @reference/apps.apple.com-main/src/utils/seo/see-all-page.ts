import type I18N from '@amp/web-apps-localization';
import type { SeeAllPage } from '@jet-app/app-store/api/models';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';

export function seoDataForSeeAllPage(page: SeeAllPage, i18n: I18N): SeoData {
    let title = i18n.t('ASE.Web.AppStore.Meta.Product.Title');
    const shelfName = {
        reviews: 'productRatings',
        'customers-also-bought-apps': 'similarItems',
        'developer-other-apps': 'moreByDeveloper',
    }[page.seeAllType];

    if (shelfName) {
        const shelf = page.shelfMapping[shelfName];
        title = `${page.title} - ${shelf.title}`;
    }

    const titleWithSiteName = i18n.t(
        'ASE.Web.AppStore.Meta.TitleWithSiteName',
        { title },
    );

    const descriptionLocKey =
        {
            reviews: 'ASE.Web.AppStore.SeeAll.Reviews.Meta.Description',
            'customers-also-bought-apps':
                'ASE.Web.AppStore.SeeAll.CustomersAlsoBoughtApps.Meta.Description',
            'developer-other-apps':
                'ASE.Web.AppStore.SeeAll.DeveloperOtherApps.Meta.Description',
        }[page.seeAllType] ||
        'ASE.Web.AppStore.Meta.Product.DescriptionWithoutDeveloperName';
    const description = i18n.t(descriptionLocKey, {
        appName: page.title,
    });

    const artworkUrl = page.lockup.icon?.template;

    return {
        pageTitle: titleWithSiteName,
        socialTitle: titleWithSiteName,
        appleTitle: titleWithSiteName,
        description,
        socialDescription: description,
        appleDescription: description,
        artworkUrl,
    };
}
