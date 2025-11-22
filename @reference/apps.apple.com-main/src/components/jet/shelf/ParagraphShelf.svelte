<script lang="ts" context="module">
    import type { Paragraph, Shelf } from '@jet-app/app-store/api/models';

    interface ParagraphShelf extends Shelf {
        contentType: 'paragraph';
        items: Paragraph[];
    }

    export function isParagraphShelf(shelf: Shelf): shelf is ParagraphShelf {
        return shelf.contentType === 'paragraph' && Array.isArray(shelf.items);
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ParagraphShelfItem from '~/components/jet/item/ParagraphShelfItem.svelte';

    export let shelf: ParagraphShelf;
</script>

<ShelfWrapper {shelf} withBottomPadding={false}>
    <div slot="title" class="title-container">
        {#if shelf.title}
            <h2>{shelf.title}</h2>
        {/if}
    </div>

    <div class="content-container">
        {#each shelf.items as item}
            <ParagraphShelfItem {item} />
        {/each}
    </div>
</ShelfWrapper>

<style>
    h2 {
        color: var(--systemPrimary);
        font: var(--title-2-emphasized);
        text-wrap: pretty;
        margin: 16px 0;
    }

    .title-container,
    .content-container {
        margin: 0 var(--bodyGutter);
    }

    /* Whenever this shelf is nested in a modal, we don't want to add extra margin since the modal provides its own */
    :global(.modal-content) .content-container {
        margin: unset;
    }
</style>
