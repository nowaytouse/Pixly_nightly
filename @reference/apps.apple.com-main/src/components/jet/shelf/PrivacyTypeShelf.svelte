<script lang="ts" context="module">
    import type { PrivacyType, Shelf } from '@jet-app/app-store/api/models';

    export interface PrivacyTypeShelf extends Shelf {
        items: PrivacyType[];
    }

    export function isPrivacyTypeShelf(
        shelf: Shelf,
    ): shelf is PrivacyTypeShelf {
        let { contentType, items } = shelf;

        return contentType === 'privacyType' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import PrivacyTypeItem from '~/components/jet/item/PrivacyTypeItem.svelte';

    export let shelf: PrivacyTypeShelf;
</script>

<ShelfWrapper {shelf} withBottomPadding={false}>
    <ShelfItemLayout {shelf} gridType="B" let:item>
        <PrivacyTypeItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
