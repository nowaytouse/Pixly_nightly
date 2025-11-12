<script lang="ts">
    import type { ComponentType } from 'svelte';

    export let icon: ComponentType;
    export let label: string;
</script>

<div class="navigation-item__content">
    {#if $$slots['prefix']}
        <slot name="prefix" />
    {/if}

    <span class="navigation-item__icon">
        <slot name="icon">
            <svelte:component this={icon} aria-hidden="true" />
        </slot>
    </span>

    <span class="navigation-item__label">
        <slot name="label">
            {label}
        </slot>
    </span>
</div>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'amp/stylekit/core/mixins/line-clamp' as *;
    @use 'amp/stylekit/core/mixins/overflow-bleed' as *;
    @use 'ac-sasskit/core/locale' as *;

    .navigation-item__content {
        border-radius: inherit;
        display: flex;
        align-items: center;
        width: 100%;
        column-gap: 8px;
        color: var(--navigation-item-text-color, var(--systemPrimary));

        :global(.navigation-item--selected) & {
            font: var(--title-2-emphasized);

            @media (--sidebar-visible) {
                font: var(--title-3-medium);
            }
        }
    }

    .navigation-item__icon {
        line-height: 0; // Normalize line height
        flex: 0 0;
        flex-basis: var(--navigation-item-icon-size, 32px);

        :global(svg) {
            width: 100%;
            height: 100%;
            fill: var(--navigation-item-icon-color, var(--keyColor));
        }

        @media (--sidebar-visible) {
            flex-basis: var(--navigation-item-icon-size, 24px);
        }
    }

    .navigation-item__label {
        flex: 1;

        @include line-clamp;
        @include overflow-bleed(4px);
    }
</style>
