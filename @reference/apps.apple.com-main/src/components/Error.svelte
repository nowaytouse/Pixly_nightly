<script lang="ts">
    import ErrorPage from '@amp/web-app-components/src/components/Error/ErrorPage.svelte';
    import { getI18n } from '~/stores/i18n';

    export let error: Error;

    const i18n = getI18n();
</script>

<ErrorPage translateFn={$i18n.t} {error} />
