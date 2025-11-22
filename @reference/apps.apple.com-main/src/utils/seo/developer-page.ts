import {
    type Opt,
    unwrapOptional as unwrap,
} from '@jet/environment/types/optional';
import type { Organization, WithContext } from 'schema-dts';

import type { AppStoreObjectGraph } from '@jet-app/app-store/foundation/runtime/app-store-object-graph';
import {
    type Data,
    type DataContainer,
    dataFromDataContainer,
} from '@jet-app/app-store/foundation/media/data-structure';
import { attributeAsString } from '@jet-app/app-store/foundation/media/attributes';
import { relationshipCollection } from '@jet-app/app-store/foundation/media/relationships';

import type I18N from '@amp/web-apps-localization';
import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';

import { uniqueById } from '~/utils/array';
import { basicSoftwareApplicationSchema } from '~/utils/seo/product-page';

/**
 * Generate a basic {@linkcode Person} schema for a "developer" page
 *
 * Note: this is appropriate to be embedded into another schema that
 * needs to reference the developer
 */
export function basicDeveloperSchema(data: Data) {
    return {
        '@type': 'Organization',
        name: attributeAsString(data, 'name') ?? undefined,
        url: attributeAsString(data, 'url') ?? undefined,
    } satisfies Organization;
}

export function buildDeveloperDescription(
    props: {
        name: string;
    },
    appData: Data[],
    i18n: I18N,
) {
    const { name: developerName } = props;

    switch (appData.length) {
        case 0:
            return i18n.t(
                'ASE.Web.AppStore.Meta.Developer.Description.ZeroApps',
                {
                    developerName,
                },
            );
        case 1:
            return i18n.t(
                'ASE.Web.AppStore.Meta.Developer.Description.OneApp',
                {
                    developerName,
                    listing1: attributeAsString(appData[0], 'name'),
                },
            );
        case 2:
            return i18n.t(
                'ASE.Web.AppStore.Meta.Developer.Description.TwoApps',
                {
                    developerName,
                    listing1: attributeAsString(appData[0], 'name'),
                    listing2: attributeAsString(appData[1], 'name'),
                },
            );
        case 3:
            return i18n.t(
                'ASE.Web.AppStore.Meta.Developer.Description.ThreeApps',
                {
                    developerName,
                    listing1: attributeAsString(appData[0], 'name'),
                    listing2: attributeAsString(appData[1], 'name'),
                    listing3: attributeAsString(appData[2], 'name'),
                },
            );
        default:
            return i18n.t(
                'ASE.Web.AppStore.Meta.Developer.Description.ManyApps',
                {
                    developerName,
                    listing1: attributeAsString(appData[0], 'name'),
                    listing2: attributeAsString(appData[1], 'name'),
                    listing3: attributeAsString(appData[2], 'name'),
                },
            );
    }
}

/**
 * Builds the Schema.org meta-data for a "Developer" page
 *
 * @param objectGraph The Object Graph
 * @param developerPageData The `Data` for the Developer page
 * @param appData The `Data` for all apps related to the Developer apge
 * @param props Pre-formatted properties also used outside of the Schema
 * @returns
 */
function developerOrganizationSchemaSeoData(
    objectGraph: AppStoreObjectGraph,
    developerPageData: Data,
    appData: Data[],
    props: {
        description: string;
    },
): Opt<Partial<SeoData>> {
    const { description } = props;

    const schemaContent: WithContext<Organization> = {
        '@context': 'https://schema.org',

        ...basicDeveloperSchema(developerPageData),

        description,
        hasOfferCatalog: {
            '@type': 'OfferCatalog',
            itemListElement: appData.map((app) =>
                basicSoftwareApplicationSchema(objectGraph, app),
            ),
        },
    };

    return {
        schemaName: 'developer',
        schemaContent,
    };
}

/**
 * Builds the full `SeoData` requirements for a "Developer" page
 */
export function seoDataForDeveloperPage(
    objectGraph: AppStoreObjectGraph,
    container: Opt<DataContainer>,
    i18n: I18N,
): Partial<SeoData> {
    if (!container) {
        return {};
    }

    const developerPageData = dataFromDataContainer(objectGraph, container);
    if (!developerPageData) {
        return {};
    }

    const allApps = uniqueById([
        ...unwrap(relationshipCollection(developerPageData, 'atv-apps')),
        ...unwrap(relationshipCollection(developerPageData, 'app-bundles')),
        ...unwrap(relationshipCollection(developerPageData, 'imessage-apps')),
        ...unwrap(relationshipCollection(developerPageData, 'ios-apps')),
        ...unwrap(relationshipCollection(developerPageData, 'mac-apps')),
        ...unwrap(relationshipCollection(developerPageData, 'watch-apps')),
    ]);

    const name = unwrap(attributeAsString(developerPageData, 'name'));
    const description = buildDeveloperDescription({ name }, allApps, i18n);

    return {
        description,
        socialDescription: description,
        appleDescription: description,
        ...developerOrganizationSchemaSeoData(
            objectGraph,
            developerPageData,
            allApps,
            {
                description,
            },
        ),
    };
}
