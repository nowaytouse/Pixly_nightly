import type { Offer, SoftwareApplication, WithContext } from 'schema-dts';

import {
    type Opt,
    unwrapOptional as unwrap,
} from '@jet/environment/types/optional';
import type { ShelfBasedProductPage } from '@jet-app/app-store/api/models';
import type { PreviewPlatform } from '@jet-app/app-store/api/models/preview-platform';
import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import {
    type AttributePlatform,
    type Data,
    type DataContainer,
    dataFromDataContainer,
} from '@jet-app/app-store/foundation/media/data-structure';
import {
    attributeAsArrayOrEmpty,
    attributeAsDictionary,
    attributeAsNumber,
    attributeAsString,
} from '@jet-app/app-store/foundation/media/attributes';
import {
    platformAttributeAsBooleanOrFalse,
    platformAttributeAsDictionary,
    platformAttributeAsString,
} from '@jet-app/app-store/foundation/media/platform-attributes';
import {
    relationship,
    relationshipCollection,
} from '@jet-app/app-store/foundation/media/relationships';
import {
    asString,
    asNumber,
} from '@jet-app/app-store/foundation/json-parsing/server-data';
import { bestAttributePlatformFromData } from '@jet-app/app-store/common/content/attributes';
import { offerDataFromData } from '@jet-app/app-store/common/offers/offers';

import type I18N from '@amp/web-apps-localization';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';
import type { CropCode } from '@amp/web-app-components/src/components/Artwork/types';

import { basicDeveloperSchema } from './developer-page';
import { buildOpenGraphImageURL, buildImageURL } from './image-url';
import { truncateAroundLimit } from '~/utils/string-formatting';
import { MAX_DESCRIPTION_LENGTH } from '~/utils/seo/common';
import { isProductBadgeShelf } from '~/components/jet/shelf/ProductBadgeShelf.svelte';

/// MARK: Primary Image

/**
 * Determine if the data for a product represents an app that **only** supports iMessage
 */
function isMessagesOnly(data: Data, attributePlatform: AttributePlatform) {
    const hasMessagesExtension = platformAttributeAsBooleanOrFalse(
        data,
        attributePlatform,
        'hasMessagesExtension',
    );
    const isHiddenFromSpringboard = platformAttributeAsBooleanOrFalse(
        data,
        attributePlatform,
        'isHiddenFromSpringboard',
    );

    return hasMessagesExtension && isHiddenFromSpringboard;
}

function buildProductArtworkImage(
    data: Data,
    attributePlatform: AttributePlatform,
) {
    let iconCropCode: CropCode | undefined = undefined;

    if (isMessagesOnly(data, attributePlatform)) {
        iconCropCode = 'wb';
    }

    const deviceFamilies = attributeAsArrayOrEmpty(data, 'deviceFamilies');
    const hasIOSApp = deviceFamilies.includes('iphone');

    if (hasIOSApp) {
        iconCropCode = 'wa';
    }

    const artworkDefinition =
        platformAttributeAsDictionary(data, attributePlatform, 'artwork') ??
        attributeAsDictionary(data, 'artwork');

    return buildOpenGraphImageURL(artworkDefinition, iconCropCode);
}

/// MARK: Screenshots

const PREFERRED_SCREENSHOT_TYPE_BY_PLATFORM: Record<PreviewPlatform, string[]> =
    {
        iphone: [
            'iphone_d74',
            'iphone_d73',
            'iphone_6_5',
            'iphone_5_8',
            'iphone6+',
            'iphone6',
            'iphone5',
            'iphone',
        ],
        ipad: ['ipadPro_2018', 'ipad_11', 'ipad', 'ipad_10_5', 'ipadPro'],
        watch: [
            'appleWatch_2024',
            'appleWatch_2022',
            'appleWatch_2021',
            'appleWatch_2018',
            'appleWatch',
        ],
        tv: ['appletv', 'appleTV'],
        mac: [],
        vision: [],
    };

function buildProductScreenshots(
    data: Data,
    attributePlatform: AttributePlatform,
    previewPlatform: PreviewPlatform,
) {
    const screenshotsByType = platformAttributeAsDictionary(
        data,
        attributePlatform,
        'screenshotsByType',
    );
    if (!screenshotsByType) {
        return undefined;
    }

    const preferredScreenshotType = PREFERRED_SCREENSHOT_TYPE_BY_PLATFORM[
        previewPlatform
    ]?.find((preferredType) => preferredType in screenshotsByType);
    if (!preferredScreenshotType) {
        return undefined;
    }

    const screenshotArtworkDefinitions = screenshotsByType[
        preferredScreenshotType
    ] as Array<MapLike<JSONValue>>;

    return screenshotArtworkDefinitions
        .map((screenshotArtworkDefinition) =>
            buildImageURL(screenshotArtworkDefinition),
        )
        .filter((screenshot) => typeof screenshot !== 'undefined');
}

function buildOffer(
    objectGraph: AppStoreObjectGraph,
    data: Data,
    attributePlatform: AttributePlatform,
): Offer | undefined {
    const offer = offerDataFromData(objectGraph, data, attributePlatform);
    if (!offer) {
        return undefined;
    }

    const price = asNumber(offer, 'price') ?? undefined;
    const priceCurrency = asString(offer, 'currencyCode') ?? undefined;
    const category = !price || price === 0 ? 'free' : undefined;

    return {
        '@type': 'Offer',
        price,
        priceCurrency,
        category,
    };
}

