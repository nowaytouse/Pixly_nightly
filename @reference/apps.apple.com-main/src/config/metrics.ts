import { BUILD } from './build';

const APP_NAME = 'com.apple.apps';
const APP_DELEGATE = 'web-appstore-app';

export const config = {
    baseFields: {
        appName: APP_NAME,
        delegateApp: APP_DELEGATE,
        appVersion: BUILD,
        resourceRevNum: BUILD,
    },
    clickstream: {
        constraintProfiles: ['AMPWeb'],
        topic: 'xp_amp_appstore_unidentified',
    },
};
