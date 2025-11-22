<script lang="ts" context="module">
    import type {
        TodayCardOverlay,
        TodayCardLockupOverlay,
    } from '@jet-app/app-store/api/models';

    export function isLockupOverlay(
        overlay: TodayCardOverlay,
    ): overlay is TodayCardLockupOverlay {
        return overlay.kind === 'lockup';
    }
</script>

<script lang="ts">
    import TodayCardLockupListOverlay, {
        isLockupListOverlay,
    } from '~/components/jet/today-card/overlay/TodayCardLockupListOverlay.svelte';
    import SmallLockupItem from '~/components/jet/item/SmallLockupItem.svelte';

    export let overlay: TodayCardOverlay;
    export let buttonVariant: 'gray' | 'blue' | 'transparent' = 'transparent';
</script>

{#if isLockupOverlay(overlay)}
    <div class="small-lockup-item-config">
        <SmallLockupItem
            {buttonVariant}
            item={overlay.lockup}
            titleLineCount={1}
            appIconProfile="app-icon"
        />
    </div>
{:else if isLockupListOverlay(overlay)}
    <TodayCardLockupListOverlay {overlay} />
{/if}

<style>
    .small-lockup-item-config {
        --title-color: var(--text-color, currentColor);
        --subtitle-color: var(--text-accent-color, currentColor);
        --linkColor: currentColor;
        --eyebrow-color: var(--text-accent-color, currentColor);
        --button-blend-mode: var(--text-accent-blend-mode);
        --subtitle-blend-mode: var(--text-accent-blend-mode);
        --eyebrow-blend-mode: var(--text-accent-blend-mode);
        display: contents;
    }
</style>
