<script lang="ts">
    import type { HTMLAnchorAttributes } from 'svelte/elements';
    import type { ExternalUrlAction } from '@jet-app/app-store/api/models';
    import ArrowIcon from '@amp/web-app-components/assets/icons/arrow.svg';
    import { getJetPerform } from '~/jet';

    type AllowedAnchorAttributes = Omit<
        HTMLAnchorAttributes,
        // The `href` attribute is not allowed because it will be provided
        // by the `ExternalUrlAction`
        'href'
    >;

    interface $$Props extends AllowedAnchorAttributes {
        destination: ExternalUrlAction;
        includeArrowIcon?: boolean;
    }

    const perform = getJetPerform();

    export let destination: ExternalUrlAction;
    export let includeArrowIcon: boolean = true;

    function handleClickAction() {
        perform(destination);
    }
</script>

<a
    {...$$restProps}
    data-test-id="external-link"
    href={destination.url}
    target="_blank"
    rel="nofollow noopener noreferrer"
    on:click={handleClickAction}
>
    <slot />
    {#if includeArrowIcon}
        <ArrowIcon class="external-link-arrow" aria-hidden="true" />
    {/if}
</a>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;

    a :global(.external-link-arrow) {
        @include rtl {
            transform: rotate(-90deg);
        }
    }
</style>
