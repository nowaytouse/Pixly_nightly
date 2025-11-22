/**
 * Defines a route based on a given default route and
 * otherwise falls back to the base storefront path
 *
 * @param defaultRoute - ie 'browse', 'listen-now', or empty string
 * @param storefront - storefront id ie 'us'
 * @param language - language tag ie 'en-US'
 * @returns route - ie /us/browse?l=es-MX
 */
export function getStorefrontRoute(
    defaultRoute: string,
    storefront: string,
    language?: string,
): string {
    let route;

    if (defaultRoute === '') {
        route = `/${storefront}`;
    } else {
        route = `/${storefront}/${defaultRoute}`;
    }

    // add optional language tag if that is passed in
    if (language) {
        route = `${route}?l=${language}`;
    }

    return route;
}
