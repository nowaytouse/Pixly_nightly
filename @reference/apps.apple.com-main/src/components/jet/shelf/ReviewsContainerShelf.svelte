<script lang="ts" context="module">
    import type {
        Shelf,
        ReviewsContainer,
    } from '@jet-app/app-store/api/models';

    export interface ReviewsContainerShelf extends Shelf {
        items: [ReviewsContainer];
    }

    export function isReviewsContainerShelf(
        shelf: Shelf,
    ): shelf is ReviewsContainerShelf {
        return (
            shelf.contentType === 'reviewsContainer' &&
            Array.isArray(shelf.items)
        );
    }
</script>

<script lang="ts">
    import RatingComponent from '@amp/web-app-components/src/components/Rating/Rating.svelte';

    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import ShelfTitle from '~/components/Shelf/Title.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import Grid from '~/components/Grid.svelte';
    import { getI18n } from '~/stores/i18n';
    import { getJet } from '~/jet/svelte';

    const i18n = getI18n();
    const jet = getJet();

    export let shelf: ReviewsContainerShelf;

    $: reviewsContainer = shelf.items[0];
    $: ({ productAction, ratings } = reviewsContainer);

    $: numberOfRatings = jet.localization.formattedCount(
        ratings.totalNumberOfRatings,
    );
</script>

<ShelfWrapper {shelf}>
    <header slot="title">
        {#if productAction}
            <div class="product-action">
                <LinkWrapper action={productAction}>
                    {productAction.title}
                </LinkWrapper>
            </div>
        {/if}

        <ShelfTitle title={shelf.title ?? ''} />

        <Grid gridType="A" items={[1]}>
            <div class="rating">
                <RatingComponent
                    averageRating={ratings.ratingAverage}
                    ratingCount={ratings.totalNumberOfRatings}
                    ratingCountText={$i18n.t(
                        'ASE.Web.AppStore.Ratings.CountText',
                        {
                            numberOfRatings,
                        },
                    )}
                    ratingCountsList={ratings.ratingCounts}
                    totalText={$i18n.t('ASE.Web.AppStore.Ratings.TotalText')}
                />
            </div>
        </Grid>
    </header>
</ShelfWrapper>

<style>
    .product-action {
        --linkColor: var(--keyColor);
        margin: 0 var(--bodyGutter) 6px;
    }

    .rating {
        --ratingBarColor: var(--systemPrimary);
    }
</style>
