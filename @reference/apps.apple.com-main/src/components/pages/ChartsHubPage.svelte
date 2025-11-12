<script lang="ts">
    import type { ChartsHubPage } from '@jet-app/app-store/api/models';

    import TopChartsPage from './TopChartsPage.svelte';

    export let page: ChartsHubPage;
</script>

{#each page.charts as chart}
    <TopChartsPage page={chart} />
{/each}
