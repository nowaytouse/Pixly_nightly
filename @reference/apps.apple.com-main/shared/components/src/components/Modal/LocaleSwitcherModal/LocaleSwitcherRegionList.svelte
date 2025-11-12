<script lang="ts">
    import type { Storefront } from '@amp/web-app-components/src/components/buttons/LocaleSwitcherButton/types';
    import { getStorefrontRoute } from '@amp/web-app-components/src/utils/getStorefrontRoute';

    export let regionList: Storefront[];
    export let defaultRoute: string;

    const getRoute = (storefront: Storefront) => {
        // the language param is only needed for non-default storefronts
        return storefront.isDefault
            ? getStorefrontRoute(defaultRoute, storefront.id)
            : getStorefrontRoute(
                  defaultRoute,
                  storefront.id,
                  storefront.language,
              );
    };
</script>

<ul>
    {#each regionList as storefront}
        <li>
            <a href={getRoute(storefront)} data-testid="region-list-link">
                <span>{storefront.name}</span>
            </a>
        </li>
    {/each}
</ul>

<style lang="scss">
    ul,
    li {
        list-style-type: none;
        margin: 0;
        padding: 0;
    }

    ul {
        columns: 1 auto;
        margin-bottom: 25px;

        @media (width >= 600px) {
            columns: 3 auto;
        }

        @media (--small) {
            columns: 4 auto;
        }

        @media (--large) {
            columns: 5 auto;
        }

        @media (--xlarge) {
            columns: 6 auto;
        }
    }

    li {
        padding-right: 40px;
        padding-bottom: 26px;
        display: inline-block;
        width: 100%;
        font: var(--callout);

        a {
            --linkColor: var(--systemPrimary);
        }
    }
</style>
