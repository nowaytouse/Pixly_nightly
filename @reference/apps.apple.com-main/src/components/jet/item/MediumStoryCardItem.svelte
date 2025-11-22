<script lang="ts" context="module">
    import type {
        EditorialStoryCard,
        TodayCard,
    } from '@jet-app/app-store/api/models';

    export type Item = EditorialStoryCard | TodayCard;

    function isEditorialStoryCard(item: Item): item is EditorialStoryCard {
        return 'artwork' in item;
    }
</script>

<script lang="ts">
    import EditorialStoryCardItem from '~/components/jet/item/MediumStoryCard/EditorialStoryCardItem.svelte';
    import SmallStoryCardWithMediaItem, {
        isSmallStoryCardWithMediaItem,
    } from '~/components/jet/item/SmallStoryCardWithMediaItem.svelte';

    export let item: Item;
</script>

{#if isEditorialStoryCard(item)}
    <EditorialStoryCardItem {item} />
{:else if isSmallStoryCardWithMediaItem(item)}
    <SmallStoryCardWithMediaItem {item} />
{/if}
