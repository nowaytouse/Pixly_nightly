<script lang="ts">
    import type { RatingCountsList } from './types';
    import { calculatePercentages } from './utils';
    import FilledStarIcon from '@amp/web-app-components/assets/icons/star-filled.svg';

    /**
     * @name Rating
     *
     * @description
     * This implements the standard rating lockup showing aggregate ratings
     *
     * Design:
     * https://pd-hi.apple.com/viewvc/Common/Modules/macOS/Podcasts/Lockups/Review%20Lockup.png?revision=57299
     *
     * Aria Discussions:
     * https://quip-apple.com/yvZaAbJMnAK0#JeB9CAOHPMd
     *
     * POTW difference:
     * No write a review on the web
     */

    export let averageRating: number | string;
    export let ratingCount: number;
    export let ratingCountText: string;
    export let ratingCountsList: RatingCountsList;
    export let totalText: string;

    $: ratingPercentList = calculatePercentages(ratingCountsList, ratingCount);
</script>

<div class="amp-rating" data-testid="rating-component">
    <div class="stats" aria-label={`${averageRating} ${totalText}`}>
        <div class="stats__main" data-testid="amp-rating__average-rating">
            {averageRating}
        </div>
        <div class="stats__total" data-testid="amp-rating__total-text">
            {totalText}
        </div>
    </div>
    <div class="numbers">
        <div class="numbers__star-graph">
            {#each ratingPercentList as value, i}
                <div
                    class={`numbers__star-graph__row row-${i}`}
                    aria-label={`${5 - i} star, ${value}%`}
                >
                    <!-- TODO: rdar://79873131 (Localize Aria Label in Rating Shared Component) -->
                    <div class="numbers__star-graph__row__stars">
                        <!-- In order to display the 5 stars to 1 stars we use the 5 - index as 0 index means 1 star and so on -->
                        {#each { length: 5 - i } as _}
                            <div class="star"><FilledStarIcon /></div>
                        {/each}
                    </div>
                    <div class="numbers__star-graph__row__bar">
                        <div
                            class="numbers__star-graph__row__bar__foreground"
                            style={`width: ${value}%`}
                            data-testid={`star-row-${5 - i}`}
                        />
                    </div>
                </div>
            {/each}
        </div>
        <div class="numbers__count" data-testid="amp-rating__rating-count-text">
            {ratingCountText}
        </div>
    </div>
</div>

<style lang="scss">
    .amp-rating {
        display: flex;
    }

    .stats {
        margin-right: 10px;
        flex: 0 80px;
    }

    .stats__main {
        font-size: 50px;
        font-weight: bold;
        display: flex;
        justify-content: center;
    }

    .stats__total {
        display: flex;
        justify-content: center;
        color: var(--systemSecondary-text);
        font: var(--body-emphasized);
    }

    .numbers {
        width: 100%;
    }

    .numbers__count {
        display: flex;
        align-items: flex-end;
        justify-content: flex-end;
        color: var(--systemSecondary-text);
    }

    .numbers__star-graph {
        margin-top: 12px;
        line-height: 9px;
    }

    .numbers__star-graph__row {
        display: flex;
        width: 100%;
    }

    .numbers__star-graph__row__stars {
        display: flex;
        min-width: 45px;
        font-size: 8px;
        justify-content: flex-end;
        margin-right: 6px;

        & :global(.star) {
            fill: var(--systemSecondary);
            width: 8px;
            height: 8px;
        }
    }

    .numbers__star-graph__row__bar {
        height: 2px;
        width: 100%;
        background: var(--systemQuaternary);
        margin-top: 3px;
    }

    .numbers__star-graph__row__bar__foreground {
        height: 2px;
        background: var(--ratingBarColor, --systemSecondary);
        max-width: 100%;
    }
</style>
