<script lang="ts">
    import { type Annotation } from '@jet-app/app-store/api/models';
    import ModernAnnotationItemRenderer from '~/components/jet/item/Annotation/ModernAnnotationItemRenderer.svelte';
    import LegacyAnnotationRenderer from '~/components/jet/item/Annotation/LegacyAnnotationRenderer.svelte';

    export let item: Annotation;

    $: ({ items, items_V3, linkAction, summary } = item);

    $: shouldRenderModernAnnotation = items_V3.length > 0;
</script>

{#if shouldRenderModernAnnotation}
    <ModernAnnotationItemRenderer items={items_V3} {summary} />
{:else}
    <LegacyAnnotationRenderer {items} {linkAction} />
{/if}
