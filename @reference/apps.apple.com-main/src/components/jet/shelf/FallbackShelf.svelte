<script lang="ts" context="module">
    import type { Shelf, ShelfModel } from '@jet-app/app-store/api/models';

    interface FallbackShelf extends Shelf {
        items: ShelfModel[];
    }

    export function isFallbackShelf(shelf: Shelf): shelf is FallbackShelf {
        return Array.isArray(shelf.items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: FallbackShelf;

    const isPlaceholder = shelf.contentType === 'placeholder';
</script>

<ShelfWrapper withTopBorder>
    <ShelfItemLayout {shelf} gridType="C">
        <div class="wip">
            {isPlaceholder
                ? `🔄 Placeholder for ${shelf.placeholderContentType}`
                : `🚧 ${shelf.contentType}`}
        </div>
    </ShelfItemLayout>
</ShelfWrapper>

<style>
    .wip {
        background: #f8f8f8;
        padding: 16px;
        border-radius: 8px;
        border: 1px solid #ccc;
    }
</style>
