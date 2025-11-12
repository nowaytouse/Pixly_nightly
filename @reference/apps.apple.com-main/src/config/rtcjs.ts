import { platform } from '@amp/web-apps-utils';
import { HLSJS_CDN, HLSJS_VERSION } from './hlsjs';

declare global {
    interface Window {
        rtc?: any;
    }
}

export type ReportingOptions = {
    storeBagURL: string;
    clientName: string;
    serviceName: string;
    applicationName: string;
    applicationVersion: string;
    browserName: string;
    browserMajorVersion: string;
    browserMinorVersion: string;
    osName: string;
    osVersion: string;
};

/**
 * Generate a URL for loading HLS.js.
 */
export function generateRTCJSURL(version?: string): URL {
    // FIXME: Add a local storage override for the HLS.js version
    version = version ?? HLSJS_VERSION;

    return new URL(`${HLSJS_CDN}/${version}/rtc.js/rtc.js`);
}

export function getRTCNamespace() {
    if (window.rtc === undefined) {
        throw new Error('Unable to load RTC library');
    }

    return window.rtc;
}

export function getReportingOptions(): ReportingOptions {
    // FIXME: Add correct information for RTC reporting for Web App Store
    return {
        storeBagURL:
            'https://mediaservices.cdn-apple.com/store_bags/hlsjs/aasw/v1/rtc_storebag.json',

        // Application
        clientName: 'AASW',
        serviceName: 'com.apple.apps.external',
        applicationName: 'AppleAppStoreVWeb',
        applicationVersion: 'WebAppStore/1.0.0',

        // Browser
        browserName: platform.clientName() ?? '',
        browserMajorVersion: platform.majorVersion()?.toString() ?? '0',
        browserMinorVersion: platform.minorVersion()?.toString() ?? '0',

        // Operating System
        osName: platform.osName() ?? '',
        osVersion: platform.osName() ?? '',
    } as const;
}

/**
 * Generate the configuration used for an `RTCReportingAgent`.
 *
 * @see {@link makeReportingAgent}
 */
export function generateReportingConfig(rtc: any) {
    rtc = rtc ?? getRTCNamespace();
    const options = getReportingOptions();
    const key = rtc.RTCReportingAgentConfigKeys;

    return {
        [key.Sender]: 'HLSJS',
        [key.ClientName]: options.clientName,
        [key.ServiceName]: options.serviceName,
        [key.ApplicationName]: options.applicationName,
        [key.DeviceName]: options.osVersion,
        [key.ReportingStoreBag]: new rtc.RTCStorebag.RTCReportingStoreBag(
            options.storeBagURL,
            options.clientName,
            options.serviceName,
            options.applicationName,
            options.browserName,
            { iTunesAppVersion: options.applicationVersion },
        ),

        // Fake out these fields
        model: options.browserName,
        firmwareVersion: `${options.browserMajorVersion}.${options.browserMinorVersion}`,
    };
}

/**
 * Create an `RTCReportingAgent` with default configuration from `generateReportingConfig`.
 *
 * The reporting agent can be used with HLS.js playback to enable RTC reporting.
 */
export function makeReportingAgent(rtc: any): any {
    rtc = rtc ?? getRTCNamespace();
    return new rtc.RTCReportingAgent(generateReportingConfig(rtc));
}
