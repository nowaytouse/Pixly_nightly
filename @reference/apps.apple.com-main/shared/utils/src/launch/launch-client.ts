import { createClientLink } from './scheme';
import type { Platform } from '../platform';

/**
 * Navigator for older Microsoft (MS) browsers like Internet Explorer.
 */
type MSNavigator = Navigator & {
    msLaunchUri: (
        href: string | URL,
        successCallback: () => void,
        failureCallback: () => void,
    ) => void;
};

/**
 * Check if the given value is an MSNavigator.
 */
function isMSNavigator(value: Partial<MSNavigator>): value is MSNavigator {
    return typeof value?.msLaunchUri === 'function';
}

/**
 * Callback for client launches.
 */
export type LaunchCallback = (result: {
    link: URL;
    success: boolean;
}) => void | Promise<void>;

/**
 * Attempt to launch the native client for the given Web URL.
 */
export function launchClient(
    url: string | URL,
    platform: Platform,
    callback: LaunchCallback = () => {},
): void {
    const { window, browser, os } = platform;

    /** URL for opening the native application */
    const link = createClientLink(url, { platform });

    // macOS Safari
    if (os.isMacOS && browser.isSafari) {
        launchOnMacOS(link, platform, callback);
    }
    // Proprietary msLaunchUri method (IE 10+ on Windows 8+)
    else if (isMSNavigator(platform.navigator)) {
        platform.navigator.msLaunchUri(
            String(link),
            () => callback({ link, success: true }),
            () => callback({ link, success: false }),
        );
    }
    // Other platforms
    else {
        try {
            // on iOS, Windows and Android simply opening the href works
            window!.top!.window.location.href = String(link);
            callback({ link, success: true });
        } catch (e) {
            // we know this is NOT installed
            callback({ link, success: false });
        }
    }
}

function launchOnMacOS(
    link: URL,
    platform: Platform,
    callback: LaunchCallback,
): void {
    const { window } = platform;

    if (typeof window === 'undefined') {
        callback({ link, success: false });
        return;
    }

    /** Timer for blur fallback */
    let timer: number;

    /** IFrame reference for opening the client link */
    let iframe: HTMLIFrameElement | undefined;

    /** Cleanup function run after the client launch has been initiated */
    function finalize() {
        clearTimeout(timer);
        window!.removeEventListener('blur', finalize);
        if (iframe !== undefined) {
            window!.document.body.removeChild(iframe);
        }

        callback({ link, success: true });
    }

    // Add an iFrame window to the current document to open the URL
    iframe = window.document.createElement('iframe');
    iframe.id = 'launch-client-opener';
    iframe.style.display = 'none';
    window.document.body.appendChild(iframe);

    // Redirect the iFrame to the client link to trigger it to open
    iframe.contentWindow!.location.href = String(link);

    // Wait a tiny amount of time for the client launch to appear
    window.addEventListener('blur', finalize);
    timer = setTimeout(finalize, 50) as unknown as number;
}
