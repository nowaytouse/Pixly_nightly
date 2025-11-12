<script lang="ts" context="module">
    import type { StaticMessagePage } from '~/jet/models';
</script>

<script lang="ts">
    import { getI18n } from '~/stores/i18n';

    export let page: StaticMessagePage;

    const i18n = getI18n();
</script>

<div class="static-message-page-container">
    <div class="static-message-text-wrapper">
        {#if page.titleLocKey}
            <h1>{$i18n.t(page.titleLocKey)}</h1>
        {/if}

        <section>
            {#if page.contentType === 'win-back' || page.contentType === 'contingent-price'}
                <p>
                    {$i18n.t('ASE.Web.AppStore.WinBack.Subhead')}
                </p>

                <p>
                    <b>
                        {$i18n.t('ASE.Web.AppStore.WinBack.DirectionalTitle')}
                    </b>
                </p>

                <ul>
                    <li>
                        {$i18n.t('ASE.Web.AppStore.WinBack.Update.iOS')}
                    </li>
                    <li>
                        {$i18n.t('ASE.Web.AppStore.WinBack.Update.macOS')}
                    </li>
                </ul>

                <p>
                    {$i18n.t('ASE.Web.AppStore.WinBack.Body')}
                </p>
            {:else if page.contentType === 'carrier'}
                <p class="carrier__instructions">
                    {$i18n.t('ASE.Web.AppStore.Carrier.Update.iOS')}
                </p>
                <p>
                    {$i18n.t('ASE.Web.AppStore.Carrier.Body')}
                </p>
            {:else if page.contentType === 'invoice'}
                <p class="invoice__instructions">
                    {$i18n.t('ASE.Web.AppStore.Invoice.Body')}
                </p>
            {/if}
        </section>
    </div>
</div>

<style lang="scss">
    @use 'ac-sasskit/modules/viewportcontent/core' as *;
    @use 'amp/stylekit/core/viewports' as *;

    .static-message-page-container {
        display: flex;
        flex-grow: 1;
        width: 100%;
        max-width: viewport-content-for(xlarge);
        margin: 0 auto;
        align-items: center;
    }

    @media (--range-sidebar-visible-up) {
        .static-message-page-container {
            height: 100%;
        }
    }

    .static-message-text-wrapper {
        display: inline-flex;
        flex-direction: column;
        align-items: flex-start;
        width: auto;
        margin: 0 auto;
    }

    .static-message-page-container h1 {
        padding: 13px var(--bodyGutter) 0;
        font: var(--header-emphasized);
        color: var(--systemPrimary, #000);
        position: relative;
        z-index: 1;
        margin-bottom: 16px;
    }

    .static-message-page-container section {
        margin: 0 var(--bodyGutter);
        font: var(--title-3);
    }

    .static-message-page-container li {
        list-style-type: disc;
    }

    .static-message-page-container p,
    .static-message-page-container ul {
        margin-bottom: 16px;
        text-wrap: pretty;
    }

    .static-message-page-container ul {
        padding-inline-start: 1em;
    }
</style>
