<script lang="ts" context="module">
    import type {
        TodayCardMedia,
        TodayCardMediaBrandedSingleApp,
    } from '@jet-app/app-store/api/models';

    export function isTodayCardMediaBrandedSingleApp(
        media: TodayCardMedia,
    ): media is TodayCardMediaBrandedSingleApp {
        return media.kind === 'brandedSingleApp';
    }
</script>

<script lang="ts">
    import TodayCardMediaWithArtwork from '~/components/jet/today-card/media/TodayCardMediaWithArtwork.svelte';
    import TodayCardMediaVideo from '~/components/jet/today-card/media/TodayCardMediaVideo.svelte';
    import type { Profile } from '~/components/Artwork.svelte';

    export let media: TodayCardMediaBrandedSingleApp;

    /**
     * A `Profile` to override the default for the card's media
     */
    export let artworkProfile: Profile | undefined = undefined;

    // There is a small but non-zero set of old legacy Today Cards that can appear on the Today page,
    // and those cards have their safe area on the left side of the artwork, rather than the center,
    // like all the modern artwork. For those cases, we pin the artwork to the left edge of the card.
    $: pinnedToLeft =
        media.artworkLayoutsWithMetrics[0].ltr.collapsedLayoutInsets.left < 0;
</script>

{#if media.videos.length > 0}
    <TodayCardMediaVideo {media} {artworkProfile} />
{:else if media.artworks.length > 0}
    <TodayCardMediaWithArtwork
        {media}
        {artworkProfile}
        pinArtworkToLeft={pinnedToLeft}
    />
{/if}
