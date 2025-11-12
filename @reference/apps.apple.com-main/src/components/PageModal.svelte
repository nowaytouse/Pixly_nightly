<script lang="ts">
    import { onMount, type SvelteComponent } from 'svelte';
    import type { GenericPage } from '@jet-app/app-store/api/models';

    import Modal from '@amp/web-app-components/src/components/Modal/Modal.svelte';
    import { getModalPageStore } from '~/stores/modalPage';
    import ShelfComponent from '~/components/jet/shelf/Shelf.svelte';
    import ContentModal from '~/components/jet/item/ContentModal.svelte';
    import { LICENSE_AGREEMENT_MODAL_ID } from '~/utils/metrics';

    let modalElement: SvelteComponent;
    let modalPage = getModalPageStore();
    let page: GenericPage | undefined;

    $: page = $modalPage?.page;
    $: shelves = page?.shelves ?? [];
    $: title = page?.title ?? null;
    $: targetId =
        $modalPage?.pageDetail === 'licenseAgreement'
            ? LICENSE_AGREEMENT_MODAL_ID
            : undefined;

    onMount(() => {
        return modalPage.clearPage;
    });

    $: {
        if ($modalPage) {
            modalElement?.showModal();
        } else {
            handleModalClose();
        }
    }

    function handleModalClose() {
        modalElement?.close();
        modalPage.clearPage();
    }
</script>

<Modal
    modalTriggerElement={null}
    bind:this={modalElement}
    on:close={handleModalClose}
>
    <div class="modal-content">
        {#if page}
            <ContentModal
                {title}
                subtitle={null}
                on:close={handleModalClose}
                {targetId}
            >
                <svelte:fragment slot="content">
                    {#each shelves as shelf}
                        <ShelfComponent {shelf}>
                            <slot
                                name="marker-shelf"
                                slot="marker-shelf"
                                let:shelf
                                {shelf}
                            />
                        </ShelfComponent>
                    {/each}
                </svelte:fragment>
            </ContentModal>
        {/if}
    </div>
</Modal>

<style lang="scss">
    .modal-content :global(p) {
        user-select: text;
        margin-bottom: 15px;
        overflow-wrap: break-word;
    }

    :global(.noscroll) {
        overflow: hidden;
        touch-action: none;
    }
</style>
