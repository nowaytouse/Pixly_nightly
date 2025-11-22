<script lang="ts" context="module">
    import type { PrivacyFooter, Shelf } from '@jet-app/app-store/api/models';

    interface PrivacyFooterShelf extends Shelf {
        items: [PrivacyFooter];
    }

    export function isPrivacyFooterShelf(
        shelf: Shelf,
    ): shelf is PrivacyFooterShelf {
        let { contentType, items } = shelf;

        return contentType === 'privacyFooter' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import LinkableTextItem from '~/components/jet/item/LinkableTextItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: PrivacyFooterShelf;

    $: bodyText = shelf.items[0].bodyText;
</script>

<ShelfWrapper {shelf} centered>
    <p>
        <LinkableTextItem item={bodyText} />
    </p>
</ShelfWrapper>

<style>
    p {
        font: var(--body-tall);
    }

    p :global(a) {
        color: var(--keyColor);
    }
</style>
