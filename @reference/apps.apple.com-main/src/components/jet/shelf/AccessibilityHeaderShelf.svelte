<script lang="ts" context="module">
    import {
        type Action,
        type FlowAction,
        type GenericPage,
        type AccessibilityParagraph,
        type Shelf,
        isFlowAction,
    } from '@jet-app/app-store/api/models';

    import {
        isAccessibilityFeaturesShelf,
        type AccessibilityFeaturesShelf,
    } from '~/components/jet/shelf/AccessibilityFeaturesShelf.svelte';

    interface AccessibilityParagraphShelf extends Shelf {
        items: [AccessibilityParagraph];
    }

    interface AccessibilityHeaderShelf extends AccessibilityParagraphShelf {
        items: [AccessibilityParagraph];
    }

    interface AccessibilityDetailPage extends GenericPage {
        shelves: (AccessibilityFeaturesShelf | AccessibilityParagraphShelf)[];
    }

    interface AccessibilityDetailPageFlowAction extends FlowAction {
        page: 'accessibilityDetails';
        pageData: AccessibilityDetailPage;
    }

    export function isAccessibilityHeaderShelf(
        shelf: Shelf,
    ): shelf is AccessibilityHeaderShelf {
        let { contentType, items, title } = shelf;

        return (
            contentType === 'accessibilityParagraph' &&
            !!title &&
            Array.isArray(items)
        );
    }

    function isAccessibilityParagraphShelf(
        shelf: Shelf,
    ): shelf is AccessibilityParagraphShelf {
        let { contentType, items } = shelf;

        return contentType === 'accessibilityParagraph' && Array.isArray(items);
    }

    function isAccessibilityDetailFlowAction(
        action: Action,
    ): action is AccessibilityDetailPageFlowAction {
        return isFlowAction(action) && action.page === 'accessibilityDetails';
    }
</script>

<script lang="ts">
    import Modal from '@amp/web-app-components/src/components/Modal/Modal.svelte';
    import ContentModal from '~/components/jet/item/ContentModal.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ShelfTitle from '~/components/Shelf/Title.svelte';
    import AccessibilityParagraphItem from '~/components/jet/item/AccessibilityParagraphItem.svelte';
    import AccessibilityFeaturesItem from '~/components/jet/item/AccessibilityFeaturesItem.svelte';
    import { getI18n } from '~/stores/i18n';
    import { getAccessibilityLayoutConfiguration } from '~/context/accessibility-layout';

    export let shelf: AccessibilityHeaderShelf;

    $: ({ withBottomPadding } = getAccessibilityLayoutConfiguration(shelf));

    let modalComponent: Modal | undefined;
    let modalTriggerElement: HTMLElement | null = null;

    const { seeAllAction } = shelf;
    const i18n = getI18n();
    const translateFn = (key: string) => $i18n.t(key);
    const handleModalClose = () => modalComponent?.close();
    const handleOpenModalClick = (e: Event) => {
        modalTriggerElement = e.target as HTMLElement;
        modalComponent?.showModal();
    };

    const destination =
        seeAllAction && isAccessibilityDetailFlowAction(seeAllAction)
            ? seeAllAction
            : undefined;
    const pageData = destination?.pageData;
</script>

<ShelfWrapper {shelf} {withBottomPadding}>
    <div slot="title" class="title-container">
        {#if shelf.title}
            {#if destination}
                <button on:click={handleOpenModalClick}>
                    <ShelfTitle
                        title={shelf.title}
                        seeAllAction={destination}
                    />
                </button>
            {:else}
                <ShelfTitle title={shelf.title} />
            {/if}
        {/if}

        {#if pageData}
            <Modal {modalTriggerElement} bind:this={modalComponent}>
                <ContentModal
                    on:close={handleModalClose}
                    title={pageData.title || null}
                    subtitle={null}
                >
                    <svelte:fragment slot="content">
                        <div class="modal-content-container">
                            {#each pageData.shelves as shelf}
                                <div class="content-section">
                                    {#if isAccessibilityParagraphShelf(shelf)}
                                        {#each shelf.items as item}
                                            <AccessibilityParagraphItem
                                                {item}
                                            />
                                        {/each}
                                    {/if}

                                    {#if isAccessibilityFeaturesShelf(shelf)}
                                        {#each shelf.items as item}
                                            <AccessibilityFeaturesItem
                                                {item}
                                                isDetailView={true}
                                            />
                                        {/each}
                                    {/if}
                                </div>
                            {/each}
                        </div>
                    </svelte:fragment>
                </ContentModal>
            </Modal>
        {/if}
    </div>

    <div class="header-container">
        <div>
            <AccessibilityParagraphItem item={shelf.items[0]} />
        </div>
    </div>
</ShelfWrapper>

<style>
    .title-container {
        display: flex;
        justify-content: space-between;
        padding-top: 16px;
        padding-inline-end: var(--bodyGutter);
    }

    .header-container {
        margin: 0 var(--bodyGutter);
    }

    .header-container div {
        @media (--range-medium-up) {
            width: 66%;
        }
    }

    .modal-content-container {
        font: var(--body-tall);
        white-space: normal;
    }

    .modal-content-container .content-section {
        padding-top: 20px;
        border-top: 1px solid var(--defaultLine);
    }

    .modal-content-container .content-section:not(:first-child) {
        margin-top: 20px;
    }
</style>
