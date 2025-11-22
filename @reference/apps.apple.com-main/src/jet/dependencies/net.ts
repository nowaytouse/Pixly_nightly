import type { Network, FetchRequest, FetchResponse } from '@jet/environment';
import { fromEntries } from '@amp/web-apps-utils';

import {
    shouldUseSearchJWT,
    makeSearchJWTAuthorizationHeader,
} from '~/config/media-api';

const CORRELATION_KEY_HEADER = 'x-apple-jingle-correlation-key';

type FetchFunction = typeof window.fetch;

// TODO: these URLs are also referenced in `bag` definition; we should have a single
// source-of-truth for these domains
const MEDIA_API_ORIGINS = [
    'https://amp-api.apps.apple.com',
    'https://amp-api-edge.apps.apple.com',
    'https://amp-api-search-edge.apps.apple.com',
];

export interface FeaturesCallbacks {
    getITFEValues(): string | undefined;
}

export class Net implements Network {
    private readonly underlyingFetch: FetchFunction;
    private readonly getITFEValues: () => string | undefined = () => undefined;

    constructor(
        underlyingFetch: FetchFunction,
        featuresCallbacks?: FeaturesCallbacks,
    ) {
        this.underlyingFetch = underlyingFetch;
        this.getITFEValues =
            featuresCallbacks?.getITFEValues ?? this.getITFEValues;
    }

    async fetch(request: FetchRequest): Promise<FetchResponse> {
        const requestStartTime = getTimestampMs();
        const requestURL = new URL(request.url);

        request.headers = request.headers ?? {};

        if (MEDIA_API_ORIGINS.includes(requestURL.origin)) {
            // Need to fake this for the server due to Kong origin checks.
            // Has no effect clientside.
            request.headers['origin'] = 'https://apps.apple.com';

            const itfe = this.getITFEValues?.();

            if (itfe) {
                // Add ITFE value as query string when set
                requestURL.searchParams.set('itfe', itfe);
            }
        }

        // The App Store Client will have already injected the JWT from the
        // `media-token-service` ObjectGraph dependency into the headers. However,
        // some endpoints need a different JWT. Here we determine if that's the
        // case and override the existing JWT if necessary.
        if (shouldUseSearchJWT(requestURL)) {
            request.headers = {
                ...request.headers,
                ...makeSearchJWTAuthorizationHeader(),
            };
        }

        // TODO: rdar://78158575: timeout
        const response = await this.underlyingFetch(requestURL.toString(), {
            ...request,
            cache: request.cache ?? undefined,
            credentials: 'include',
            headers: request.headers ?? undefined,
            method: request.method ?? undefined,
        });

        const responseStartTime = getTimestampMs();

        const { ok, redirected, status, statusText, url } = response;

        const headers = fromEntries(response.headers);
        const body = await response.text();

        const responseEndTime = getTimestampMs();

        return {
            ok,
            headers,
            redirected,
            status,
            statusText,
            url,
            body,
            // TODO: rdar://78158575: redirect: 'manual' to get all metrics?
            metrics: [
                {
                    clientCorrelationKey: response.headers.get(
                        CORRELATION_KEY_HEADER,
                    ),
                    pageURL: response.url,
                    requestStartTime,
                    responseStartTime,
                    responseEndTime,
                    // TODO: rdar://78158575: responseWasCached?
                    // TODO: rdar://78158575: parseStartTime/parseEndTime
                },
            ],
        };
    }
}

/**
 * Returns the current UTC timestamp in milliseconds.
 */
function getTimestampMs(): number {
    return Date.now();
}
