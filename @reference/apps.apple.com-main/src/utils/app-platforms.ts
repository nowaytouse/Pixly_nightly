import type { AppPlatform } from '@jet-app/app-store/api/models';

export const PlatformToExclusivityText: Partial<Record<AppPlatform, string>> = {
    watch: 'ASE.Web.AppStore.App.OnlyForWatch',
    tv: 'ASE.Web.AppStore.App.OnlyForAppleTV',
    messages: 'ASE.Web.AppStore.App.OnlyForiMessage',
    mac: 'ASE.Web.AppStore.App.OnlyForMac',
    phone: 'ASE.Web.AppStore.App.OnlyForPhone',
};

export function isPlatformSupported(
    platform: AppPlatform,
    appPlatforms: AppPlatform[],
) {
    const dedupedPlatforms = new Set(appPlatforms);
    return dedupedPlatforms.has(platform);
}

export function isPlatformExclusivelySupported(
    platform: AppPlatform,
    appPlatforms: AppPlatform[],
) {
    const dedupedPlatforms = new Set(appPlatforms);
    return dedupedPlatforms.has(platform) && dedupedPlatforms.size === 1;
}
