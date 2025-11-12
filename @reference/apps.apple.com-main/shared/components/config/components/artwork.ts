// default params used by artwork component.
import type { Profile } from '@amp/web-app-components/src/components/Artwork/types';
import type { Breakpoints } from '@amp/web-app-components/src/types';
import { ASPECT_RATIOS } from '@amp/web-app-components/src/components/Artwork/constants';

export type ArtworkProfileMap<ProfileName extends string = string> = Map<
    ProfileName,
    Profile
>;
export interface ArtworkConfigOptions {
    BREAKPOINTS?: Breakpoints;
    PROFILES?: ArtworkProfileMap;
}

interface ArtworkConfig {
    get: () => ArtworkConfigOptions;
    set: (obj: ArtworkConfigOptions) => void;
}

function artworkConfig(): ArtworkConfig {
    const {
        HD,
        ONE,
        HERO,
        THREE_QUARTERS,
        SUPER_HERO_WIDE,
        UBER,
        ONE_THIRD,
        HD_ASPECT_RATIO,
        EDITORIAL_DEFAULT,
    } = ASPECT_RATIOS;
    let config: ArtworkConfigOptions = {
        BREAKPOINTS: {
            xsmall: {
                max: 739,
            },
            small: {
                min: 740,
                max: 999,
            },
            medium: {
                min: 1000,
                max: 1319,
            },
            large: {
                min: 1320,
                max: 1679,
            },
            xlarge: {
                min: 1680,
            },
        },
        PROFILES: new Map([
            ['brick', [[340, 340, 290, 290], HD, 'sr']],
            ['brick-sporting-event', [[340, 340, 290, 290], HD, 'sh']],
            ['product', [[500, 500, 300, 270], ONE, 'bb']],
            ['episode', [[330, 330, 305, 295], HD, 'sr']],
            [
                'editorial-card',
                [[530, 530, 480, 300, 300], EDITORIAL_DEFAULT, 'fa'],
            ],
            ['editorial-card-cover-artwork', [[60], ONE, 'cc']],
            ['editorial-card-video-art', [[88], HD_ASPECT_RATIO, 'mv']],
            ['hero', [[530, 530, 600, 450], HERO, 'sr']],
            ['superHeroLockup', [[330, 330, 305, 295], THREE_QUARTERS, 'bb']],
            ['superHeroTall', [[600, 600, 450], THREE_QUARTERS, 'sr']],
            [
                'superHeroWide',
                [[1200, 1200, 900, 600, 450], SUPER_HERO_WIDE, 'sr'],
            ],
            ['uber', [[1200], UBER, 'bb']],
            ['episode-lockup', [[316, 316, 296, 296], ONE, 'cc']],
            ['upsell-artwork', [[94], ONE, 'cc']],
            ['upsell-wordmark', [[140], 140 / 14, 'bb']],
            ['ellipse-lockup', [[243, 243, 220, 190, 160], ONE, 'cc']],
            ['standard', [[243, 243, 220, 190, 160], ONE, 'bb']],
            ['powerswoosh', [[300], ONE, 'cc']],
            ['powerswooshTall', [[600, 450], THREE_QUARTERS, 'sr']],
            ['category-brick', [[1040, 1040, 1040, 680], ONE_THIRD, 'sr']],
            ['info-fullscreen', [[600, 600, 450], ONE, 'bb']],
            ['track-list', [[40], ONE, 'bb']],
        ]),
    };

    const setConfig = (obj: ArtworkConfigOptions) => {
        config = {
            PROFILES: new Map([...config.PROFILES, ...obj.PROFILES]),
            BREAKPOINTS: {
                ...config.BREAKPOINTS,
                ...(obj?.BREAKPOINTS ?? {}),
            },
        };
    };

    const getConfig = (): ArtworkConfigOptions => config;

    return {
        get: getConfig,
        set: setConfig,
    };
}

export const ArtworkConfig = artworkConfig();
