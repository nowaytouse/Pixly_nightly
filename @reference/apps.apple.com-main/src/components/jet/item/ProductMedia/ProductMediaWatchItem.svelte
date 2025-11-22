<script lang="ts">
    import type {
        ProductMediaItem,
        MediaType,
    } from '@jet-app/app-store/api/models';
    import Artwork from '~/components/Artwork.svelte';

    export let item: ProductMediaItem;
    export let mediaType: MediaType | undefined;
</script>

{#if item.screenshot}
    <article>
        <div
            class="artwork-container"
            class:apple-watch-2018={mediaType === 'appleWatch_2018'}
            class:apple-watch-2021={mediaType === 'appleWatch_2021'}
            class:apple-watch-2022={mediaType === 'appleWatch_2022'}
            class:apple-watch-2024={mediaType === 'appleWatch_2024'}
        >
            <Artwork artwork={item.screenshot} profile="screenshot-watch" />
        </div>
    </article>
{/if}

<style>
    .artwork-container {
        mask-position: center;
        mask-size: contain;
        mask-repeat: no-repeat;
        border-radius: 12px;
        overflow: hidden;
    }

    .apple-watch-2018 {
        mask-image: url('/assets/images/masks/apple-watch-2018-mask.svg');
    }

    .apple-watch-2021 {
        mask-image: url('/assets/images/masks/apple-watch-2021-mask.svg');
    }

    .apple-watch-2022 {
        mask-image: url('/assets/images/masks/apple-watch-2022-mask.svg');
    }

    .apple-watch-2024 {
        mask-image: url('/assets/images/masks/apple-watch-2024-mask.svg');
    }
</style>
