<script lang="ts">
    import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';
    import type { Optional } from '@jet/environment/types/optional';
    import type { AppEvent } from '@jet-app/app-store/api/models';
    import { getJet } from '~/jet';
    import {
        chooseAppEventDate,
        renderDate,
        computeAppEventFormattedDates,
        type RequiredAppEventFormattedDate,
    } from '~/jet/utils/app-event-formatted-date';

    const jet = getJet();

    /**
     * New pattern (*prefered*): accept appEvent object and compute formattedDates on client-side.
     * This avoids timezone differences in SSR server (UTC) which cause incorrect event date and time.
     * By computing dates in the browser, we ensure the user sees dates in their local timezone.
     */
    export let appEvent:
        | Pick<AppEvent, 'appEventBadgeKind' | 'startDate' | 'endDate'>
        | undefined = undefined;

    // Legacy pattern: accept pre-computed formattedDates from Jet
    export let formattedDates: RequiredAppEventFormattedDate[] | undefined =
        undefined;

    let appEventDate: Optional<RequiredAppEventFormattedDate>;

    onMount(() => {
        const dates = appEvent
            ? computeAppEventFormattedDates(
                  jet.objectGraph,
                  appEvent.appEventBadgeKind,
                  appEvent.startDate,
                  appEvent.endDate,
              )
            : formattedDates;

        if (dates) {
            appEventDate = chooseAppEventDate(dates);
        }
    });

    /**
     * `Date` instances in the view-model will have been serialized to `string`
     * instances by ServerKit when delivered to the client; we need to normalize
     * this so that we have a `string` both client- and server-side.
     */
    function normalizeDate(date: Date | string): string {
        return typeof date === 'string' ? date : date.toISOString();
    }
</script>

{#if appEventDate}
    <time
        transition:fade={{ duration: 210 }}
        datetime={appEventDate.displayFromDate &&
            normalizeDate(appEventDate.displayFromDate)}
    >
        {renderDate(jet.objectGraph.loc, appEventDate)}
    </time>
{:else}
    <span aria-hidden="true">&hellip;</span>
{/if}

<style>
    span {
        color: transparent;
    }
</style>
