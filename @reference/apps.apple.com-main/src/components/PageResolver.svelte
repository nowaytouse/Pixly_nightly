<script lang="ts">
    import type { Page } from '~/jet/models';

    import PageComponent from '~/components/Page.svelte';
    import ErrorComponent from '~/components/Error.svelte';
    import LoadingSpinner from '@amp/web-app-components/src/components/LoadingSpinner/LoadingSpinner.svelte';

    export let page: Promise<Page> | Page;
    export let isFirstPage: boolean;
</script>

{#await page}
    <div data-testid="page-loading">
        <!--
            Delay showing the spinner on initial page load after app boot.
            After that, the FlowAction handler already waits 500ms before
            it changes DOM, so we only need to wait 1000ms.
        -->
        <LoadingSpinner delay={isFirstPage ? 1500 : 1000} />
    </div>
{:then page}
    <PageComponent {page} />
{:catch error}
    <ErrorComponent {error} />
{/await}
