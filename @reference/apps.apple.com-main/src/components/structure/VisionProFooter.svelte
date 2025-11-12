<script lang="ts">
    import ShelfTitle from '~/components/Shelf/Title.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import Grid from '~/components/Grid.svelte';
    import { getI18n } from '~/stores/i18n';
    import { getLocale } from '~/utils/locale';

    const locale = getLocale();
    const i18n = getI18n();

    let links: Record<string, string>;

    function getAboutAppStoreUrl(storefront: string, language: string) {
        let storefrontSlug = `${storefront}/`;

        if (storefront === 'us') {
            storefrontSlug = '';
        } else if (storefront === 'gb') {
            // The UK "About App Store" link is https://www.apple.com/uk/app-store/, not https://www.apple.com/gb/app-store/.
            storefrontSlug = 'uk/';
        } else if (storefront === 'ae' && language === 'ar') {
            storefrontSlug = 'ae-ar/';
        }

        return `https://www.apple.com/${storefrontSlug}app-store/`;
    }

    $: storefront = locale.storefront;
    $: links = {
        'ASE.Web.AppStore.VisionPro.Footer.Links.AboutAppStore':
            getAboutAppStoreUrl(storefront, locale.language),
        'ASE.Web.AppStore.VisionPro.Footer.Links.AboutPurchases': `https://apps.apple.com/${storefront}/story/id1436214772`,
        'ASE.Web.AppStore.VisionPro.Footer.Links.RequestRefund': `https://www.apple.com/${storefront}/shop/goto/help/sales_refunds`,
        'ASE.Web.AppStore.VisionPro.Footer.Links.PaymentMethods': `https://support.apple.com/118429`,
    };

    $: if (storefront === 'fr') {
        links[
            'AppStore.QuickLinks.AboutFrenchAppStore.Title'
        ] = `https://apps.apple.com/${storefront}/story/1700848501`;
    }
</script>

<ShelfWrapper centered={false} withBottomPadding={false}>
    <section data-test-id="vision-footer">
        <p class="blurb">
            {$i18n.t('ASE.Web.AppStore.VisionPro.Footer.Blurb')}
        </p>

        <article class="quick-links-container">
            <ShelfTitle
                title={$i18n.t('ASE.Web.AppStore.VisionPro.Footer.LinksTitle')}
            />

            <navigation>
                <Grid
                    items={Object.entries(links)}
                    gridType="FooterLink"
                    let:item
                >
                    {@const [title, href] = item}
                    <a {href}>{$i18n.t(title)}</a>
                </Grid>
            </navigation>
        </article>

        <article class="disclaimer-container">
            <p>
                {$i18n.t('ASE.Web.AppStore.VisionPro.Footer.Disclaimer')}
            </p>
        </article>
    </section>
</ShelfWrapper>

<style lang="scss">
    @use 'ac-sasskit/modules/viewportcontent/core' as *;
    @use 'amp/stylekit/core/viewports' as *;

    section {
        font: var(--body-tall);
    }

    .blurb {
        flex-grow: 1;
        width: 100%;
        max-width: calc(viewport-content-for(xlarge) * 0.66);
        margin: 40px auto 50px;
        padding: 0 var(--shelfGridPaddingInline, 40px);
        text-align: center;
    }

    .quick-links-container {
        max-width: viewport-content-for(xlarge);
        margin: 50px auto;
        padding: 0 var(--bodyGutter);
    }

    a {
        display: block;
        padding: var(--grid-column-gap-medium) 0 var(--grid-column-gap-medium);
        word-break: break-all;
        font: var(--title-2);
        color: var(--keyColor);
        border-bottom: 1px solid var(--systemQuinary);

        @media (--range-xsmall-down) {
            padding: var(--grid-column-gap-xsmall) 0
                var(--grid-column-gap-xsmall);
        }
    }

    @media (--range-medium-up) {
        .quick-links-container li:nth-child(n + 4) a {
            border-bottom: none;
        }
    }

    @media (--small) {
        .quick-links-container li:nth-child(n + 5) a {
            border-bottom: none;
        }
    }

    @media (--range-xsmall-down) {
        .quick-links-container li:last-child a {
            border-bottom: none;
        }
    }

    .disclaimer-container {
        flex-grow: 1;
        width: 100%;
        color: var(--systemTertiary);
        background-color: var(--footerBg);
    }

    .disclaimer-container p {
        max-width: viewport-content-for(xlarge);
        margin: 0 auto;
        padding: 32px var(--bodyGutter, 40px);
    }
</style>
