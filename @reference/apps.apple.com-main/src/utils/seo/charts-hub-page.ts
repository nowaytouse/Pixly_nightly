import type { ChartsHubPage, Lockup } from '@jet-app/app-store/api/models';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';
import type I18N from '@amp/web-apps-localization';
import { getPlatformFromPage } from '~/utils/seo/common';
import { truncateAroundLimit } from '~/utils/string-formatting';

export function seoDataForChartsHubPage(
    page: ChartsHubPage,
    i18n: I18N,
    language: string,
): SeoData {
    const platform = getPlatformFromPage(page);
    const title = i18n.t('ASE.Web.AppStore.Meta.TitleWithSiteName', {
        title: i18n.t('ASE.Web.AppStore.Meta.ChartsHub.Title', {
            platform,
        }),
    });

    let description;
    const items = page.charts[0].segments[0].shelves[0].items as Array<Lockup>;

    if (items) {
        const appsTitles = items.map(({ title }) => title);

        description = truncateAroundLimit(
            i18n.t('ASE.Web.AppStore.Meta.ChartsHub.Description', {
                platform,
                listing1: appsTitles[0],
                listing2: appsTitles[1],
                listing3: appsTitles[2],
                listing4: appsTitles[3],
            }),
            160,
            language,
        );
    }

    return {
        pageTitle: title,
        socialTitle: title,
        appleTitle: title,
        description,
        socialDescription: description,
        appleDescription: description,
    };
}
