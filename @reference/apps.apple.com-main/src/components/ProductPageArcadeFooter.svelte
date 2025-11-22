<script lang="ts">
    import AppleArcadeLogo from '~/components/icons/AppleArcadeLogo.svg';
    import { getI18n } from '~/stores/i18n';

    const i18n = getI18n();
</script>

<article>
    <div class="metadata-container">
        <div class="logo-container">
            <AppleArcadeLogo />
        </div>

        <h2>
            {$i18n.t('ASE.Web.AppStore.Arcade.UpsellFooter.LineOne')}
            <br />
            {$i18n.t('ASE.Web.AppStore.Arcade.UpsellFooter.LineTwo')}
            <br />
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
</article>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;
    @use 'ac-sasskit/modules/viewportcontent/core' as *;
    @use 'amp/stylekit/core/viewports' as *;

    .logo-container {
        width: 72px;
        margin-bottom: 20px;
        line-height: 0;

        @media (--range-xsmall-only) {
            width: 62px;
            margin-bottom: 16px;
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
        padding: 40px;

        @media (--range-xsmall-only) {
            align-items: center;
            justify-content: end;
            width: unset;
            text-align: center;
        }
    }

    h2 {
        margin-bottom: 20px;
        font: var(--header-emphasized);
        line-height: 54px;

        @media (--range-xsmall-only) {
            font: var(--title-1-emphasized);
        }
    }

    a {
        display: flex;
        font: var(--title-3-emphasized);
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

    article {
        flex-grow: 1;
        width: 100%;
        max-width: calc(viewport-content-for(xlarge) - var(--bodyGutter) * 2);
        aspect-ratio: 2.55;
        margin: 0 auto;
        color: var(--systemPrimary-onDark);
        background: #000;
        background-size: cover;

        @media (--range-xsmall-only) {
            max-width: 338px;
            aspect-ratio: 35/48;
            border-radius: var(--global-border-radius-medium);
            background-image: url('/assets/images/arcade/upsell/footer-280@1x.png');

            @media (resolution >= 192dpi) {
                background-image: url('/assets/images/arcade/upsell/footer-280@2x.png');
            }
        }

        @media (--range-small-only) {
            aspect-ratio: 173/96;
            background-image: url('/assets/images/arcade/upsell/footer-692@1x_LTR.png');

            @include rtl {
                background-image: url('/assets/images/arcade/upsell/footer-692@1x_RTL.png');
            }

            @media (resolution >= 192dpi) {
                background-image: url('/assets/images/arcade/upsell/footer-692@2x_LTR.png');

                @include rtl {
                    background-image: url('/assets/images/arcade/upsell/footer-692@2x_RTL.png');
                }
            }
        }

        @media (--range-medium-up) {
            background-image: url('/assets/images/arcade/upsell/footer-980@1x_LTR.png');

            @include rtl {
                background-image: url('/assets/images/arcade/upsell/footer-980@1x_RTL.png');
            }

            @media (resolution >= 192dpi) {
                background-image: url('/assets/images/arcade/upsell/footer-980@2x_LTR.png');

                @include rtl {
                    background-image: url('/assets/images/arcade/upsell/footer-980@2x_RTL.png');
                }
            }
        }

        @media (--range-xlarge-up) {
            border-radius: var(--global-border-radius-medium);
        }
    }
</style>
