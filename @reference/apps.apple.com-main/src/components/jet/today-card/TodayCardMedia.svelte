<script lang="ts">
    import type { TodayCardMedia } from '@jet-app/app-store/api/models';

    import type { Profile } from '~/components/Artwork.svelte';
    import TodayCardMediaAppEvent, {
        isTodayCardMediaAppEvent,
    } from '~/components/jet/today-card/media/TodayCardMediaAppEvent.svelte';
    import TodayCardMediaAppIcon, {
        isTodayCardMediAppIcon,
    } from '~/components/jet/today-card/media/TodayCardMediaAppIcon.svelte';
    import TodayCardMediaBrandedSingleApp, {
        isTodayCardMediaBrandedSingleApp,
    } from '~/components/jet/today-card/media/TodayCardMediaBrandedSingleApp.svelte';
    import TodayCardMediaList, {
        isTodayCardMediaList,
    } from '~/components/jet/today-card/media/TodayCardMediaList.svelte';
    import TodayCardMediaRiver, {
        isTodayCardMediaRiver,
    } from '~/components/jet/today-card/media/TodayCardMediaRiver.svelte';
    import TodayCardMediaWithArtwork, {
        isTodayCardMediaWithArtwork,
    } from '~/components/jet/today-card/media/TodayCardMediaWithArtwork.svelte';
    import TodayCardMediaVideo, {
        isTodayCardMediaVideo,
    } from '~/components/jet/today-card/media/TodayCardMediaVideo.svelte';

    export let media: TodayCardMedia;

    /**
     * A `Profile` to override the default for the card's media
     */
    export let artworkProfile: Profile | undefined = undefined;
</script>

{#if isTodayCardMediaAppEvent(media)}
    <TodayCardMediaAppEvent {media} {artworkProfile} />
{:else if isTodayCardMediAppIcon(media)}
    <TodayCardMediaAppIcon {media} />
{:else if isTodayCardMediaBrandedSingleApp(media)}
    <TodayCardMediaBrandedSingleApp {media} {artworkProfile} />
{:else if isTodayCardMediaList(media)}
    <TodayCardMediaList {media} />
{:else if isTodayCardMediaWithArtwork(media)}
    <TodayCardMediaWithArtwork {media} {artworkProfile} />
{:else if isTodayCardMediaRiver(media)}
    <TodayCardMediaRiver {media} />
{:else if isTodayCardMediaVideo(media)}
    <TodayCardMediaVideo {media} {artworkProfile} />
{/if}
