<script lang="ts">
    import type { WebNavigationLink } from '@jet-app/app-store/api/models/web-navigation';

    import SFSymbol from '~/components/SFSymbol.svelte';
    import PlatformSelectorItem from '~/components/jet/web-navigation/PlatformSelectorItem.svelte';
    import { getI18n } from '~/stores/i18n';
    import Menu from '~/components/Menu.svelte';
    import { getJet } from '~/jet';

    export let platformSelectors: WebNavigationLink[];

    const i18n = getI18n();
    const jet = getJet();

    $: activeSelector = platformSelectors.find((selector) => selector.isActive);

    const handleShowMenu = () => {
        jet.recordCustomMetricsEvent({
            eventType: 'click',
            actionType: 'open',
            targetType: 'button',
            targetId: 'PlatformSelector',
        });
    };
</script>

{#if activeSelector}
    <nav>
        <Menu options={platformSelectors} forcedXPosition={25} {handleShowMenu}>
            <svelte:fragment slot="trigger">
                <span
                    class="platform-selector-text"
                    id="platform-selector-text"
                    aria-labelledby="app-store-icon-contianer platform-selector-text"
                    aria-haspopup="menu"
                >
                    {$i18n.t(
                        'ASE.Web.AppStore.Navigation.PlatformSelectorText',
                        {
                            platform: activeSelector.action.title,
                        },
                    )}

                    <SFSymbol name="chevron.down" />
                </span>
            </svelte:fragment>

            <svelte:fragment slot="option" let:option>
                <PlatformSelectorItem platformSelector={option} />
            </svelte:fragment>
        </Menu>
    </nav>
{/if}

<style>
    nav {
        --menu-item-padding: 0;
        --menu-item-margin: 0 0 8px 0;
        --menu-popover-padding: 8px;
        --menu-common-padding: 8px;
        --menu-trigger-padding: 0;
        --menu-popover-background-color: var(--pageBg);
        --menu-popover-box-shadow: 10px 10px 10px 0
            var(--systemQuaternary-onLight);
        --menu-popover-border-radius: 14px;
        --menu-popover-border: 1px solid var(--systemQuaternary);
        --menu-popover-z-index: calc(var(--z-web-chrome) + 1);
    }

    .platform-selector-text {
        display: flex;
        align-items: center;
        gap: var(--platform-selector-trigger-gap, 4px);
        font: var(--title-2);
        white-space: nowrap;
    }

    .platform-selector-text :global(svg) {
        height: 0.7em;
        position: relative;
        top: 2px;
        fill: var(--systemPrimary);
    }

    nav :global(.menu-popover) {
        width: 211px;
    }
</style>
