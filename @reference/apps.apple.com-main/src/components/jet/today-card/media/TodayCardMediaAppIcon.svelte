<script lang="ts" context="module">
    import type {
        TodayCardMedia,
        TodayCardMediaAppIcon,
    } from '@jet-app/app-store/api/models';

    export function isTodayCardMediAppIcon(
        media: TodayCardMedia,
    ): media is TodayCardMediaAppIcon {
        return media.kind === 'appIcon';
    }
</script>

<script lang="ts">
    import AppIcon from '~/components/AppIcon.svelte';
    import { colorAsString } from '~/utils/color';

    export let media: TodayCardMediaAppIcon;

    $: backgroundColor = media.icon.backgroundColor
        ? colorAsString(media.icon.backgroundColor)
        : null;
</script>

<div class="container" style:--background-color={backgroundColor}>
    <div class="artwork-container">
        <AppIcon
            icon={media.icon}
            profile="app-icon-xlarge"
            fixedWidth={false}
        />
    </div>
</div>

<style>
    .container {
        height: 100%;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: var(--background-color);
        border-radius: var(--today-card-border-radius);
    }

    .artwork-container {
        width: 50%;
        height: 50%;
    }

    @container (orientation: landscape) {
        .container {
            align-items: start;
            padding-top: 5%;
        }

        .artwork-container {
            width: 30%;
            height: 30%;
        }
    }
</style>
