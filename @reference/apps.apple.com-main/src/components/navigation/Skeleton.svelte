<script lang="ts">
    import { writable } from 'svelte/store';

    import type { WebSearchFlowAction } from '@jet-app/app-store/common/search/web-search-action';

    import Navigation from '@amp/web-app-components/src/components/Navigation/Navigation.svelte';
    import AppStoreLogo from '~/components/icons/AppStoreLogo.svg';
    import SearchInput from '~/components/navigation/SearchInput.svelte';
    import { getI18n } from '~/stores/i18n';

    const i18n = getI18n();

    $: searchAction = {} as WebSearchFlowAction;
</script>

<div class="navigation-wrapper">
    <Navigation
        translateFn={$i18n.t}
        items={[]}
        currentTab={writable(null)}
        libraryItems={[]}
        personalizedItems={[]}
    >
        <div slot="logo" class="platform-selector-container">
            <span class="app-store-icon-container">
                <AppStoreLogo />
            </span>
        </div>

        <svelte:fragment slot="search">
            <div class="search-input-container">
                <SearchInput {searchAction} />
            </div>
        </svelte:fragment>
    </Navigation>
</div>

<style lang="scss">
    .navigation-wrapper {
        display: contents;
    }

    .platform-selector-container {
        @media (--sidebar-visible) {
            padding: 19px 25px 14px;
        }
    }

    .app-store-icon-container {
        display: flex;
        align-items: center;
        padding: 2px 0;
    }

    .app-store-icon-container :global(svg) {
        height: 18px;
        position: relative;
        top: 0.33px;
        width: auto;

        @media (--sidebar-visible) and (--range-xsmall-only) {
            height: 22px;
            width: auto;
        }
    }

    .search-input-container {
        margin: 0 25px;
    }

    .navigation-wrapper :global(.navigation-item__link) {
        height: 100%;
        display: flex;
    }

    .navigation-wrapper :global(.navigation-item__icon) {
        --navigation-item-icon-size: 32px;
        width: var(--navigation-item-icon-size);
        height: var(--navigation-item-icon-size);

        @media (--sidebar-visible) {
            --navigation-item-icon-size: 24px;
        }
    }
</style>
