<!--
@component
Page component for the "Today Page"

This is required so that the correct layout of the cards within each `TodayCardShelf` 
can be computed at the page level, as the algorithm for stretching the correct cards
in each shelf requires knowledge of the previously-rendered shelf
-->
<script lang="ts">
    import type { TodayPage } from '@jet-app/app-store/api/models';

    import DefaultPage from '~/components/pages/DefaultPage.svelte';
    import { setTodayCardLayoutContext } from '~/context/today-card-layout';

    export let page: TodayPage;

    $: {
        setTodayCardLayoutContext(page);
    }
</script>

<DefaultPage {page} />
