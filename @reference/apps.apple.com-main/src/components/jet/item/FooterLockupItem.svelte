<script lang="ts">
    import type { Lockup } from '@jet-app/app-store/api/models';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import AppIcon from '~/components/AppIcon.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import { getI18n } from '~/stores/i18n';

    export let item: Lockup;

    const i18n = getI18n();
</script>

<div class="footer-lockup-item">
    <LinkWrapper
        action={item.clickAction}
        label={`${$i18n.t('ASE.Web.AppStore.View')} ${
            item.title ? item.title : null
        }`}
    >
        {#if item.icon}
            <AppIcon icon={item.icon} profile="app-icon-small" />
        {/if}

        <div>
            {#if item.heading}
                <LineClamp clamp={1}>
                    <h4 dir="auto">{item.heading}</h4>
                </LineClamp>
            {/if}

            {#if item.title}
                <LineClamp clamp={1}>
                    <h3 dir="auto">{item.title}</h3>
                </LineClamp>
            {/if}

            {#if item.subtitle}
                <LineClamp clamp={1}>
                    <p dir="auto">{item.subtitle}</p>
                </LineClamp>
            {/if}
        </div>

        <span class="get-button blue" aria-hidden="true">
            {$i18n.t('ASE.Web.AppStore.View')}
        </span>
    </LinkWrapper>
</div>

<style>
    .footer-lockup-item > :global(a) {
        display: flex;
        align-items: center;
        flex-direction: column;
        width: 100%;
        padding: 32px;
        gap: 16px;
        text-align: center;
        border-radius: var(--global-border-radius-small);
        background-color: var(--systemQuinary);
        transition: background-color 210ms ease-out;
    }

    .footer-lockup-item > :global(a:hover) {
        --darken-amount: 2%;
        background-color: color-mix(
            in srgb,
            var(--systemQuinary) calc(100% - var(--darken-amount)),
            black
        );

        @media (prefers-color-scheme: dark) {
            --darken-amount: 10%;
        }
    }

    h3 {
        margin-bottom: 4px;
        font: var(--title-2-emphasized);
        color: var(--title-color);
    }

    h4 {
        text-transform: uppercase;
        font: var(--subhead-emphasized);
        color: var(--systemSecondary);
    }

    p {
        color: var(--systemSecondary);
    }
</style>
