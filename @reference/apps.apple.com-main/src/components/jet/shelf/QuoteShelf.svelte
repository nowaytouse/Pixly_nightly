<script lang="ts" context="module">
    import type { Quote, Shelf } from '@jet-app/app-store/api/models';

    interface QuoteShelf extends Shelf {
        contentType: 'quote';
        items: [Quote];
    }

    export function isQuoteShelf(shelf: Shelf): shelf is QuoteShelf {
        return shelf.contentType === 'quote' && Array.isArray(shelf.items);
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: QuoteShelf;

    $: item = shelf.items[0];
</script>

<ShelfWrapper {shelf} withBottomPadding={false}>
    <div class="outer">
        <div class="inner">
            <blockquote>
                {item.text}
            </blockquote>
            <span>{item.credit}</span>
        </div>
    </div>
</ShelfWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/app/core/globalvars' as *;
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;

    .outer {
        display: flex;
        margin-bottom: 24px;
        padding: 0 var(--bodyGutter);
        gap: 6px;
    }

    .outer::before {
        content: '❝';
        font-size: 40px;
        line-height: 2.2rem;
        color: var(--systemSecondary);

        @include rtl {
            content: '❞';
        }
    }

    .inner {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    blockquote {
        font: var(--large-title-emphasized);
        text-wrap: pretty;
    }

    blockquote::after {
        content: '❞';
        color: var(--systemSecondary);

        @include rtl {
            content: '❝';
        }
    }

    span {
        font: var(--title-3);
        color: var(--systemSecondary);
    }
</style>
