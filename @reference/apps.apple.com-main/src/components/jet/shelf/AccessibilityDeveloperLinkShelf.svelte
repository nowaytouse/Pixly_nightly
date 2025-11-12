<script lang="ts" context="module">
    import type {
        AccessibilityParagraph,
        Shelf,
    } from '@jet-app/app-store/api/models';

    interface AccessibilityDeveloperLinkShelf extends Shelf {
        items: [AccessibilityParagraph];
    }

    export function isAccessibilityDeveloperLinkShelf(
        shelf: Shelf,
    ): shelf is AccessibilityDeveloperLinkShelf {
        let { contentType, items, title } = shelf;

        return (
            contentType === 'accessibilityParagraph' &&
            !title &&
            Array.isArray(items)
        );
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import AccessibilityParagraphItem from '../item/AccessibilityParagraphItem.svelte';
    import { getAccessibilityLayoutConfiguration } from '~/context/accessibility-layout';

    export let shelf: AccessibilityDeveloperLinkShelf;

    $: ({ withBottomPadding } = getAccessibilityLayoutConfiguration(shelf));
</script>

<ShelfWrapper {shelf} centered {withBottomPadding}>
    <AccessibilityParagraphItem item={shelf.items[0]} />
</ShelfWrapper>
