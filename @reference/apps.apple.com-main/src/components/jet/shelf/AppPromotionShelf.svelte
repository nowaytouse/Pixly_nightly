<script lang="ts" context="module">
    import type {
        AppPromotion,
        AppEvent,
        Shelf,
    } from '@jet-app/app-store/api/models';

    interface AppPromotionShelf extends Shelf {
        items: AppPromotion[];
    }

    export function isAppPromotionShelf(
        shelf: Shelf,
    ): shelf is AppPromotionShelf {
        const { contentType, items } = shelf;
        return contentType === 'appPromotion' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import AppEventItem from '~/components/jet/item/AppEventItem.svelte';
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import mediaQueries from '~/utils/media-queries';

    export let shelf: AppPromotionShelf;

    $: appEventItems = shelf.items.filter(
        (item): item is AppEvent => item.promotionType === 'appEvent',
    );
    $: isArticleContext = shelf.presentationHints?.isArticleContext;
    $: gridType =
        isArticleContext && $mediaQueries !== 'small' ? 'Spotlight' : 'B';
</script>

<ShelfWrapper {shelf} withTopMargin={isArticleContext}>
    <ShelfItemLayout
        shelf={{
            ...shelf,
            items: appEventItems,
        }}
        {gridType}
        let:item
    >
        <AppEventItem {item} {isArticleContext} />
    </ShelfItemLayout>
</ShelfWrapper>
