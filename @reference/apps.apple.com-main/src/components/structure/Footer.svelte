<script lang="ts">
    import { getI18n } from '~/stores/i18n';
    import Footer, {
        type Translate,
    } from '@amp/web-app-components/src/components/Footer/Footer.svelte';
    import LocaleSwitcherButton from '@amp/web-app-components/src/components/buttons/LocaleSwitcherButton/LocaleSwitcherButton.svelte';
    import { items } from '~/constants/footer-items';
    import { getLocale } from '~/utils/locale';
    import {
        regions,
        languages,
        storefrontNameTranslations,
    } from '~/utils/storefront-data';

    const i18n = getI18n();
    const locale = getLocale();

    const translate: Translate = (key, options) => $i18n.t(key, options);
</script>

<section class="footer-container">
    <Footer footerItems={items} translateFn={translate}>
        <LocaleSwitcherButton
            slot="secondary-content"
            translateFn={translate}
            {regions}
            {languages}
            {locale}
            {storefrontNameTranslations}
            defaultRoute="iphone/today"
        />
    </Footer>
</section>

<style lang="scss">
    @use 'ac-sasskit/modules/viewportcontent/core' as *;
    @use 'amp/stylekit/core/viewports' as *;

    .footer-container {
        background-color: var(--footerBg);
    }

    .footer-container :global(footer) {
        max-width: calc(viewport-content-for(xlarge));
        margin: 0 auto;
    }
</style>