function buildAvailableDevices(data: Data): string | undefined {
    const deviceFamilies = attributeAsArrayOrEmpty(data, 'deviceFamilies');
    if (!deviceFamilies) {
        return undefined;
    }

    return deviceFamilies
        .filter((device) => typeof device === 'string')
        .map((device) => {
            if (device === 'mac') {
                return 'Mac';
            } else if (device.indexOf('ip') === 0) {
                return device.replace(/^.{2}/g, 'iP');
            } else if (device === 'tvos') {
                return 'Apple TV';
            } else if (device === 'watch') {
                return 'Apple Watch';
            }

            return undefined;
        })
        .filter((device) => !!device)
        .join(', ');
}

/**
 * Produces a minimal {@linkcode SoftwareApplication} definition from a Media API `app` response
 *
 * Appropriate for embedding within another schema
 */
export function basicSoftwareApplicationSchema(
    objectGraph: AppStoreObjectGraph,
    data: Data,
) {
    const allGenreData = relationshipCollection(data, 'genres');
    const firstGenreData = (allGenreData && allGenreData[0]) ?? undefined;

    const attributePlatformFromData: Opt<AttributePlatform> =
        bestAttributePlatformFromData(objectGraph, data);

    if (!attributePlatformFromData) {
        return null;
    }

    const attributePlatform = unwrap(attributePlatformFromData);

    return {
        '@type': 'SoftwareApplication',

        name: attributeAsString(data, 'name') ?? undefined,
        description:
            platformAttributeAsString(
                data,
                attributePlatform,
                'description.standard',
            ) ?? undefined,
        image: buildProductArtworkImage(data, attributePlatform),
        availableOnDevice: buildAvailableDevices(data),
        operatingSystem:
            platformAttributeAsString(
                data,
                attributePlatform,
                'requirementsString',
            ) ?? undefined,
        offers: buildOffer(objectGraph, data, attributePlatform),
        applicationCategory: firstGenreData
            ? attributeAsString(firstGenreData, 'name') ?? undefined
            : undefined,

        aggregateRating: {
            '@type': 'AggregateRating',
            ratingValue:
                attributeAsNumber(data, 'userRating.value') ?? undefined,
            reviewCount:
                attributeAsNumber(data, 'userRating.ratingCount') ?? undefined,
        },
    } satisfies SoftwareApplication;
}

/// MARK: Schema Definition

function softwareApplicationSchemaSeoData(
    objectGraph: AppStoreObjectGraph,
    container: Opt<DataContainer>,
): Opt<Partial<SeoData>> {
    if (!container) {
        return null;
    }

    const productPageData = dataFromDataContainer(objectGraph, container);
    if (!productPageData) {
        return null;
    }

    const developerDataContainer = relationship(productPageData, 'developer');
    const developerData = dataFromDataContainer(
        objectGraph,
        developerDataContainer,
    );

    const attributePlatform = unwrap(
        bestAttributePlatformFromData(objectGraph, productPageData),
    );

    const schemaContent: WithContext<SoftwareApplication> = {
        '@context': 'https://schema.org',

        ...basicSoftwareApplicationSchema(objectGraph, productPageData),

        author: developerData ? basicDeveloperSchema(developerData) : undefined,
        screenshot: buildProductScreenshots(
            productPageData,
            attributePlatform,
            unwrap(objectGraph.activeIntent?.previewPlatform),
        ),
    };

    return {
        schemaName: 'software-application',
        schemaContent,
    };
}

export function seoDataForProductPage(
    objectGraph: AppStoreObjectGraph,
    page: ShelfBasedProductPage,
    data: Opt<DataContainer>,
    i18n: I18N,
    language: string,
): SeoData {
    const artworkUrl = page.lockup.icon?.template;
    const badgeShelf = Object.values(page.shelfMapping).find(
        isProductBadgeShelf,
    );
    const developerName = badgeShelf?.items.find(
        ({ key }) => key === 'developer',
    )?.caption;

    const title = i18n.t('ASE.Web.AppStore.Meta.TitleWithSiteName', {
        title: i18n.t('ASE.Web.AppStore.Meta.Product.Title', {
            appName: page.lockup.title,
        }),
    });

    const descriptionLocKey = developerName
        ? 'ASE.Web.AppStore.Meta.Product.Description'
        : 'ASE.Web.AppStore.Meta.Product.DescriptionWithoutDeveloperName';

    const description = truncateAroundLimit(
        i18n.t(descriptionLocKey, {
            appName: page.lockup.title,
            developerName,
        }),
        MAX_DESCRIPTION_LENGTH,
        language,
    );

    // Removes all query parameters (including `platform=*`) to form the canonical version
    // of the URL for the `link rel="canonical"` tag.
    let url = page.canonicalURL;
    if (url) {
        const cleanCanonicalUrl = new URL(url);
        cleanCanonicalUrl.search = '';
        url = cleanCanonicalUrl.toString();
    }

    return {
        pageTitle: title,
        socialTitle: title,
        appleTitle: title,
        canonicalUrl: url,
        artworkUrl,
        description,
        socialDescription: description,
        appleDescription: description,
        imageAltTitle: i18n.t('ASE.Web.AppStore.Meta.Image.AltText', {
            title: page.title,
        }),
        ...softwareApplicationSchemaSeoData(objectGraph, data),
    };
}
