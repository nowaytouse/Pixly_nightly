<!--
@component
Component for rendering App information into the `details` slot
of the `Hero.svelte` component
-->
<script lang="ts">
    import type { Lockup } from '@jet-app/app-store/api/models';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';

    import { getI18n } from '~/stores/i18n';
    import AppIcon from '~/components/AppIcon.svelte';

    const i18n = getI18n();

    export let lockup: Lockup;
    export let isOnDarkBackground: boolean = true;
</script>

<div class="lockup-container">
    {#if lockup.icon}
        <div class="app-icon-container">
            <AppIcon icon={lockup.icon} profile="app-icon-small" />
        </div>
    {/if}

    <div class="text-container">
        {#if lockup.heading}
            <LineClamp clamp={1}>
                <h4>{lockup.heading}</h4>
            </LineClamp>
        {/if}

        {#if lockup.title}
            <LineClamp clamp={2}>
                <h3>{lockup.title}</h3>
            </LineClamp>
        {/if}

        {#if lockup.subtitle}
            <LineClamp clamp={1}>
                <p>{lockup.subtitle}</p>
            </LineClamp>
        {/if}
    </div>

    <div class="button-container">
        <span
            class="get-button"
            class:transparent={isOnDarkBackground}
            class:dark-gray={!isOnDarkBackground}
        >
            {$i18n.t('ASE.Web.AppStore.View')}
        </span>
    </div>
</div>

<style lang="scss">
    .lockup-container {
        display: flex;
        align-items: center;
        width: 100%;
        max-width: 350px;
        margin-top: 20px;
        padding-top: 20px;
        color: var(--hero-primary-color, var(--systemPrimary-onDark));
        border-top: 1px solid
            var(--hero-divider-color, var(--systemQuaternary-onDark));

        @media (--range-xsmall-down) {
            text-align: left;
            padding: 20px 0 10px;
            max-width: unset;
        }
    }

    .app-icon-container {
        flex-shrink: 0;
        width: 64px;
        margin-inline-end: 16px;
    }

    .text-container {
        width: 100%;
        margin-inline-end: 16px;
    }

    h3 {
        font: var(--title-3-emphasized);
        text-wrap: pretty;
    }

    h4 {
        color: var(--hero-secondary-color, var(--systemSecondary-onDark));
        font: var(--subhead-emphasized);
        text-transform: uppercase;
        mix-blend-mode: var(--hero-text-blend-mode, plus-lighter);
    }

    p {
        mix-blend-mode: var(--hero-text-blend-mode, plus-lighter);
    }

    .button-container {
        --get-button-font: var(--title-3-bold);
        position: relative;
        z-index: 1;
    }
</style>
