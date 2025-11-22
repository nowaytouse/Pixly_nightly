<script lang="ts">
    // TODO: rdar://92270447 (JMOTW: Refactor ButtonAction component to use Button component)
    import { createEventDispatcher, onMount } from 'svelte';
    import { makeSafeTick } from '@amp/web-app-components/src/utils/makeSafeTick';

    const dispatch = createEventDispatcher();

    const handleButtonClick = () => {
        dispatch('buttonClick');
    };

    // Button A, B, etc. refers to the button spec
    // https://pd-hi.apple.com/viewvc/Common/Modules/macOS/Music/-Common%20Elements/Buttons.png
    // alertButton and alertButtonSecondary refer to Alert Modal spec
    // https://pd-hi.apple.com/viewvc/Common/Modules/macOS/-Cross%20Product/_web%20-%20Alerts.png
    type ButtonType =
        | 'buttonA'
        | 'buttonB'
        | 'buttonD'
        | 'alertButton'
        | 'alertButtonSecondary'
        | 'pillButton'
        | 'socialProfileButton'
        | 'textButton'
        | null;

    export let buttonStyle: string | null = null;
    export let makeFocused = false;
    export let ariaLabel: string | null = null;
    export let type: 'button' | 'submit' = 'button';
    export let disabled = false;
    export let buttonElement: HTMLButtonElement = null;

    // Need to do this to resolve TS error:
    // Type 'string' is not assignable to type 'ButtonType'
    $: buttonType = buttonStyle as ButtonType;

    function handleKeyUp(e: KeyboardEvent) {
        if (e.key === 'Enter' || e.key === 'Escape') {
            handleButtonClick();
        }
    }

    const safeTick = makeSafeTick();

    onMount(async () => {
        await safeTick(async (tick) => {
            await tick();
            if (makeFocused) {
                buttonElement.focus();
            }
        });
    });
</script>

<div
    class="button"
    class:primary={buttonType === 'buttonA'}
    class:secondary={buttonType === 'buttonB'}
    class:tertiary={buttonType === 'buttonD'}
    class:alert={buttonType && buttonType.startsWith('alertButton')}
    class:alert-secondary={buttonType === 'alertButtonSecondary'}
    class:pill={buttonType === 'pillButton'}
    class:button--text-button={buttonType === 'textButton'}
    class:socialProfileButton={buttonType === 'socialProfileButton'}
    data-testid="button-base-wrapper"
