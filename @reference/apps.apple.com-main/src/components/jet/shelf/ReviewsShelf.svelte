<script lang="ts" context="module">
    import type {
        Shelf,
        Review as ReviewModel,
    } from '@jet-app/app-store/api/models';

    export interface ReviewsShelf extends Shelf {
        items: ReviewModel[];
    }

    export function isReviewsShelf(shelf: Shelf): shelf is ReviewsShelf {
        return shelf.contentType === 'reviews' && Array.isArray(shelf.items);
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ReviewItem from '~/components/jet/item/ReviewItem.svelte';

    export let shelf: ReviewsShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="A" let:item>
        <ReviewItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
