<script lang="ts">
    import type { Paragraph } from '@jet-app/app-store/api/models';
    import he from 'he';

    export let item: Paragraph;
</script>

<p>
    {@html he.decode(item.text)}
</p>

<style>
    p {
        font: var(--title-2-medium);
        color: var(--systemSecondary);
    }

    p :global(b) {
        color: var(--systemPrimary);
    }
</style>
