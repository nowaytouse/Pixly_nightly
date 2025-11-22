<script lang="ts" context="module">
    import type {
        HorizontalRule,
        HorizontalRuleStyle,
        Shelf,
    } from '@jet-app/app-store/api/models';

    export interface HorizontalRuleShelf extends Shelf {
        contentType: 'horizontalRule';
        items: [HorizontalRule];
    }

    export function isHorizontalRuleShelf(
        shelf: Shelf,
    ): shelf is HorizontalRuleShelf {
        return (
            shelf.contentType === 'horizontalRule' && Array.isArray(shelf.items)
        );
    }

    function horizontalRuleStyleToBorderStyle(
        style: HorizontalRuleStyle,
    ): string {
        return style.toLowerCase();
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import { colorAsString } from '~/utils/color';

    export let shelf: HorizontalRuleShelf;

    $: item = shelf.items[0];
    $: borderStyle = horizontalRuleStyleToBorderStyle(item.style);
</script>

<ShelfWrapper {shelf} withBottomPadding={false}>
    <hr
        style:color={colorAsString(item.color)}
        style:border-style={borderStyle}
    />
</ShelfWrapper>

<style>
    hr {
        display: block;
        height: 1px;
        border-width: 1px 0 0;
        border-color: currentColor;
        margin: 1em var(--bodyGutter);
        padding: 0;
    }
</style>
