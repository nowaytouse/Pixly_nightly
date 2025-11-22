<script lang="ts" context="module">
    import type {
        EditorsChoice,
        ProductReview,
    } from '@jet-app/app-store/api/models';

    interface EditorsChoiceReview extends ProductReview {
        sourceType: 'editorsChoice';
        review: EditorsChoice;
    }

    export function isEditorsChoiceReviewItem(
        productReview: ProductReview,
    ): productReview is EditorsChoiceReview {
        return productReview.sourceType === 'editorsChoice';
    }
</script>

<script lang="ts">
    import { getI18n } from '~/stores/i18n';
    import Modal from '@amp/web-app-components/src/components/Modal/Modal.svelte';
    import ContentModal from '~/components/jet/item/ContentModal.svelte';
    import Truncate from '@amp/web-app-components/src/components/Truncate/Truncate.svelte';
    import EditorsChoiceBadge from '~/components/EditorsChoiceBadge.svelte';
    import { getJet } from '~/jet';
    import { CUSTOMER_REVIEW_MODAL_ID } from '~/utils/metrics';

    export let item: EditorsChoiceReview;
    export let isDetailView: boolean = false;

    let modalComponent: Modal | undefined;
    let modalTriggerElement: HTMLElement | null = null;

    const translateFn = (key: string) => $i18n.t(key);
    const i18n = getI18n();
    const jet = getJet();

    const handleCloseModal = () => modalComponent?.close();
    const handleOpenModal = () => {
        modalComponent?.showModal();
        jet.recordCustomMetricsEvent({
            eventType: 'dialog',
            dialogId: 'more',
            targetId: CUSTOMER_REVIEW_MODAL_ID,
            dialogType: 'button',
        });
    };
</script>

<article class:is-detail-view={isDetailView}>
    <EditorsChoiceBadge
        --font={isDetailView
            ? 'var(--large-title-emphasized)'
            : 'var(--title-1-emphasized)'}
    />

    {#if isDetailView}
        <p>{item.review.notes}</p>
    {:else}
        <Truncate
            {translateFn}
            lines={4}
            text={item.review.notes}
            title={$i18n.t('ASE.Web.AppStore.Review.EditorsChoice')}
            isPortalModal={true}
            on:openModal={handleOpenModal}
        />
    {/if}
</article>

{#if !isDetailView}
    <Modal {modalTriggerElement} bind:this={modalComponent}>
        <ContentModal
            on:close={handleCloseModal}
            title={null}
            subtitle={null}
            targetId={CUSTOMER_REVIEW_MODAL_ID}
        >
            <svelte:fragment slot="content">
                <svelte:self {item} isDetailView={true} />
            </svelte:fragment>
        </ContentModal>
    </Modal>
{/if}

<style>
    article:not(.is-detail-view) {
        height: 186px;
        padding: 20px;
        background-color: var(--systemQuinary);
        border-radius: var(--global-border-radius-xlarge);
    }

    article :global(.more) {
        --moreTextColorOverride: var(--keyColor);
        --moreFontOverride: var(--body);
        text-transform: lowercase;
    }
</style>
