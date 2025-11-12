<script lang="ts">
    import { onMount } from 'svelte';
    import type { Artwork } from '@jet-app/app-store/api/models';
    import AppIcon, { type AppIconProfile } from '~/components/AppIcon.svelte';

    export let icons: Artwork[];
    export let profile: AppIconProfile = 'app-icon-river';

    $: aspectRatio = icons[0].width / icons[0].height;

    let mounted = false;
    const numberOfIcons = icons.length;

    // We shift the order of the bottom row of icons to ensure that the same icons aren't shown
    // next to each other. Note that this is different from purely shuffling the icons, as that
    // could still lead to the same icons being next to one another, due to how small the set is.
    // The input and output here is as such:
    // in  = [1, 2, 3, 4, 5, 6, 7]
    // out = [4, 5, 6, 7, 1, 2, 3]
    const iconsInShiftedOrder = [
        ...icons.slice(numberOfIcons / 2),
        ...icons.slice(0, numberOfIcons / 2),
    ];

    // We are quadrupling the icons we render so the flow is seamless and stretches across the
    // full width of the container.
    const topRow = Array(4).fill(icons).flat();
    const bottomRow = Array(4).fill(iconsInShiftedOrder).flat();

    // We use this `mounted` flag to defer the rendering of the `AppIconRiver`, since it's markup heavy
    // and has no semantic meaning for SEO. This deferring saves about 190kb of initial HTML per instance.
    onMount(() => (mounted = true));
</script>

{#if mounted}
    {#each [topRow, bottomRow] as iconRow}
        <ul class="app-icons">
            {#each iconRow as icon}
                <li
                    class="app-icon-container"
                    style:--aspect-ratio={aspectRatio}
                >
                    <AppIcon {icon} {profile} fixedWidth={false} />
                </li>
            {/each}
        </ul>
    {/each}
{/if}

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;

    .app-icons {
        --icon-width: var(--app-icon-river-icon-width, 128px);
        --speed: var(--app-icon-river-speed, 240s);
        --direction: -50%;

        @include rtl {
            --direction: 50%;
        }
        display: flex;
        width: fit-content;
        z-index: 2;
        animation: scroll var(--speed) linear infinite;
    }

    .app-icons:last-of-type {
        margin-bottom: 20px;
    }

    .app-icon-container {
        width: var(--icon-width);
        aspect-ratio: var(--aspect-ratio);
        margin: 8px;
    }

    .app-icons:last-of-type .app-icon-container {
        position: relative;
        right: calc((var(--icon-width) / 2) + 8px);
    }

    @keyframes scroll {
        0% {
            transform: translateX(0);
        }

        100% {
            transform: translateX(var(--direction));
        }
    }
</style>
