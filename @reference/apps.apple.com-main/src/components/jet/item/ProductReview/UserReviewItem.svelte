<script lang="ts" context="module">
    import {
        type Review as ReviewModel,
        ProductReview,
    } from '@jet-app/app-store/api/models';

    interface UserReview extends ProductReview {
        sourceType: 'user';
        review: ReviewModel;
    }

    export function isUserReviewItem(
        productReview: ProductReview,
    ): productReview is UserReview {
        return productReview.sourceType === 'user';
    }
</script>

<script lang="ts">
    import ReviewItem from '~/components/jet/item/ReviewItem.svelte';

    export let item: UserReview;
</script>

<ReviewItem item={item.review} />
