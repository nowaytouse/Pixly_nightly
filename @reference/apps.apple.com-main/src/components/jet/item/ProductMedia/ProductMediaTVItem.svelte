<script lang="ts">
    import type { ProductMediaItem } from '@jet-app/app-store/api/models';
    import Artwork from '~/components/Artwork.svelte';
    import Video from '~/components/jet/Video.svelte';

    export let item: ProductMediaItem;
</script>

{#if item.screenshot || item.video}
    <article>
        <div class="artwork-container">
            {#if item.screenshot}
                <Artwork artwork={item.screenshot} profile="screenshot-tv" />
            {:else if item.video}
                <Video autoplay video={item.video} profile="screenshot-tv" />
            {/if}
        </div>
    </article>
{/if}

<style>
    .artwork-container,
    .artwork-container :global(video) {
        border-radius: 1.3% / 1.9%;
        overflow: hidden;

        /* This `transform` is required to make the `overflow: hidden` clip properly on Chrome */
        transform: translateZ(0);
    }

    .artwork-container :global(video):fullscreen {
        border-radius: 0;
    }
</style>
