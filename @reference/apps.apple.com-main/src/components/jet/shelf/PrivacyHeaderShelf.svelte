<script lang="ts" context="module">
    import {
        type Action,
        type FlowAction,
        type GenericPage,
        type PrivacyHeader,
        type Shelf,
        isFlowAction,
    } from '@jet-app/app-store/api/models';

    import {
        isPrivacyTypeShelf,
        type PrivacyTypeShelf,
    } from '~/components/jet/shelf/PrivacyTypeShelf.svelte';

    interface PrivacyHeaderShelf extends Shelf {
        items: [PrivacyHeader];
    }

    interface PrivacyDetailPage extends GenericPage {
        shelves: (PrivacyTypeShelf | PrivacyHeaderShelf)[];
    }

    interface PrivacyDetailPageFlowAction extends FlowAction {
        page: 'privacyDetail';
        pageData: PrivacyDetailPage;
    }

    export function isPrivacyHeaderShelf(
        shelf: Shelf,
    ): shelf is PrivacyHeaderShelf {
        let { contentType, items } = shelf;
        return contentType === 'privacyHeader' && Array.isArray(items);
    }

    function isPrivacyDetailFlowAction(
        action: Action,
    ): action is PrivacyDetailPageFlowAction {
        return isFlowAction(action) && action.page === 'privacyDetail';
    }
</script>

<script lang="ts">
    import Modal from '@amp/web-app-components/src/components/Modal/Modal.svelte';
    import ContentModal from '~/components/jet/item/ContentModal.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ShelfTitle from '~/components/Shelf/Title.svelte';
    import PrivacyHeaderItem from '~/components/jet/item/PrivacyHeaderItem.svelte';
    import PrivacyTypeItem from '~/components/jet/item/PrivacyTypeItem.svelte';
    import { getI18n } from '~/stores/i18n';
    import { APP_PRIVACY_MODAL_ID } from '~/utils/metrics';

    export let shelf: PrivacyHeaderShelf;

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
        seeAllAction && isPrivacyDetailFlowAction(seeAllAction)
            ? seeAllAction
            : undefined;
    const pageData = destination?.pageData;
</script>

<ShelfWrapper {shelf} withBottomPadding={false}>
    <div slot="title" class="title-container">
        {#if shelf.title}
            <button on:click={handleOpenModalClick}>
                <ShelfTitle title={shelf.title} seeAllAction={destination} />
            </button>
        {/if}

        {#if pageData}
            <Modal {modalTriggerElement} bind:this={modalComponent}>
                <ContentModal
                    {translateFn}
                    on:close={handleModalClose}
                    title={pageData.title || null}
                    subtitle={null}
                    targetId={APP_PRIVACY_MODAL_ID}
                >
                    <svelte:fragment slot="content">
                        <ul class="modal-content-container">
                            {#each pageData.shelves as shelf}
                                {#if isPrivacyHeaderShelf(shelf)}
                                    {#each shelf.items as item}
                                        <PrivacyHeaderItem {item} />
                                    {/each}
                                {/if}

                                {#if isPrivacyTypeShelf(shelf)}
                                    {#each shelf.items as item}
                                        <PrivacyTypeItem
                                            {item}
                                            isDetailView={true}
                                        />
                                    {/each}
                                {/if}
                            {/each}
                        </ul>
                    </svelte:fragment>
                </ContentModal>
            </Modal>
        {/if}
    </div>

    <div class="header-container">
        <div>
            <PrivacyHeaderItem item={shelf.items[0]} />
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
</style>
