/**
 * Removes the protcol, host and port from a URL, returning
 * just the path and search portions
 *
 * This is useful for taking a URL that points to the production site
 * and removing anything specific to the location that it is deployed,
 * creating a partial URL that works both locally or when deployed
 */
export function stripHost(input: string): string {
    const url = new URL(input);

    return url.pathname + url.search;
}
