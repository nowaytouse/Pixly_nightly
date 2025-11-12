<script lang="ts" generics="T">
    import { getGridVars } from '@amp/web-app-components/src/components/Shelf/utils/getGridVars';
    import type { GridType } from '@amp/web-app-components/src/components/Shelf/types';

    export let items: T[] = [];
    export let gridType: GridType;

    $: style = getGridVars(gridType);
</script>

<ul {style} class="grid" data-test-id="grid">
    {#each items as item}
        <li>
            <slot {item} />
        </li>
    {/each}
</ul>

<style lang="scss">
    @mixin grid-styles-for-viewport($viewport: null) {
        grid-template-columns: repeat(var(--grid-#{$viewport}), 1fr);
        column-gap: var(--grid-column-gap-#{$viewport});
        row-gap: var(--grid-row-gap-#{$viewport});
    }

    .grid {
        display: grid;
        width: 100%;
        padding: 0 var(--bodyGutter);

        @each $viewport in ('xsmall', 'small', 'medium', 'large', 'xlarge') {
            @media (--range-#{$viewport}-only) {
                @include grid-styles-for-viewport($viewport);
            }
        }
    }
</style>
