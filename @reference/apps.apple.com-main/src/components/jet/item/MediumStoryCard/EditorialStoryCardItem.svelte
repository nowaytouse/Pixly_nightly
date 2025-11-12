<script lang="ts">
    import {
        isFlowAction,
        type EditorialStoryCard,
        type FlowAction,
    } from '@jet-app/app-store/api/models';
    import type { Opt } from '@jet/environment';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';
    import AppIcon from '~/components/AppIcon.svelte';
    import Artwork from '~/components/Artwork.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import { getI18n } from '~/stores/i18n';
    import HoverWrapper from '~/components/HoverWrapper.svelte';

    export let item: EditorialStoryCard;

    let {
        clickAction,
        collectionIcons,
        title,
        lockup: { title: lockupTitle, subtitle, heading: lockupHeading } = {},
    } = item;
    const i18n = getI18n();
    const hasMultipleCollectionIcons = (collectionIcons?.length ?? 0) > 1;
    const destination: Opt<FlowAction> =
        clickAction && isFlowAction(clickAction) ? clickAction : undefined;
</script>

<LinkWrapper action={destination}>
    <article>
        {#if item.artwork}
            <div class="artwork-container">
                <HoverWrapper element="div">
                    <Artwork
                        artwork={item.artwork}
                        profile="editorial-story-card"
                    />
                </HoverWrapper>
            </div>
        {/if}
        <div class="details-container">
            <div
                class="title-container"
                class:on-dark={item.isMediaDark}
                class:on-light={!item.isMediaDark}
            >
                {#if item.badge}
                    <h4>{item.badge.title}</h4>
                {/if}

                {#if item.title}
                    <h3>{@html sanitizeHtml(item.title)}</h3>
                {/if}

                {#if item.description}
                    <p>{@html sanitizeHtml(item.description)}</p>
                {/if}
            </div>

            {#if collectionIcons && !item.editorialDisplayOptions.suppressLockup}
                <div class="lockup-container">
                    <ul class:with-multiple-icons={hasMultipleCollectionIcons}>
                        {#each collectionIcons as collectionIcon}
                            <li class="app-icon-container">
                                <AppIcon
                                    icon={collectionIcon}
                                    fixedWidth={false}
                                    profile={hasMultipleCollectionIcons
                                        ? 'app-icon-medium'
                                        : 'app-icon'}
                                />
                            </li>
                        {/each}
                    </ul>

                    {#if !hasMultipleCollectionIcons}
                        <div class="metadata-container">
                            {#if lockupHeading}
                                <span class="lockup-eyebrow">
                                    {lockupHeading}
                                </span>
                            {/if}

                            <!--
                            Some cards with the lockup UI don't have a `lockup` property,
                            so we use the title of the item as a fallback.
                        -->
                            {#if lockupTitle || title}
                                <LineClamp clamp={1}>
                                    <h4 class="lockup-title">
                                        {lockupTitle || title}
                                    </h4>
                                </LineClamp>
                            {/if}

                            {#if subtitle}
                                <LineClamp clamp={1}>
                                    <p class="lockup-subtitle">{subtitle}</p>
                                </LineClamp>
                            {/if}
                        </div>

                        {#if destination}
                            <div class="button-container">
                                <span class="get-button transparent">
                                    {$i18n.t('ASE.Web.AppStore.View')}
                                </span>
                            </div>
                        {/if}
                    {/if}
                </div>
            {/if}
        </div>
        <div
            class="blur-overlay"
            style:--brightness={item.isMediaDark ? 0.75 : 1.25}
        />
    </article>
</LinkWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/helpers' as *;
    @use 'ac-sasskit/core/locale' as *;

    article {
        position: relative;
        overflow: hidden;
        border-radius: var(--global-border-radius-large);
        box-shadow: var(--shadow-medium);
        aspect-ratio: 3/4;
        container-type: inline-size;
        container-name: card;
    }

    .artwork-container {
        position: absolute;
        width: 100%;
        height: 100%;
    }

    .details-container {
        display: flex;
        flex-direction: column;
        justify-content: end;
        height: 100%;
        border-radius: var(--global-border-radius-large);
        overflow: hidden;
        z-index: 1;
    }

    .title-container {
        padding: 20px;
        z-index: 2;
    }

    .title-container h3 {
        margin-bottom: 2px;
        font: var(--title-1-emphasized);
        text-wrap: pretty;
    }

    .title-container h4 {
        font: var(--callout-emphasized);
    }

    .on-dark {
        color: var(--systemPrimary-onDark);
    }

    .on-light {
        color: var(--systemPrimary-onLight);
    }

    .title-container.on-dark h4 {
        color: var(--systemSecondary-onDark);
        mix-blend-mode: plus-lighter;
    }

    .title-container.on-light h4 {
        color: var(--systemSecondary-onLight);
    }

    .title-container.on-dark p {
        font: var(--body);
        color: var(--systemSecondary-onDark);
    }

    .title-container.on-light p {
        font: var(--body);
        color: var(--systemSecondary-onLight);
    }

    .lockup-container {
        display: flex;
        align-items: center;
        min-height: 80px;
        padding: 10px 20px;
        color: var(--systemPrimary-onDark);
        background-image: linear-gradient(rgba(0, 0, 0, 0.2) 0 0);
        z-index: 2;
    }

    .metadata-container {
        flex-grow: 1;
        margin-inline-end: 16px;
    }

    .lockup-title {
        font: var(--title-3-emphasized);
    }

    .lockup-eyebrow {
        color: var(--systemSecondary-onDark);
        font: var(--subhead-emphasized);
        text-transform: uppercase;
        mix-blend-mode: plus-lighter;
    }

    .lockup-subtitle {
        color: var(--systemSecondary-onDark);
        font: var(--callout);
        mix-blend-mode: plus-lighter;
    }

    .app-icon-container {
        flex-shrink: 0;
        width: 48px;
        margin-inline-end: 16px;
    }

    article:hover .blur-overlay {
        height: 52%;
        backdrop-filter: blur(70px) saturate(1.5)
            brightness(calc(var(--brightness) * 0.9));
    }

    .blur-overlay {
        position: absolute;
        z-index: 1;
        top: unset;
        bottom: 0;
        width: 100%;
        height: 50%;
        border-radius: var(--global-border-radius-large);
        mask-image: linear-gradient(
            180deg,
            rgba(255, 255, 255, 0) 5%,
            rgba(0, 0, 0, 1) 50%
        );
        backdrop-filter: blur(50px) saturate(1.5)
            brightness((var(--brightness)));
        transition-property: height, backdrop-filter;
        transition-duration: 210ms;
        transition-timing-function: ease-out;
    }

    ul.with-multiple-icons {
        width: 100%;
        display: grid;
        gap: 12px;

        .app-icon-container {
            width: 100%;
            margin-inline-end: unset;
        }
    }

    // In the following container queries, we are specifying column counts and hiding icons past
    // that number to ensure a reasonable number of icons are shown for different size cards.
    @container card (max-width: 300px) {
        ul.with-multiple-icons {
            // Think of "4" as the number of columns to show
            grid-template-columns: repeat(4, 1fr);
        }

        // And "5" as the number of columns to hide past
        .app-icon-container:nth-child(n + 5) {
            display: none;
        }
    }

    @container card (min-width: 300px) and (max-width: 400px) {
        ul.with-multiple-icons {
            grid-template-columns: repeat(5, 1fr);
        }

        .app-icon-container:nth-child(n + 6) {
            display: none;
        }
    }

    @container card (min-width: 400px) {
        ul.with-multiple-icons {
            grid-template-columns: repeat(6, 1fr);
        }

        .app-icon-container:nth-child(n + 7) {
            display: none;
        }
    }
</style>
