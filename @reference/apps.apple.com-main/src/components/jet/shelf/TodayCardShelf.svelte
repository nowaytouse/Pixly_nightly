<script lang="ts" context="module">
    import type {
        Shelf,
        TodayCard as TodayCardModel,
    } from '@jet-app/app-store/api/models';

    export interface TodayCardShelf extends Shelf {
        contentType: 'todayCard';

        items: TodayCardModel[];
    }

    export function isTodayCardShelf(shelf: Shelf): shelf is TodayCardShelf {
        return shelf.contentType === 'todayCard' && Array.isArray(shelf.items);
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import TodayCard from '~/components/jet/today-card/TodayCard.svelte';

    import { getTodayCardLayoutConfiguration } from '~/context/today-card-layout';

    export let shelf: TodayCardShelf;

    $: ({
        wrap: { shouldStretchFirstCard: shouldStretchFirstCardWrap },
        nowrap: { shouldStretchFirstCard: shouldStretchFirstCardNoWrap },
    } = getTodayCardLayoutConfiguration(shelf));
</script>

<ShelfWrapper {shelf}>
    <div>
        <div
            class="today-card-row"
            class:today-card-row__stretch-first-wrap={shouldStretchFirstCardWrap &&
                shelf.items.length >= 2}
            class:today-card-row__stretch-first-nowrap={shouldStretchFirstCardNoWrap &&
                shelf.items.length >= 2}
            class:today-card-row__stretch-last-wrap={!shouldStretchFirstCardWrap &&
                shelf.items.length >= 2}
            class:today-card-row__stretch-last-nowrap={!shouldStretchFirstCardNoWrap &&
                shelf.items.length >= 2}
            class:today-card-row__1-card={shelf.items.length == 1}
            class:today-card-row__2-card={shelf.items.length == 2}
            class:today-card-row__3-card={shelf.items.length == 3}
            class:today-card-row__4-card={shelf.items.length >= 4}
        >
            {#each shelf.items.slice(0, 4) as card}
                <div class="today-card-wrapper">
                    <TodayCard {card} />
                </div>
            {/each}
        </div>
    </div>
</ShelfWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use '@amp/web-shared-styles/app/core/globalvars' as *;
    @use 'ac-sasskit/modules/viewportcontent/core' as *;
    @use 'amp/stylekit/core/viewports' as *;
    @use 'amp/stylekit/core/mixins/browser-targets' as *;

    @mixin stretch-card($flex-shrink: 1) {
        aspect-ratio: unset;
        justify-self: stretch;
        align-self: stretch;
        width: auto;
        flex-shrink: $flex-shrink;
        flex-grow: 1;
    }

    .today-card-row {
        --card-default-width: 407px;
        --card-default-height: 534px;
        --card-row-gap: 16px;
        min-width: min(var(--card-default-width), 100vw);
        padding: 0 25px;
        display: flex;
        flex-direction: column;
        gap: var(--card-row-gap);

        @media (--range-medium-up) {
            padding: 0 40px;
            flex-direction: row;
            flex-wrap: wrap;
        }
    }

    .today-card-wrapper {
        --artworkShadowInset: 0;
        --afterShadowBorderRadius: 0px;
        aspect-ratio: 3 / 4;
        width: 100%;
        flex-shrink: 0;
        max-height: 600px;
        min-height: 100px;

        > :global(a) {
            display: block;
            width: 100%;
            height: 100%;
        }

        @include target-safari {
            @media screen and (760px <= width) {
                height: 600px;
                aspect-ratio: unset;
            }
        }

        @media (--range-medium-up) {
            width: auto;
            height: var(--card-default-height);
            aspect-ratio: 3 / 4;
        }
    }

    @media (--range-medium-up) {
        .today-card-row__1-card .today-card-wrapper {
            @include stretch-card;
        }
    }

    @media (--range-medium-up) and (--range-large-down) {
        .today-card-row__2-card {
            &.today-card-row__stretch-first-wrap
                .today-card-wrapper:first-child,
            &.today-card-row__stretch-last-wrap .today-card-wrapper:last-child {
                @include stretch-card;
            }
        }

        .today-card-row__3-card {
            .today-card-wrapper:first-child {
                flex-basis: 100%;

                @include stretch-card(0);
            }

            &.today-card-row__stretch-first-wrap
                .today-card-wrapper:nth-child(2),
            &.today-card-row__stretch-last-wrap .today-card-wrapper:last-child {
                @include stretch-card;
            }
        }
    }

    @media (--range-medium-up) {
        .today-card-row__4-card {
            &.today-card-row__stretch-first-wrap
                .today-card-wrapper:first-child,
            &.today-card-row__stretch-first-wrap .today-card-wrapper:last-child,
            &.today-card-row__stretch-last-wrap
                .today-card-wrapper:nth-child(2),
            &.today-card-row__stretch-last-wrap
                .today-card-wrapper:nth-child(3) {
                flex-basis: calc(
                    100% - var(--card-default-width) - var(--card-row-gap)
                );

                @include stretch-card;
            }
        }
    }

    @media (--range-xlarge-up) {
        .today-card-row__2-card {
            &.today-card-row__stretch-first-nowrap
                .today-card-wrapper:first-child,
            &.today-card-row__stretch-last-nowrap
                .today-card-wrapper:last-child {
                @include stretch-card;
            }
        }

        .today-card-row__3-card {
            &.today-card-row__stretch-first-nowrap
                .today-card-wrapper:first-child,
            &.today-card-row__stretch-last-nowrap
                .today-card-wrapper:last-child {
                @include stretch-card;
            }
        }
    }
</style>
