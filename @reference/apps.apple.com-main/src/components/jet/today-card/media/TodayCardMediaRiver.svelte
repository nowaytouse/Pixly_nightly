<script lang="ts" context="module">
    import type {
        TodayCardMedia,
        TodayCardMediaRiver,
    } from '@jet-app/app-store/api/models';

    export function isTodayCardMediaRiver(
        media: TodayCardMedia,
    ): media is TodayCardMediaRiver {
        return media.kind === 'river';
    }
</script>

<script lang="ts">
    import {
        getBackgroundGradientCSSVarsFromArtworks,
        getLuminanceForRGB,
    } from '~/utils/color';
    import AppIconRiver from '~/components/AppIconRiver.svelte';

    /**
     * The actual properties of {@linkcode TodayCardMediaRiver} that are required
     * to render this component
     */
    type TodayCardMediaRiverRequirements = Pick<TodayCardMediaRiver, 'lockups'>;

    export let media: TodayCardMediaRiverRequirements;

    $: icons = media.lockups.map((lockup) => lockup.icon);
    $: backgroundGradientCssVars = getBackgroundGradientCSSVarsFromArtworks(
        icons,
        {
            // sorts from darkest to lightest
            sortFn: (a, b) => getLuminanceForRGB(a) - getLuminanceForRGB(b),
        },
    );
</script>

<div class="container" style={backgroundGradientCssVars}>
    {#if icons.length}
        <AppIconRiver {icons} />
    {/if}
</div>

<style>
    .container {
        --app-icon-river-icon-width: 96px;
        height: 100%;
        width: 100%;
        padding-top: 10%;
        overflow: hidden;
        border-radius: var(--today-card-border-radius);
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

        @media (--range-small-only) {
            padding-top: 5%;
        }
    }
</style>
