<script lang="ts" context="module">
    import type { Shelf, RibbonBarItem } from '@jet-app/app-store/api/models';

    interface RibbonBarShelf extends Shelf {
        items: RibbonBarItem[];
    }

    export function isRibbonBarShelf(shelf: Shelf): shelf is RibbonBarShelf {
        return shelf.contentType === 'ribbonBar' && Array.isArray(shelf.items);
    }
</script>

<script lang="ts">
    import Artwork, { getNaturalProfile } from '~/components/Artwork.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import SystemImage, {
        isSystemImageArtwork,
    } from '~/components/SystemImage.svelte';

    export let shelf: RibbonBarShelf;
</script>

<ShelfWrapper {shelf} withBottomPadding={false} withPaddingTop={false}>
    <div class="scroll">
        <ul>
            {#each shelf.items as ribbonBarItem}
                {@const action = ribbonBarItem.clickAction}
                {@const artwork = ribbonBarItem.artwork}
                {@const title = ribbonBarItem.title}
                <li>
                    <LinkWrapper {action}>
                        {#if artwork}
                            <div
                                class="artwork-container"
                                style:--aspect-ratio={artwork.width /
                                    artwork.height}
                            >
                                {#if isSystemImageArtwork(artwork)}
                                    <SystemImage {artwork} />
                                {:else}
                                    <Artwork
                                        {artwork}
                                        profile={getNaturalProfile(artwork, [
                                            17,
                                        ])}
                                        hasTransparentBackground
                                    />
                                {/if}
                            </div>
                        {/if}
                        {title}
                    </LinkWrapper>
                </li>
            {/each}
        </ul>
    </div>
</ShelfWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/helpers' as *;
    @use 'ac-sasskit/core/locale' as *;

    .scroll {
        --gradient-direction: 90deg;
        overflow-x: auto;
        scrollbar-width: none;
        padding-inline-start: var(--bodyGutter);
        margin-inline-end: var(--bodyGutter);
        // A small gradient that fades out the ribbon, to indicate that there is more
        mask-image: linear-gradient(
            var(--gradient-direction),
            black calc(100% - 8px),
            transparent 100%
        );

        @include rtl {
            --gradient-direction: -90deg;
        }
    }

    ul {
        font: var(--body-emphasized);
        display: flex;
        gap: 4px;
        padding-bottom: 16px;
        padding-top: 13px;
    }

    li {
        display: flex;
        margin-inline-end: 8px;
        flex-shrink: 0;
    }

    li:last-of-type {
        padding-inline-end: 8px;
    }

    li :global(a) {
        position: relative;
        display: flex;
        align-items: center;
        gap: 4px;
        flex-shrink: 0;
        background: var(--pageBG);
        border-radius: var(--global-border-radius-small);
        padding: 6px 10px;

        &::after {
            content: '';
            display: block;
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            border-radius: var(--global-border-radius-small);
            box-shadow: var(--shadow-small);
            z-index: calc(var(--z-default) - 1);
        }

        @media (prefers-color-scheme: dark) {
            background: var(--systemGray5-default_IC);
        }
    }

    .artwork-container {
        --artwork-override-height: 17px;
        flex-shrink: 0;
        aspect-ratio: var(--aspect-ratio);
        height: 17px;
    }
</style>
