<script lang="ts" context="module">
    import type { Shelf, Annotation } from '@jet-app/app-store/api/models';

    interface AnnotationShelf extends Shelf {
        items: Annotation[];
    }

    export function isAnnotationShelf(shelf: Shelf): shelf is AnnotationShelf {
        const { contentType, items } = shelf;

        return contentType === 'annotation' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import Grid from '~/components/Grid.svelte';
    import CollapsableContent from '~/components/CollapsableContent.svelte';
    import AnnotationItem from '~/components/jet/item/Annotation/AnnotationItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: AnnotationShelf;
</script>

<ShelfWrapper {shelf}>
    <dl>
        <Grid items={shelf.items} gridType="F" let:item>
            <dt>{item.title}</dt>

            {#if item.summary}
                <CollapsableContent>
                    <svelte:fragment slot="summary">
                        {item.summary}
                    </svelte:fragment>

                    <AnnotationItem {item} />
                </CollapsableContent>
            {:else}
                <AnnotationItem {item} />
            {/if}
        </Grid>
    </dl>
</ShelfWrapper>

<style>
    dt {
        color: var(--systemSecondary);
        margin-bottom: 4px;
    }
</style>
