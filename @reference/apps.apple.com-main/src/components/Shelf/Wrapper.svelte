<script lang="ts">
    import type { Shelf } from '@jet-app/app-store/api/models';
    import ShelfTitle from '~/components/Shelf/Title.svelte';

    export let shelf: Shelf | undefined = undefined;

    /**
     * Whether or not to automatically display the shelf "centered" at the normal
     * page width for the App Store
     *
     * When `false`, the shelf is not constrained horizontally in any way
     *
     * The value of this property may be ignored when the shelf's `.presentationHints`
     * indicate that it is being rendered in a context where "centering" would not be
     * appropriate
     *
     * @default true
     */
    export let centered: boolean = false;

    export let withTopBorder: boolean = false;
    export let withTopMargin: boolean = false;
    export let withPaddingTop: boolean = true;
    export let withBottomPadding: boolean = true;

    $: seeAllAction =
        shelf?.header?.titleAction ??
        shelf?.header?.accessoryAction ??
        shelf?.seeAllAction;
</script>

<section
    id={shelf?.id}
    data-test-id="shelf-wrapper"
    class="shelf"
    class:centered
    class:border-top={withTopBorder}
    class:margin-top={withTopMargin}
    class:padding-top={withPaddingTop}
    class:padding-bottom={withBottomPadding}
>
    {#if $$slots['title']}
        <slot name="title" />
    {:else if shelf?.header?.title}
        <ShelfTitle
            title={shelf.header.title}
            subtitle={shelf.header.subtitle}
            {seeAllAction}
        />
    {:else if shelf?.title}
        <ShelfTitle
            title={shelf.title}
            subtitle={shelf.subtitle}
            {seeAllAction}
        />
    {/if}

    <slot />
</section>

<style>
    .padding-top {
        padding-top: 13px;
    }

    .centered {
        margin: 0 var(--bodyGutter);
    }

    .margin-top {
        margin-top: 13px;
    }

    .border-top {
        border-top: 1px solid var(--systemGray4);
    }

    .shelf.padding-bottom {
        padding-bottom: 32px;
    }
</style>
