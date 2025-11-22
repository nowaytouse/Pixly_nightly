<script lang="ts" context="module">
    import type {
        TodayCard,
        TodayCardMediaRiver,
    } from '@jet-app/app-store/api/models';

    export interface TodayCardWithMediaRiver extends TodayCard {
        media: TodayCardMediaRiver;
    }

    export function isSmallStoryCardWithMediaRiver(
        item: TodayCard,
    ): item is TodayCardWithMediaRiver {
        return !!item.media && item.media.kind === 'river';
    }
</script>

<script lang="ts">
    import type { Opt } from '@jet/environment/types/optional';
    import HoverWrapper from '~/components/HoverWrapper.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import AppIconRiver from '~/components/AppIconRiver.svelte';
    import {
        getBackgroundGradientCSSVarsFromArtworks,
        getLuminanceForRGB,
    } from '~/utils/color';

    export let item: TodayCardWithMediaRiver;

    $: icons = item.media.lockups.map((lockup) => lockup.icon);
    $: backgroundGradientCssVars = getBackgroundGradientCSSVarsFromArtworks(
        icons,
        {
            // sorts from darkest to lightest
            sortFn: (a, b) => getLuminanceForRGB(a) - getLuminanceForRGB(b),
        },
    );

    let title: Opt<string>;
    let eyebrow: Opt<string>;
    $: {
        eyebrow = item.heading;
        title = item.title;

        if (item.inlineDescription) {
            eyebrow = item.title;
            title = item.inlineDescription;
        }
    }
</script>

<LinkWrapper action={item.clickAction}>
    <HoverWrapper>
        <div class="river-container" style={backgroundGradientCssVars}>
            <AppIconRiver {icons} profile="app-icon" />
        </div>
    </HoverWrapper>

    <div class="text-container">
        {#if eyebrow}
            <h4>{eyebrow}</h4>
        {/if}

        {#if title}
            <h3>{title}</h3>
        {/if}
    </div>
</LinkWrapper>

<style>
    .river-container {
        --app-icon-river-icon-width: 48px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        aspect-ratio: 16 / 9;
        width: 100%;
        border-radius: 8px;
        background: radial-gradient(
                circle at 3% -50%,
                var(--top-left, #000) 20%,
                transparent 70%
            ),
            radial-gradient(
                circle at -50% 120%,
                var(--bottom-left, #000) 40%,
                transparent 80%
            ),
            radial-gradient(
                circle at 140% -50%,
                var(--top-right, #000) 60%,
                transparent 80%
            ),
            radial-gradient(
                circle at 62% 100%,
                var(--bottom-right, #000) 50%,
                transparent 100%
            );
    }

    .river-container :global(.app-icons:last-of-type) {
        margin-bottom: 0;
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
