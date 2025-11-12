<script lang="ts">
    import { isSome } from '@jet/environment/types/optional';
    import type { WebNavigationLink } from '@jet-app/app-store/api/models/web-navigation';
    import { isSearchResultsPageIntent } from '@jet-app/app-store/api/intents/search-results-page-intent';

    import FlowAction from '~/components/jet/action/FlowAction.svelte';
    import SystemImage, {
        isSystemImageArtwork,
    } from '~/components/SystemImage.svelte';
    import SFSymbol from '~/components/SFSymbol.svelte';
    import { getI18n } from '~/stores/i18n';

    export let platformSelector: WebNavigationLink;

    const i18n = getI18n();

    $: ({ action, isActive } = platformSelector);
    $: ({ artwork } = action);
</script>

<FlowAction destination={action}>
    <span class="platform-selector" class:is-active={isActive}>
        {#if isSome(artwork) && isSystemImageArtwork(artwork)}
            <div class="icon-container">
                <SystemImage {artwork} />
            </div>
        {/if}

        <span
            class="platform-title"
            aria-label={$i18n.t(
                'ASE.Web.AppStore.Navigation.AX.PlatformSelectorItem',
                {
                    platform: action.title,
                },
            )}
        >
            {action.title}
        </span>

        {#if action.destination && isSearchResultsPageIntent(action.destination)}
            <span aria-hidden={true} class="search-icon-container">
                <SFSymbol name="magnifyingglass" />
            </span>
        {/if}
    </span>
</FlowAction>

<style lang="scss">
    @use '@amp/web-shared-styles/app/core/globalvars' as *;

    .platform-selector {
        display: flex;
        border-radius: var(--global-border-radius-medium);
        padding: 8px;
        margin-bottom: 4px;
        gap: 10px;
        transition: background-color 175ms ease-in;
    }

    .platform-selector:not(.is-active):hover {
        background-color: rgba(45, 45, 45, 0.035);

        @media (prefers-color-scheme: dark) {
            background-color: rgba(45, 45, 45, 0.35);
        }
    }

    .platform-selector.is-active {
        background-color: var(--systemQuinary);
    }

    .icon-container {
        display: flex;
        justify-content: center;
        padding-inline-end: 2px;
    }

    .icon-container :global(svg) {
        max-height: 16px;
        width: 23px;
    }

    .search-icon-container {
        display: flex;
    }

    .search-icon-container :global(svg) {
        fill: var(--systemSecondary);
        width: 16px;
    }

    .platform-title {
        font: var(--body);
        flex-grow: 1;
    }
</style>
