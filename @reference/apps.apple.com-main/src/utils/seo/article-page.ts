import type { Opt } from '@jet/environment/types/optional';
import type {
    Article,
    CollectionPage,
    CreativeWork,
    WithContext,
} from 'schema-dts';

import type { ArticlePage } from '@jet-app/app-store/api/models';
import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import {
    type DataContainer,
    type Data,
    dataFromDataContainer,
} from '@jet-app/app-store/foundation/media/data-structure';
import {
    attributeAsDictionary,
    attributeAsString,
} from '@jet-app/app-store/foundation/media/attributes';
import { relationshipCollection } from '@jet-app/app-store/foundation/media/relationships';

import type I18N from '@amp/web-apps-localization';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';
import type { CropCode } from '@amp/web-app-components/src/components/Artwork/types';

import { isSmallLockupShelf } from '~/components/jet/shelf/SmallLockupShelf.svelte';
import { isLockupOverlay } from '~/components/jet/today-card/TodayCardOverlay.svelte';
import { isLockupListOverlay } from '~/components/jet/today-card/overlay/TodayCardLockupListOverlay.svelte';
import { isTodayCardMediaWithArtwork } from '~/components/jet/today-card/media/TodayCardMediaWithArtwork.svelte';
import { isTodayCardMediaVideo } from '~/components/jet/today-card/media/TodayCardMediaVideo.svelte';
import { isTodayCardMediaRiver } from '~/components/jet/today-card/media/TodayCardMediaRiver.svelte';
import { isTodayCardMediaBrandedSingleApp } from '~/components/jet/today-card/media/TodayCardMediaBrandedSingleApp.svelte';
import { isTodayCardMediaAppEvent } from '~/components/jet/today-card/media/TodayCardMediaAppEvent.svelte';

import { AppleOrganization } from './common';
import { buildOpenGraphImageURL } from './image-url';
import { basicSoftwareApplicationSchema } from './product-page';
import { stripTags, truncateAroundLimit } from '~/utils/string-formatting';

/// MARK: Schema Data

/**
 * SEO-related props that have already been computed, and will be re-used within the schema
 */
interface SeoProps {
    title: string;
    description: string | undefined;
}

function commonSchemaForArticlePage(
    data: Data,
    { title, description }: SeoProps,
): WithContext<CreativeWork> {
    const artwork =
        attributeAsDictionary(
            data,
            'editorialArtwork.storyCenteredStatic16x9',
        ) ?? undefined;
    const lastPublishedDate =
        attributeAsString(data, 'lastPublishedDate') ?? undefined;

    return {
        '@type': 'CreativeWork',
        '@context': 'https://schema.org',

        description,
        headline: title,
        name: title,

        dateModified: lastPublishedDate,
        datePublished: lastPublishedDate,
        image: artwork ? buildOpenGraphImageURL(artwork) : undefined,

        author: AppleOrganization,
        publisher: AppleOrganization,
    };
}

function articleSchemaForArticlePage(
    objectGraph: AppStoreObjectGraph,
    data: Data,
): WithContext<Article> {
    const cardContents = relationshipCollection(data, 'card-contents') ?? [];
    const [app] = cardContents;

    return {
        '@context': 'https://schema.org',
        '@type': 'Article',

        mainEntityOfPage: app
            ? basicSoftwareApplicationSchema(objectGraph, app)
            : undefined,
    };
}

function collectionPageSchemaForArticlePage(
    objectGraph: AppStoreObjectGraph,
    data: Data,
): WithContext<CollectionPage> {
    const cardContents = relationshipCollection(data, 'card-contents') ?? [];

    return {
        '@context': 'https://schema.org',
        '@type': 'CollectionPage',

        mentions: cardContents.map((app) =>
            basicSoftwareApplicationSchema(objectGraph, app),
        ),
    };
}

/**
 *
 * @param objectGraph
 * @param response the API response for the Article page
 * @param props SEO-related props that have already been derrived for the page
 */
export function schemaDataForArticlePage(
    objectGraph: AppStoreObjectGraph,
    response: Opt<DataContainer>,
    props: SeoProps,
): Partial<SeoData> {
    if (!response) {
        return {};
    }

    const articleData = dataFromDataContainer(objectGraph, response);
    if (!articleData) {
        return {};
    }

    let schemaContent = commonSchemaForArticlePage(articleData, props);

    const kind = attributeAsString(articleData, 'kind');

    if (kind === 'Collection') {
        schemaContent = {
            ...schemaContent,
            ...collectionPageSchemaForArticlePage(objectGraph, articleData),
        };
    } else {
        schemaContent = {
            ...schemaContent,
            ...articleSchemaForArticlePage(objectGraph, articleData),
        };
    }

    return {
        schemaName: 'article-page',
        schemaContent,
    };
}

