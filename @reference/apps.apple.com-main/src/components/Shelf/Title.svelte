<!--
@component

Renders the "Title" and "See All action" for a `Shelf`

### Supported CSS Variables

- `--shelf-title-font`: overrides the font used for the "title" element

-->
<script lang="ts">
    import { type Opt, isSome } from '@jet/environment/types/optional';
    import { type Action, isFlowAction } from '@jet-app/app-store/api/models';

    import SFSymbol from '~/components/SFSymbol.svelte';
    import LinkWrapper from '../LinkWrapper.svelte';

    export let title: string;
    export let subtitle: Opt<string> = undefined;
    export let seeAllAction: Opt<Action> = undefined;
</script>

<div class="title-action-wrapper" class:with-subtitle={!!subtitle}>
    <LinkWrapper action={seeAllAction} label={title}>
        <div class="link-contents">
            <h2 class="shelf-title" data-test-id="shelf-title">{title}</h2>

            {#if isSome(seeAllAction) && isFlowAction(seeAllAction)}
                <span
                    class="chevron-container"
                    data-test-id="shelf-see-all-chevron"
                    aria-hidden="true"
                >
                    <SFSymbol name="chevron.forward" />
                </span>
            {/if}
        </div>
    </LinkWrapper>
</div>

{#if subtitle}
    <p>{subtitle}</p>
{/if}

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/helpers' as *;
    @use 'ac-sasskit/core/locale' as *;

    .title-action-wrapper {
        display: flex;
        align-items: end;
        justify-content: space-between;
        margin: 0 var(--bodyGutter) 13px;
    }

    .title-action-wrapper.with-subtitle {
        margin-bottom: 3px;
    }

    .title-action-wrapper :global(a:hover) {
        text-decoration: none;
    }

    p {
        font: var(--title-3-tall);
        color: var(--systemSecondary);
        margin: 0 var(--bodyGutter) 13px;
    }

    h2 {
        color: var(--systemPrimary, #000);
        font: var(--shelf-title-font, var(--title-2-emphasized));
        text-wrap: pretty;
    }

    .link-contents {
        display: flex;
        align-items: center;
        gap: 6px;
    }

    .chevron-container {
        line-height: 0;
        padding: 6px 0 4px;
        display: block;
    }

    .chevron-container :global(svg) {
        height: 12px;
        display: block;
        translate: 0 0;
        transition: translate 210ms ease-out;

        @include rtl {
            transform: rotate(180deg);
        }
    }

    .chevron-container :global(svg path:not([fill='none'])) {
        fill: var(--systemGray2);
    }

    .link-contents:hover .chevron-container :global(svg) {
        translate: 1px 0;

        @include rtl {
            transform: rotate(180deg);
            translate: -1px 0;
        }
    }
</style>
