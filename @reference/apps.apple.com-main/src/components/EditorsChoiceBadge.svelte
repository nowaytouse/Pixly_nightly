<script lang="ts">
    import LaurelIcon from '~/sf-symbols/laurel.left.svg';
    import { getI18n } from '~/stores/i18n';

    const i18n = getI18n();
</script>

<h4>
    <span class="icon-container left" aria-hidden="true">
        <LaurelIcon />
    </span>
    {$i18n.t('ASE.Web.AppStore.Review.EditorsChoice')}
    <span class="icon-container right" aria-hidden="true">
        <LaurelIcon />
    </span>
</h4>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;

    h4 {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        margin-bottom: 10px;
        gap: 10px;
        font: var(--font, var(--title-1-emphasized));
        color: var(--systemSecondary);
    }

    .icon-container.right {
        transform: rotateY(180deg);

        @include rtl {
            transform: rotateY(0);
        }
    }

    .icon-container.left {
        @include rtl {
            transform: rotateY(180deg);
        }
    }

    .icon-container :global(svg) {
        overflow: visible;
        height: 42px;
        transform: translateY(3px);
    }

    .icon-container :global(svg path) {
        fill: var(--systemSecondary);
    }
</style>
