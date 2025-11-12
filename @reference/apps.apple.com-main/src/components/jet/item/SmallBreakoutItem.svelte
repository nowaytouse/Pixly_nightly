<script lang="ts">
    import {
        type Artwork as JetArtworkType,
        type SmallBreakout,
        isFlowAction,
    } from '@jet-app/app-store/api/models';
    import { isSome } from '@jet/environment/types/optional';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import AppIcon from '~/components/AppIcon.svelte';
    import HoverWrapper from '~/components/HoverWrapper.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import SFSymbol from '~/components/SFSymbol.svelte';
    import { colorAsString } from '~/utils/color';

    export let item: SmallBreakout;

    $: ({ backgroundColor, iconArtwork, clickAction: action = null } = item);

    $: backgroundColorForCss = backgroundColor
        ? colorAsString(backgroundColor)
        : '#000';
</script>

<LinkWrapper {action}>
    <HoverWrapper>
        <div class="container" style:--background-color={backgroundColorForCss}>
            {#if iconArtwork}
                <div class="artwork-container">
                    <AppIcon
                        icon={iconArtwork}
                        profile="app-icon-xlarge"
                        fixedWidth={false}
                    />
                </div>
            {/if}

            <div
                class="text-container"
                class:with-dark-background={item.details.backgroundStyle ===
                    'dark'}
            >
                {#if item.details?.badge}
                    <LineClamp clamp={1}>
                        <h4>{item.details.badge}</h4>
                    </LineClamp>
                {/if}

                {#if item.details.title}
                    <LineClamp clamp={2}>
                        <h3>{item.details.title}</h3>
                    </LineClamp>
                {/if}

                {#if item.details.description}
                    <LineClamp clamp={3}>
                        <p>{item.details.description}</p>
                    </LineClamp>
                {/if}

                {#if isSome(action) && isFlowAction(action)}
                    <span class="link-container">
                        {action.title}
                        <span aria-hidden="true">
                            <SFSymbol name="chevron.forward" />
                        </span>
                    </span>
                {/if}
            </div>
        </div>
    </HoverWrapper>
</LinkWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/helpers' as *;
    @use 'ac-sasskit/core/locale' as *;

    .container {
        width: 100%;
        max-height: 460px;
        aspect-ratio: 16/9;
        background-color: var(--background-color);
        container-type: inline-size;
        container-name: container;

        @media (--range-small-up) {
            aspect-ratio: 13/5;
        }
    }

    .artwork-container {
        --rotation: -30deg;
        position: absolute;
        width: 33%;
        max-width: 430px;
        inset-inline-end: -10%;
        transform: translateY(-8%) rotate(var(--rotation));

        @include rtl {
            --rotation: 30deg;
        }
    }

    @container container (min-width: 1150px) {
        .artwork-container {
            transform: translateY(-11%) rotate(var(--rotation));
        }
    }

    .artwork-container :global(.artwork-component) {
        --angle: -7px;
        box-shadow: var(--angle) 5px 12px 0 rgba(0, 0, 0, 0.15);

        @include rtl {
            --angle: 7px;
        }
    }

    .text-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        width: 66%;
        height: 100%;
        padding: 0 20px;
        text-wrap: pretty;

        @media (--range-small-up) {
            width: 33%;
        }

        @media (--range-large-up) {
            width: 33%;
        }
    }

    .text-container.with-dark-background {
        color: var(--systemPrimary-onDark);
    }

    .link-container {
        display: flex;
        gap: 4px;
        margin-top: 16px;
        font: var(--title-3-emphasized);

        @media (--range-small-up) {
            font: var(--title-2-emphasized);
        }
    }

    .link-container :global(svg) {
        width: 10px;
        height: 10px;
        fill: currentColor;

        @include rtl {
            transform: rotate(180deg);
        }
    }

    h3 {
        text-wrap: balance;
        font: var(--title-1-emphasized);

        @media (--range-small-up) {
            font: var(--large-title-emphasized);
        }
    }

    h4 {
        font: var(--subhead-emphasized);

        @media (--range-small-up) {
            font: var(--headline);
        }
    }

    p {
        margin-top: 8px;

        @media (--range-small-up) {
            font: var(--title-3);
        }
    }
</style>
