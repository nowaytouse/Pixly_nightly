<script lang="ts">
    import type { MixedMediaLockup } from '@jet-app/app-store/api/models';

    import SmallLockupItem from '~/components/jet/item/SmallLockupItem.svelte';
    import Video from '~/components/jet/Video.svelte';

    export let item: MixedMediaLockup;

    let video = item.trailers?.[0]?.videos[0];
</script>

<div class="mixed-media-lockup-item">
    <div class="video-wrapper">
        {#if video}
            <Video {video} profile="brick" shouldSuperimposePosterImage />
        {/if}
    </div>
    <SmallLockupItem {item} />
</div>

<style>
    .mixed-media-lockup-item {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .video-wrapper {
        --mixed-media-lockup-video-aspect-ratio: 16/9;
        aspect-ratio: var(--mixed-media-lockup-video-aspect-ratio);
        overflow: hidden;
        border-radius: 7px;
    }

    .video-wrapper :global(video) {
        aspect-ratio: var(--mixed-media-lockup-video-aspect-ratio);
        object-fit: cover;
    }
</style>
