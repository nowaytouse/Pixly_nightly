<script lang="ts" context="module">
    import type {
        TodayCardMedia,
        TodayCardMediaAppEvent,
    } from '@jet-app/app-store/api/models';

    export function isTodayCardMediaAppEvent(
        media: TodayCardMedia,
    ): media is TodayCardMediaAppEvent {
        return media.kind === 'appEvent';
    }
</script>

<script lang="ts">
    import type { Profile } from '~/components/Artwork.svelte';
    import TodayCardMediaWithArtwork from '~/components/jet/today-card/media/TodayCardMediaWithArtwork.svelte';
    import TodayCardMediaVideo from '~/components/jet/today-card/media/TodayCardMediaVideo.svelte';
    import AppEventDate from '~/components/AppEventDate.svelte';

    export let media: TodayCardMediaAppEvent;

    /**
     * A `Profile` to override the default for the card's media
     */
    export let artworkProfile: Profile | undefined = undefined;
</script>

<div class="event-container">
    <span class="time-container">
        <AppEventDate formattedDates={media.formattedDates} />
    </span>

    <div class="artwork-container">
        {#if media.videos.length > 0}
            <TodayCardMediaVideo {media} {artworkProfile} />
        {:else if media.artworks.length > 0}
            <TodayCardMediaWithArtwork {media} {artworkProfile} />
        {/if}
    </div>
</div>

<style>
    .event-container {
        --today-card-border-width: 4px;
        border: var(--today-card-border-width) solid
            var(--today-card-accent-color);
        border-radius: var(--today-card-border-radius);
        position: relative;
        aspect-ratio: 0.75;
        width: 100%;
        height: 100%;
        overflow: hidden;
    }

    @container (orientation: landscape) {
        .event-container {
            aspect-ratio: 16/9;
        }
    }

    .artwork-container {
        height: 100%;
        border-radius: calc(
            var(--today-card-border-radius) - var(--today-card-border-width)
        );
    }

    .time-container :global(time),
    .time-container :global(span) {
        background: var(--today-card-accent-color);
        border-end-end-radius: var(--today-card-border-radius);
        font: var(--headline);
        padding: 6px 10px 6px 8px;
        position: absolute;
        top: 0;
        z-index: 3;
    }
</style>
