<script lang="ts">
    import type { SearchLandingPage } from '@jet-app/app-store/api/models';
    import type { WebRenderablePage } from '@jet-app/app-store/api/models/web-renderable-page';
    import type { WebSearchFlowAction } from '@jet-app/app-store/common/search/web-search-action';
    import { unwrapOptional as unwrap } from '@jet/environment/types/optional';

    type SearchPage = SearchLandingPage;

    import DefaultPage from './DefaultPage.svelte';
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import SearchInput from '~/components/navigation/SearchInput.svelte';
    import { getI18n } from '~/stores/i18n';

    export let page: SearchPage;

    const i18n = getI18n();

    $: webNavigation = unwrap((page as WebRenderablePage).webNavigation);
    $: searchAction = webNavigation.searchAction as WebSearchFlowAction;
    $: hasShelves = !!page.shelves.filter(({ items }) => items?.length).length;

    $: pageWithoutEmptyShelves = {
        ...page,
        shelves: hasShelves ? page.shelves : [],
        title: $i18n.t('ASE.Web.AppStore.Meta.SearchLanding.Title'),
    };
</script>

<DefaultPage page={pageWithoutEmptyShelves}>
    <ShelfWrapper slot="before-shelves" centered>
        <SearchInput {searchAction} big />
    </ShelfWrapper>
</DefaultPage>
