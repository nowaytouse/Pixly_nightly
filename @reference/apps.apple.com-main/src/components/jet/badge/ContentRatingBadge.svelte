<script lang="ts" context="module">
    import type { Badge, BadgeType } from '@jet-app/app-store/api/models';

    const ARTWORK_TYPE: BadgeType = 'artwork';
    const CONTENT_RATING_TYPE: BadgeType = 'contentRating';
    const CONTENT_RATING_KEY = 'contentRating';

    interface ContentRatingBadge extends Badge {
        type: typeof CONTENT_RATING_TYPE;
    }

    export function isContentRatingBadge(
        badge: Badge,
    ): badge is ContentRatingBadge {
        return (
            badge.type === CONTENT_RATING_TYPE ||
            (badge.key === CONTENT_RATING_KEY && badge.type === ARTWORK_TYPE)
        );
    }
</script>

<script lang="ts">
    import SystemImage, {
        isSystemImageArtwork,
    } from '~/components/SystemImage.svelte';

    export let badge: ContentRatingBadge;

    $: ({ artwork, accessibilityTitle } = badge);
</script>

{#if artwork && isSystemImageArtwork(artwork)}
    <div class="pictogram-container" aria-label={accessibilityTitle}>
        <SystemImage {artwork} />
    </div>
{:else}
    <span>
        {badge.content.contentRating}
    </span>
{/if}

<style>
    span {
        height: 25px;
        margin: 4px 0 2px;
        font: var(--title-1-emphasized);
        color: var(--color);
    }

    .pictogram-container {
        height: 25px;
        padding: 2px;
        aspect-ratio: 1/1;
        margin: 4px 0 2px;
    }

    .pictogram-container :global(svg) {
        width: 21px;
        height: 21px;
    }
</style>
