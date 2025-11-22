import type { SearchResultsPage } from '@jet-app/app-store/api/models';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';
import type I18N from '@amp/web-apps-localization';
import {
    isSearchResultShelf,
    isRenderableInSearchResultsShelf,
} from '~/components/jet/shelf/SearchResultShelf.svelte';
import { commaSeparatedList } from '../string-formatting';

export function seoDataForSearchResultsPage(
    page: SearchResultsPage,
    i18n: I18N,
    language: string,
): SeoData {
    const term = page?.searchTermContext?.term;
    const pageTitle = i18n.t('ASE.Web.AppStore.Meta.TitleWithSiteName', {
        title: page?.searchTermContext?.term,
    });
    const shareTitle = i18n.t('ASE.Web.AppStore.Meta.TitleWithSiteName', {
        title: i18n.t('ASE.Web.AppStore.Meta.SearchResults.Title', {
            term: page?.searchTermContext?.term,
        }),
    });

    const resultsShelf = page?.shelves?.find(isSearchResultShelf) ?? null;

    const renderableItems = (resultsShelf?.items ?? []).filter(
        isRenderableInSearchResultsShelf,
    );

    const appNames = renderableItems
        .slice(0, 3)
        .map((item) => item.lockup.title);

    let description;
    if (appNames.length) {
        description = i18n.t(
            'ASE.Web.AppStore.Meta.SearchResults.Description',
            {
                term,
                listOfApps: commaSeparatedList(appNames, language),
            },
        );
    }

    return term
        ? {
              pageTitle,
              socialTitle: shareTitle,
              appleTitle: shareTitle,
              description,
              socialDescription: description,
              appleDescription: description,
          }
        : {};
}
