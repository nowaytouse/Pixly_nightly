import { removeScheme } from '..';
import { Platform } from '../platform';

/**
 * Check if the URL hostname matches the given value.
 */
const matchesHostName = (url: URL, hostName: string) =>
    url.hostname === hostName;

/**
 * Check if the URL `?app=xyz` search param matches the given value.
 */
const matchesAppName = (url: URL, appName: string) =>
    url.searchParams.get('app') === appName;

/**
 * Check if the URL `?mt=n` search param matches any of the given values.
 */
const matchesMediaType = (url: URL, mediaTypes: string[]) => {
    const mt = url.searchParams.get('mt');
    return mt ? mediaTypes.includes(mt) : false;
};

/**
 * Check if the URL pathname matches the given pattern.
 */
const matchesPathName = (url: URL, pattern: RegExp | string) =>
    new RegExp(pattern).test(url.pathname);

/**
 * Check if the URL is for Audiobooks
 */
const isAudiobookURL = (url: URL): boolean =>
    matchesAppName(url, 'audiobook') ||
    matchesMediaType(url, ['3']) ||
    matchesPathName(url, /\/(audiobook\/|viewAudiobook)/i);

/**
 * Check if the URL is for Books.
 */
const isBooksURL = (url: URL): boolean =>
    !isAudiobookURL(url) &&
    (matchesHostName(url, 'books.apple.com') ||
        matchesAppName(url, 'books') ||
        matchesMediaType(url, ['11', '13']) ||
        matchesPathName(url, '/book/'));

/**
 * Check if the URL is for Commerce.
 */
const isCommerceURL = (url: URL): boolean =>
    matchesHostName(url, 'finance-app.itunes.apple.com') ||
    matchesPathName(url, '/account/');

/**
 * Check if the URL is for a macOS App.
 */
const isMacAppURL = (url: URL): boolean =>
    matchesAppName(url, 'mac-app') ||
    matchesMediaType(url, ['12']) ||
    matchesPathName(url, '/mac-app/');

/**
 * Check if the URL is an AppStore Story.
 */
const isStoryURL = (url: URL): boolean =>
    matchesAppName(url, 'story') || matchesPathName(url, '/story/');

/**
 * Check if the URL is for Messages.
 */
const isMessagesURL = (url: URL): boolean => matchesAppName(url, 'messages');

/**
 * Check if the URL is for Music.
 */
const isMusicURL = (url: URL): boolean =>
    matchesHostName(url, 'music.apple.com') ||
    matchesAppName(url, 'music') ||
    matchesPathName(
        url,
        /\/(album|artist|playlist|station|curator|music-video)\//i,
    );

/**
 * Check if the URL is for Podcasts.
 */
const isPodcastsURL = (url: URL): boolean =>
    matchesHostName(url, 'podcasts.apple.com') ||
    matchesAppName(url, 'podcasts') ||
    matchesMediaType(url, ['2']) ||
    matchesPathName(url, '/podcast/');

/**
 * Check if the URL is for TV.
 */
const isTVURL = (url: URL): boolean =>
    matchesHostName(url, 'tv.apple.com') ||
    matchesPathName(
        url,
        /\/(episode|movie|movie-collection|show|season|sporting-event|person)\//i,
    );

/**
 * Check if the URL is for the Watch.
 */
const isWatchURL = (url: URL): boolean => matchesAppName(url, 'watch');

/**
 * Check if the URL is developer.apple.com related.
 */
const isDeveloperURL = (url: URL): boolean =>
    matchesAppName(url, 'developer') || matchesPathName(url, '/developer/');

/**
 * Check if the URL is for an app.
 */
const isAppsURL = (url: URL): boolean =>
    matchesMediaType(url, ['8']) && !isMessagesURL(url) && !isWatchURL(url);

/**
 * Function for identifying application schemes from web URLs.
 */
type SchemeIdentifier = (url: URL, platform: Platform) => boolean;

/**
 * List of schemes and functions to identify them based on a URL and Platform details.
 *
 * These schemes are derived from [Jingle Properties](https://github.pie.apple.com/amp-dev/Jingle/blob/6392929afb8540ac488315647992c3f46a9cc82f/MZConfig/Properties/apps/MZInit2/common.properties#L993).
 *
 * ```java
 * // <rdar://problem/66551318> iOS Bag: Move mobile-url-handlers to a property defined list
 * MZInit.iOS.acceptedUrlHandlers=("applenews", "applenewss", "applestore", "applestore-sec", "bridge", "com.apple.tv", "disneymoviesanywhere",\
 * "http", "https", "itms", "itmss", "itms-apps", "itms-appss", "itms-books", "itms-bookss", "itms-gc", "itms-gcs", "itms-itunesu",\
 * "itms-itunesus", "itms-podcast", "itms-podcasts", "itms-ui", "its-music", "its-musics", "its-news", "its-newss", "its-videos",\
 * "its-videoss", "itsradio", "livenation", "mailto", "message", "moviesanywhere", "music", "musics", "prefs", "shoebox")
 * ```
 */
