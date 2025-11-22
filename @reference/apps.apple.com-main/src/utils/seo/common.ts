import type { Opt } from '@jet/environment/types/optional';
import type { Organization } from 'schema-dts';
import type { WebRenderablePage } from '@jet-app/app-store/api/models/web-renderable-page';

import type I18N from '@amp/web-apps-localization';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';

export const MAX_DESCRIPTION_LENGTH = 160;

export const AppleOrganization: Organization = {
    '@type': 'Organization',
    name: 'Apple Inc',
    url: 'http://www.apple.com',
    logo: {
        '@type': 'ImageObject',
        url: 'https://www.apple.com/ac/structured-data/images/knowledge_graph_logo.png',
    },
};

export function updateCanonicalURL(
    page: WebRenderablePage,
    canonicalURL: string,
): void {
    const seoData = page.seoData as Opt<SeoData>;

    if (!seoData) {
        return;
    }

    seoData.url = canonicalURL;
}

export function seoDataForAnyPage(
    page: WebRenderablePage,
    i18n: I18N,
): SeoData {
    const pageTitle =
        'title' in page
            ? i18n.t('ASE.Web.AppStore.Meta.TitleWithPlatformAndSiteName', {
                  title: page.title,
                  platform: getPlatformFromPage(page),
              })
            : i18n.t('ASE.Web.AppStore.Meta.SiteName');

    const description = i18n.t('ASE.Web.AppStore.Meta.Description');

    return {
        url: page.canonicalURL ?? '',
        siteName: i18n.t('ASE.Web.AppStore.Meta.SiteName'),

        pageTitle,
        socialTitle: pageTitle,
        appleTitle: pageTitle,

        description,
        socialDescription: description,
        appleDescription: description,

        width: 1200,
        height: 630,
        twitterWidth: 1200,
        twitterHeight: 630,
        twitterCropCode: 'wa',
        crop: 'wa',
        fileType: 'jpg',
        artworkUrl: '/assets/images/share/app-store.png',

        twitterSite: '@AppStore',
    };
}

export function getPlatformFromPage(page: WebRenderablePage): Opt<string> {
    return page.webNavigation?.platforms.find((platform) => platform.isActive)
        ?.action.title;
}
