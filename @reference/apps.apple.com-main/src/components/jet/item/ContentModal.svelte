<script lang="ts">
    import ContentModal from '@amp/web-app-components/src/components/Modal/ContentModal.svelte';
    import { getI18n } from '~/stores/i18n';
    import { createEventDispatcher } from 'svelte';
    import { getJet } from '~/jet';

    export let title: string | null;
    export let subtitle: string | null;
    export let text: string | null = null;
    export let dialogTitleId: string | null = null;
    export let targetId: string = 'close';

    const i18n = getI18n();
    const jet = getJet();
    const dispatch = createEventDispatcher();

    const translateFn = (key: string) => $i18n.t(key);

    const handleCloseModal = () => {
        dispatch('close');
        jet.recordCustomMetricsEvent({
            eventType: 'click',
            targetId,
            targetType: 'button',
            actionType: 'close',
        });
    };
</script>

<ContentModal
    on:close={handleCloseModal}
    {translateFn}
    {title}
    {subtitle}
    text={text || undefined}
    {dialogTitleId}
>
    <slot name="content" slot="content" />
</ContentModal>
