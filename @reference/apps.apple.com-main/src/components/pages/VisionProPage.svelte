<script lang="ts">
    import type { GenericPage } from '@jet-app/app-store/api/models';

    import DefaultPage from './DefaultPage.svelte';
    import VisionProFooter from '~/components/structure/VisionProFooter.svelte';

    export let page: GenericPage;
</script>

<DefaultPage {page} />

<VisionProFooter />
