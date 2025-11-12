import { BUILD } from './build';
import type { ErrorKitConfig } from '@amp/web-apps-logger/src/errorkit';

const APPS_PROD_SUBDOMAIN = ['apps'];
const PROJECT_ID = 'onyx_apps';

const getSentryEnv = () => {
    const location =
        typeof window !== 'undefined' && window.location.host.split('.')[0];
    return APPS_PROD_SUBDOMAIN.includes(location) ? 'prod' : 'dev';
};

export const ERROR_KIT_CONFIG: ErrorKitConfig = {
    project: PROJECT_ID,
    environment: getSentryEnv(),
    release: BUILD,
};
