<script lang="ts" context="module">
    import type { AppShowcase, Shelf } from '@jet-app/app-store/api/models';

    interface AppShowcaseShelf extends Shelf {
        contentType: 'appShowcase';
        items: [AppShowcase];
    }

    export function isAppShowcaseShelf(
        shelf: Shelf,
    ): shelf is AppShowcaseShelf {
        return (
            shelf.contentType === 'appShowcase' && Array.isArray(shelf.items)
        );
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import SmallLockup from '~/components/jet/item/SmallLockupItem.svelte';

    export let shelf: AppShowcaseShelf;

    $: item = shelf.items[0];
</script>

<ShelfWrapper {shelf} withTopMargin centered>
    <SmallLockup item={item.lockup} />
</ShelfWrapper>
