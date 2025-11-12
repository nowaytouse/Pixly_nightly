<script lang="ts">
    import type { HTMLAnchorAttributes } from 'svelte/elements';
    import type { FlowAction } from '@jet-app/app-store/api/models';
    import { getJetPerform } from '~/jet';

    type AllowedAnchorAttributes = Omit<
        HTMLAnchorAttributes,
        // The `href` attribute is not allowed because it will be provided
        // by the `FlowAction`
        'href'
    >;

    interface $$Props extends AllowedAnchorAttributes {
        destination: FlowAction;
    }

    const perform = getJetPerform();

    export let destination: FlowAction;

    // Web cannot support internal protocols, so this guard prevents
    // them from showing up in anchor tags.
    $: pageUrl = destination.pageUrl?.includes('x-as3-internal:')
        ? '#'
        : destination?.pageUrl;

    function onClick(event: MouseEvent) {
        event.preventDefault();

        perform(destination);
    }
</script>

<a
    {...$$restProps}
    href={pageUrl}
    data-test-id="internal-link"
    on:click={onClick}
>
    <slot />
</a>