>
    <button
        on:click={handleButtonClick}
        data-testid="button-base"
        aria-label={ariaLabel}
        bind:this={buttonElement}
        on:keyup={handleKeyUp}
        class:link={buttonType === 'textButton'}
        {type}
        {disabled}
    >
        {#if $$slots['icon-before']}
            <div class="button__icon button__icon--before">
                <slot name="icon-before" />
            </div>
        {/if}
        <slot />
        {#if $$slots['icon-after']}
            <div class="button__icon button__icon--after">
                <slot name="icon-after" />
            </div>
        {/if}
    </button>
</div>

<style lang="scss">
    @use '@amp/web-shared-styles/app/core/globalvars' as *;
    @use '@amp/web-shared-styles/app/core/mixins/keycolor-button-states' as *;

    // TODO: rdar://104573582 (Refactor <Button> and <ButtonAction> styles)
    .button {
        width: var(--buttonWrapperWidth, 100%);

        @media (--medium) {
            width: var(--buttonWrapperWidth, auto);
        }

        /* TODO: rdar://78161351: this is kind of messy */
        button {
            width: var(--buttonWidth, 100%);
            height: var(--buttonHeight, 36px);
            display: var(--buttonDisplay, flex);
            color: var(--buttonTextColor, white);
            background-color: var(
                --buttonBackgroundColor,
                var(--keyColorBG, var(--systemBlue))
            );
            align-items: center;
            justify-content: var(--buttonJustifyContent, center);
            border-radius: var(--buttonRadius, #{$global-border-radius-xsmall});
            font: var(--buttonFont, var(--body-emphasized));

            @media (--medium) {
                width: var(--buttonWidth, auto);
                min-width: 100px;
                height: var(--buttonHeight, #{$action-button-size});
            }

            &[disabled] {
                opacity: var(--buttonDisabledOpacity, 0.75);
                background-color: var(
                    --buttonDisabledBGColor,
                    var(--systemQuinary)
                );
                color: var(--buttonDisabledTextColor, var(--systemTertiary));
                cursor: default;

                @media (prefers-color-scheme: dark) {
                    opacity: var(--buttonDisabledOpacityDark, 1);
                    background-color: var(
                        --buttonDisabledBGColorDark,
                        rgba(255, 255, 255, 0.5)
                    );
                    color: var(
                        --buttonDisabledTextColorDark,
                        var(--systemTertiary-onLight)
                    );
                }
            }
        }

        &.primary button {
            color: var(--buttonTextColor, white);
            background-color: var(
                --buttonBackgroundColor,
                var(--keyColorBG, var(--systemBlue))
            );
            padding: 0 10px;

            &:disabled {
                opacity: 0.5;
            }
        }

        &.secondary {
            width: auto;

            button {
                --buttonBackgroundColor: transparent;
                min-width: var(--buttonMinWidth, 108px);
                color: var(--buttonTextColor, var(--keyColor));
                border: 1px solid
                    var(--buttonBorderColor, var(--keyColor, var(--systemBlue)));
                font: var(--body-tall);
                padding-inline-start: 16px;
                padding-inline-end: 16px;
            }
        }

        // the tertiary styles are used for button type D
        // currently only used in the snapshot project
        &.tertiary {
            width: auto;

            button {
                --buttonBackgroundColor: var(--keyColorBG, var(--systemBlue));
                --buttonTextColor: white;
                padding-inline-start: 22px;
                padding-inline-end: 22px;
                width: var(--buttonWidth, auto);
                height: var(--buttonHeight, 45px);
                font: var(--buttonFont, var(--body-reduced-semibold));

                &:hover,
                &:focus,
                &:focus-within {
                    --buttonBackgroundColor: var(
                        --buttonBackgroundColorHover,
                        var(--keyColorBG, var(--systemBlue))
                    );
                    transition: all 100ms ease-in-out;
                }
            }
        }

        &.alert {
            // Prevent button inside modal from shrinking in wide viewport
            --buttonWrapperWidth: 100%;
            --buttonWidth: 100%;
            --buttonHeight: 28px;
            --buttonRadius: 6px;
        }

        &.alert-secondary {
            --buttonTextColor: var(--systemPrimary);
            --buttonBackgroundColor: var(--systemQuinary);

            @media (prefers-color-scheme: dark) {
                --buttonBackgroundColor: var(--systemTertiary);
            }
        }

        &.pill {
            --buttonBackgroundColor: rgba(var(--keyColor-rgb), 0.06);
            --buttonTextColor: var(--keyColor);

            button {
                min-width: var(--buttonMinWidth, 90px);
                width: var(--buttonWidth, auto);
                height: var(--buttonHeight, 28px);
                border-radius: var(--buttonBorderRadius, 16px);
                padding-inline-start: var(--buttonPadding, 16px);
                padding-inline-end: var(--buttonPadding, 16px);
                font: var(--body-semibold-tall);
            }
        }

        &.socialProfileButton {
            height: auto;
            border-radius: 10px;
            margin-top: 27px;
            width: unset; /* unset inherited value from .button */
            min-width: 90px;
            background-color: var(--keyColorBG);
            z-index: var(--z-default);

            @include keycolor-button-states;
        }

        &.socialProfileButton button {
            padding-top: 9px;
            padding-bottom: 9px;
            color: var(--systemPrimary-onDark);
            height: auto;
            font: var(--title-2);
            padding-inline-start: 22px;
            padding-inline-end: 22px;

            :global(.web-to-native__action) {
                fill: var(--systemPrimary-onDark);
            }
        }
    }

    // Works in conjuction with `link` class in @amp-stylekit/base/typography
    .button--text-button {
        --buttonBackgroundColor: transparent;
        --buttonTextColor: var(--keyColor); // `link` class will inherit this
        --linkHoverTextDecoration: none; // `link` custom property

        button {
            white-space: nowrap;
            font: var(--buttonFont, var(--body));
        }
    }

    .button__icon {
        display: flex;
        fill: var(--buttonIconFill, currentColor);
        height: var(--buttonIconHeight, 1em);
        width: var(--buttonIconWidth, 1em);
        padding: var(--buttonIconPadding, 0);
        margin-top: var(--buttonIconMarginTop, 0);
        margin-bottom: var(--buttonIconMarginBottom, 0);

        &:empty,
        &:has(div:empty) {
            margin: 0;
        }

        @media (hover: hover) {
            button:hover & {
                fill: var(
                    --buttonIconFillHover,
                    var(--buttonIconFill, currentColor)
                );
            }
        }

        @supports #{'selector(:has(:focus-visible))'} {
            button:focus-visible & {
                fill: var(
                    --buttonIconFillFocus,
                    var(--buttonIconFill, currentColor)
                );
            }
        }

        &:active {
            button:active & {
                fill: var(
                    --buttonIconFillActive,
                    var(--buttonIconFill, currentColor)
                );
            }
        }
    }

    .button__icon--before {
        margin-inline-end: var(--buttonIconMargin-inlineEnd, 0.25em);
        margin-inline-start: var(--buttonIconMargin-inlineStart, 0);
    }

    .button__icon--after {
        margin-inline-start: var(--buttonIconMargin-inlineStart, 0.25em);
        margin-inline-end: var(--buttonIconMargin-inlineEnd, 0);
    }
</style>
