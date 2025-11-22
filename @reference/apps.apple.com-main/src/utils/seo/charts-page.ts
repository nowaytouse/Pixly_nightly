import type { TopChartsPage, Lockup } from '@jet-app/app-store/api/models';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';
import type I18N from '@amp/web-apps-localization';
import { getPlatformFromPage } from '~/utils/seo/common';
import {
    commaSeparatedList,
    truncateAroundLimit,
} from '~/utils/string-formatting';

export function seoDataForChartsPage(
    page: TopChartsPage,
    i18n: I18N,
    language: string,
): SeoData {
    // Genre 36 and 6014 are the "All Apps" and "All Games" genres, which we do not want to
    // include in the page title, since it would then read as "Best All Games Apps - App Store".
    const category = page.categoriesButtonTitle;
    const isAllAppsOrGames = ['36', '6014'].includes(page.genreId);
    const titleLocKey =
        isAllAppsOrGames || !category
            ? 'ASE.Web.AppStore.Meta.ChartsHub.Title'
            : 'ASE.Web.AppStore.Meta.Charts.Title';
    const platform = getPlatformFromPage(page);

    const title = i18n.t(titleLocKey, {
        category,
        platform,
    });

    let description;
    const items = page.segments[0].shelves[0].items as Array<Lockup>;

    if (items) {
        const appTitles = items.map(({ title }) => title).slice(0, 3);
        const locKey =
            category && !isAllAppsOrGames
                ? 'ASE.Web.AppStore.Meta.Charts.Description'
                : 'ASE.Web.AppStore.Meta.Charts.DescriptionWithoutCategory';

        description = truncateAroundLimit(
            i18n.t(locKey, {
                category,
                platform,
                listOfApps: commaSeparatedList(appTitles, language),
            }),
            160,
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
