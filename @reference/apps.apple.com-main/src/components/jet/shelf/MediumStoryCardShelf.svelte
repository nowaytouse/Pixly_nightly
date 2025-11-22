<script lang="ts" context="module">
    import type { Shelf } from '@jet-app/app-store/api/models';

    import MediumStoryCardItem, {
        type Item as MediumStoryCardItemModel,
    } from '~/components/jet/item/MediumStoryCardItem.svelte';

    interface MediumStoryCardShelf extends Shelf {
        items: MediumStoryCardItemModel[];
    }

    export function isMediumStoryCardShelf(
        shelf: Shelf,
    ): shelf is MediumStoryCardShelf {
        const { contentType, items } = shelf;
        return contentType === 'mediumStoryCard' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: MediumStoryCardShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="B" let:item>
        <MediumStoryCardItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
