<script lang="ts" context="module">
    import type {
        Shelf,
        ProductDescription,
    } from '@jet-app/app-store/api/models';

    interface ProductDescriptionShelf extends Shelf {
        items: [ProductDescription];
    }

    export function isProductDescriptionShelf(
        shelf: Shelf,
    ): shelf is ProductDescriptionShelf {
        const { contentType, items } = shelf;

        return contentType === 'productDescription' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';
    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import { getI18n } from '~/stores/i18n';

    export let shelf: ProductDescriptionShelf;

    const i18n = getI18n();
    const description = shelf.items[0]?.paragraph.text;
    const handleMoreClick = () => (isOpen = true);
    let isOpen = false;

    function handleLineClampResize(event: CustomEvent) {
        if (!event.detail.truncated) {
            isOpen = true;
        }
    }
</script>

<ShelfWrapper centered>
    <article>
        <p>
            {#if isOpen}
                {@html sanitizeHtml(description)}
            {:else}
                <LineClamp observe clamp={5} on:resize={handleLineClampResize}>
                    {@html sanitizeHtml(description)}
                </LineClamp>
            {/if}

            {#if !isOpen}
                <button on:click={handleMoreClick}>
                    {$i18n.t('ASE.Web.AppStore.More')}
                </button>
            {/if}
        </p>
    </article>
</ShelfWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;

    p {
        white-space: break-spaces;
        font: var(--body-tall);
        position: relative;
        display: flex;
        flex-direction: column;

        @media (--range-medium-up) {
            width: 66%;
        }
    }

    button {
        --gradient-direction: 270deg;
        display: flex;
        justify-content: end;
        position: absolute;
        bottom: 0;
        inset-inline-end: 0;
        padding-inline-start: 20px;
        color: var(--keyColor);
        background: linear-gradient(
            var(--gradient-direction),
            var(--pageBg) 72%,
            transparent 100%
        );

        @include rtl {
            --gradient-direction: 90deg;
        }
    }
</style>
