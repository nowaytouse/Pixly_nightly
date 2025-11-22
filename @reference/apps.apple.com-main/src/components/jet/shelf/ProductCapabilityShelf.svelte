<script lang="ts" context="module">
    import type {
        Shelf,
        ProductCapability,
    } from '@jet-app/app-store/api/models';

    interface ProductCapabilityShelf extends Shelf {
        items: ProductCapability[];
    }

    export function isProductCapabilityShelf(
        shelf: Shelf,
    ): shelf is ProductCapabilityShelf {
        const { contentType, items } = shelf;
        return contentType === 'productCapability' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ProductCapabilityItem from '../item/ProductCapabilityItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: ProductCapabilityShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="SearchLink" let:item>
        <ProductCapabilityItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
