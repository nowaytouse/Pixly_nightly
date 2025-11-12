<!--
@component
Renders a set of `Shelf` items in either a horizontal shelf
or a grid, depending on the `shelf` configuration

Note: when configuring the `gridType` property, a single value will be used
for both the shelf-based or grid-based item layouts. If two different grid types
are needed instead, `gridTypeForShelf` and `gridTypeForGrid` are needed instead;
these properties cannot be used alongside the general-purpose `gridType`.
-->
<script lang="ts" generics="Item">
    import type { Shelf } from '@jet-app/app-store/api/models';

    import type { GridType } from '@amp/web-app-components/src/components/Shelf/types';

    import type { XOR } from '~/utils/types';
    import HorizontalShelf from '~/components/jet/shelf/HorizontalShelf.svelte';
    import Grid from '~/components/Grid.svelte';

    /**
     * The sub-set of {@linkcode Shelf} that is necesary to render this component
     */
    interface RequiredShelf
        extends Pick<Shelf, 'rowsPerColumn' | 'isHorizontal'> {
        items: Item[];
    }

    interface $$Slots {
        default: {
            item: Item;
        };
    }

    /**
     * Represents the `gridType` properties of this component
     *
     * Either a `gridType` that will be used for both the shelf or grid
     * layouts can be provided, OR specific properties for the grid type
     * for the shelf and grid respectively; this `XOR` here prevents
     * these approachs from being mixed-and-matched.
     */
    type GeneralOrIndividualGridType = XOR<
        {
            gridType: GridType;
        },
        {
            gridTypeForGrid: GridType;
            gridTypeForShelf: GridType;
        }
    >;

    type $$Props = GeneralOrIndividualGridType & {
        shelf: RequiredShelf;
        rowsPerColumnOverride?: number | null;
    };

    /**
     * The shelf to render items for
     */
    export let shelf: RequiredShelf;

    /**
     * An optional override of the shelfs `rowsPerColumn` property
     */
    export let rowsPerColumnOverride: number | null = null;

    /**
     * Determine the grid type configuration for the shelf or grid layouts
     * based on the mutually-exclusive properties of {@linkcode GeneralOrIndividualGridType}
     */
    function extractGridTypes(props: $$Props) {
        if (typeof props.gridType === 'string') {
            return {
                gridTypeForShelf: props.gridType,
                gridTypeForGrid: props.gridType,
            };
        } else {
            return props;
        }
    }

    $: ({ gridTypeForShelf, gridTypeForGrid } = extractGridTypes(
        $$props as $$Props,
    ));

    $: isHorizontal = shelf.isHorizontal;
    $: gridRows = rowsPerColumnOverride ?? shelf.rowsPerColumn ?? undefined;
</script>

{#if isHorizontal}
    <HorizontalShelf
        items={shelf.items}
        {gridRows}
        gridType={gridTypeForShelf}
        let:item
    >
        <slot {item} />
    </HorizontalShelf>
{:else}
    <Grid items={shelf.items} gridType={gridTypeForGrid} let:item>
        <slot {item} />
    </Grid>
{/if}
