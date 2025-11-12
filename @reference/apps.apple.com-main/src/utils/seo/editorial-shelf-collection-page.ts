import type I18N from '@amp/web-apps-localization';
import type { GenericPage } from '@jet-app/app-store/api/models';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';
import { isPageHeaderShelf } from '~/components/jet/shelf/PageHeaderShelf.svelte';
import { getPlatformFromPage } from '~/utils/seo/common';
import { commaSeparatedList } from '../string-formatting';

export function seoDataForEditorialShelfCollectionPage(
    page: GenericPage,
    i18n: I18N,
): SeoData {
    let title = page.title;
    let description;
    const headerShelf = page.shelves.find(isPageHeaderShelf);

    if (headerShelf) {
        title = headerShelf.items[0].title;
        description = headerShelf.items[0].subtitle;
    }

    if (!description) {
        const platform = getPlatformFromPage(page);
        const titles = page.shelves
            .filter((shelf) => !isPageHeaderShelf(shelf))
            .flatMap(({ items }) => items)
            .slice(0, 3)
            .map((item) => item.title);

        description = i18n.t(
            'ASE.Web.AppStore.Meta.EditorialShelfCollection.Description',
            {
                platform,
                listOfApps: commaSeparatedList(titles),
            },
        );
    }

    const titleWithSiteName = i18n.t(
        'ASE.Web.AppStore.Meta.TitleWithSiteName',
        { title },
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
