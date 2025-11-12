<script lang="ts">
    import type {
        ProductMediaItem,
        MediaType,
    } from '@jet-app/app-store/api/models';
    import { getAspectRatio } from '@amp/web-app-components/src/components/Artwork/utils/artProfile';
    import Artwork from '~/components/Artwork.svelte';
    import Video from '~/components/jet/Video.svelte';
    import type { NamedProfile } from '~/config/components/artwork';

    export let item: ProductMediaItem;
    export let hasPortraitMedia: boolean;
    export let mediaType: MediaType | undefined;

    const getArtworkProfile = (
        mediaType: MediaType | undefined,
        hasPortraitMedia: boolean,
    ): NamedProfile => {
        const suffix = hasPortraitMedia ? '_portrait' : '';

        // Map specific media types to their artwork profile names
        const mediaTypeProfiles: Record<string, string> = {
            iphone_6_5: 'screenshot-iphone_6_5',
            iphone_5_8: 'screenshot-iphone_5_8',
            iphone_d74: 'screenshot-iphone_d74',
        };

        const baseProfile =
            mediaType && mediaTypeProfiles[mediaType]
                ? mediaTypeProfiles[mediaType]
                : 'screenshot-phone';

        return `${baseProfile}${suffix}` as NamedProfile;
    };

    $: isLandscapeScreenshot =
        item.screenshot && item.screenshot.width > item.screenshot.height;
    $: profile = getArtworkProfile(mediaType, !isLandscapeScreenshot);
    $: restOfShelfAspectRatio = getAspectRatio(
        getArtworkProfile(mediaType, hasPortraitMedia),
    );
</script>

{#if item.screenshot || item.video}
    <article
        class:with-rotated-artwork={isLandscapeScreenshot && hasPortraitMedia}
        style:--aspect-ratio={`${restOfShelfAspectRatio}`}
    >
        <div
            class="artwork-container"
            class:iphone-6-5={mediaType === 'iphone_6_5'}
            class:iphone-5-8={mediaType === 'iphone_5_8'}
            class:iphone-d74={mediaType === 'iphone_d74'}
            class:portrait={hasPortraitMedia}
        >
            {#if item.screenshot}
                <Artwork
                    {profile}
                    artwork={item.screenshot}
                    disableAutoCenter={true}
                    withoutBorder={true}
                />
            {:else if item.video}
                <Video autoplay video={item.video} {profile} />
            {/if}
        </div>
    </article>
{/if}

<style>
    article.with-rotated-artwork {
        position: relative;
        aspect-ratio: var(--aspect-ratio);
    }

    /*
     * For iPhone screenshots that are landscape, but in a shelf/list with portrait screenshots,
     * as denoted by `hasPortraitMedia`, we rotate the landscape screenshot to be in the portrait
     * orientation, and scale it up so it fills the container.
     */
    article.with-rotated-artwork .artwork-container {
        position: absolute;
        top: 50%;
        left: 50%;
        height: auto;
        width: calc((1 / var(--aspect-ratio)) * 100%);
        transform: translate(-50%, -50%) rotate(-90deg);
        transform-origin: center;
    }

    .artwork-container,
    .artwork-container :global(video) {
        mask-position: center;
        mask-repeat: no-repeat;
        mask-size: 100%;
        border-radius: 20px;
        overflow: hidden;

        /* This `transform` is required to make the `overflow: hidden` clip properly on Chrome */
        transform: translateZ(0);
    }

    .iphone-5-8,
    .iphone-5-8 :global(video) {
        /* need to confirm with design for correct value */
        border-radius: 23px;
        mask-image: url('/assets/images/masks/iphone-5-8-mask-landscape.svg');
    }

    .iphone-5-8.portrait,
    .iphone-5-8.portrait :global(video) {
        mask-image: url('/assets/images/masks/iphone-5-8-mask.svg');
    }

    .iphone-6-5,
    .iphone-6-5 :global(video) {
        /* need to confirm with design for correct value */
        border-radius: 21px;
        mask-image: url('/assets/images/masks/iphone-6-5-mask-landscape.svg');
    }

    .iphone-6-5.portrait,
    .iphone-6-5.portrait :global(video) {
        mask-image: url('/assets/images/masks/iphone-6-5-mask.svg');
    }

    .iphone-d74,
    .iphone-d74 :global(video) {
        border-radius: 5.7% / 12.8%;
    }

    .iphone-d74.portrait,
    .iphone-d74.portrait :global(video) {
        border-radius: 12.8% / 5.7%;
    }

    .artwork-container :global(video):fullscreen {
        mask-image: none;
        border-radius: 0;
        object-fit: contain;
    }
</style>
