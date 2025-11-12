<script lang="ts">
    import type { AccessibilityParagraph } from '@jet-app/app-store/api/models';
    import LinkableTextItem from '~/components/jet/item/LinkableTextItem.svelte';

    export let item: AccessibilityParagraph;
</script>

<div>
    <p>
        <LinkableTextItem item={item.text} />
    </p>
</div>

<style>
    p {
        font: var(--body-tall);
    }

    p :global(a) {
        color: var(--keyColor);
    }
</style>
