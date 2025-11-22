<script lang="ts" context="module">
    import type { Shelf, TodayCard } from '@jet-app/app-store/api/models';

    interface SmallStoryCardShelf extends Shelf {
        contentType: 'smallStoryCard';
        items: TodayCard[];
    }

    export function isSmallStoryCardShelf(
        shelf: Shelf,
    ): shelf is SmallStoryCardShelf {
        const { contentType, items } = shelf;
        return contentType === 'smallStoryCard' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import SmallStoryCardWithMediaItem, {
        isSmallStoryCardWithMediaItem,
    } from '~/components/jet/item/SmallStoryCardWithMediaItem.svelte';
    import SmallStoryCardWithArtworkItem, {
        isSmallStoryCardWithArtworkItem,
    } from '~/components/jet/item/SmallStoryCardWithArtworkItem.svelte';
    import SmallStoryCardWithMediaRiver, {
        isSmallStoryCardWithMediaRiver,
    } from '~/components/jet/item/SmallStoryCardWithMediaRiver.svelte';
    import SmallStoryCardWithMediaAppIcon, {
        isSmallStoryCardWithMediaAppIcon,
    } from '~/components/jet/item/SmallStoryCardWithMediaAppIcon.svelte';
    import SmallStoryCardMediaBrandedSingleApp, {
        isSmallStoryCardMediaBrandedSingleApp,
    } from '~/components/jet/item/SmallStoryCardMediaBrandedSingleApp.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';

    export let shelf: SmallStoryCardShelf;

    $: ({ isArticleContext = false } = shelf.presentationHints ?? {});
    $: gridType = (() => {
        if (isArticleContext) {
            return 'SmallStoryCard';
        }

        if (shelf.items.some(isSmallStoryCardWithArtworkItem)) {
            return 'D';
        }

        return 'B';
    })();
</script>

<ShelfWrapper {shelf} withBottomPadding={!isArticleContext}>
    <ShelfItemLayout {shelf} {gridType} let:item>
        {#if isSmallStoryCardWithMediaRiver(item)}
            <SmallStoryCardWithMediaRiver {item} />
        {:else if isSmallStoryCardWithMediaAppIcon(item)}
            <SmallStoryCardWithMediaAppIcon {item} />
        {:else if isSmallStoryCardMediaBrandedSingleApp(item)}
            <SmallStoryCardMediaBrandedSingleApp {item} />
        {:else if isSmallStoryCardWithMediaItem(item)}
            <SmallStoryCardWithMediaItem {item} />
        {:else if isSmallStoryCardWithArtworkItem(item)}
            <SmallStoryCardWithArtworkItem {item} />
        {/if}
    </ShelfItemLayout>
</ShelfWrapper>
