<script lang="ts">
    import ArrowIcon from '@amp/web-app-components/assets/icons/arrow.svg';
    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import { getJet } from '~/jet';
    import { getI18n } from '~/stores/i18n';
    import { launchAppOnMac } from '~/utils/launch-client';

    export let url: string;

    const i18n = getI18n();
    const jet = getJet();

    function handleButtonClick(event: MouseEvent) {
        // Need to call both event.preventDefault() and event.stopPropagation()
        // to prevent navigation to the production page on web
        event.preventDefault();
        event.stopPropagation();

        if (url) {
            launchAppOnMac(url);
            jet.recordCustomMetricsEvent({
                eventType: 'click',
                targetId: 'OpenInMacAppStore',
                targetType: 'button',
                actionType: 'open',
            });
        }
    }
</script>

<button
    class="get-button blue"
    aria-label={$i18n.t('ASE.Web.AppStore.CTA.MacAppStore.AX')}
    on:click={handleButtonClick}
>
    <LineClamp clamp={1}>
        {$i18n.t('ASE.Web.AppStore.CTA.MacAppStore.Action')}
        <span>
            {$i18n.t('ASE.Web.AppStore.CTA.MacAppStore.App')}
        </span>
    </LineClamp>
    <ArrowIcon class="external-link-arrow" aria-hidden="true" />
</button>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;

    button {
        display: inline-flex;
    }

    button span {
        font-weight: 500;
    }

    button :global(.external-link-arrow) {
        align-self: center;
        width: var(--launch-native-button-arrow-size, 9px);
        height: var(--launch-native-button-arrow-size, 9px);
        padding-top: 1px;
        margin-inline-start: 4px;
        fill: var(--systemPrimary-onDark);

        @include rtl {
            transform: rotate(-90deg);
        }
    }
</style>
