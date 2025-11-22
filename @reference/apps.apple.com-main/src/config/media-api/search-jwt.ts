export function shouldUseSearchJWT(url: URL): boolean {
    // We should only ever use the "search" JWT on the server
    if (!import.meta.env.SSR) {
        return false;
    }

    // Search API Endpoint
    if (url.pathname.endsWith('/search')) {
        return true;
    }

    // All other endpoints should use the default JWT
    return false;
}

/**
 * Creates the `Authorization` header using the App Store "search JWT"
 *
 * Note: this function specifically returns a bad value for a "browser"
 * build so that the "search JWT" is removed from the browser payload
 * by dead-code elimination
 */
export function makeSearchJWTAuthorizationHeader() {
    return import.meta.env.SSR
        ? { Authorization: `Bearer ${import.meta.env.SEARCH_MEDIA_API_JWT}` }
        : { Authorization: '' };
}
