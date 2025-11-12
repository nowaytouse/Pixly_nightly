<script lang="ts">
    import type { Lockup } from '@jet-app/app-store/api/models';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import AppIcon, { type AppIconProfile } from '~/components/AppIcon.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import { getI18n } from '~/stores/i18n';

    export let item: Lockup;

    /**
     * Controls the `get-button` variant class that is applied to the "View" button
     *
     * @default "gray"
     */
    export let buttonVariant: 'gray' | 'blue' | 'transparent' = 'gray';
    export let shouldShowLaunchNativeButton: boolean = false;
    export let titleLineCount: number = 2;
    export let appIconProfile: AppIconProfile = 'app-icon-small';

    const i18n = getI18n();
</script>

<div class="small-lockup-item">
    <LinkWrapper
        action={item.clickAction}
        label={`${$i18n.t('ASE.Web.AppStore.View')} ${
            item.title ? item.title : null
        }`}
    >
        {#if item.icon}
            <div class="app-icon-container">
                <AppIcon icon={item.icon} profile={appIconProfile} />
            </div>
        {/if}

        <div class="metadata-container">
            {#if item.heading}
                <LineClamp clamp={1}>
                    <h4 dir="auto">{item.heading}</h4>
                </LineClamp>
            {/if}

            {#if item.title}
                <LineClamp clamp={titleLineCount}>
                    <h3 dir="auto">{item.title}</h3>
                </LineClamp>
            {/if}

            {#if item.subtitle}
                <LineClamp clamp={1}>
                    <p dir="auto">{item.subtitle}</p>
                </LineClamp>
            {/if}
        </div>

        <div class="button-container" aria-hidden="true">
            {#if shouldShowLaunchNativeButton && $$slots['launch-native-button']}
                <slot name="launch-native-button" />
            {:else}
                <span class="get-button {buttonVariant}">
                    {$i18n.t('ASE.Web.AppStore.View')}
                </span>
            {/if}
        </div>
    </LinkWrapper>
</div>

<style>
    .small-lockup-item,
    .small-lockup-item :global(a) {
        display: flex;
        align-items: center;
        width: 100%;
    }

    .app-icon-container {
        flex-shrink: 0;
        margin-inline-end: 16px;
    }

    .metadata-container {
        margin-inline-end: 16px;
    }

    h3 {
        color: var(--title-color);
        font: var(--title-3-emphasized);
    }

    h4 {
        color: var(--eyebrow-color, var(--systemSecondary));
        font: var(--subhead-emphasized);
        text-transform: uppercase;
        mix-blend-mode: var(--eyebrow-blend-mode);
    }

    p {
        font: var(--callout);
        color: var(--subtitle-color, var(--systemSecondary));
        mix-blend-mode: var(--subtitle-blend-mode);
    }

    .button-container {
        margin-inline-start: auto;
        margin-inline-end: var(--margin-inline-end, 0);
        mix-blend-mode: var(--button-blend-mode);
        flex-shrink: 0;
    }
</style>
