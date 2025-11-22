import { ASPECT_RATIOS } from '@amp/web-app-components/src/components/Artwork/constants';
import { ArtworkConfig } from '@amp/web-app-components/config/components/artwork';
import type { ArtworkProfileMap } from '@amp/web-app-components/config/components/artwork';
import type { CropCode } from '@amp/web-app-components/src/components/Artwork/types';

const { HD, ONE, THREE_QUARTERS, HD_ASPECT_RATIO } = ASPECT_RATIOS;

const AppIconSize = {
    DEFAULT: [48],
    SMALL: [64],
    MEDIUM: [100],
    LARGE: [200],
    XLARGE: [800],
};

const cardSizes = [525, 525, 490, 394, 738];
const brickSizes = [520, 400, 450, 450, 300];
const heroSizes = [1600, 1240, 920, 920, 490];

export type NamedProfile =
    | 'app-event-detail'
    | 'app-event-detail-small'
    | 'app-icon'
    | 'app-icon-large'
    | 'app-icon-medium'
    | 'app-icon-small'
    | 'app-icon-xlarge'
    | 'app-icon-pill'
    | 'app-icon-large-pill'
    | 'app-icon-medium-pill'
    | 'app-icon-small-pill'
    | 'app-icon-river-pill'
    | 'app-icon-tv-rect'
    | 'app-icon-large-tv-rect'
    | 'app-icon-xlarge-tv-rect'
    | 'app-icon-medium-tv-rect'
    | 'app-icon-small-tv-rect'
    | 'app-icon-river-tv-rect'
    | 'app-icon-river'
    | 'app-promotion'
    | 'app-promotion-in-article'
    | 'app-trailer-lockup-video'
    | 'brick'
    | 'brick-app-icon'
    | 'card'
    | 'card-horizontal'
    | 'category-brick'
    | 'editorial-story-card'
    | 'in-app-purchase'
    | 'large-brick'
    | 'large-hero'
    | 'large-hero-portrait'
    | 'large-hero-portrait-iphone'
    | 'large-hero-breakout'
    | 'large-hero-breakout-rtl'
    | 'large-hero-west'
    | 'large-hero-east'
    | 'large-hero-story-card'
    | 'large-hero-story-card-portrait'
    | 'large-hero-story-card-rtl'
    | 'large-image-lockup'
    | 'poster-lockup'
    | 'poster-title'
    | 'medium-story-card'
    | 'screenshot-vision'
    | 'screenshot-phone'
    | 'screenshot-phone_portrait'
    | 'screenshot-iphone_5_8'
    | 'screenshot-iphone_5_8_portrait'
    | 'screenshot-iphone_6_5'
    | 'screenshot-iphone_6_5_portrait'
    | 'screenshot-iphone_d74'
    | 'screenshot-iphone_d74_portrait'
    | 'screenshot-mac'
    | 'screenshot-tv'
    | 'screenshot-pad'
    | 'screenshot-pad-portrait'
    | 'screenshot-watch'
    | 'small-brick'
    | 'small-story-card-portrait'
    | 'small-story-card'
    | 'small-story-card-legacy'
    | 'uber-shelf';

