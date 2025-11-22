<script lang="ts" context="module">
    import {
        type Action,
        type FlowAction,
        type GenericPage,
        type Shelf,
        type TitledParagraph,
        isFlowAction,
    } from '@jet-app/app-store/api/models';

    interface TitledParagraphShelf extends Shelf {
        items: [TitledParagraph];
    }

    interface VersionHistoryPage extends FlowAction {
        page: 'versionHistory';
        pageData: GenericPage;
    }

    export function isTitledParagraphShelf(
        shelf: Shelf,
    ): shelf is TitledParagraphShelf {
        const { contentType, items } = shelf;

        return contentType === 'titledParagraph' && Array.isArray(items);
    }

    function isVersionHistoryFlowAction(
        action: Action,
    ): action is VersionHistoryPage {
        return isFlowAction(action) && action.page === 'versionHistory';
    }
</script>

<script lang="ts">
    import { createEventDispatcher, type SvelteComponent } from 'svelte';
    import Modal from '@amp/web-app-components/src/components/Modal/Modal.svelte';
    import ContentModal from '~/components/jet/item/ContentModal.svelte';
    import TitledParagraphItem, {
        isTitledParagraphItem,
    } from '~/components/jet/item/TitledParagraphItem.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ShelfTitle from '~/components/Shelf/Title.svelte';
    import { getI18n } from '~/stores/i18n';
    import { getJetPerform } from '~/jet';
    import { VERSION_HISTORY_MODAL_ID } from '~/utils/metrics';

    const perform = getJetPerform();
    export let shelf: TitledParagraphShelf;

    let modalComponent: SvelteComponent;
    let modalTriggerElement: HTMLElement | null = null;

    const { seeAllAction } = shelf;
    const i18n = getI18n();
    const translateFn = (key: string) => $i18n.t(key);
    const handleModalClose = () => modalComponent.close();
    const handleOpenModalClick = (e: Event) => {
        modalTriggerElement = e.target as HTMLElement;
        modalComponent?.showModal();
        perform(destination);
    };

    const destination =
        seeAllAction && isVersionHistoryFlowAction(seeAllAction)
            ? seeAllAction
            : undefined;

    const pageData = destination?.pageData;
</script>

<ShelfWrapper {shelf}>
    <div slot="title" class="title-container">
        {#if shelf.title}
            <button on:click={handleOpenModalClick}>
                <ShelfTitle title={shelf.title} seeAllAction={destination} />
            </button>
        {/if}

        {#if pageData}
            <Modal {modalTriggerElement} bind:this={modalComponent}>
                <ContentModal
                    on:close={handleModalClose}
                    title={pageData.title || null}
                    subtitle={null}
                    targetId={VERSION_HISTORY_MODAL_ID}
                >
                    <svelte:fragment slot="content">
                        <ul>
                            {#each pageData.shelves as shelf}
                                {#each shelf.items || [] as item}
                                    {#if isTitledParagraphItem(item)}
                                        <li>
                                            <TitledParagraphItem {item} />
                                        </li>
                                    {/if}
                                {/each}
                            {/each}
                        </ul>
                    </svelte:fragment>
                </ContentModal>
            </Modal>
        {/if}
    </div>

    {#each shelf.items as item}
        <TitledParagraphItem {item} />
    {/each}
</ShelfWrapper>

<style>
    .title-container {
        display: flex;
        justify-content: space-between;
        padding-top: 16px;
        padding-inline-end: var(--bodyGutter);
    }
</style>
