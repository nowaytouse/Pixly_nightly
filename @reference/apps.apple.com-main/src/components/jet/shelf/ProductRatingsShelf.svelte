<script lang="ts" context="module">
    import { type Ratings, type Shelf } from '@jet-app/app-store/api/models';

    interface ProductRatingsShelf extends Shelf {
        items: Ratings[];
    }

    export function isProductRatingsShelf(
        shelf: Shelf,
    ): shelf is ProductRatingsShelf {
        let { contentType, items } = shelf;

        return contentType === 'productRatings' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ProductRatingsItem from '~/components/jet/item/ProductRatingsItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: ProductRatingsShelf;
</script>

<ShelfWrapper {shelf} withBottomPadding={false}>
    <ShelfItemLayout {shelf} gridType="A" let:item>
        <ProductRatingsItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
