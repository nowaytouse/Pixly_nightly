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
                <Artwork
                    artwork={item.screenshot}
                    profile="screenshot-vision"
                />
            {:else if item.video}
                <Video
                    autoplay
                    video={item.video}
                    profile="screenshot-vision"
                />
            {/if}
        </div>
    </article>
{/if}

<style>
    .artwork-container,
    .artwork-container :global(video) {
        border-radius: 20px;
        overflow: hidden;
    }

    .artwork-container :global(video):fullscreen {
        border-radius: 0;
    }
</style>
