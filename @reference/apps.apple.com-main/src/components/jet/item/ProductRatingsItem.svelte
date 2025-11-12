<script lang="ts">
    import type { Ratings } from '@jet-app/app-store/api/models';

    import RatingComponent from '@amp/web-app-components/src/components/Rating/Rating.svelte';
    import { getJet } from '~/jet/svelte';
    import { getI18n } from '~/stores/i18n';

    export let item: Ratings;

    const i18n = getI18n();
    const jet = getJet();
    const numberOfRatings = jet.localization.formattedCount(
        item.totalNumberOfRatings,
    );
</script>

<article>
    {#if item.totalNumberOfRatings === 0}
        {item.status}
    {:else}
        <RatingComponent
            averageRating={jet.localization.decimal(item.ratingAverage, 1)}
            ratingCount={item.totalNumberOfRatings}
            ratingCountText={$i18n.t('ASE.Web.AppStore.Ratings.CountText', {
                numberOfRatings: numberOfRatings,
            })}
            totalText={$i18n.t('ASE.Web.AppStore.Ratings.TotalText')}
            ratingCountsList={item.ratingCounts}
        />
    {/if}
</article>

<style>
    article {
        --ratingBarColor: var(--systemPrimary);
    }
</style>
