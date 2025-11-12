<script lang="ts" context="module">
    import type { Badge, Shelf } from '@jet-app/app-store/api/models';

    interface ProductBadgeShelf extends Shelf {
        items: Badge[];
    }

    export function isProductBadgeShelf(
        shelf: Shelf,
    ): shelf is ProductBadgeShelf {
        const { contentType, items } = shelf || {};
        return contentType === 'productBadge' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ProductBadgeItem from '~/components/jet/item/ProductBadgeItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: ProductBadgeShelf;

    $: shelf.items = shelf.items.filter(
        (item) => item.type !== 'friendsPlaying',
    );
</script>

<ShelfWrapper {shelf} withBottomPadding={false} withTopMargin={true}>
    <div class="inforibbon-shelf-wrapper">
        <ShelfItemLayout {shelf} gridType="ProductBadge" let:item>
            <ProductBadgeItem {item} />
        </ShelfItemLayout>
    </div>
</ShelfWrapper>

<style lang="scss">
    .inforibbon-shelf-wrapper {
        padding-bottom: 16px;
    }

    .inforibbon-shelf-wrapper :global(ul) {
        display: grid;

        /*
            Here we are overriding the grid template styles from `ShelfItemLayout -> Grid`,
            to make it so the badge row always takes up the full-width of the browser until
            when not in the XS/mobile view.
        */
        @media (--range-small-up) {
            display: flex;
            justify-content: space-between;
        }
    }

    // prevent collapse of focus outlines
    .inforibbon-shelf-wrapper :global(a) {
        display: block;
    }
</style>
