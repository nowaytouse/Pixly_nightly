<script lang="ts" context="module">
    import type { ErrorPage } from '~/jet/models';
</script>

<script lang="ts">
    import SharedErrorPage from '@amp/web-app-components/src/components/Error/ErrorPage.svelte';
    import { getI18n } from '~/stores/i18n';

    export let page: ErrorPage;

    const i18n = getI18n();
</script>

<div class="error-page-container">
    <SharedErrorPage translateFn={$i18n.t} error={page.error} />
</div>

<style>
    .error-page-container :global(.page-error) {
        /* -50px compensates for the global footer */
        top: calc(50% - 50px);
    }
</style>
