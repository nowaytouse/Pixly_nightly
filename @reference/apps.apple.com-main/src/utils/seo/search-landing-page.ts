import type { SearchResultsPage } from '@jet-app/app-store/api/models';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';
import type I18N from '@amp/web-apps-localization';

export function seoDataForSearchLandingPage(
    page: SearchResultsPage,
    i18n: I18N,
): SeoData {
    const title = i18n.t('ASE.Web.AppStore.Meta.TitleWithSiteName', {
        title: i18n.t('ASE.Web.AppStore.Meta.SearchLanding.Title'),
    });

    return {
        pageTitle: title,
        socialTitle: title,
        appleTitle: title,
    };
}
