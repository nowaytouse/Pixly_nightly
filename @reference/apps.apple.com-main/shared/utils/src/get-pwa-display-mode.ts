export enum PWADisplayMode {
    TWA = 'twa',
    BROWSER = 'browser',
    STANDALONE = 'standalone',
    MINIMAL = 'minimal-ui',
    FULLSCREEN = 'fullscreen',
    OVERLAY = 'window-controls-overlay',
    UNKNOWN = 'unknown',
}

/**
 * For PWA, reads the "display" value from the manifest.json and returns the proper value if it matches.
 * Inspired by the sample snippet here: https://web.dev/learn/pwa/detection
 */
export const getPWADisplayMode = (): PWADisplayMode => {
    switch (true) {
        case document.referrer.startsWith('android-app://'):
            return PWADisplayMode.TWA;

        case window.matchMedia('(display-mode: browser)').matches:
            return PWADisplayMode.BROWSER;

        case window.matchMedia('(display-mode: standalone)').matches:
            return PWADisplayMode.STANDALONE;

        case window.matchMedia('(display-mode: minimal-ui)').matches:
            return PWADisplayMode.MINIMAL;

        case window.matchMedia('(display-mode: fullscreen)').matches:
            return PWADisplayMode.FULLSCREEN;

        case window.matchMedia('(display-mode: window-controls-overlay)')
            .matches:
            return PWADisplayMode.OVERLAY;

        default:
            return PWADisplayMode.UNKNOWN;
    }
};
