<script lang="ts">
    import Modal from '@amp/web-app-components/src/components/Modal/Modal.svelte';
    import LocaleSwitcherModal from '@amp/web-app-components/src/components/Modal/LocaleSwitcherModal/LocaleSwitcherModal.svelte';
    import LocaleSwitcherLanguages from '@amp/web-app-components/src/components/buttons/LocaleSwitcherButton/LocaleSwitcherLanguages.svelte';
    import type {
        Region,
        Languages,
        Language,
    } from '@amp/web-app-components/src/components/buttons/LocaleSwitcherButton/types';
    import type { Locale } from '@amp/web-app-components/src/types';
    import type { SvelteComponent } from 'svelte';
    import type { StorefrontNames } from '@amp/web-app-components/src/components/banners/types';

    export let translateFn: (
        str: string,
        values?: Record<string, string | number>,
    ) => string;
    export let locale: Locale;
    export let regions: Region[];
    export let languages: Languages;
    export let defaultRoute: string;
    export let storefrontNameTranslations: StorefrontNames;

    $: language = locale.language;
    $: storefront = locale.storefront;

    let modalTriggerElement = null;
    let modalElement: SvelteComponent;

    const handleOpenModalClick = () => {
        // only open modal on click if regions is not empty
        if (regions.length) {
            modalElement.showModal();
        }
    };

    $: otherLanguages = languages[storefront].filter(
        (l: Language) => l.tag.toLowerCase() !== language.toLowerCase(),
    );

    $: storefrontName =
        storefrontNameTranslations[storefront]?.[language] ??
        storefrontNameTranslations[storefront]?.['default'];

    // rdar://102181852 (CHN AM Web app is showing language selector in traditional Chinese.)
    // We should not show the locale switcher or language selector when on the CN storefront
    $: isCNStorefront = storefront === 'cn';
</script>

{#if storefrontName && !isCNStorefront}
    <div
        class="button-container"
        class:languages-new-line={otherLanguages.length >= 6}
    >
        <button
            on:click={handleOpenModalClick}
            class="link"
            data-testid="locale-switcher-button"
        >
            {storefrontName}
        </button>
        <LocaleSwitcherLanguages {translateFn} {otherLanguages} />
    </div>
{/if}

<Modal {modalTriggerElement} bind:this={modalElement}>
    <LocaleSwitcherModal
        {translateFn}
        {regions}
        {defaultRoute}
        on:close={modalElement.close}
    />
</Modal>

<style lang="scss">
    .button-container {
        --linkColor: var(--systemPrimary);
        display: flex;
        margin-bottom: 25px;

        &.languages-new-line {
            @media (--range-small-down) {
                flex-direction: column;

                button {
                    margin-bottom: 5px;
                }
            }
        }
    }

    button {
        line-height: 1;
        display: inline-flex;
        margin-top: 6px;
        vertical-align: middle;
        white-space: nowrap;
    }
</style>
