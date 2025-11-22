<script lang="ts" context="module">
    import type { Shelf, ProductPageLink } from '@jet-app/app-store/api/models';

    interface ProductPageLinkShelf extends Shelf {
        items: ProductPageLink[];
    }

    export function isProductPageLinkShelf(
        shelf: Shelf,
    ): shelf is ProductPageLinkShelf {
        const { contentType, items } = shelf;
        return contentType === 'productPageLink' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ProductPageLinkItem from '~/components/jet/item/ProductPageLinkItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: ProductPageLinkShelf;
</script>

<ShelfWrapper {shelf}>
    <div class="product-page-link-shelf">
        {#each shelf.items as item}
            <li class="product-page-link-item">
                <ProductPageLinkItem {item} />
            </li>
        {/each}
    </div>
</ShelfWrapper>

<style lang="scss">
    $product-page-link-border: 1px solid var(--systemGray4);

    .product-page-link-shelf {
        display: flex;
        justify-content: center;
        align-items: center;
        column-gap: 20px;

        @media (--range-xsmall-down) {
            flex-direction: column;
            align-items: flex-start;
        }
    }

    @media (--range-xsmall-down) {
        .product-page-link-item:first-child {
            border-top: $product-page-link-border;
        }

        .product-page-link-item {
            width: 100%;
            border-bottom: $product-page-link-border;
            padding: 0 var(--bodyGutter);
        }
    }
</style>
