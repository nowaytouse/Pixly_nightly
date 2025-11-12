<script lang="ts">
    import { createEventDispatcher, onMount } from 'svelte';
    import CloseIcon from '@amp/web-app-components/assets/icons/close.svg';
    import { updateScrollAndWindowDependentVisuals } from '@amp/web-app-components/src/actions/updateScrollAndWindowDependentVisuals';
    import { focusNodeOnMount } from '@amp/web-app-components/src/actions/focus-node-on-mount';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';

    export let title: string | null;
    export let subtitle: string | null;
    export let text: string = null;
    export let translateFn: (key: string) => string;
    export let dialogTitleId: string | null = null;

    let contentContainerElement: HTMLElement;
    let contentIsScrolling = false;
    let hideGradient = false;

    const dispatch = createEventDispatcher();

    const handleCloseButton = (e: Event) => {
        e.preventDefault();
        e.stopPropagation();

        dispatch('close');
    };

    onMount(() => {
        // get initial state for hideGradient value, before user has scrolled
        let { scrollHeight, offsetHeight } = contentContainerElement;
        hideGradient = scrollHeight - offsetHeight === 0;
    });
</script>

<div
    data-testid="content-modal"
    class="content-modal-container"
    class:hide-gradient={hideGradient}
    dir="auto"
>
    <div class="button-container">
        <button
            data-testid="content-modal-close-button"
            class="close-button"
            type="button"
            on:click={handleCloseButton}
            aria-label={translateFn('AMP.Shared.AX.Close')}
            use:focusNodeOnMount
        >
            <CloseIcon data-testid="content-modal-close-button-svg" />
        </button>
        {#if $$slots['button-container']}
            <slot name="button-container" />
        {/if}
    </div>
    {#if title || subtitle}
        <div
            class="header-container"
            class:content-is-scrolling={contentIsScrolling}
        >
            {#if title}
                <h1
                    id={dialogTitleId}
                    data-testid="content-modal-title"
                    class="title"
                >
                    {title}
                </h1>
            {/if}
            {#if subtitle}
                <h2 data-testid="content-modal-subtitle" class="subtitle">
                    {subtitle}
                </h2>
            {/if}
        </div>
    {/if}
    {#if text || $$slots['content']}
        <div
            class="content-container"
            bind:this={contentContainerElement}
            use:updateScrollAndWindowDependentVisuals
            on:scrollStatus={(e) => {
                contentIsScrolling = e.detail.contentIsScrolling;
                hideGradient = e.detail.hideGradient;
            }}
        >
            {#if $$slots['content']}
                <slot name="content" />
            {:else}
                <p data-testid="content-modal-text">
                    {@html sanitizeHtml(text)}
                </p>
            {/if}
        </div>
    {/if}
</div>

<style lang="scss">
    .content-modal-container {
        position: relative;
        min-height: 230px;
        max-height: calc(100vh - 160px);
        height: auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        max-width: 691px;
        width: 80vw;
        overflow: hidden;
        background-color: var(--pageBG);
        border-radius: var(--modalBorderRadius);

        @media (--range-xsmall-only) {
            max-width: auto;
            width: calc(100vw - 50px);
        }

        &::after {
            position: absolute;
            bottom: 0;
            height: 64px;
            opacity: 1;
            pointer-events: none;
            transition-delay: 0s;
            transition-duration: 300ms;
            transition-property: height, width, background;
            width: calc(100% - 60px);
            content: '';
            background: linear-gradient(
                to top,
                var(--pageBG) 0%,
                rgba(var(--pageBG-rgb), 0) 100%
            );
            z-index: var(--z-default);

            @media (--range-xsmall-only) {
                width: calc(100% - 40px);
            }
        }
    }

    .header-container {
        pointer-events: none;
        position: sticky;
        transition-delay: 0s;
        transition-duration: 500ms;
        transition-property: height, width;
        width: 100%;
        max-height: 120px;
        padding-bottom: 22px;
        z-index: var(--z-default);
    }

    .content-is-scrolling {
        box-shadow: 0 3px 5px var(--systemQuaternary);
    }

    .button-container {
        display: flex;
        align-self: flex-start;
        justify-content: space-between;
        width: 100%;
    }

    .close-button {
        margin-top: 16px;
        margin-bottom: 20px;
        width: 18px;
        height: 18px;
        fill: var(--systemSecondary);
        margin-inline-start: 20px;
    }

    .title {
        color: var(--systemPrimary);
        padding: 0 30px;
        font: var(--title-1-emphasized);

        @media (--range-xsmall-only) {
            padding-inline-start: 20px;
            padding-inline-end: 20px;
        }

        @media (--small) {
            font: var(--large-title-emphasized);
        }
    }

    .subtitle {
        color: var(--systemSecondary);
        padding: 0 30px;
        font: var(--body);

        @media (--range-xsmall-only) {
            padding-inline-start: 20px;
            padding-inline-end: 20px;
        }
    }

    .content-container {
        position: relative;
        width: 100%;
        height: calc(100% - 120px);
        padding-bottom: 42px;
        overflow-y: auto;
        white-space: pre-wrap;
        text-align: start;
        font: var(--title-3-tall);
        padding-inline-start: 30px;
        padding-inline-end: 30px;

        @media (--range-xsmall-only) {
            padding-inline-start: 20px;
            padding-inline-end: 20px;
        }
    }

    .hide-gradient {
        &::after {
            opacity: 0;
        }
    }
</style>
