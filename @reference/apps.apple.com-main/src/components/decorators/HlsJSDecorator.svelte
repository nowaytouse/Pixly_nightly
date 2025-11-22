<script lang="ts" context="module">
    // This store is used to keep track of in-flight requests, ensuring that we don't attempt
    // to load the same src (which is stored in the Map key) multiple times.
    const inFlightRequests = new Map<string, Promise<void>>();
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import { generateHLSJSURL } from '~/config/hlsjs';
    import { generateRTCJSURL } from '~/config/rtcjs';

    export let version: string | undefined = undefined;

    let hlsjsSourceURL = generateHLSJSURL(version).toString();
    let rtcjsSourceURL = generateRTCJSURL(version).toString();

    function loadScript(src: string): Promise<void> {
        // If we have an in-flight request for this `src`, return it.
        const inFlightRequest = inFlightRequests.get(src);
        if (inFlightRequest) {
            return inFlightRequest;
        }

        const promise = new Promise<void>(function (resolve, reject) {
            const scriptElement = document.createElement('script');
            scriptElement.src = src;
            scriptElement.onload = () => resolve();
            scriptElement.onerror = () => {
                // If a script fails to load due to a network blip, we remove it from the store,
                // so that the next attempt in an `onMount` will try to load the `src` again.
                inFlightRequests.delete(src);
                reject();
            };

            document.head.appendChild(scriptElement);
        });

        // Add the given `src` to the store so we can keep track of in-flight requests.
        inFlightRequests.set(src, promise);

        return promise;
    }

    let loading: Promise<[void, void]> | undefined;

    onMount(() => {
        loading = Promise.all([
            window.Hls ?? loadScript(hlsjsSourceURL),
            window.rtc ?? loadScript(rtcjsSourceURL),
        ]);
    });
</script>

{#if loading}
    {#await loading}
        <slot name="loading-component" />
    {:then}
        <slot HLS={window.Hls} RTC={window.rtc} />
    {:catch}
        <div>
            <p>
                Failed to load HLS.js {version} from
                <a href={hlsjsSourceURL}>{hlsjsSourceURL}</a>
            </p>
        </div>
    {/await}
{/if}
