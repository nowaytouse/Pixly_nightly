<script lang="ts">
    import type {
        ProductMediaItem,
        MediaType,
    } from '@jet-app/app-store/api/models';
    import Artwork from '~/components/Artwork.svelte';
    import Video from '~/components/jet/Video.svelte';

    export let item: ProductMediaItem;
    export let hasPortraitMedia: boolean;
    export let mediaType: MediaType | undefined;
</script>

{#if item.screenshot || item.video}
    <article>
        <div
            class="artwork-container"
            class:ipad-pro-2018={mediaType === 'ipadPro_2018'}
            class:ipad-11={mediaType === 'ipad_11'}
            class:portrait={hasPortraitMedia}
        >
            {#if item.screenshot}
                <Artwork
                    artwork={item.screenshot}
                    profile={hasPortraitMedia
                        ? 'screenshot-pad-portrait'
                        : 'screenshot-pad'}
                />
            {:else if item.video}
                <Video
                    autoplay
                    video={item.video}
                    profile={hasPortraitMedia
                        ? 'screenshot-pad-portrait'
                        : 'screenshot-pad'}
                />
            {/if}
        </div>
    </article>
{/if}

<style>
    .artwork-container,
    .artwork-container :global(video) {
        mask-position: center;
        mask-repeat: no-repeat;
        mask-size: contain;
        border-radius: 1.3% / 1.9%;
        overflow: hidden;

        /* This `transform` is required to make the `overflow: hidden` clip properly on Chrome */
        transform: translateZ(0);
    }

    .artwork-container.portrait {
        aspect-ratio: 3/4;
        background: var(--systemQuaternary);
    }

    .artwork-container.portrait,
    .artwork-container.portrait :global(video) {
        border-radius: 1.9% / 1.3%;
    }

    .ipad-pro-2018,
    .ipad-pro-2018 :global(video) {
        mask-image: url('/assets/images/masks/ipad-pro-2018-mask-landscape.svg');
    }

    .ipad-pro-2018.portrait,
    .ipad-pro-2018.portrait :global(video) {
        mask-image: url('/assets/images/masks/ipad-pro-2018-mask.svg');
    }

    .ipad-11,
    .ipad-11 :global(video) {
        mask-image: url('/assets/images/masks/ipad-11-mask-landscape.svg');
    }

    .ipad-11.portrait,
    .ipad-11.portrait :global(video) {
        mask-image: url('/assets/images/masks/ipad-11-mask.svg');
    }

    .artwork-container :global(video):fullscreen {
        mask-image: none;
        border-radius: 0;
    }
</style>
