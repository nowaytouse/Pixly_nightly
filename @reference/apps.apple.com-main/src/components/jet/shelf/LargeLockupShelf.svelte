<script lang="ts" context="module">
    import type { Lockup, Shelf } from '@jet-app/app-store/api/models';

    interface LargeLockupShelf extends Shelf {
        items: Lockup[];
    }

    export function isLargeLockupShelf(
        shelf: Shelf,
    ): shelf is LargeLockupShelf {
        const { contentType, items } = shelf;
        return contentType === 'largeLockup' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import LargeLockupItem from '~/components/jet/item/LargeLockupItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: LargeLockupShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="LargeLockup" let:item>
        <LargeLockupItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
