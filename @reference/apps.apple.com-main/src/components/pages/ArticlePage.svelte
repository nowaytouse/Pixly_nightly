<script lang="ts" context="module">
    import type { ArticlePage } from '@jet-app/app-store/api/models';

    import type { DefaultPageRequirements } from './DefaultPage.svelte';

    /**
     * Just the `Page` props that we actually need to render this component
     */
    export type ArticlePageRequirements = DefaultPageRequirements & {
        card: ArticlePage['card'];
        footerLockup: ArticlePage['footerLockup'];
    };
</script>

<script lang="ts">
    import TodayCard from '~/components/jet/today-card/TodayCard.svelte';
    import ShelfComponent from '~/components/jet/shelf/Shelf.svelte';
    import FooterLockupItem from '~/components/jet/item/FooterLockupItem.svelte';
    export let page: ArticlePageRequirements;

    $: ({ card } = page);
</script>

<div class="article-page-container" data-testid="article-page-container">
    <div class="article-layout">
        {#if card}
            <div class="card-container">
                <TodayCard {card} suppressClickAction />
            </div>
        {/if}

        <div class="story-container">
            {#each page.shelves as shelf}
                {#if !shelf.isHidden}
                    <ShelfComponent {shelf} />
                {/if}
            {/each}

            {#if page.footerLockup}
                <div class="footer-lockup-container">
                    <FooterLockupItem item={page.footerLockup} />
                </div>
            {/if}
        </div>
    </div>
</div>

<style lang="scss">
    @use 'ac-sasskit/modules/viewportcontent/core' as *;
    @use 'amp/stylekit/core/viewports' as *;

    .article-page-container {
        flex-grow: 1;
        width: 100%;
        margin: 0 auto;
    }

    .article-layout {
        --article-page-padding: 20px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: var(--article-page-padding);
        max-width: 1600px;
        margin: 0 auto;

        @media (--range-small-up) {
            padding: 2em var(--bodyGutter);
        }

        @media (--range-small-only) {
            --article-page-padding: 40px;
        }

        @media (--range-medium-up) {
            align-items: flex-start;
            flex-direction: row;
        }

        @media (--range-medium-only) {
            --article-page-padding: 20px;
        }

        @media (--range-large-up) {
            --article-page-padding: 40px;
        }
    }

    .card-container {
        flex-shrink: 0;
        aspect-ratio: 3/4;
        width: 100%;

        @media (--range-xsmall-only) {
            --border-radius: 0;
        }

        @media (--range-small-only) {
            aspect-ratio: 16/9;
        }

        @media (--range-small-up) {
            width: 100%;
        }

        @media (--range-medium-up) {
            position: sticky;
            top: 2em;
            aspect-ratio: 3 / 4;
            height: min(calc(100vh - 80px), calc(33vw * 4 / 3));
            min-height: 420px;
            max-height: 700px;
            width: auto;
        }
    }

    .story-container {
        width: 100%;
        margin-top: 20px;
        padding-bottom: var(--bodyGutter);

        @media (--range-small-up) {
            width: calc(100%);
            margin-top: 0;
        }

        @media (--range-medium-up) {
            min-width: calc(50% - calc(var(--article-page-padding)));
        }
    }

    .story-container :global(.shelf:first-of-type) {
        padding-top: 0;
        padding-bottom: 13px;
    }

    .footer-lockup-container {
        margin: var(--bodyGutter);
    }
</style>
