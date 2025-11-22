<script lang="ts">
    import ChevronDown from '~/sf-symbols/chevron.down.svg';
</script>

<details>
    <summary>
        <slot name="summary" />
        <ChevronDown />
    </summary>

    <slot />
</details>

<style>
    details[open] summary {
        display: none;
    }

    summary {
        list-style: none;
        cursor: pointer;
    }

    summary::-webkit-details-marker {
        display: none;
    }

    summary :global(svg) {
        overflow: visible;
        width: 14px;
        fill: var(--systemTertiary);
        position: relative;
        top: 3px;
        left: 2px;
    }
</style>
