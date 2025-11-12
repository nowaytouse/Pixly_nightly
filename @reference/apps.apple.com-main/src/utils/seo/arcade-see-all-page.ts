import type I18N from '@amp/web-apps-localization';
import type { GenericPage } from '@jet-app/app-store/api/models';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';
import { isAppTrailerLockupShelf } from '~/components/jet/shelf/AppTrailerLockupShelf.svelte';

export function seoDataForArcadeSeeAllPage(
    page: GenericPage,
    i18n: I18N,
): SeoData {
    const titleWithSiteName = i18n.t(
        'ASE.Web.AppStore.Meta.TitleWithSiteName',
        {
            title: i18n.t('ASE.Web.AppStore.ArcadeSeeAll.Meta.Title'),
        },
    );

    const appNames = page.shelves
        .filter(isAppTrailerLockupShelf)
        .flatMap((shelf) => shelf.items)
        .slice(0, 3)
        .map((item) => item.title);

    const description = i18n.t(
        'ASE.Web.AppStore.ArcadeSeeAll.Meta.Description',
        {
            listing1: appNames[0],
            listing2: appNames[1],
            listing3: appNames[2],
        },
    );

    return {
        pageTitle: titleWithSiteName,
        socialTitle: titleWithSiteName,
        appleTitle: titleWithSiteName,
        description,
        socialDescription: description,
        appleDescription: description,
    };
}
