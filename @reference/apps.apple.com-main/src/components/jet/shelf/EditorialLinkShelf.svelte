<script lang="ts" context="module">
    import type { Shelf, EditorialLink } from '@jet-app/app-store/api/models';

    interface EditorialLinkShelf extends Shelf {
        contentType: 'smallStoryCard';
        items: [EditorialLink];
    }

    export function isEditorialLinkShelf(
        shelf: Shelf,
    ): shelf is EditorialLinkShelf {
        const { contentType, items } = shelf;
        return contentType === 'editorialLink' && Array.isArray(items);
    }
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ChevronRightIcon from '~/sf-symbols/chevron.right.svg';
    import LinkWrapper from '~/components/LinkWrapper.svelte';

    export let shelf: EditorialLinkShelf;
    $: item = shelf.items[0];
    $: ({ clickAction, descriptionText, summaryText } = item);
</script>

<ShelfWrapper {shelf} withBottomPadding={false}>
    <article>
        <LinkWrapper
            action={clickAction}
            includeExternalLinkArrowIcon={false}
            label={descriptionText}
        >
            <svelte:fragment>
                <div>
                    <span class="title">{descriptionText}</span>
                    <span class="subtitle">{summaryText}</span>
                </div>

                <span class="icon-container" aria-hidden="true">
                    <ChevronRightIcon />
                </span>
            </svelte:fragment>
        </LinkWrapper>
    </article>
</ShelfWrapper>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/helpers' as *;
    @use 'ac-sasskit/core/locale' as *;

    article {
        display: flex;
        align-items: center;
        flex-direction: row;
        justify-content: space-between;
        padding: 16px;
        margin: 0 var(--bodyGutter);
        border-radius: var(--global-border-radius-medium);
        background-color: var(--systemQuinary);
        transition: background-color 210ms ease-out;
    }

    article:hover {
        cursor: pointer;
        // a fallback for browsers that don't support relative colors (e.g. the `from` syntax)
        background-color: var(--systemQuinary);
        // stylelint-disable-next-line color-function-notation
        background-color: rgb(
            from var(--systemQuinary) r g b / calc(alpha + 0.02)
        );
    }

    article:hover .icon-container {
        transform: translateX(2px);

        @include rtl {
            transform: translateX(-2px) rotate(-180deg);
        }
    }

    div {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .title {
        font: var(--body-emphasized);
    }

    .subtitle {
        color: var(--systemSecondary);
    }

    .icon-container {
        position: relative;
        height: 10px;
        aspect-ratio: 0.9;
        transition: transform 210ms ease-out;

        @include rtl {
            transform: rotate(-180deg);
        }
    }

    .icon-container :global(path:not([fill='none'])) {
        fill: var(--systemPrimary);
    }

    article :global(a) {
        display: flex;
        justify-content: space-between;
        align-items: center;
        width: 100%;

        &:hover {
            text-decoration: none;
        }
    }
</style>
