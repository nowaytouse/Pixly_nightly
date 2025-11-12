<script lang="ts">
    import { onMount, createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let modalTriggerElement: HTMLElement | null;
    export let error: boolean = false;
    export let dialogId: string = '';
    export let dialogClassNames: string = '';

    /**
     * Disable the background scrim for this modal. Used with fullscreen modal
     * variants that don't apply a scrim while transitioning in or out of view.
     */
    export let disableScrim: boolean = false;

    /**
     * Whether to immediately display the modal when the component is mounted.
     */
    export let showOnMount: boolean = false;

    /**
     * If true, suppress the default `close` event fired by the native <dialog> element.
     * Instead, a `close` event is dispatched to be handled by the consuming component.
     * This is useful for modals that implement custom transitions and need to wait for
     * transitions to end on child elements before <dialog> removes them from the DOM.
     *
     * Note that if this option is used, the consuming component *must* call `close()`
     * on this component to properly close the modal!
     */
    export let preventDefaultClose: boolean = false;

    /**
     * ID for element that contains accessible modal title.
     */
    export let ariaLabelledBy: string | null = null;

    /**
     * Accessible modal title. Note that this should only be used when there is no element
     * containing the modal title that can be associated using `ariaLabelledBy`.
     */
    export let ariaLabel: string | null = null;

    let ariaHidden: boolean = true;

    let dialogElement: HTMLDialogElement;
    let needsPolyfill: boolean = false;
    let isDialogInShadow: boolean;

    export function showModal() {
        // noscroll class ensures that when this component is in a shadow DOM context,
        // the parent app can control the background scroll behavior
        document.body.classList.add('noscroll');

        /*
            in non-shadow DOM contexts, add the dialog directly to the body to
            avoid stacking context issues where the the dialog hides behind side nav on Music
            see: https://github.com/GoogleChrome/dialog-polyfill#stacking-context
            if the dialog is within the shadow DOM (being used as a web component)
            do not append to the body and use showModal method to keep dialog within the shadow DOM
        */
        if (needsPolyfill) {
            isDialogInShadow = isInShadow(dialogElement);
            if (!isDialogInShadow) {
                document.body.appendChild(dialogElement);
            }
        }
        ariaHidden = false;
        dialogElement.showModal();
    }

    export function close() {
        document.body.classList.remove('noscroll');

        // in non-shadow DOM + polyfill instances we added the dialog
        // directly to the body, this removes it
        if (needsPolyfill && !isDialogInShadow) {
            document.body.removeChild(dialogElement);
        }

        ariaHidden = true;
        dialogElement.close();
        modalTriggerElement?.focus();
    }

    function handleClose(e: Event) {
        if (preventDefaultClose) {
            e.preventDefault();
        } else {
            close();
        }
        dispatch('close');
    }

    function isInShadow(node: HTMLElement | ParentNode) {
        for (; node; node = node.parentNode) {
            if (node.toString() === '[object ShadowRoot]') {
                return true;
            }
        }
        return false;
    }

    onMount(async () => {
        // register polyfill for native <dialog> element if needed
        needsPolyfill = !('showModal' in dialogElement);
        if (needsPolyfill) {
            const { default: dialogPolyfill } = await import('dialog-polyfill');
            dialogPolyfill.registerDialog(dialogElement);
            dialogElement.classList.add('dialog-polyfill');
        }

        if (showOnMount) {
            showModal();
        }
    });
</script>

<!--
  @component
  Dialog element wrapping a slot.
  This component is multipurpose and should be used
  anywhere a centered modal with a backdrop is needed
 -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<dialog
    data-testid="dialog"
    class:error
    class:no-scrim={disableScrim}
    class={dialogClassNames}
    class:needs-polyfill={needsPolyfill}
    id={dialogId}
    bind:this={dialogElement}
    on:click|self={handleClose}
    on:close={handleClose}
    on:cancel={handleClose}
    aria-labelledby={ariaLabelledBy}
    aria-label={ariaLabel}
    aria-hidden={ariaHidden}
>
    <slot {handleClose} />
</dialog>

<style lang="scss">
    @use '@amp/web-shared-styles/app/core/globalvars' as *;

    /* dialog polyfill styles need to be available
     globally to avoid being stripped out */
    :global(.needs-polyfill) {
        position: absolute;
        left: 0;
        right: 0;
        width: fit-content;
        height: fit-content;
        margin: auto;
        border: solid;
        padding: 1em;
        background: white;
        color: black;
        display: block;

        &:not([open]) {
            display: none;
        }

        & + .backdrop {
            position: fixed;
            top: 0;
            right: 0;
            bottom: 0;
            left: 0;
            background: rgba(0, 0, 0, 0.1);
        }

        &._dialog_overlay {
            position: fixed;
            top: 0;
            right: 0;
            bottom: 0;
            left: 0;
        }

        &.fixed {
            position: fixed;
            top: 50%;
            transform: translate(0, -50%);
        }
    }

    /* dialog polyfill sets position: absolute - this
     needs to be reset to ensure the dialog does not
     scroll to top on open */
    dialog:modal {
        position: fixed;
    }

    dialog {
        width: var(--modalWidth, fit-content);
        height: var(--modalHeight, fit-content);
        max-width: var(--modalMaxWidth, initial);
        max-height: var(--modalMaxHeight, initial);
        border-radius: var(--modalBorderRadius, $modal-border-radius);
        border: 0;
        padding: 0;
        color: var(--systemPrimary);
        background: transparent;

        // Hide scrollbar while opening sliding modal
        overflow: var(--modalOverflow, auto);
        top: var(--modalTop, 0);
        font: var(--body);

        &:focus {
            outline: none;
        }

        &::backdrop,
        & + :global(.backdrop) /* for polyfill */ {
            background-color: var(--modalScrimColor, rgba(0, 0, 0, 0.45));
        }

        // ::backdrop does not inherit from anything, so CSS properties must be set on
        // it directly in order to have any effect.
        &.no-scrim::backdrop,
        &.no-scrim + :global(.backdrop) {
            --modalScrimColor: transparent;
        }
    }

    // disable error modal animation until svelte animations are implemented
    // rdar://92356192 (JMOTW: Error Modal: Use Svelte animations)
    // $error-modal-duration: 0.275s;
    // dialog.error {
    //     box-shadow: $dialog-inset-shadow, $dialog-shadow;
    //     animation-name: modalZoomIn;
    //     animation-duration: $error-modal-duration;
    //     animation-timing-function: cubic-bezier(0.27, 1.01, 0.43, 1.19);
    // }
    // @keyframes modalZoomIn {
    //     from {
    //         opacity: 0;
    //         transform: scale3d(0, 0, 0);
    //     }
    // }
</style>
