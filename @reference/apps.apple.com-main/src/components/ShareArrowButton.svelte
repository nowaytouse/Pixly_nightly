<script lang="ts" context="module">
    export function isShareSupported() {
        return (
            typeof navigator !== 'undefined' &&
            typeof navigator.share === 'function'
        );
    }
</script>

<script lang="ts">
    import SFSymbol from '~/components/SFSymbol.svelte';
    import { getI18n } from '~/stores/i18n';

    export let url: string;
    export let withLabel: boolean = false;

    const i18n = getI18n();

    $: isShareSheetOpen = false;

    async function handleShareClick() {
        isShareSheetOpen = !isShareSheetOpen;

        try {
            await navigator.share({ url });
            isShareSheetOpen = false;
        } catch {
            isShareSheetOpen = false;
        }
    }
</script>

<button
    on:click={handleShareClick}
    aria-label={$i18n.t('ASE.Web.AppStore.Share.Button.AccessibilityValue')}
    class:active={isShareSheetOpen}
    class:with-label={withLabel}
>
    <SFSymbol name="square.and.arrow.up" ariaHidden={true} />

    {#if withLabel}
        {$i18n.t('ASE.Web.AppStore.Share.Button.Value')}
    {/if}
</button>

<style lang="scss">
    button {
        position: relative;
        z-index: 2;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
        width: var(--share-arrow-size, 32px);
        height: var(--share-arrow-size, 32px);
        border-radius: var(--share-arrow-size, 32px);
        background: var(--systemQuaternary-onDark);
        transition: background-color 210ms ease-out;
        mix-blend-mode: plus-lighter;
    }

    button.with-label {
        display: flex;
        align-items: center;
        width: auto;
        padding: 0 16px;
        gap: 8px;
        font: var(--body-emphasized);

        :global(svg) {
            height: 16px;
            width: auto;
            top: -2px;
            position: relative;
        }
    }

    button.active,
    button:hover {
        // stylelint-disable color-function-notation
        background-color: rgb(from var(--systemTertiary-onDark) r g b/.13);
        // stylelint-enable color-function-notation
    }

    button :global(svg) {
        width: 37%;
        fill: var(--systemPrimary-onDark);
        overflow: visible;
    }
</style>
