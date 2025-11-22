<script lang="ts">
    import type { Review as ReviewModel } from '@jet-app/app-store/api/models';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import Modal from '@amp/web-app-components/src/components/Modal/Modal.svelte';
    import ContentModal from '~/components/jet/item/ContentModal.svelte';
    import Truncate from '@amp/web-app-components/src/components/Truncate/Truncate.svelte';
    import StarRating from '~/components/StarRating.svelte';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';
    import { getI18n } from '~/stores/i18n';
    import { getJet } from '~/jet/svelte';
    import {
        escapeHtml,
        stripUnicodeWhitespace,
    } from '~/utils/string-formatting';
    import { CUSTOMER_REVIEW_MODAL_ID } from '~/utils/metrics';

    export let item: ReviewModel;
    export let isDetailView: boolean = false;

    let modalComponent: Modal | undefined;
    let modalTriggerElement: HTMLElement | null = null;

    const jet = getJet();
    const i18n = getI18n();
    const translateFn = (key: string) => $i18n.t(key);

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

    $: ({ id, reviewerName, rating, contents, title, date, response } = item);
    $: dateForDisplay = jet.localization.timeAgo(new Date(date));
    $: dateForAttribute = new Date(date).toISOString();
    $: titleId = `review-${id}-title`;
    $: maximumLinesForReview = response ? 3 : 5;
    $: responseDateForDisplay =
        response && jet.localization.timeAgo(new Date(response.date));
    $: responseDateForAttribute =
        response && new Date(response.date).toISOString();
    $: reviewContents = stripUnicodeWhitespace(escapeHtml(contents));
    $: responseContents =
        response && stripUnicodeWhitespace(escapeHtml(response.contents));
</script>

<article class:is-detail-view={isDetailView} aria-labelledby={titleId}>
    <div class="header">
        <div class="title-and-rating-container">
            {#if !isDetailView}
                <h3 id={titleId} class="title">
                    <LineClamp clamp={1}>
                        {title}
                    </LineClamp>
                </h3>
            {/if}

            <StarRating
                {rating}
                --fill-color="var(--systemOrange)"
                --star-size={isDetailView ? '24px' : '12px'}
            />
        </div>

        <div class="review-header">
            <time class="date" datetime={dateForAttribute}>
                {dateForDisplay}
            </time>

            <LineClamp clamp={1}>
                <p class="author">
                    {reviewerName}
                </p>
            </LineClamp>
        </div>
    </div>

    {#if isDetailView}
        <p>
            {@html sanitizeHtml(reviewContents, {
                allowedTags: [''],
                keepChildrenWhenRemovingParent: true,
            })}

            {#if response}
                <div class="developer-response-container">
                    <div class="developer-response-header">
                        <span class="developer-response-heading">
                            {$i18n.t(
                                'ASE.Web.AppStore.Review.DeveloperResponse',
                            )}
                        </span>

                        <time class="date" datetime={responseDateForAttribute}>
                            {responseDateForDisplay}
                        </time>
                    </div>

                    {@html sanitizeHtml(responseContents, {
                        allowedTags: [''],
                        keepChildrenWhenRemovingParent: true,
                    })}
                </div>
            {/if}
        </p>
    {:else}
        <div class="content">
            <Truncate
                on:openModal={handleOpenModal}
                {title}
                lines={maximumLinesForReview}
                {translateFn}
                text={reviewContents}
                isPortalModal={true}
            />

            {#if item.response}
                <div class="developer-response-container">
                    <span class="developer-response-heading">
                        {$i18n.t('ASE.Web.AppStore.Review.DeveloperResponse')}
                    </span>
                    <Truncate
                        on:openModal={handleOpenModal}
                        {title}
                        {translateFn}
                        lines={1}
                        text={responseContents}
                        isPortalModal={true}
                    />
                </div>
            {/if}
        </div>
    {/if}
</article>

{#if !isDetailView}
    <Modal {modalTriggerElement} bind:this={modalComponent}>
        <ContentModal
            on:close={handleCloseModal}
            {title}
            subtitle={null}
            targetId={CUSTOMER_REVIEW_MODAL_ID}
        >
            <svelte:fragment slot="content">
                <svelte:self {item} isDetailView={true} />
            </svelte:fragment>
        </ContentModal>
    </Modal>
{/if}

<style lang="scss">
    article:not(.is-detail-view) {
        height: 186px;
        padding: 20px 16px;
        background-color: var(--systemQuinary);
        border-radius: var(--global-border-radius-xlarge);

        @media (--small) {
            padding: 20px;
        }
    }

    .header {
        display: flex;
        gap: 8px;
        margin-bottom: 18px;
        align-items: center;
        justify-content: space-between;

        .is-detail-view & {
            margin-bottom: 0;
        }
    }

    .title-and-rating-container {
        .is-detail-view & {
            display: flex;
        }
    }

    .title {
        color: var(--systemPrimary);
        font: var(--body-emphasized);
        margin-bottom: 4px;
    }

    .date,
    .author {
        color: var(--systemSecondary);
        font: var(--callout);
        word-break: normal;
    }

    .content {
        position: relative;
        word-wrap: break-word; /* Break to fit the review block, even when people leave a review with long text without spaces */
        text-align: start;
        font: var(--body);
    }

    .review-header {
        text-align: end;
    }

    .developer-response-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 5px;
        margin-top: 20px;
    }

    .developer-response-heading {
        font: var(--body-emphasized);

        .is-detail-view & {
            display: block;
            font: var(--title-3-emphasized);
        }
    }

    .developer-response-container {
        margin-top: 10px;
    }

    article :global(.more) {
        --moreTextColorOverride: var(--keyColor);
        --moreFontOverride: var(--body);
        text-transform: lowercase;
    }
</style>
