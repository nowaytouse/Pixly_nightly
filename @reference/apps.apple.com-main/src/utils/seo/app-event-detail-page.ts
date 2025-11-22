import type { GenericPage } from '@jet-app/app-store/api/models';
import type I18N from '@amp/web-apps-localization';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';

import { isAppEventDetailShelf } from '~/components/jet/shelf/AppEventDetailShelf.svelte';
import { truncateAroundLimit } from '~/utils/string-formatting';
import { MAX_DESCRIPTION_LENGTH } from '~/utils/seo/common';

export function seoDataForAppEventDetailPage(
    page: GenericPage,
    i18n: I18N,
    language: string,
): SeoData {
    const appEventDetailShelf = page.shelves.find(isAppEventDetailShelf);

    const { appEvent } = appEventDetailShelf?.items[0] || {};

    if (!appEvent) {
        return {};
    }

    const title = appEvent.title;
    const description = truncateAroundLimit(
        appEvent.detail,
        MAX_DESCRIPTION_LENGTH,
        language,
    );

    return {
        pageTitle: title,
        socialTitle: title,
        appleTitle: title,
        description,
        socialDescription: description,
        appleDescription: description,
        crop: 'fo',
        twitterCropCode: 'fo',
        artworkUrl: appEvent?.moduleArtwork?.template,
        imageAltTitle: i18n.t('ASE.Web.AppStore.Meta.Image.AltText', {
            title: title,
        }),
    };
}
