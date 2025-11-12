<script lang="ts" context="module">
    import type {
        AccessibilityFeatures,
        Shelf,
    } from '@jet-app/app-store/api/models';

    export interface AccessibilityFeaturesShelf extends Shelf {
        items: AccessibilityFeatures[];
    }

    export function isAccessibilityFeaturesShelf(
        shelf: Shelf,
    ): shelf is AccessibilityFeaturesShelf {
        let { contentType, items } = shelf;

        return contentType === 'accessibilityFeatures' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import AccessibilityFeaturesItem from '~/components/jet/item/AccessibilityFeaturesItem.svelte';
    import { getAccessibilityLayoutConfiguration } from '~/context/accessibility-layout';

    export let shelf: AccessibilityFeaturesShelf;

    $: ({ withBottomPadding } = getAccessibilityLayoutConfiguration(shelf));
</script>

<ShelfWrapper {shelf} {withBottomPadding}>
    <ShelfItemLayout {shelf} gridType="B" let:item>
        <AccessibilityFeaturesItem {item} />
    </ShelfItemLayout>
</ShelfWrapper>
