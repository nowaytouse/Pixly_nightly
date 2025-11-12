<script lang="ts">
    // Delay until the spinner fades in
    export let delay: number = 0;
    export let inset: boolean = false;
    export let small: boolean = false;
    export let ariaLoading: string = '';
</script>

<div
    class="loading-spinner"
    class:inset
    class:loading-spinner--small={small}
    data-testid="loading-spinner"
    style="animation-delay: {delay}ms"
    aria-label={ariaLoading}
>
    <div class="pulse-spinner">
        <div class="pulse-spinner__container">
            <div class="pulse-spinner__nib pulse-spinner__nib--1" />
            <div class="pulse-spinner__nib pulse-spinner__nib--2" />
            <div class="pulse-spinner__nib pulse-spinner__nib--3" />
            <div class="pulse-spinner__nib pulse-spinner__nib--4" />
            <div class="pulse-spinner__nib pulse-spinner__nib--5" />
            <div class="pulse-spinner__nib pulse-spinner__nib--6" />
            <div class="pulse-spinner__nib pulse-spinner__nib--7" />
            <div class="pulse-spinner__nib pulse-spinner__nib--8" />
        </div>
    </div>
</div>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;
    @use 'ac-sasskit/core/selectors' as *;
    @use 'amp/stylekit/core/mixins/materials' as *;
    @use 'sass:math';

    // Loading spinner contains `@amp/pulse-spinner`

    .loading-spinner {
        margin: auto;
        opacity: 0;
        animation: fade-in 100ms;
        animation-fill-mode: forwards;
        text-align: center;
        z-index: var(--z-default);

        &:not(.inset) {
            position: absolute;
            top: 50%;
            left: 50%; // RTL not needed

            @media (--small) {
                &:not(.loading-spinner--small) {
                    transform: translate(-50%, -50%);
                }
            }
        }

        &.inset {
            transform: translateX(50%);

            @include rtl {
                transform: translateX(-50%);
            }
        }
    }

    @keyframes fade-in {
        0% {
            opacity: 0;
        }

        100% {
            opacity: 1;
        }
    }

    ////
    /// Pulse Spinner (Big Sur)
    /// Styles from `@amp/pulse-spinner`
    /// https://github.pie.apple.com/amp-web/pulse-spinner
    ////

    ///
    /// Spinner small container size
    ///
    /// @type Number
    ///
    $spinner-container-small: 16px;

    ///
    /// Spinner large container size
    ///
    /// @type Number
    ///
    $spinner-container-large: 32px;

    ///
    /// Spinner nib distance
    ///
    /// @type Value
    ///
    $spinner-nib-distance: 40px;

    ///
    /// Spinner nib count
    ///
    /// @type Number
    ///
    $spinner-nibs: 8;

    ///
    /// Spinner duration
    ///
    /// @type Number
    ///
    $spinner-duration: 0.8s;

    ///
    /// Spinner small scaling value
    ///
    /// @type Value | Number
    ///
    $spinner-small-scale: scale(0.075);

    ///
    /// Spinner large scaling value
    ///
    /// @type Value | Number
    ///
    $spinner-large-scale: 0.15;

    ///
    /// Spinner inactive opacity
    ///
    /// @type Number
    ///
    $spinner-inactive-opacity: 0.5;

    .pulse-spinner {
        position: relative;
        width: $spinner-container-small;
        height: $spinner-container-small;

        @include feature-detect($inactive-window-classname) {
            opacity: $spinner-inactive-opacity; // AppKit inactive style, when window is not in focus
        }

        @media (--small) {
            .loading-spinner:not(.loading-spinner--small) & {
                width: $spinner-container-large;
                height: $spinner-container-large;
            }
        }
    }

    .pulse-spinner__container {
        position: absolute;
        width: 0;
        transform: $spinner-small-scale;
        z-index: var(--z-default);

        @media (--small) {
            .loading-spinner:not(.loading-spinner--small) & {
                top: 50%;
                left: 50%;
                transform: scale(#{$spinner-large-scale});

                @include rtl {
                    // Adjust for scale
                    right: #{$spinner-large-scale * 100%};
                }
            }
        }
    }

    .pulse-spinner__nib {
        position: absolute;
        top: -12.5px;
        width: 66px;
        height: 28px;
        background: transparent;
        border-radius: 25% / 50%;
        transform-origin: left center;

        &::before {
            width: 100%;
            height: 100%;
            display: block;
            content: '';
            background: rgb(0, 0, 0);
            border-radius: 25% / 50%;
            animation-duration: $spinner-duration;
            animation-timing-function: linear;
            animation-iteration-count: infinite;
            animation-direction: normal;
            animation-fill-mode: none;
            animation-play-state: running;
            animation-name: spinner-line-fade-default;

            @media (prefers-color-scheme: dark) {
                background: rgb(255, 255, 255);
            }

            @media (prefers-contrast: more) {
                animation-name: spinner-line-fade-increased-contrast;
            }
        }
    }

    @for $i from 0 to $spinner-nibs {
        .pulse-spinner__nib--#{$i + 1} {
            $degrees: math.div(360, $spinner-nibs) * $i;
            $nib-delay: $spinner-duration -
                (math.div($spinner-duration, $spinner-nibs) * $i);
            transform: rotate(#{$degrees}deg) translateX($spinner-nib-distance);

            &::before {
                animation-delay: -$nib-delay;
            }
        }
    }

    $spinner-nib-minimum-opacity: 0.08;
    $spinner-nib-maxiumum-opacity: 0.55;
    $spinner-nib-minimum-opacity-increased-contrast: 0.1;
    $spinner-nib-maxiumum-opacity-increased-contrast: 0.8;

    @keyframes spinner-line-fade-default {
        0%,
        100% {
            opacity: $spinner-nib-maxiumum-opacity;
        }

        95% {
            opacity: $spinner-nib-minimum-opacity; // minimum opacity
        }

        1% {
            opacity: $spinner-nib-maxiumum-opacity; // maximum opacity
        }
    }

    // Increased Contrast Fade
    @keyframes spinner-line-fade-increased-contrast {
        0%,
        100% {
            opacity: $spinner-nib-maxiumum-opacity-increased-contrast;
        }

        95% {
            opacity: $spinner-nib-minimum-opacity-increased-contrast; // minimum opacity
        }

        1% {
            opacity: $spinner-nib-maxiumum-opacity-increased-contrast; // maximum opacity
        }
    }
</style>
