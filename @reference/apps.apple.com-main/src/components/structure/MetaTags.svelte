<script lang="ts">
    import type { Opt } from '@jet/environment/types/optional';
    import type { Organization, WithContext } from 'schema-dts';
    import type { WebRenderablePage } from '@jet-app/app-store/api/models/web-renderable-page';

    import MetaTags from '@amp/web-app-components/src/components/MetaTags/MetaTags.svelte';
    import type { SeoData } from '@amp/web-app-components/src/components/MetaTags/types';
    import { getLocale } from '@amp/web-app-components/src/utils/internal/locale';
    import { getPageDir } from '@amp/web-apps-localization/src';

    import { getI18n } from '~/stores/i18n';

    export let page: WebRenderablePage;

    const i18n = getI18n();
    const locale = getLocale();

    const organizationSchema: WithContext<Organization> = {
        '@context': 'https://schema.org',
        '@id': 'https://apps.apple.com/#organization',
        '@type': 'Organization',
        name: 'App Store',
        url: 'https://apps.apple.com',
        logo: 'https://apps.apple.com/assets/app-store.png',
        sameAs: [
            'https://www.wikidata.org/wiki/Q368215',
            'https://twitter.com/AppStore',
            'https://www.instagram.com/appstore/',
            'https://www.facebook.com/appstore/',
        ],
        parentOrganization: {
            '@type': 'Organization',
            name: 'Apple',
            '@id': 'https://www.apple.com/#organization',
            url: 'https://www.apple.com/',
        },
    };

    // This cast of `.seoData` is technically a little risky, but our app fully
    // defines this property, which should make it fairly safe. Whatever is returned
    // for the page from the `SEO` dependency on the Object Graph will be the value
    // reflected here.
    $: seoData = (page.seoData as Opt<SeoData>) ?? undefined;

    // Provide default title for pages not yet set up with SEO data
    $: defaultTitle = $i18n.t('ASE.Web.AppStore.Meta.SiteName');
    $: pageDir = getPageDir(locale.language) ?? 'ltr';
</script>

<MetaTags
    {defaultTitle}
    {locale}
    {pageDir}
    {seoData}
    origin={'https://apps.apple.com/'}
>
    <svelte:fragment slot="schemaOrganizationData">
        {#if import.meta.env.SSR}
            <svelte:element
                this="script"
                id="organization"
                type="application/ld+json"
            >
                {JSON.stringify(organizationSchema)}
            </svelte:element>
        {/if}
    </svelte:fragment>
</MetaTags>
