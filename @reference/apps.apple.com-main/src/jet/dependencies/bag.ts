import type { Bag as NativeBag, BagKeyDescriptor } from '@jet/environment';
import type { Opt } from '@jet/environment';
import type { Logger, LoggerFactory } from '@amp/web-apps-logger';

import type { Locale } from './locale';
import {
    EU_STOREFRONTS,
    SUPPORTED_STOREFRONTS_FOR_VISION,
    UNSUPPORTED_STOREFRONTS_FOR_ARCADE,
} from '~/constants/storefront';

export type BagRetrievalMethod = Exclude<keyof NativeBag, 'registerBagKeys'>;

export function makeUnimplementedKeyRequestWarning(
    method: BagRetrievalMethod,
    key: string,
) {
    return `requested unimplemented \`${method}\` key \`${key}\``;
}

export class WebBag implements NativeBag {
    private readonly log: Logger;
    private readonly locale: Locale;

    constructor(loggerFactory: LoggerFactory, locale: Locale) {
        this.log = loggerFactory.loggerFor('Bag');
        this.locale = locale;
    }

    private provideNoValue(method: BagRetrievalMethod, key: string): null {
        this.log.warn(makeUnimplementedKeyRequestWarning(method, key));

        return null;
    }

    registerBagKeys(_keys: BagKeyDescriptor[]): void {
        // We hardcode, so registration is a no-op
    }

    double(key: string): Opt<number> {
        switch (key) {
            case 'game-controller-recommended-rollout-rate':
                return 1.0; // set to 1.0 to enable `learn more` button for game controller capability
            case 'icon-artwork-rollout-rate':
                return 1.0; // set to 1.0 to enable new icon artwork style
            default:
                return this.provideNoValue('double', key);
        }
    }

    integer(key: string): Opt<number> {
        return this.provideNoValue('integer', key);
    }

    boolean(key: string): Opt<boolean> {
        switch (key) {
            case 'enableAppEvents':
                return true;
            case 'enable-app-accessibility-labels':
                return true;
            case 'enable-app-store-age-ratings':
                return true;
            case 'enable-external-purchase':
                return true;
            case 'enable-privacy-nutrition-labels':
                return true;
            case 'enable-system-app-reviews':
                return true;
            case 'enable-vision-platform':
                return SUPPORTED_STOREFRONTS_FOR_VISION.has(
                    this.locale.activeStorefront,
                );
            case 'arcade-enabled':
                return !UNSUPPORTED_STOREFRONTS_FOR_ARCADE.has(
                    this.locale.activeStorefront,
                );

            // Enable required `GroupingPage` features
            case 'enable-featured-categories-on-groupings':
            case 'enable-category-bricks-on-groupings':
                return true;
            case 'enable-seller-info':
                return true;
            case 'enable-preview-platform-for-web':
                return false;
            case 'enableProductPageVariants':
                return true;
            case 'game-center-extend-supported-features':
                return true;
            case 'enable-product-page-install-size':
                return true;
            case 'enable-icon-artwork':
                return true;
            default:
                return this.provideNoValue('boolean', key);
        }
    }

