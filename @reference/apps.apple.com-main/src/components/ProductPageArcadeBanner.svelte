<script lang="ts">
    import AppleArcadeLogo from '~/components/icons/AppleArcadeLogo.svg';
    import { getI18n } from '~/stores/i18n';

    const i18n = getI18n();
</script>

<aside>
    <div class="arcade-banner">
        <div class="metadata-container">
            <div class="logo-container">
                <AppleArcadeLogo />
            </div>

            <h2>
                {$i18n.t('ASE.Web.AppStore.Arcade.UpsellFooter.LineOne')}
                <br />
                {$i18n.t('ASE.Web.AppStore.Arcade.UpsellFooter.LineTwo')}
                {$i18n.t('ASE.Web.AppStore.Arcade.UpsellFooter.LineThree')}
            </h2>

            <a href="https://www.apple.com/apple-arcade/" target="_blank">
                <span>
                    {$i18n.t(
                        'ASE.Web.AppStore.Arcade.UpsellFooter.CallToActionText',
                    )}
                </span>
                {$i18n.t(
                    'ASE.Web.AppStore.Arcade.UpsellFooter.CallToActionDisclaimerMark',
                )}
            </a>
        </div>
    </div>
</aside>

<style lang="scss">
    @use '@amp/web-shared-styles/app/core/globalvars' as *;
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;
    @use 'ac-sasskit/modules/viewportcontent/core' as *;
    @use 'amp/stylekit/core/viewports' as *;

    .logo-container {
        width: 62px;
        margin-bottom: 10px;
        line-height: 0;

        @media (--range-xsmall-only) {
            width: 48px;
            margin-bottom: 8px;
        }
    }

    .logo-container :global(path) {
        color: var(--systemPrimary-onDark);
    }

    .metadata-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        width: 60%;
        height: 100%;
        padding: 0 20px;

        @media (--range-xsmall-only) {
            align-items: flex-start;
            justify-content: center;
        }
    }

    h2 {
        margin-bottom: 10px;
        font: var(--title-1-emphasized);

        @media (--range-xsmall-only) {
            margin-bottom: 8px;
            font: var(--title-3-emphasized);
        }
    }

    a {
        display: flex;
        font: var(--title-3-emphasized);

        @media (--range-xsmall-only) {
            font: var(--body-emphasized);
        }
    }

    a::after {
        content: '↗';
        font-weight: normal;
        margin-inline-start: 4px;
    }

    a:hover {
        text-decoration: none;
    }

    a:hover span {
        text-decoration: underline;
    }

    aside {
        width: 100%;
        max-width: calc(viewport-content-for(xlarge));
        height: 152px;
        margin: 0 auto 32px;
        padding: 0 var(--bodyGutter);

        @media (--range-xsmall-only) {
            max-width: 100%;
            padding: 0;
        }
    }

    .arcade-banner {
        width: 100%;
        height: 100%;
        color: var(--systemPrimary-onDark);
        border-radius: var(--global-border-radius-medium);
        background: #000;
        background-repeat: no-repeat;
        background-position: right;
        background-size: contain;

        @media (prefers-color-scheme: dark) {
            border: 1px solid var(--systemQuaternary-onDark);
        }

        @media (--range-xsmall-only) {
            border-radius: 0;
            background-image: url('/assets/images/arcade/upsell/banner-692@1x_LTR.png');
            background-size: cover;

            @include rtl {
                background-image: url('/assets/images/arcade/upsell/banner-692@1x_RTL.png');
                background-position: left;
            }

            @media (resolution >= 192dpi) {
                background-image: url('/assets/images/arcade/upsell/banner-692@2x_LTR.png');

                @include rtl {
                    background-image: url('/assets/images/arcade/upsell/banner-692@2x_RTL.png');
                    background-position: left;
                }
            }
        }

        @media (--range-small-only) {
            background-image: url('/assets/images/arcade/upsell/banner-692@1x_LTR.png');

            @include rtl {
                background-image: url('/assets/images/arcade/upsell/banner-692@1x_RTL.png');
                background-position: left;
            }

            @media (resolution >= 192dpi) {
                background-image: url('/assets/images/arcade/upsell/banner-692@2x_LTR.png');

                @include rtl {
                    background-image: url('/assets/images/arcade/upsell/banner-692@2x_RTL.png');
                    background-position: left;
                }
            }
        }

        @media (--range-medium-up) {
            background-image: url('/assets/images/arcade/upsell/banner-980@1x_LTR.png');

            @include rtl {
                background-image: url('/assets/images/arcade/upsell/banner-980@1x_RTL.png');
                background-position: left;
            }

            @media (resolution >= 192dpi) {
                background-image: url('/assets/images/arcade/upsell/banner-980@2x_LTR.png');

                @include rtl {
                    background-image: url('/assets/images/arcade/upsell/banner-980@2x_RTL.png');
                    background-position: left;
                }
            }
        }
    }
</style>
