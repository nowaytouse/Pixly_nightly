<script lang="ts" context="module">
    import type {
        TodayCardOverlay,
        TodayCardLockupListOverlay,
    } from '@jet-app/app-store/api/models';

    export function isLockupListOverlay(
        overlay: TodayCardOverlay,
    ): overlay is TodayCardLockupListOverlay {
        return overlay.kind === 'lockupList';
    }
</script>

<script lang="ts">
    import AppIcon from '~/components/AppIcon.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';

    export let overlay: TodayCardLockupListOverlay;
</script>

<div class="lockup-list">
    {#each overlay.lockups as lockup}
        <LinkWrapper action={lockup.clickAction}>
            <AppIcon icon={lockup.icon} />
        </LinkWrapper>
    {/each}
</div>

<style>
    .lockup-list {
        display: flex;
        gap: 12px;

        @media (--range-xsmall-only) and (--sidebar-visible) {
            gap: 10px;
        }

        @media (--range-small-up) {
            gap: 16px;
        }
    }
</style>
