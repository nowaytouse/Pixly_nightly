<script lang="ts">
    import type { SeeAllPage } from '@jet-app/app-store/api/models';
    import type { WebRenderablePage } from '@jet-app/app-store/api/models/web-renderable-page';

    import DefaultPage from '~/components/pages/DefaultPage.svelte';
    import { getProductPageShelvesForOrdering } from '~/utils/shelves';
    import { setAccessibilityLayoutContext } from '~/context/accessibility-layout';
    import { isSmallLockupShelf } from '~/components/jet/shelf/SmallLockupShelf.svelte';
    import { isProductReviewShelf } from '~/components/jet/shelf/ProductReviewShelf.svelte';
    import { isProductRatingsShelf } from '~/components/jet/shelf/ProductRatingsShelf.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';

    export let page: SeeAllPage & WebRenderablePage;

    $: shelves = getProductPageShelvesForOrdering(page, 'notPurchasedOrdering')
        .filter((shelf) => {
            const isShelfForReviewPage =
                isProductReviewShelf(shelf) || isProductRatingsShelf(shelf);

            return (
                isSmallLockupShelf(shelf) ||
                (isShelfForReviewPage && page.seeAllType === 'reviews')
            );
        })
        .map((shelf) => {
            shelf.isHorizontal = false;
            shelf.seeAllAction = null;
            return shelf;
        });

    $: {
        setAccessibilityLayoutContext({ shelves });
    }
</script>

<DefaultPage page={{ shelves, title: null }}>
    <svelte:fragment slot="before-shelves">
        <h1>
            <LinkWrapper action={page.lockup.clickAction}>
                {page.lockup.title}
            </LinkWrapper>
        </h1>
    </svelte:fragment>
</DefaultPage>

<style>
    h1 {
        font: var(--title-1);
        color: var(--keyColor);
        margin: 13px var(--bodyGutter) 0;
    }

    h1 :global(a:hover) {
        text-decoration: underline;
    }
</style>
