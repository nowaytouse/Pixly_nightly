<!--
@component
Wraps a link around the provided slot contents if a valid `FlowAction` or `ExternalUrlAction` is given.
If no valid action is provided, the contents are rendered as-is with no decoration.

💡 For accessibility, this component should ideally wrap the entire visual block (e.g., `div`, `article`) so that
screen readers and keyboard users interpret the entire element as a single link.

@example
```
    <LinkWrapper action={item.clickAction}>
        <article>
            <Artwork artwork={item.artwork} />
            {item.title}
        </article>
    </LinkWrapper>
```
-->
<script lang="ts">
    import { type Action, isFlowAction } from '@jet-app/app-store/api/models';
    import { type Opt, isSome } from '@jet/environment/types/optional';

    import FlowActionComponent from '~/components/jet/action/FlowAction.svelte';
    import { isExternalUrlAction } from '~/jet/models';
    import ExternalUrlAction from './jet/action/ExternalUrlAction.svelte';
    import ShelfBasedPageScrollAction, {
        isShelfBasedPageScrollAction,
    } from './jet/action/ShelfBasedPageScrollAction.svelte';

    export let action: Opt<Action> = null;
    export let label: Opt<string> = null;
    export let withoutLabel: Opt<boolean> = false;
    export let includeExternalLinkArrowIcon: boolean = true;
</script>

{#if isSome(action) && isFlowAction(action) && isSome(action.pageUrl)}
    <FlowActionComponent
        destination={action}
        aria-label={withoutLabel ? null : label || action.title}
    >
        <slot />
    </FlowActionComponent>
{:else if isSome(action) && isExternalUrlAction(action)}
    <ExternalUrlAction
        destination={action}
        aria-label={withoutLabel ? null : label || action.title}
        includeArrowIcon={includeExternalLinkArrowIcon}
    >
        <slot />
    </ExternalUrlAction>
{:else if isSome(action) && isShelfBasedPageScrollAction(action)}
    <ShelfBasedPageScrollAction
        destination={action}
        aria-label={withoutLabel ? null : label || action.title}
    >
        <slot />
    </ShelfBasedPageScrollAction>
{:else}
    <slot />
{/if}
