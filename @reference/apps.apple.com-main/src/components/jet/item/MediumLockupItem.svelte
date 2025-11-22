<script lang="ts">
    import {
        type FlowAction,
        type Lockup,
        isFlowAction,
    } from '@jet-app/app-store/api/models';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import AppIcon from '~/components/AppIcon.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import { getI18n } from '~/stores/i18n';
    import type { Opt } from '@jet/environment';

    export let item: Lockup;

    const i18n = getI18n();

    const { clickAction } = item;
    const destination: Opt<FlowAction> = isFlowAction(clickAction)
        ? clickAction
        : undefined;
</script>

<LinkWrapper action={destination}>
    <article>
        <div class="app-icon-container">
            <AppIcon
                icon={item.icon}
                profile="app-icon-medium"
                fixedWidth={false}
            />
        </div>

        <div class="metadata-container">
            {#if item.heading}
                <span class="heading">{item.heading}</span>
            {/if}

            {#if item.title}
                <LineClamp clamp={1}>
                    <h3>{item.title}</h3>
                </LineClamp>
            {/if}

            {#if item.subtitle}
                <LineClamp clamp={1}>
                    <p>{item.subtitle}</p>
                </LineClamp>
            {/if}

            {#if destination}
                <div class="button-container">
                    <span class="get-button gray">
                        {$i18n.t('ASE.Web.AppStore.View')}
                    </span>
                </div>
            {/if}
        </div>
    </article>
</LinkWrapper>

<style>
    article {
        display: flex;
        align-items: center;
    }

    .app-icon-container {
        flex-shrink: 0;
        width: 85px;
        margin-inline-end: 16px;
    }

    .metadata-container {
        margin-inline-end: 16px;
    }

    h3 {
        font: var(--title-3);
        margin-bottom: 2px;
    }

    p {
        font: var(--callout);
        color: var(--systemSecondary);
    }

    .heading {
        font: var(--callout-emphasized);
    }

    .button-container {
        margin-inline-start: auto;
        margin-top: 8px;
    }
</style>