const identifiers: [string, SchemeIdentifier, ...SchemeIdentifier[]][] = [
    [
        'itms-apps',
        (url, platform) =>
            platform.os.isIOS &&
            (isCommerceURL(url) ||
                isAppsURL(url) ||
                isStoryURL(url) ||
                isDeveloperURL(url)),
    ],

    // Watch app on mobile
    [
        'itms-watch',
        (url, platform) => platform.browser.isMobile && isWatchURL(url),
    ],

    // Messages app on mobile
    [
        'itms-messages',
        function (url: URL, platform: Platform) {
            return platform.browser.isMobile && isMessagesURL(url);
        },
    ],

    [
        'itms-books',
        (url, platform) =>
            platform.os.isMacOS &&
            platform.os.gte('10.15') &&
            isAudiobookURL(url),
        (url, _platform) => isBooksURL(url),
    ],

    // Music on Android
    [
        'apple-music',
        (url, platform) => platform.os.isAndroid && isMusicURL(url),
    ],

    // Music on iOS/macOS
    [
        'music',
        (url, platform) => platform.os.isIOS && isMusicURL(url),
        (url, platform) => {
            return (
                platform.os.isMacOS &&
                platform.os.gte('10.15') &&
                isMusicURL(url)
            );
        },
    ],

    // Podcasts on iOS
    [
        'itms-podcasts',
        (url, platform) => platform.os.isIOS && isPodcastsURL(url),
    ],

    // Podcasts on macOS
    [
        'podcasts',
        (url, platform) =>
            platform.os.isMacOS &&
            platform.os.gte('10.15') &&
            isPodcastsURL(url),
    ],

    // TV on iOS
    [
        'com.apple.tv',
        (url, platform) =>
            platform.os.isIOS && platform.os.gte('10.2') && isTVURL(url),
    ],

    // TV on macOS
    [
        'videos',
        (url: URL, platform: Platform) =>
            platform.os.isMacOS && platform.os.gte('10.15') && isTVURL(url),
    ],

    [
        'macappstore',
        (url, _platform) => isMacAppURL(url),
        (url, platform) =>
            platform.os.isMacOS &&
            platform.os.gte('10.15') &&
            isCommerceURL(url),

        // Story and developer pages should launch Mac App Store on Mojave(10.14)+
        // <rdar://problem/46461633> Story page with ls=1 QP should attempt to open Mac App Store on Mojave +
        // rdar://81291713 (Star: https://apps.apple.com/developer/id463855590?ls=1 launches Music App)
        (url, platform) =>
            platform.os.isMacOS &&
            platform.os.gte('10.14') &&
            (isStoryURL(url) || isDeveloperURL(url)),
    ],

    // Catch All
    ['itms', (_url, _platform) => true],
];

/**
 * Get the Scheme for attempting to open a platform native application.
 *
 * @see {@link https://en.wikipedia.org/wiki/Uniform_Resource_Identifier#Syntax}
 */
export function detectClientScheme(
    url: string | URL,
    options?: { platform?: Platform },
): string {
    url = new URL(url);

    // Assume that any URLs that don't have the http(s) scheme already have the
    // correct scheme assigned.
    if (/https?/i.test(url.protocol)) {
        const platform = options?.platform ?? Platform.detect();

        for (const [scheme, ...fns] of identifiers) {
            for (const fn of fns) {
                if (fn(url, platform)) {
                    return scheme;
                }
            }
        }
    }

    // At this point something should have matched. If not just return the original
    // scheme and have the browser or system handle it.
    return normalizeScheme(url.protocol);
}

/**
 * Check if the given URL has an Apple specific Scheme.
 *
 * @example
 * ```javascript
 * hasAppleClientScheme('music://music.apple.com/browse') // => true
 * hasAppleClientScheme('https://music.apple.com/browse') // => false
 * ```
 */
export function hasAppleClientScheme(
    url: URL | string,
    _options?: { platform?: Platform },
) {
    const pattern =
        /^(?:itms(?:-.*)?|macappstore|podcast|video|(?:apple-)?music)s?(:|$)/im;
    return pattern.test(new URL(url).protocol);
}

/**
 * Create a link for attempting to open a platform native application based on a web URL.
 *
 * @example
 * ```javascript
 * createClientLink('https://music.apple.com/browse');
 * // => 'music://music.apple.com/browse'
 * ```
 */
export function createClientLink(
    url: string | URL,
    options?: { platform?: Platform },
): URL {
    const link = new URL(url);

    // Removes any development prefixes in order to correctly identify the scheme
    link.host = link.host.replace(
        /^(?:[^-]+[-.])?([^.]+)\.apple\.com/,
        '$1.apple.com',
    );

    // Remove any port designation, this should not be present in application links
    link.port = '';

    const scheme = detectClientScheme(link, {
        platform: options?.platform,
    });

    // If the identified scheme is already assigned we want to leave the URL unmodified
    if (scheme === normalizeScheme(link.protocol)) {
        return new URL(url);
    }

    return new URL(scheme + '://' + removeScheme(link));
}

/**
 * Normalize a scheme value by removing any separators from it.
 *
 * @example
 * ```javascript
 * normalizeScheme('music') // => 'music'
 * normalizeScheme('TV') // => 'tv'
 * normalizeScheme('https:') // => 'https'
 * normalizeScheme('https://') // => 'https'
 * ```
 */
function normalizeScheme(value: string): string {
    return value.replace(/[:]+$/, '').toLowerCase();
}
