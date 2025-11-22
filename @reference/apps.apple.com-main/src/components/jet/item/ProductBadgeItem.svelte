<script lang="ts">
    import type { Badge } from '@jet-app/app-store/api/models';
    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import Artwork from '~/components/Artwork.svelte';
    import StarRating from '~/components/StarRating.svelte';
    import GameController from '~/sf-symbols/gamecontroller.fill.svg';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import SystemImage, {
        isSystemImageArtwork,
    } from '~/components/SystemImage.svelte';
    import SFSymbol from '~/components/SFSymbol.svelte';
    import ContentRatingBadge, {
        isContentRatingBadge,
    } from '../badge/ContentRatingBadge.svelte';

    export let item: Badge;

    const { artwork, content, type } = item;

    $: isParagraph = type === 'paragraph';
    $: isRating = type === 'rating';
    $: isEditorsChoice = type === 'editorsChoice';
    $: isController = type === 'controller';
    $: hasImageArtwork = artwork && !isSystemImageArtwork(artwork);
</script>

<LinkWrapper withoutLabel action={item.clickAction}>
    <div class="badge-container">
        <div class="badge">
            <div class="badge-dt" role="term">
                <LineClamp clamp={1}>
                    {item.heading}
                </LineClamp>
            </div>

            <div class="badge-dd" role="definition">
                {#if isContentRatingBadge(item)}
                    <ContentRatingBadge badge={item} />
                {:else if isParagraph}
                    <span class="text-container">{content.paragraphText}</span>
                {:else if isRating && !content.rating}
                    <span class="text-container">
                        {content.ratingFormatted}
                    </span>
                {:else if isEditorsChoice}
                    <span class="editors-choice">
                        <SFSymbol name="laurel.leading" ariaHidden={true} />

                        <span>
                            <LineClamp clamp={2}>
                                {item.accessibilityTitle}
                            </LineClamp>
                        </span>

                        <SFSymbol name="laurel.trailing" ariaHidden={true} />
                    </span>
                {:else if artwork && hasImageArtwork}
                    <div class="artwork-container" aria-hidden="true">
                        <Artwork
                            {artwork}
                            profile="app-icon"
                            hasTransparentBackground
                        />
                    </div>
                {:else if artwork && isSystemImageArtwork(artwork)}
                    <div class="icon-container color" aria-hidden="true">
                        <SystemImage {artwork} />
                    </div>
                {:else if isController}
                    <div class="icon-container" aria-hidden="true">
                        <GameController />
                    </div>
                {/if}

                {#if isRating && content.rating}
                    <span class="text-container" aria-hidden="true">
                        {content.ratingFormatted}
                    </span>
                    <StarRating rating={content.rating} />
                {:else}
                    <LineClamp clamp={1}>{item.caption}</LineClamp>
                {/if}
            </div>
        </div>
    </div>
</LinkWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;

    .badge-container {
        --color: var(--systemGray3-onDark);
        --accent-color: var(--systemSecondary);
        display: flex;
        align-items: center;
        flex-direction: column;
        transition: filter 210ms ease-in;

        @media (prefers-color-scheme: dark) {
            --color: var(--systemGray3-onLight);
        }
    }

    .badge {
        text-align: center;
    }

    .artwork-container {
        height: 25px;
        aspect-ratio: 1/1;
        margin: 4px 0 2px;
        opacity: 0.7;

        @media (prefers-color-scheme: dark) {
            filter: invert(1);
        }
    }

    .icon-container {
        display: flex;
        width: 35px;
        height: 25px;
        margin: 4px 0 2px;
        line-height: 0;
    }

    .icon-container.color {
        filter: brightness(1);
    }

    .badge-dt {
        text-transform: uppercase;
        font: var(--subhead-emphasized);
        color: var(--accent-color);
        margin-bottom: 4px;
    }

    .text-container {
        height: 25px;
        margin: 4px 0 2px;
        font: var(--title-1-emphasized);
        color: var(--color);
    }

    .editors-choice {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 30px;

        :global(svg) {
            height: 20px;
            flex-shrink: 0;

            @include rtl {
                transform: rotateY(180deg);
            }
        }

        @media (--range-medium-only) {
            gap: 2px;
        }

        :global(svg path:not([fill='none'])) {
            fill: var(--color);
        }
    }

    .editors-choice span {
        width: 50%;
        font: var(--subhead-medium);

        @media (--range-medium-only) {
            width: 55%;
        }
    }

    .badge-dd {
        --fill-color: var(--color);
        display: flex;
        align-items: center;
        flex-direction: column;
        font: var(--subhead-tall);
        color: var(--color);
        gap: 4px;
    }
</style>
