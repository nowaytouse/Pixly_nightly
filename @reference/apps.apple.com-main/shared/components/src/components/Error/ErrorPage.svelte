<script lang="ts">
    import Button from '@amp/web-app-components/src/components/buttons/Button.svelte';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    interface ErrorUserInfo {
        status: number;
    }

    interface AppError {
        message?: string;
        isFirstPage?: boolean;
        userInfo?: ErrorUserInfo;
        statusCode?: number;
    }

    export let translateFn: (
        str: string,
        values?: Record<string, string | number>,
    ) => string;

    export let isRetryError: (error: AppError) => boolean = () => false;

    export let error: AppError | null = null;
    export let errorLocKey: string | null = null;

    // podcasts-client-js can currently return a 204 if there is no content found.
    // We want to treat this as a 204.  If the following radar is ever addressed,
    // we can remove the 204 conditional here:
    // rdar://106657358 (Investigate if we can switch from 204 to 404s for network errors)
    $: locKey =
        errorLocKey ||
        (error?.userInfo?.status === 404 ||
        error?.message === '404' ||
        error?.statusCode === 404 ||
        error?.statusCode === 204
            ? 'AMP.Shared.Error.ItemNotFound'
            : 'FUSE.Error.AnErrorOccurred');

    function retry(): void {
        dispatch('retryAction');
    }
</script>

<!-- TODO: rdar://92841405 (JMOTW: Show error page when user has lost internet connection) -->
<div role="status" class="page-error">
    <h1 class="page-error__title" data-testid="page-error-title">
        {translateFn(locKey)}
    </h1>

    {#if isRetryError(error)}
        <Button buttonStyle="buttonB" on:buttonClick={retry}>
            {translateFn('FUSE.Error.TryAgain')}
        </Button>
    {/if}
</div>

<style lang="scss">
    .page-error {
        --buttonTextColor: var(--systemSecondary);
        --buttonBorderColor: var(--systemSecondary);
        margin: auto;
        padding: 0 25px;
        max-width: 440px;
        color: var(--systemSecondary);
        position: absolute;
        top: 50%;
        left: 50%; // RTL not needed
        transform: translate(-50%, -50%);
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 12px;
        text-align: center;
        z-index: var(--z-default);
    }

    .page-error__title {
        margin-bottom: 5px;
        font: var(--title-2);
    }
</style>
