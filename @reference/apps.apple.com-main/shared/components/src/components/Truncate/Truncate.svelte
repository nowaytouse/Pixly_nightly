<script lang="ts">
    import { onMount } from 'svelte';
    import { makeSafeTick } from '@amp/web-app-components/src/utils/makeSafeTick';
    import Modal from '@amp/web-app-components/src/components/Modal/Modal.svelte';
    import ContentModal from '@amp/web-app-components/src/components/Modal/ContentModal.svelte';
    import { debounce } from '@amp/web-app-components/src/utils/debounce';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';
    import type { SvelteComponent } from 'svelte';
    import { getUniqueIdGenerator } from '@amp/web-app-components/src/utils/uniqueId';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    /**
     * @name Truncate
     *
     * @description
     * This implements Truncate component that used to show truncated text with modal.
     *
     * Design:
     * https://pd-hi.apple.com/viewvc/Common/Modules/macOS/Music/-Common%20Elements/Truncation.png?revision=55587
     *
     */

    export let text: string;
    export let lines: number = 4; // Indicate how many lines to truncate, default to 4
    export let title: string | null = null;
    export let subtitle: string | null = null;
    export let translateFn: (key: string) => string;
    export let modalType: 'contentModal' | null = null;
    export let typography: 'title-3' | null = null;
    export let bodyTypography: 'body' | null = null;
    export let isPortalModal: boolean = false;
    export let expandText: boolean = false;
    export let usePillVariant: boolean = false;
    export let sanitizeHtmlOptions: object = {
        allowedTags: [''],
        keepChildrenWhenRemovingParent: true,
    };

    let modalComponent: SvelteComponent;
    let truncateContent: HTMLElement;
    let needsTruncation = false;
    let modalTriggerElement = null;

    function detectTruncate() {
        needsTruncation =
            truncateContent.scrollHeight > truncateContent.clientHeight;
    }

    function handleMoreBtnClick(e: Event) {
        e.preventDefault();
        e.stopPropagation();

        if (expandText) {
            needsTruncation = false;
            truncateContent.style.setProperty('--lines', 'unset');
        } else {
            handleOpenModalClick(e);
        }
    }

    function handleOpenModalClick(e: Event) {
        modalTriggerElement = e.target;
        dispatch('openModal', e);

        if (modalComponent) {
            modalComponent.showModal();
        }
    }

    function handleModalClose() {
        modalComponent.close();
    }

    const dialogTitleId = getUniqueIdGenerator()();
    const safeTick = makeSafeTick();
    const moreButtonText = translateFn('AMP.Shared.Truncate.More') ?? '';

    onMount(async () => {
        await safeTick(async (tick) => {
            // To make sure Modal bind:this setup properly before onmount
            await tick();
            detectTruncate();
        });
    });
</script>

<!-- Detect whether need truncated or not when window resizing -->
<svelte:window on:resize={debounce(detectTruncate, 100)} />

<div class="truncate-wrapper" class:pill={usePillVariant && needsTruncation}>
    <p
        data-testid="truncate-text"
        bind:this={truncateContent}
        dir="auto"
        class="content"
        class:with-more-button={needsTruncation}
        class:title-3={typography === 'title-3'}
        class:body={bodyTypography === 'body'}
        style:--lines={lines ?? 4}
        style:--line-height="var(--lineHeight, 16)"
        style:--link-length={moreButtonText.length}
    >
        {@html sanitizeHtml(text, sanitizeHtmlOptions)}
    </p>
    {#if needsTruncation}
        <button
            data-testid="truncate-more-button"
            class="more"
            type="button"
            on:click={handleMoreBtnClick}
        >
            {moreButtonText}
        </button>
    {/if}
</div>

{#if needsTruncation && !isPortalModal}
    <Modal
        {modalTriggerElement}
        bind:this={modalComponent}
        ariaLabelledBy={dialogTitleId}
    >
        {#if modalType === 'contentModal'}
            <ContentModal
                {title}
                {subtitle}
                {text}
                {translateFn}
                {dialogTitleId}
                on:close={handleModalClose}
            />
        {/if}
    </Modal>
{/if}

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;
    @use 'amp/stylekit/core/mixins/line-clamp' as *;

    .truncate-wrapper {
        position: relative;
        z-index: var(--z-default);
    }

    .content {
        white-space: pre-wrap;
        font: var(--truncate-font, var(--body-tall));

        @include line-clamp(var(--lines));

        &.title-3 {
            font: var(--title-3);

            // The next line applies if `--lineHeight` was set by a parent.
            line-height: calc(var(--lineHeight) * 1px);
        }

        &.body {
            font: var(--body);

            // The next line applies if `--lineHeight` was set by a parent.
            line-height: calc(var(--lineHeight) * 1px);
        }
    }

    .with-more-button {
        // CSS properties to build the mask based on the "MORE" button
        // --one-ch property controls character width and font size
        --fade-direction: 270deg;
        word-break: break-word;
        position: relative; // For `More` link positioning.
        // prettier-ignore
        mask: linear-gradient(
                0deg,
                transparent 0,
                transparent calc(var(--line-height) * 1px),
                #000 calc(var(--line-height) * 1px)
            ),
            linear-gradient(
                var(--fade-direction),
                transparent 0,
                transparent calc((var(--link-length) * var(--one-ch, 8)) * 1px + var(--inline-mask-offset, 0px)),
                #000 calc(((var(--link-length) * var(--one-ch, 8)) + (var(--line-height) * 2)) * 1px + var(--inline-mask-offset, 0px)),
            );
        mask-size: initial, initial;
        mask-position: right bottom;
        z-index: var(--z-default);

        @include rtl {
            --fade-direction: 90deg;
            mask-position: left bottom;
        }
    }

    .more {
        position: absolute;
        bottom: var(--moreBottomPositionOverride, 1px);
        color: var(--moreTextColorOverride, var(--systemPrimary));
        inset-inline-end: 0;
        padding-inline-start: 5px;
        font: var(--moreFontOverride, var(--subhead-emphasized));
        z-index: var(--z-default);
    }

    .pill {
        --inline-mask-offset: 12px; // accommodate pill width in text mask

        .more {
            padding: 0 6px;
            border-radius: 8px;
            margin-inline-start: 3px;
            inset-inline-end: 2px;
            bottom: var(--moreBottomPositionOverride, 2px);
            font: var(--subhead-emphasized);
            background-color: var(--systemSecondary-onDark);
            color: white; // white per spec, no vars
        }
    }
</style>
