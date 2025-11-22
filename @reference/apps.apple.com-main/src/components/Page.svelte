<script lang="ts">
    import {
        type Page,
        hasVisionProUrl,
        isAppEventDetailPage,
        isArticlePage,
        isChartsHubPage,
        isGenericPage,
        isSearchLandingPage,
        isShelfBasedProductPage,
        isTopChartsPage,
        isTodayPage,
        isSearchResultsPage,
        isStaticMessagePage,
        isSeeAllPage,
        isErrorPage,
    } from '~/jet/models';

    import AppEventDetailPage from './pages/AppEventDetailPage.svelte';
    import ArticlePage from './pages/ArticlePage.svelte';
    import ChartsHubPage from './pages/ChartsHubPage.svelte';
    import DefaultPage from './pages/DefaultPage.svelte';
    import ErrorPage from './pages/ErrorPage.svelte';
    import ProductPage from './pages/ProductPage.svelte';
    import VisionProPage from './pages/VisionProPage.svelte';
    import StaticMessagePageComponent from './pages/StaticMessagePage.svelte';
    import SearchLandingPage from './pages/SearchLandingPage.svelte';
    import SearchResultsPage from './pages/SearchResultsPage.svelte';
    import TopChartsPage from './pages/TopChartsPage.svelte';
    import TodayPage from './pages/TodayPage.svelte';
    import SeeAllPage from './pages/SeeAllPage.svelte';
    import MetaTags from '~/components/structure/MetaTags.svelte';
    import PageModal from '~/components/PageModal.svelte';

    export let page: Page;
</script>

<MetaTags {page} />

<PageModal />

{#if isAppEventDetailPage(page)}
    <AppEventDetailPage {page} />
{:else if isArticlePage(page)}
    <ArticlePage {page} />
{:else if isChartsHubPage(page)}
    <ChartsHubPage {page} />
{:else if isSearchLandingPage(page)}
    <SearchLandingPage {page} />
{:else if isSearchResultsPage(page)}
    <SearchResultsPage {page} />
{:else if isShelfBasedProductPage(page)}
    <ProductPage {page} />
{:else if isTopChartsPage(page)}
    <TopChartsPage {page} />
{:else if isGenericPage(page) && hasVisionProUrl(page)}
    <VisionProPage {page} />
{:else if isTodayPage(page)}
    <TodayPage {page} />
{:else if isStaticMessagePage(page)}
    <StaticMessagePageComponent {page} />
{:else if isSeeAllPage(page)}
    <SeeAllPage {page} />
{:else if isErrorPage(page)}
    <ErrorPage {page} />
{:else}
    <DefaultPage {page} />
{/if}
