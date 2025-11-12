<script lang="ts" context="module">
    import type { Shelf, LinkableText } from '@jet-app/app-store/api/models';

    interface LinkableTextShelf extends Shelf {
        contentType: 'linkableText';
        items: [LinkableText];
    }

    export function isLinkableTextShelf(
        shelf: Shelf,
    ): shelf is LinkableTextShelf {
        return (
            shelf.contentType === 'linkableText' && Array.isArray(shelf.items)
        );
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    import LinkableTextItem from '~/components/jet/item/LinkableTextItem.svelte';

    export let shelf: LinkableTextShelf;
</script>

<ShelfWrapper centered withPaddingTop={true}>
    <div class="banner">
        <LinkableTextItem item={shelf.items[0]} />
    </div>
</ShelfWrapper>

<style>
    .banner {
        background: rgba(var(--keyColor-rgb), 0.07);
        padding: 8px 16px;
        text-align: center;
        border-radius: var(--global-border-radius-small);
    }

    .banner :global(a) {
        color: var(--keyColor);
    }
</style>
