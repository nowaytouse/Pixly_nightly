<script lang="ts" context="module">
    import type { PageHeader, Shelf } from '@jet-app/app-store/api/models';

    interface PageHeaderShelf extends Shelf {
        items: [PageHeader];
    }

    export function isPageHeaderShelf(shelf: Shelf): shelf is PageHeaderShelf {
        const { contentType, items } = shelf;
        return contentType === 'pageHeader' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ShelfTitle from '~/components/Shelf/Title.svelte';

    export let shelf: PageHeaderShelf;

    $: [item] = shelf.items;
</script>

<ShelfWrapper {shelf} withBottomPadding={false}>
    <div class="shelf-title-wrapper" slot="title">
        <ShelfTitle title={item.title} subtitle={item.subtitle} />
    </div>
</ShelfWrapper>

<style>
    .shelf-title-wrapper {
        --shelf-title-font: var(--title-1-emphasized);
        display: contents;
    }
</style>
