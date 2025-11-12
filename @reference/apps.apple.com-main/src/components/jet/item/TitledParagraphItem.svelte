<script lang="ts" context="module">
    import type {
        ShelfModel,
        TitledParagraph,
    } from '@jet-app/app-store/api/models';

    export function isTitledParagraphItem(
        item: ShelfModel | string,
    ): item is TitledParagraph {
        return typeof item !== 'string' && 'text' in item;
    }
</script>

<script lang="ts">
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';
    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import { getNumericDateFromDateString } from '@amp/web-app-components/src/utils/date';
    import { getJet } from '~/jet/svelte';
    import { getI18n } from '~/stores/i18n';

    export let item: TitledParagraph;

    const i18n = getI18n();
    const jet = getJet();
    const isDetailView = item.style === 'detail';
    const dateForDisplay = jet.localization.timeAgo(
        new Date(item.secondarySubtitle),
    );
    const dateForAttribute = getNumericDateFromDateString(
        item.secondarySubtitle,
    );

    let isTruncated = true;
</script>

<article class:detail={isDetailView} class:overview={!isDetailView}>
    <div class="container">
        <p>
            {#if item.text}
                {#if !isTruncated || isDetailView}
                    {item.text}
                {:else}
                    <LineClamp
                        clamp={5}
                        observe
                        on:resize={({ detail }) =>
                            (isTruncated = detail.truncated)}
                    >
                        {@html sanitizeHtml(item.text)}
                    </LineClamp>

                    {#if isTruncated}
                        <button on:click={() => (isTruncated = false)}>
                            {$i18n.t('ASE.Web.AppStore.More')}
                        </button>
                    {/if}
                {/if}
            {/if}
        </p>

        <div class="metadata">
            <h4>{item.primarySubtitle}</h4>
            <time datetime={dateForAttribute}>{dateForDisplay}</time>
        </div>
    </div>
</article>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;

    article {
        display: flex;
        flex-direction: column-reverse;
        font: var(--body-tall);
        color: var(--systemPrimary);
        margin: 0 var(--bodyGutter);

        @media (--range-small-up) {
            flex-direction: row;
        }
    }

    .container {
        display: flex;
        width: 100%;
    }

    p {
        position: relative;
        display: flex;
        flex-direction: column;
        white-space: break-spaces;
        font: var(--body-tall);
    }

    .metadata {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        margin: 0 0 8px 8px;
        text-align: end;
        color: var(--systemSecondary);
    }

    h4 {
        font: var(--body-tall);
    }

    button {
        --gradient-direction: 270deg;
        position: absolute;
        bottom: 0;
        display: flex;
        justify-content: end;
        color: var(--keyColor);
        inset-inline-end: 0;
        padding-inline-start: 20px;
        background: linear-gradient(
            var(--gradient-direction),
            var(--pageBg) 72%,
            transparent 100%
        );

        @include rtl {
            --gradient-direction: 90deg;
        }
    }

    time {
        color: var(--systemSecondary);
        white-space: nowrap;
    }

    .detail {
        flex-direction: column-reverse;
        margin: 0;
        padding: 16px 0 0;
        border-top: 1px solid var(--systemGray4);
    }

    .detail .metadata {
        gap: 2px;
    }

    .detail h4 {
        font: var(--body-emphasized-tall);
        color: var(--systemPrimary);
    }

    .overview .container {
        @media (--range-medium-up) {
            width: 66%;
        }
    }

    .overview .metadata {
        flex-grow: 1;
        gap: 4px;
    }

    .overview p {
        @media (--range-small-up) {
            width: 66%;
        }

        @media (--range-large-up) {
            width: 50%;
        }
    }

    .detail .container {
        justify-content: space-between;
    }
</style>
