import {
    buildFeatureConfig,
    buildRuntimeFeatureKitConfig,
    ENVIRONMENT,
    loadFeatureKit,
    type OnyxFeatures,
} from '@amp/web-apps-featurekit';
import type { LoggerFactory } from '@amp/web-apps-logger';
import { BUILD } from '~/config/build';

export async function setupRuntimeFeatures(
    logger: LoggerFactory,
): Promise<OnyxFeatures | void> {
    // load featureKit only for internal builds
    if (import.meta.env.APP_SCOPE === 'internal' || import.meta.env.DEV) {
        const features = await import('./consts');

        // Build FeatureKit Config with overrides
        const config = buildRuntimeFeatureKitConfig(features, {
            [features.__FF_SHOW_RADAR]: buildFeatureConfig({
                [ENVIRONMENT.DEV]: true,
            }),
            [features.__FF_ARYA]: {
                ...buildFeatureConfig({ [ENVIRONMENT.DEV]: false }),
                itfe: ['y9ttlj15'],
            },
        });
        // Load runtime featureKit
        return loadFeatureKit(
            'com.apple.apps',
            ENVIRONMENT.DEV,
            config,
            logger,
            {
                enableToolbar: true,
                radarConfig: {
                    component: 'ASE Web',
                    app: 'App Store',
                    build: BUILD,
                },
            },
        );
    }
}
