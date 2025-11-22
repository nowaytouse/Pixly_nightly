import {
    parseUserAgent,
    flagsExtension,
    compareExtension,
} from '@amp/runtime-detect';
import { launchClient, type LaunchCallback } from './launch/launch-client';

type NavigatorLike = {
    userAgent: string;
    maxTouchPoints?: number;
};

/**
 * Detect a Platform descriptor from the browsers user agent.
 */
function detectDescriptor(options?: {
    window?: Window;
    navigator?: NavigatorLike;
}) {
    const defaultNavigator: NavigatorLike =
        typeof options?.window?.navigator !== 'undefined'
            ? options.window.navigator
            : {
                  userAgent: '',
                  maxTouchPoints: 0,
              };

    return parseUserAgent(options?.navigator ?? defaultNavigator, {
        extensions: [flagsExtension, compareExtension],
    });
}

export type PlatformDescriptor = ReturnType<typeof detectDescriptor>;

export class Platform {
    static detect(
        this: typeof Platform,
        options?: { window?: Window; navigator?: NavigatorLike },
    ) {
        const window = options?.window ?? globalThis?.window;
        return new this({
            window: window,
            descriptor: detectDescriptor({
                window: window,
                navigator: options?.navigator,
            }),
        });
    }

    /**
     * Descriptor from detecting platform data.
     */
    readonly descriptor: PlatformDescriptor;

    /**
     * Navigator value used to create the platform descriptor.
     */
    readonly navigator: NavigatorLike;

    /**
     * Reference to the platform Window object. This might be `undefined` in some
     * environments.
     */
    readonly window: Window | undefined;

    /**
     * User Agent string the platform descriptor was parsed from.
     */
    readonly ua: string;

    /**
     * Browser descriptor for the Platform.
     */
    readonly browser: PlatformDescriptor['browser'];

    /**
     * Browser Engine descriptor for the Platform.
     */
    readonly engine: PlatformDescriptor['engine'];

    /**
     * Operating System descriptor for the Platform.
     */
    readonly os: PlatformDescriptor['os'];

    constructor(config: {
        descriptor: PlatformDescriptor;
        window?: Window;
        navigator?: NavigatorLike;
    }) {
        const { descriptor } = config;
        this.descriptor = descriptor;
        this.navigator = config.navigator ?? descriptor.navigator;
        this.window = config.window;

        this.ua = descriptor.ua;
        this.browser = descriptor.browser;
        this.engine = descriptor.engine;
        this.os = descriptor.os;
    }

    /**
     * Check if Apple native applications can be opened on the Platform.
     */
    canOpenNative(): boolean {
        return this.ismacOS() || this.isiOS();
    }

    /**
     * Check if the Platform is running a mobile browser.
     */
    isMobile(): boolean {
        return this.browser.isMobile;
    }

    /**
     * Check if the Platform registers as running the Android operating system.
     */
    isAndroid(): boolean {
        return this.os.isAndroid;
    }

    /**
     * Check if the Platform registers as running the iOS operating system.
     */
    isiOS(): boolean {
        return this.os.isIOS;
    }

    /**
     * Check if the Platform registers as running the iPadOS operating system.
     */
    isiPadOS(): boolean {
        return this.os.isIPadOS;
    }

    /**
     * Check if the Platform registers as running the macOS operating system.
     */
    ismacOS(): boolean {
        return this.os.isMacOS;
    }

    /**
     * Check if the Platform registers as running the Windows operating system.
     */
    isWindows(): boolean {
        return this.os.isWindows;
    }

    /**
     * Check if the Platform registers as running a Linux operating system.
     */
    isLinux(): boolean {
        return this.os.isLinux;
    }

    /**
     * Check if the Platform is running the Apple Safari browser.
     */
    isSafari(): boolean {
        return this.browser.isSafari;
    }

    /**
     * Check if the Platform is running the Google Chrome browser.
     */
    isChrome(): boolean {
        return this.browser.isChrome;
    }

    /**
     * Check if the Platform is running the Mozilla Firefox browser.
     */
    isFirefox(): boolean {
        return this.browser.isFirefox;
    }

    /**
     * Check if the Platform is running the Microsoft Edge browser.
     */
    isEdge(): boolean {
        return this.browser.isEdge;
    }

    /**
     * Get name for the Platform browser.
     * @deprecated Use `platform.browser.name` directly
     */
    clientName(): string {
        return this.browser.name[0].toUpperCase() + this.browser.name.slice(1);
    }

    /**
     * Get the Platform browser major version number.
     * @deprecated Use `platform.browser.major` directly
     */
    majorVersion(): number {
        return this.browser.major ?? 0;
    }

    /**
     * Get the Platform browser minor version number.
     * @deprecated Use `platform.browser.minor` directly
     */
    minorVersion(): number {
        return this.browser.minor ?? 0;
    }

    /**
     * Get the name for the Platform operating system.
     * @deprecated Use `platform.os.name` directly
     */
    osName(): string {
        return this.os.name;
    }

    /**
     * Attempt to launch a native client for the given web URL.
     *
     * The callback is called with a report if the attempt was successful.
     *
     * @example
     * ```javascript
     * platform.launchClient(
     *   'https://music.apple.com/browse',
     *   function ({ link, success }) {
     *     if (success) {
     *       console.log(`Opened client with ${link}`);
     *     } else {
     *       console.log(`Failed to open client with ${link}`);
     *     }
     *   }
     * );
     * ```
     */
    launchClient(url: string, callback?: LaunchCallback): void {
        launchClient(url, this, callback);
    }

    /**
     * Check if the platform has full support for playing encrypted HLS content.
     */
    hasEncryptedPlaybackSupport(): boolean {
        return !this.os.isIOS || this.os.gte('17.5');
    }
}

export const platform = Platform.detect();
