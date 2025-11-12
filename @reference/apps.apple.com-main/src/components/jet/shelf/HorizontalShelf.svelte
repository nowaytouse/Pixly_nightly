<script lang="ts">
    import type { Opt } from '@jet/environment';
    import Shelf from '@amp/web-app-components/src/components/Shelf/Shelf.svelte';
    import type {
        ArrowOffset,
        GridType,
    } from '@amp/web-app-components/src/components/Shelf/types';
    import { getI18n } from '~/stores/i18n';

    type T = $$Generic;

    export let items: T[];
    export let gridType: GridType;
    export let gridRows: number = 1;
    export let arrowOffset: Opt<ArrowOffset> = null;

    const i18n = getI18n();
    // This makes the let:item of type T, because it doesn't know type when it comes back from the Shelf component.
    function castGenericItem(x: T): T {
        return x;
    }
</script>

<div class="horizontal-shelf" data-test-id="horizontal-shelf">
    <Shelf translateFn={$i18n.t} {items} {gridType} {gridRows} {arrowOffset}>
        <svelte:fragment slot="item" let:item let:index let:numberOfItems>
            <slot item={castGenericItem(item)} {index} {numberOfItems} />
        </svelte:fragment>
    </Shelf>
</div>

<style>
    .horizontal-shelf :global(.shelf-grid) {
        --shelfGridPaddingInline: var(--bodyGutter);
        --shelfGridGutterWidth: var(--bodyGutter);
    }

    .horizontal-shelf :global(.shelf-grid__list) {
        @media (--range-xsmall-only) {
            scroll-padding-inline-start: var(
                --shelfScrollPaddingInline,
                var(--bodyGutter)
            );
        }
    }

    .horizontal-shelf
        :global(.shelf-grid__list--grid-type-Spotlight .shelf-grid__list-item) {
        @media (--range-xsmall-only) {
            --standard-lockup-shadow-offset: var(--shelfScrollPaddingInline, 0);
        }
    }
</style>
