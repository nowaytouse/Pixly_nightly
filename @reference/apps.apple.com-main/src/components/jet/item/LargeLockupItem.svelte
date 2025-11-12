<script lang="ts">
    import {
        isFlowAction,
        type FlowAction,
        type Lockup,
    } from '@jet-app/app-store/api/models';
    import type { Opt } from '@jet/environment';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import AppIcon from '~/components/AppIcon.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import { getI18n } from '~/stores/i18n';

    export let item: Lockup;
    const i18n = getI18n();
    const { clickAction } = item;
    const destination: Opt<FlowAction> = isFlowAction(clickAction)
        ? clickAction
        : undefined;

    $: secondaryLine = item.editorialTagline || item.subtitle;
</script>

<LinkWrapper action={destination}>
    <article>
        <div class="app-icon-container">
            <AppIcon
                fixedWidth={false}
                icon={item.icon}
                profile="app-icon-large"
            />
        </div>

        <div class="metadata-container">
            {#if item.heading}
                <LineClamp clamp={2}>
                    <h4>{item.heading}</h4>
                </LineClamp>
            {/if}

            {#if item.title}
                <LineClamp clamp={2}>
                    <h3>{item.title}</h3>
                </LineClamp>
            {/if}

            {#if !item.heading && secondaryLine}
                <LineClamp clamp={1}>
                    <p>{secondaryLine}</p>
                </LineClamp>
            {/if}

            {#if item.tertiaryTitle}
                <LineClamp clamp={1}>
                    <p class="tertiary-text">{item.tertiaryTitle}</p>
                </LineClamp>
            {/if}
        </div>

        {#if destination}
            <div class="button-container">
                <span class="get-button gray">
                    {$i18n.t('ASE.Web.AppStore.View')}
                </span>
            </div>
        {/if}
    </article>
</LinkWrapper>

<style>
    article {
        display: flex;
        flex-direction: column;
        min-height: 290px;
        padding: 20px;
        border-radius: var(--global-border-radius-large);
        background: var(--systemPrimary-onDark);
        box-shadow: var(--shadow-small);
    }

    @media (prefers-color-scheme: dark) {
        article {
            background: var(--systemQuaternary);
        }
    }

    .app-icon-container {
        --artwork-override-height: 100px;
        --artwork-override-width: auto;
        display: flex;
        margin-bottom: 10px;
    }

    .metadata-container {
        flex-grow: 1;
    }

    h3 {
        margin-bottom: 3px;
        font: var(--title-2-emphasized);
    }

    h4 {
        margin-bottom: 3px;
        color: var(--systemSecondary);
        font: var(--subhead-emphasized);
        text-transform: uppercase;
    }

    p {
        margin: 3px 0;
        font: var(--body);
        color: var(--systemSecondary);
        text-wrap: pretty;
    }

    .tertiary-text {
        font: var(--callout);
        color: var(--systemTertiary);
    }
</style>
