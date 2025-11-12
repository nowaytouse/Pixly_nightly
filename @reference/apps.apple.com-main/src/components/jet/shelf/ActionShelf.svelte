<script lang="ts" context="module">
    import type { Shelf, Action } from '@jet-app/app-store/api/models';

    interface ActionShelf extends Shelf {
        items: Action[];
    }

    export function isActionShelf(shelf: Shelf): shelf is ActionShelf {
        return shelf.contentType === 'action' && Array.isArray(shelf.items);
    }
</script>

<script lang="ts">
    import Artwork, { getNaturalProfile } from '~/components/Artwork.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';

    export let shelf: ActionShelf;
</script>

<ShelfWrapper {shelf}>
    <ShelfItemLayout {shelf} gridType="F" let:item>
        {@const action = item}
        {@const artwork = item.artwork}
        {@const title = item.title}

        <div class="container">
            <LinkWrapper {action}>
                {#if artwork}
                    <div class="artwork-container" aria-hidden="true">
                        <Artwork
                            {artwork}
                            profile={getNaturalProfile(artwork, [24])}
                            hasTransparentBackground
                        />
                    </div>
                {/if}
                {title}
            </LinkWrapper>
        </div>
    </ShelfItemLayout>
</ShelfWrapper>

<style>
    .container :global(a) {
        display: flex;
        align-items: center;
        gap: 8px;
        flex-shrink: 0;
        background: var(--pageBG);
        border-radius: var(--global-border-radius-medium);
        box-shadow: var(--shadow-small);
        padding: 16px 10px;
        width: 100%;
        font: var(--title-3-medium);
        transition: background-color 210ms ease-out;
    }

    .container :global(a:hover) {
        /* stylelint-disable color-function-notation */
        background-color: rgb(from var(--pageBG) r g b/0.1);
        /* stylelint-enable color-function-notation */

        @media (prefers-color-scheme: dark) {
            /* stylelint-disable color-function-notation */
            background-color: rgb(from var(--pageBG) r g b/0.85);
            /* stylelint-enable color-function-notation */
        }
    }

    .artwork-container {
        width: 24px;
        height: 24px;
    }

    .container :global(.external-link-arrow) {
        height: 10px;
    }
</style>
