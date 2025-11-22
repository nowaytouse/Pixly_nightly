<script lang="ts">
    export let shouldDarken: boolean = true;
</script>

<div class="gradient-overlay" style:--brightness={shouldDarken ? 0.85 : 1} />

<style>
    .gradient-overlay {
        position: absolute;
        z-index: 1;
        bottom: 0;
        width: 100%;
        height: var(--height, 60%);
        border-radius: var(--border-radius, var(--global-border-radius-large));
        background: linear-gradient(
            transparent,
            var(--color, var(--systemSecondary-onLight)) var(--height, 100%)
        );
        backdrop-filter: blur(10px);
        filter: saturate(1.5) brightness(var(--brightness));
        mask-image: linear-gradient(180deg, transparent 6%, rgb(0, 0, 0.5) 85%);
    }
</style>
