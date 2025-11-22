/**
 * Remove the scheme and separators from the given URL.
 *
 * @example
 * ```javascript
 * removeScheme('https://music.apple.com/browse') // => 'music.apple.com/browse'
 * removeScheme('apple-music://music.apple.com/browse') // => 'music.apple.com/browse'
 * removeScheme('music.apple.com/browse') // => 'music.apple.com/browse'
 * ```
 */
export function removeScheme(
    url: string | URL | null | undefined,
): string | undefined {
    if (url === null || url === undefined) {
        return undefined;
    }

    return String(url).replace(/^((?:[^:]*:[/]{0,2})|(?::?\/\/))/i, '');
}

/**
 * Strip scheme and host (hostname + port) from a URL, leaving just the path, query
 * params, and hash.
 *
 * @param {string} url The URL possibly containing a host
 * @returns {string} hostlessUrl The url without its host
 */
export function removeHost(
    url: string | URL | null | undefined,
): string | undefined {
    return removeScheme(url)?.replace(/^([^/]*)/i, '');
}

/**
 * Strip query params and fragment from a URL.
 */
export function removeQueryParams(
    url: string | URL | undefined,
): string | undefined {
    if (url === undefined) {
        return undefined;
    }

    const value = String(url);
    const splitIndex = value.indexOf('?');
    return splitIndex >= 0 ? value.slice(0, splitIndex) : value;
}

export function getBaseUrl(): string {
    const currentUrl = new URL(window.location.href);
    return `${currentUrl.protocol}//${currentUrl.host}`;
}

export function buildUrl(props: {
    protocol?: string;
    hostname: string;
    pathname?: string | string[];
    queryParams?: string | Record<string, string>;
    hash?: string;
}): URL {
    const {
        hostname,
        pathname = '/',
        queryParams = {},
        protocol = 'https',
        hash = '',
    } = props;

    // Base URL with domain
    const url = new URL(protocol + '://' + removeScheme(hostname));

    // URL path
    url.pathname = Array.isArray(pathname)
        ? '/' + pathname.map(encodeURIComponent).join('/').replace(/[/]+/, '/')
        : pathname;

    // URL search (a.k.a. queryParams)
    if (typeof queryParams === 'string') {
        url.search = queryParams;
    } else {
        for (const [key, value] of Object.entries(queryParams)) {
            url.searchParams.set(key, value);
        }
    }

    // URL hash
    url.hash = hash;

    return url;
}
