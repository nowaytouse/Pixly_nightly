import type { Locale } from './locale';

export class WebClient implements Client {
    private readonly locale: Locale;

    deviceType: DeviceType = 'web';

    // Tell the App Store Client that we're *really* the "web", even if the `DeviceType`
    // says otherwise
    __isReallyWebClient = true as const;

    // TODO: how do we define this for the "client" web, when it can change over time?
    screenSize: { width: number; height: number } = { width: 0, height: 0 };

    // TODO: how is this used? We can't have a consistent value across multiple sessions
    guid: string = 'xxx-xx-xxx';

    screenCornerRadius: number = 0;

    newPaymentMethodEnabled = false;

    isActivityAvailable = false;

    isElectrocardiogramInstallationAllowed = false;

    isScandiumInstallationAllowed = false;

    isSidepackingEnabled = false;

    isTinkerWatch = false;

    supportsHEIF: boolean = false;

    isMandrakeSupported: boolean = false;

    isCharonSupported: boolean = false;

    buildType: BuildType;

    maxAppContentRating: number = 1000;

    isIconArtworkCapable: boolean = true;

    constructor(buildType: BuildType, locale: Locale) {
        this.buildType = buildType;
        this.locale = locale;
    }

    get storefrontIdentifier(): string {
        return this.locale.activeStorefront;
    }

    deviceHasCapabilities(_capabilities: string[]): boolean {
        return false;
    }

    deviceHasCapabilitiesIncludingCompatibilityCheckIsVisionOSCompatibleIOSApp(
        _capabilities: string[],
        _supportsVisionOSCompatibleIOSBinary: boolean,
    ): boolean {
        return false;
    }

    isActivePairedWatchSystemVersionAtLeastMajorVersionMinorVersionPatchVersion(
        _majorVersion: number,
        _minorVersion: number,
        _patchVersion: number,
    ): boolean {
        return false;
    }

    canDevicePerformAppActionWithAppCapabilities(
        _appAction: string,
        _appCapabilities: string[] | undefined | null,
    ): boolean {
        return false;
    }

    isAutomaticDownloadingEnabled(): boolean {
        return false;
    }

    isAuthorizedForUserNotifications(): boolean {
        return false;
    }

    deletableSystemAppCanBeInstalledOnWatchWithBundleID(
        _bundleId: string,
    ): boolean {
        return false;
    }

    isDeviceEligibleForDomain(_domain: string): boolean {
        return false;
    }
}
