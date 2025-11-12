<script lang="ts" context="module">
    import type { Shelf, Uber } from '@jet-app/app-store/api/models';

    interface UberShelf extends Shelf {
        contentType: 'uber';
        items: [Uber];
    }

    export function isUberShelf(shelf: Shelf): shelf is UberShelf {
        return shelf.contentType === 'uber' && Array.isArray(shelf.items);
    }
</script>

<script lang="ts">
    import Artwork from '~/components/Artwork.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: UberShelf;

    $: uber = shelf.items[0];
    $: artwork = uber.artwork;
</script>

{#if artwork}
    <ShelfWrapper withPaddingTop={false} withBottomPadding={false}>
        <div class="artwork-container">
            <Artwork {artwork} profile="uber-shelf" />
        </div>
    </ShelfWrapper>
{/if}

<style>
    .artwork-container {
        border-bottom: 1px solid var(--systemQuaternary-onDark);

        @media (--range-xlarge-only) {
            border: 1px solid var(--systemQuaternary-onDark);
        }
    }
</style>
