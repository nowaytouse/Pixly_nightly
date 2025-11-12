import { platform } from '@amp/web-apps-utils';

const setupUrlForMac = (url: string) => {
    const incomingUrl = new URL(url);
    incomingUrl.searchParams.set('mt', '12');
    return incomingUrl.toString();
};

export const launchAppOnMac = (url: string) => {
    const appUrl = setupUrlForMac(url);

    platform.launchClient(appUrl, () => {});
};
