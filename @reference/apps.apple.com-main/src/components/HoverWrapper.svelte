<script lang="ts">
    export let element: keyof HTMLElementTagNameMap = 'article';
    export let hasChin: boolean = false;
</script>

<svelte:element this={element} class="hover-wrapper" class:has-chin={hasChin}>
    <slot />
</svelte:element>

<style lang="scss">
    @use '@amp/web-shared-styles/app/core/mixins/scrim-opacity-controller' as *;
    @use 'amp/stylekit/core/mixins/hover-style' as *;

    .hover-wrapper {
        position: relative;
        display: var(--display, flex);
        overflow: hidden;
        align-items: center;
        cursor: pointer;
        border-radius: var(--global-border-radius-large);
        box-shadow: var(--shadow-small);

        @include scrim-opacity-controller;
    }

    .hover-wrapper.has-chin,
    .hover-wrapper.has-chin::after {
        // For chins, we cannot use `border-raidus` due a Chrome bug with unequal radii
        // (e.g. there is no rounding at the bottom) and mask-image. To get around that,
        // we use clip-path to the same effect.
        // https://issues.chromium.org/issues/40778541.
        border-radius: unset;
        clip-path: inset(
            0 0 0 0 round var(--global-border-radius-large)
                var(--global-border-radius-large) 0 0
        );
    }

    /* stylelint-disable order/order */
    .hover-wrapper::after {
        mix-blend-mode: soft-light;

        @include content-container-hover-style;

        // These properties are overriding those provided by `content-container-hover-style`
        border-radius: var(--global-border-radius-large);
        transition: opacity 210ms ease-out;
    }
    /* stylelint-enable order/order */

    .hover-wrapper:hover::after {
        @include scrim-opacity;
    }
</style>
