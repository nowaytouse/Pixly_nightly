// helper functions available for use at runtime
/**
 * @param {Region[]} regions - array of region objects that include region name and locales
 * @returns {StorefrontNameTranslations} - storefront ID (ie: us) mapped to that storefront name translated in all supported languages
 */
export function getFormattedStorefrontNameTranslations(regions) {
    return Object.fromEntries(
        regions.flatMap(({ locales }) => {
            const storefronts = {};
            for (const locale of locales) {
                if (!(locale.id in storefronts)) {
                    storefronts[locale.id] = { default: locale.name };
                }
                storefronts[locale.id][locale.language] = locale.name;
            }
            return Object.entries(storefronts);
        }),
    );
}
