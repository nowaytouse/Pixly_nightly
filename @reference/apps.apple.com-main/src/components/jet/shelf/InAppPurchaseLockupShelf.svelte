<script lang="ts" context="module">
    import type {
        InAppPurchaseLockup,
        Shelf,
    } from '@jet-app/app-store/api/models';

    interface InAppPurchaseLockupShelf extends Shelf {
        items: InAppPurchaseLockup[];
    }

    export function isInAppPurchaseLockupShelf(
        shelf: Shelf,
    ): shelf is InAppPurchaseLockupShelf {
        const { contentType, items } = shelf;
        return contentType === 'inAppPurchaseLockup' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import InAppPurchaseLockupComponent from '~/components/jet/item/InAppPurchaseLockup.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: InAppPurchaseLockupShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="InAppPurchaseLockup" let:item>
        <InAppPurchaseLockupComponent {item} />
    </ShelfItemLayout>
</ShelfWrapper>
