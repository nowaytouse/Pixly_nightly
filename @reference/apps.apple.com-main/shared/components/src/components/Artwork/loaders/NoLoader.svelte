<!--
    NoLoader Component
    This component should be used when loading="lazy"
    is supported.

    DO NOT USE DIRECTLY use LoaderSelector
-->
<script lang="ts">
    let mounted = false;

    export function onSlotMount(_artworkComponent: Element) {
        mounted = true;
    }

    const ssr = typeof window === 'undefined';

    $: isVisible = mounted || ssr;
</script>

<slot {isVisible} />
