<script lang="ts" context="module">
    import type { Lockup } from '@jet-app/app-store/api/models';

    interface SmallLockupWithOrdinalItem extends Lockup {
        ordinal: string;
    }

    export function isSmallLockupWithOrdinalItem(
        item: Lockup,
    ): item is SmallLockupWithOrdinalItem {
        return !!item?.ordinal;
    }
</script>

<script lang="ts">
    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import AppIcon from '~/components/AppIcon.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import { getI18n } from '~/stores/i18n';
    import mediaQueries from '~/utils/media-queries';

    export let item: Lockup;

    $: titleLineCount = item.heading || $mediaQueries === 'xsmall' ? 1 : 2;

    const i18n = getI18n();
</script>

<LinkWrapper action={item.clickAction}>
    <article>
        {#if item.ordinal}
            <div class="ordinal">
                {item.ordinal}
            </div>
        {/if}

        {#if item.icon}
            <div
                class="app-icon-container"
                style:--icon-aspect-ratio={item.icon.width / item.icon.height}
            >
                <AppIcon
                    icon={item.icon}
                    profile="app-icon-medium"
                    fixedWidth={false}
                />
            </div>
        {/if}
        <div class="metadata-container">
            {#if item.heading}
                <LineClamp clamp={1}>
                    <h4>{item.heading}</h4>
                </LineClamp>
            {/if}

            {#if item.title}
                <LineClamp clamp={titleLineCount}>
                    <h3 title={item.title}>{item.title}</h3>
                </LineClamp>
            {/if}

            {#if item.subtitle}
                <LineClamp clamp={1}>
                    <p>{item.subtitle}</p>
                </LineClamp>
            {/if}
        </div>

        <div class="button-container">
            <span class="get-button gray">
                {$i18n.t('ASE.Web.AppStore.View')}
            </span>
        </div>
    </article>
</LinkWrapper>

<style>
    article {
        position: relative;
        aspect-ratio: 0.9;
        height: 100%;
        padding: 16px;
        gap: 10px;
        display: flex;
        align-items: center;
        flex-direction: column;
        justify-content: center;
        text-align: center;
        border-radius: var(--global-border-radius-xlarge);
        background: var(--systemPrimary-onDark);
        box-shadow: var(--shadow-small);
        container-type: inline-size;
        container-name: container;

        @media (prefers-color-scheme: dark) {
            background: var(--systemQuaternary);
        }

        @media (--sidebar-visible) and (--range-xsmall-only) {
            aspect-ratio: 1;
        }

        @media (--range-medium-up) {
            aspect-ratio: 1;
        }
    }

    .app-icon-container {
        flex-shrink: 0;
        margin-top: 4px;
        aspect-ratio: var(--icon-aspect-ratio);
        height: clamp(40px, 40cqi, 100px);
        width: auto;
    }

    .metadata-container {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    h3 {
        text-wrap: balance;
        font: var(--body-emphasized);
        line-height: 1.1;
        color: var(--title-color);
    }

    h4 {
        text-transform: uppercase;
        font: var(--subhead-emphasized);
        color: var(--systemSecondary);
    }

    p {
        font: var(--subhead);
        color: var(--systemSecondary);
    }

    .button-container {
        --get-button-font: var(--subhead-bold);
        align-content: end;
        flex-grow: 1;
    }

    .ordinal {
        position: absolute;
        top: 12px;
        inset-inline-start: 12px;
        font: var(--title-1-semibold);
        color: var(--systemTertiary);
    }

    @container container (width >= 180px) {
        h3 {
            font: var(--title-3-emphasized);
        }
    }

    @container container (width >= 250px) {
        h3 {
            font: var(--title-2-emphasized);
            margin-bottom: 4px;
        }

        p {
            font: var(--body);
        }
    }

    @container container (width >= 200px) {
        .button-container {
            --get-button-font: unset;
        }
    }
</style>
