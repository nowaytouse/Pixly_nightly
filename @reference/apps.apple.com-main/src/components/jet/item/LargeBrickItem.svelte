<script lang="ts">
    import type { Brick } from '@jet-app/app-store/api/models';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import Artwork from '~/components/Artwork.svelte';
    import GradientOverlay from '~/components/GradientOverlay.svelte';
    import HoverWrapper from '~/components/HoverWrapper.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import { colorAsString } from '~/utils/color';
    import { isRtl } from '~/utils/locale';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';

    export let item: Brick;
    const artwork =
        isRtl() && item.rtlArtwork ? item.rtlArtwork : item.artworks?.[0];
    const collectionIcon = item.collectionIcons?.[0];
    let artworkFallbackColor: string | null = null;

    const gradientOverlayColor: string = artwork?.backgroundColor
        ? colorAsString(artwork.backgroundColor)
        : '#000';

    if (!artwork) {
        artworkFallbackColor = collectionIcon?.backgroundColor
            ? colorAsString(collectionIcon.backgroundColor)
            : '#000';
    }
</script>

<LinkWrapper action={item.clickAction}>
    <HoverWrapper>
        {#if artwork}
            <div class="artwork-container">
                <Artwork {artwork} profile="large-brick" />
            </div>
        {:else}
            <div
                class="gradient-container"
                style={`--color: ${artworkFallbackColor};`}
            />
        {/if}

        <div class="text-container">
            <div class="metadata-container">
                {#if item.caption}
                    <LineClamp clamp={1}>
                        <h4>{item.caption}</h4>
                    </LineClamp>
                {/if}

                {#if item.title}
                    <LineClamp clamp={2}>
                        <h3>{@html sanitizeHtml(item.title)}</h3>
                    </LineClamp>
                {/if}

                {#if item.subtitle}
                    <LineClamp clamp={2}>
                        <p>{item.subtitle}</p>
                    </LineClamp>
                {/if}
            </div>
        </div>

        <GradientOverlay --color={gradientOverlayColor} />
    </HoverWrapper>
</LinkWrapper>

<style>
    .artwork-container {
        width: 100%;
    }

    .gradient-container {
        width: 100%;
        aspect-ratio: 16 / 9;
        background-color: var(--color);
    }

    .text-container {
        position: absolute;
        z-index: 2;
        bottom: 0;
        display: flex;
        align-items: center;
        width: 66%;
        padding-inline: 20px;
        padding-bottom: 20px;
        color: var(--systemPrimary-onDark);
    }

    h3 {
        font: var(--title-1-emphasized);
        text-wrap: balance;
    }

    h4 {
        font: var(--callout-emphasized);
        margin-bottom: 3px;
    }

    p {
        font: var(--body-emphasized);
        margin-top: 6px;
    }
</style>