    array(key: string): Opt<unknown> {
        switch (key) {
            // URL patterns that are opted into the "edge" domains
            // https://github.pie.apple.com/app-store/ios-appstore-app/blob/83834eea5dfcad22d902fe395c4d140ec7fa8cea/src/foundation/media/url-builder.ts#L350
            case 'apps-media-api-edge-end-points':
                return [
                    // Including a pattern that matches our "search" API endpoint ensures
                    // that the built URL uses the `apps-media-api-search-edge-host` host
                    // https://github.pie.apple.com/app-store/ios-appstore-app/blob/83834eea5dfcad22d902fe395c4d140ec7fa8cea/src/foundation/media/url-builder.ts#L352
                    '/search',
                ];
            case 'enabled-external-purchase-placements':
                return ['product-page-banner', 'product-page-info-section'];
            case 'tabs/standard':
                return [
                    {
                        id: 'today',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.Today',
                        ),
                        'image-identifier': 'text.rectangle.page',
                    },
                    {
                        id: 'apps',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.Apps',
                        ),
                        'image-identifier': 'app.3.stack.3d.fill',
                    },
                    {
                        id: 'apps-and-games',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.AppsAndGames',
                        ),
                        'image-identifier': 'rocket.fill',
                    },
                    {
                        id: 'arcade',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.Arcade',
                        ),
                        'image-identifier': 'joystickcontroller.fill',
                    },
                    {
                        id: 'create',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.Create',
                        ),
                        'image-identifier': 'paintbrush.fill',
                    },
                    {
                        id: 'discover',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.Discover',
                        ),
                        'image-identifier': 'star.fill',
                    },
                    {
                        id: 'games',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.Games',
                        ),
                        'image-identifier': 'rocket.fill',
                    },
                    {
                        id: 'work',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.Work',
                        ),
                        'image-identifier': 'paperplane.fill',
                    },
                    {
                        id: 'play',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.Play',
                        ),
                        'image-identifier': 'rocket.fill',
                    },
                    {
                        id: 'develop',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.Develop',
                        ),
                        'image-identifier': 'hammer.fill',
                    },
                    {
                        id: 'categories',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.Categories',
                        ),
                        'image-identifier': 'square.grid.2x2.fill',
                    },
                    {
                        id: 'search',
                        title: this.locale.i18n.t(
                            'ASE.Web.AppStore.Navigation.LandingPage.Search',
                        ),
                        'image-identifier': 'magnifyingglass',
                    },
                ];
            default:
                return this.provideNoValue('array', key);
        }
    }

    dictionary(key: string): Opt<unknown> {
        return this.provideNoValue('dictionary', key);
    }

    url(key: string): Opt<string> {
        switch (key) {
            case 'apps-media-api-host':
                return 'amp-api-edge.apps.apple.com';
            case 'apps-media-api-edge-host':
                return 'amp-api-edge.apps.apple.com';
            case 'apps-media-api-search-edge-host':
                return 'amp-api-search-edge.apps.apple.com';

            default:
                return this.provideNoValue('url', key);
        }
    }

    string(key: string): Opt<string> {
        switch (key) {
            case 'countryCode':
                return this.locale.activeStorefront;

            case 'language-tag':
                return this.locale.activeLanguage;

            case 'language':
                // TODO: rdar://78159789: util for this? What about zh-Hant, etc.
                return this.locale.activeLanguage.split('-')[0];

            // Some URLs are accessed as strings
            // TODO: fix this upstream in `ios-appstore-app` so it uses `.url()` instead
            case 'apps-media-api-edge-host':
            case 'apps-media-api-search-edge-host':
                return this.url(key);

            case 'game-controller-learn-more-editorial-item-id':
                return '1687769242';

            case 'familySubscriptionsLearnMoreEditorialItemId':
                return '1563279606';

            case 'external-purchase-learn-more-editorial-item-id':
                if (this.locale.activeStorefront === 'kr') {
                    return 'id1727067165';
                }

                return 'id1760810284';

            case 'appPrivacyLearnMoreEditorialItemId':
                return 'id1538632801';

            case 'ageRatingLearnMoreEditorialItemId':
                return '1825160725';

            case 'accessibility-learn-more-editorial-item-id':
                return '1814164299';

            case 'external-purchase-product-page-banner-text-variant':
                return '2';
            case 'external-purchase-product-page-annotation-variant':
                return '4';

            case 'transparencyLawEditorialItemId':
                if (EU_STOREFRONTS.includes(this.locale.activeStorefront)) {
                    return 'id1620909697';
                }

                return null;

            case 'appPrivacyDefinitionsEditorialItemId':
                return '1539235847';

            case 'metrics_topic':
                return 'xp_amp_appstore_unidentified';

            case 'in-app-purchases-learn-more-editorial-item-id':
                return '1436214772';

            case 'web-navigation-category-tabs-editorial-item-id':
                return '1842456901';

            default:
                return this.provideNoValue('string', key);
        }
    }
}