/// MARK: Full SEO Data

export function seoDataForArticlePage(
    objectGraph: AppStoreObjectGraph,
    i18n: I18N,
    page: ArticlePage,
    response: Opt<DataContainer>,
    language: string,
): SeoData {
    const { card } = page;

    if (!card) {
        return {};
    }

    const storyTitle = stripTags(card.title);
    const pageTitle = i18n.t('ASE.Web.AppStore.Meta.TitleWithSiteName', {
        title: storyTitle,
    });

    let artwork = '';
    let crop: CropCode = 'fo';
    let appNames = [];

    if (card.overlay && isLockupListOverlay(card.overlay)) {
        appNames = card.overlay.lockups.slice(0, 3).map((item) => item.title);
    } else {
        appNames = page.shelves
            .filter(isSmallLockupShelf)
            .flatMap((shelf) => shelf.items)
            .slice(0, 3)
            .map((item) => item.title);
    }

    const firstParagraphShelf = page.shelves.find(
        (shelf) => shelf.contentType === 'paragraph',
    );
    let description;

    // If an article has a paragraph shelf, we use that to populate the meta description,
    // otherwise, we build a list of app names for the description.
    if (page.shelves.length > 1 && firstParagraphShelf?.items) {
        // The article paragraphs can contain HTML tags, so we strip them out here
        const text = stripTags(firstParagraphShelf.items[0].text);

        const articleContent = truncateAroundLimit(text, 110, language);

        description = i18n.t(
            'ASE.Web.AppStore.Meta.Story.Description.WithArticleContent',
            { articleContent },
        );
    } else if (appNames.length === 1) {
        description = i18n.t('ASE.Web.AppStore.Meta.Story.Description.One', {
            storyTitle,
            featuredAppName: appNames[0],
        });
    } else if (appNames.length === 2) {
        description = i18n.t('ASE.Web.AppStore.Meta.Story.Description.Two', {
            storyTitle,
            featuredAppName: appNames[0],
            featuredAppName2: appNames[1],
        });
    } else if (appNames.length >= 3) {
        description = i18n.t('ASE.Web.AppStore.Meta.Story.Description.Three', {
            storyTitle,
            featuredAppName: appNames[0],
            featuredAppName2: appNames[1],
            featuredAppName3: appNames[2],
        });
    } else if (card.overlay && isLockupOverlay(card.overlay)) {
        const featuredAppName = card.overlay.lockup.title;

        description = i18n.t('ASE.Web.AppStore.Meta.Story.Description.One', {
            storyTitle,
            featuredAppName,
        });
    }

    if (card.media && isTodayCardMediaWithArtwork(card.media)) {
        artwork = card.media.artworks[0].template;
    } else if (card.media && isTodayCardMediaVideo(card.media)) {
        artwork = card.media.videos[0].preview.template;
    } else if (card.media && isTodayCardMediaRiver(card.media)) {
        artwork = card.media.lockups[0].icon.template;
        crop = 'wa';
    } else if (
        card.media &&
        (isTodayCardMediaBrandedSingleApp(card.media) ||
            isTodayCardMediaAppEvent(card.media))
    ) {
        if (card.media.artworks.length > 0) {
            artwork = card.media.artworks[0].template;
        } else if (card.media.videos.length > 0) {
            artwork = card.media.videos[0].preview.template;
        }
    }

    // We are setting the `link rel="canonical"` tag for iPad, Watch and TV story pages to point to
    // the iPhone page.
    let canonicalUrl = page.canonicalURL?.replace(
        /\/([a-z]{2})\/(ipad|watch|tv)\/story\//,
        '/$1/iphone/story/',
    );

    return {
        pageTitle,
        crop,
        canonicalUrl,
        socialTitle: pageTitle,
        description: description,
        socialDescription: description,
        appleDescription: description,
        artworkUrl: artwork,
        twitterCropCode: crop,
        imageAltTitle: i18n.t('ASE.Web.AppStore.Meta.Image.AltText', {
            title: storyTitle,
        }),
        ...schemaDataForArticlePage(objectGraph, response, {
            title: pageTitle,
            description,
        }),
    };
}