const PROFILES: ArtworkProfileMap<NamedProfile> = new Map([
    ['app-event-detail', [[480, 336, 336], 9 / 16, 'sr']],
    ['app-event-detail-small', [[480, 336, 336], HD_ASPECT_RATIO, 'sr']],
    ['app-icon', [AppIconSize.DEFAULT, ONE, 'bb']],
    ['app-icon-large', [AppIconSize.LARGE, ONE, 'bb']],
    ['app-icon-medium', [AppIconSize.MEDIUM, ONE, 'bb']],
    ['app-icon-small', [AppIconSize.SMALL, ONE, 'bb']],
    ['app-icon-xlarge', [AppIconSize.XLARGE, ONE, 'bb']],
    ['app-icon-pill', [AppIconSize.DEFAULT, 4 / 3, 'sr']],
    ['app-icon-large-pill', [AppIconSize.LARGE, 4 / 3, 'sr']],
    ['app-icon-medium-pill', [AppIconSize.MEDIUM, 4 / 3, 'sr']],
    ['app-icon-small-pill', [AppIconSize.SMALL, 4 / 3, 'sr']],
    ['app-icon-tv-rect', [AppIconSize.DEFAULT, HD, 'sr']],
    ['app-icon-large-tv-rect', [AppIconSize.LARGE, HD, 'sr']],
    ['app-icon-xlarge-tv-rect', [AppIconSize.XLARGE, HD, 'sr']],
    ['app-icon-medium-tv-rect', [AppIconSize.MEDIUM, HD, 'sr']],
    ['app-icon-small-tv-rect', [AppIconSize.SMALL, HD, 'bb']],
    ['app-icon-river-tv-rect', [[128, 128, 128, 88, 88], HD, 'bb']],
    ['app-icon-river', [[128, 128, 128, 88, 88], ONE, 'bb']],
    ['app-icon-river-pill', [[128, 128, 128, 88, 88], 4 / 3, 'sr']],
    ['app-promotion', [[385, 330, 400, 450, 298], 16 / 9, 'sr']],
    ['app-promotion-in-article', [[800, 600], 16 / 9, 'sr']],
    ['app-trailer-lockup-video', [[385, 330, 400, 450, 298], 16 / 10, 'sr']],
    ['brick', [brickSizes, HD, 'sr']],
    ['brick-app-icon', [[83, 60, 60, 60, 50], ONE, 'bb']],
    ['card', [cardSizes, THREE_QUARTERS, 'sr']],
    ['card-horizontal', [[1020], HD, 'sr']],
    ['category-brick', [[368, 368, 368, 208, 288], HD, 'sr']],
    ['editorial-story-card', [[385, 400, 450], THREE_QUARTERS, 'sr']],
    ['in-app-purchase', [[153, 160, 160, 140, 168], ONE, 'sr']],
    ['large-brick', [[520, 610, 450, 298, 298], HD, 'sr']],
    ['large-hero', [heroSizes, HD, 'sr']],
    ['large-hero-portrait', [[430], 9 / 16, 'sr']],
    ['large-hero-portrait-iphone', [[430], THREE_QUARTERS, 'SH.ApCSC01']],
    ['large-hero-west', [heroSizes, 2.79, 'grav.west']],
    ['large-hero-east', [heroSizes, 2.79, 'grav.east']],
    ['large-hero-story-card', [heroSizes, 2.25, 'CC.ApSHW01']],
    ['large-hero-story-card-rtl', [heroSizes, 2.25, 'sr']],
    ['large-hero-story-card-portrait', [cardSizes, 3 / 4, 'CC.ApSHT01']],
    ['large-hero-breakout', [heroSizes, 8 / 3, 'sr']],
    ['large-image-lockup', [[790, 610, 919, 298, 298], HD, 'sr']],
    ['poster-lockup', [[520, 520, 400, 450, 300], HD, 'sr']],
    ['poster-title', [[400, 300, 200], HD, 'bb']],
    [
        'large-hero-breakout-rtl',
        [[1600, 1240, 920, 920, 688], 8 / 3, 'gk' as CropCode],
    ], // the `rtl` version of `large-hero-breakout` assets max out at 1344px wide
    ['medium-story-card', [[298, 579, 490, 394], THREE_QUARTERS, 'sr']],
    ['small-brick', [[300, 300, 300, 200, 300], HD, 'sr']],
    ['small-story-card-portrait', [[182, 232, 215, 192], THREE_QUARTERS, 'sr']],
    ['screenshot-vision', [[480, 480, 335, 520, 520], HD, 'sr']],
    ['screenshot-phone', [[313, 643, 313, 480, 643], 20 / 9, 'w']],
    ['screenshot-phone_portrait', [[230, 230, 157, 300, 300], 9 / 20, 'w']],
    ['screenshot-iphone_5_8', [[313, 643, 313, 480, 643], 498 / 230, 'w']],
    [
        'screenshot-iphone_5_8_portrait',
        [[230, 230, 157, 300, 300], 230 / 498, 'w'],
    ],
    ['screenshot-iphone_6_5', [[313, 643, 313, 480, 643], 498 / 230, 'w']],
    [
        'screenshot-iphone_6_5_portrait',
        [[230, 230, 157, 300, 300], 230 / 498, 'w'],
    ],
    ['screenshot-iphone_d74', [[313, 643, 313, 480, 643], 466 / 215, 'w']],
    [
        'screenshot-iphone_d74_portrait',
        [[230, 230, 157, 300, 300], 215 / 466, 'w'],
    ],
    ['screenshot-mac', [[313, 643, 313, 480, 643], 16 / 10, 'w']],
    ['screenshot-tv', [[313, 643, 313, 480, 643], HD, 'w']],
    ['screenshot-pad', [[313, 643, 313, 480, 643], 4 / 3, 'w']],
    ['screenshot-pad-portrait', [[480, 528, 313, 480, 643], 3 / 4, 'w']],
    ['screenshot-watch', [[300, 157, 230, 230, 230], 4 / 5, 'w']],
    ['small-story-card', [brickSizes, HD, 'CC.ApSHSC01']],
    ['small-story-card-legacy', [brickSizes, HD, 'SCS.ApDPCS01']],
    ['uber-shelf', [[1680, 1680, 1320, 1000, 390], 8 / 3, 'sr']],
]);

ArtworkConfig.set({ PROFILES });
