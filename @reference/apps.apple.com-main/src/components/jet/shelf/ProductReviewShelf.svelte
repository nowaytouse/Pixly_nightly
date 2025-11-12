<script lang="ts" context="module">
    import type { ProductReview, Shelf } from '@jet-app/app-store/api/models';

    interface ProductReviewShelf extends Shelf {
        items: ProductReview[];
    }

    export function isProductReviewShelf(
        shelf: Shelf,
    ): shelf is ProductReviewShelf {
        let { contentType, items } = shelf;

        return contentType === 'productReview' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import EditorsChoiceReviewItem, {
        isEditorsChoiceReviewItem,
    } from '~/components/jet/item/ProductReview/EditorsChoiceReviewItem.svelte';
    import UserReviewItem, {
        isUserReviewItem,
    } from '~/components/jet/item/ProductReview/UserReviewItem.svelte';

    export let shelf: ProductReviewShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="A" let:item>
        {#if isUserReviewItem(item)}
            <UserReviewItem {item} />
        {:else if isEditorsChoiceReviewItem(item)}
            <EditorsChoiceReviewItem {item} />
        {/if}
    </ShelfItemLayout>
</ShelfWrapper>
