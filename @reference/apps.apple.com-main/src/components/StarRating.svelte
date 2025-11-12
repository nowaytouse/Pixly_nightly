<script lang="ts" context="module">
    export function calculateStarFillPercentages(rating: number) {
        return [1, 2, 3, 4, 5].map((position) => {
            if (position <= Math.floor(rating)) {
                return 100;
            }

            if (position > Math.ceil(rating)) {
                return 0;
            }

            return Math.round((rating % 1) * 100);
        });
    }
</script>

<script lang="ts">
    import StarFilledIcon from '@amp/web-app-components/assets/icons/star-filled.svg';
    import StarHollowIcon from '@amp/web-app-components/assets/icons/star-hollow.svg';
    import { getI18n } from '~/stores/i18n';

    export let rating: number;

    const i18n = getI18n();

    $: starFillPercentages = calculateStarFillPercentages(rating);
    $: label = $i18n.t('ASE.Web.AppStore.Review.StarsAria', {
        count: rating,
    });
</script>

<ol class="stars" aria-label={label}>
    {#each starFillPercentages as fillPercent}
        <li class="star">
            {#if fillPercent === 100}
                <StarFilledIcon />
            {:else if fillPercent === 0}
                <StarHollowIcon />
            {:else}
                <div
                    class="partial-star"
                    style:--partial-star-width={`${fillPercent}%`}
                >
                    <StarFilledIcon />
                </div>

                <StarHollowIcon />
            {/if}
        </li>
    {/each}
</ol>

<style>
    .stars {
        display: flex;
    }

    .star {
        position: relative;
        margin-inline-end: 2px;
        line-height: 0;
    }

    .star :global(svg) {
        height: var(--star-size, 10px);
        width: var(--star-size, 10px);
        fill: var(--fill-color, rgb(127, 127, 127));
    }

    .partial-star {
        position: absolute;
        overflow: hidden;
        width: var(--partial-star-width);
        fill: var(--fill-color, rgb(127, 127, 127));
    }

    .partial-star :global(path) {
        stroke: transparent;
    }
</style>
