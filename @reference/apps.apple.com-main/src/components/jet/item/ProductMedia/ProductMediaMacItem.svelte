<script lang="ts">
    import type { ProductMediaItem } from '@jet-app/app-store/api/models';
    import Artwork from '~/components/Artwork.svelte';
    import Video from '~/components/jet/Video.svelte';

    export let item: ProductMediaItem;
</script>

{#if item.screenshot}
    <article>
        <Artwork artwork={item.screenshot} profile="screenshot-mac" />
    </article>
{:else if item.video}
    <article>
        <Video autoplay video={item.video} profile="screenshot-mac" />
    </article>
{/if}

<style>
    article {
        overflow: hidden;
    }

    article :global(.video) {
        aspect-ratio: 16/10;
    }

    article :global(video) {
        object-fit: cover;
    }
</style>
