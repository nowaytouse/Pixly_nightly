<script lang="ts" context="module">
    import type {
        TodayCard,
        TodayCardMediaAppIcon,
    } from '@jet-app/app-store/api/models';

    export interface TodayCardWithMediAppIcon extends TodayCard {
        media: TodayCardMediaAppIcon;
    }

    export function isSmallStoryCardWithMediaAppIcon(
        item: TodayCard,
    ): item is TodayCardWithMediAppIcon {
        return !!item.media && item.media.kind === 'appIcon';
    }
</script>

<script lang="ts">
    import { buildSrc } from '@amp/web-app-components/src/components/Artwork/utils/srcset';
    import AppIcon from '~/components/AppIcon.svelte';
    import Artwork from '~/components/Artwork.svelte';
    import HoverWrapper from '~/components/HoverWrapper.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import { colorAsString } from '~/utils/color';

    export let item: TodayCardWithMediAppIcon;

    $: artwork = item.heroMedia?.artworks[0];
    $: appIcon = item.media.icon;
    $: backgroundImage = appIcon
        ? buildSrc(
              appIcon.template,
              {
                  crop: 'bb',
                  width: 160,
                  height: 160,
                  fileType: 'webp',
              },
              {},
          )
        : undefined;
    $: backgroundColor = appIcon.backgroundColor
        ? colorAsString(appIcon.backgroundColor)
        : '#000';
</script>

<LinkWrapper action={item.clickAction}>
    <HoverWrapper>
        <div
            class="container"
            style:--background-color={backgroundColor}
            style:--background-image={`url(${backgroundImage})`}
        >
            <div class="protection" />

            {#if artwork}
                <Artwork {artwork} profile="brick" />
            {:else}
                <div class="app-icon-container">
                    <div class="app-icon-normal">
                        <AppIcon
                            icon={appIcon}
                            profile="app-icon-medium"
                            fixedWidth={false}
                        />
                    </div>

                    <div class="app-icon-glow">
                        <AppIcon
                            icon={appIcon}
                            profile="app-icon-medium"
                            fixedWidth={false}
                        />
                    </div>
                </div>
            {/if}
        </div>
    </HoverWrapper>

    <div class="text-container">
        <h4>{item.heading}</h4>
        <h3>{item.title}</h3>
    </div>
</LinkWrapper>

<style lang="scss">
    @use 'amp/stylekit/core/mixins/browser-targets' as *;

    .container {
        aspect-ratio: 16 / 9;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
        background: linear-gradient(
                to bottom,
                transparent 20%,
                rgba(0, 0, 0, 0.33) 100%
            ),
            var(--background-image), var(--background-color, #000);
        background-size: cover;
        background-position: center;

        // Safari has issues rendering the overlaid `backdrop-filter` from `.proection` atop the
        // background image of `.container`, so in Safari only we are forgoing the use of
        // `var(--background-image)` and just using colors.
        @include target-safari {
            background: linear-gradient(
                    to bottom,
                    transparent 20%,
                    rgba(0, 0, 0, 0.33) 100%
                ),
                var(--background-color, #000);
        }
    }

    .protection {
        position: absolute;
        width: 100%;
        height: 100%;
        backdrop-filter: blur(80px) saturate(1.5);
    }

    .app-icon-container {
        position: relative;
        width: 80px;
    }

    .app-icon-normal {
        position: relative;
        z-index: 1;
        filter: drop-shadow(0 0 13px rgba(0, 0, 0, 0.15));
    }

    .app-icon-glow {
        position: absolute;
        inset: 0;
        width: 100%;
        transform: scale(1.4);
        filter: blur(25px);
    }

    .text-container {
        margin-top: 8px;
    }

    h3 {
        font: var(--title-3);
    }

    h4 {
        margin-bottom: 2px;
        font: var(--callout-emphasized);
        color: var(--systemSecondary);
    }
</style>
