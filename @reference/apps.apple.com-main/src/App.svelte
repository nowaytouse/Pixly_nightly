<script lang="ts">
    import { onMount } from 'svelte';

    import { BUILD } from '~/config/build';
    import { getJet } from '~/jet';
    import { makeErrorPageIntent } from '~/jet/intents/error-page-intent-controller';
    import { getLocale } from '~/utils/locale';

    // Types
    import type { Page } from './jet/models/page';

    // Components
    import Fonts from '~/components/structure/Fonts.svelte';
    import Footer from '~/components/structure/Footer.svelte';
    import Navigation from '~/components/navigation/Navigation.svelte';
    import NavigationSkeleton from '~/components/navigation/Skeleton.svelte';
    import PageResolver from '~/components/PageResolver.svelte';

    const locale = getLocale();
    const jet = getJet();

    $: language = locale.language;

    export let page: Promise<Page> | Page = new Promise(() => {});
    export let isFirstPage: boolean = true;

    $: pageWithRejectionErrorPage = transformRejectionIntoErrorPage(page);

    // Critically, this function is not async. We want to preserve the behavior
    // where if page is not a promise than neither is
    // pageWithRejectionErrorPage.
    function transformRejectionIntoErrorPage(
        page: Promise<Page> | Page,
    ): Promise<Page> | Page {
        if (!(page instanceof Promise)) {
            return page;
        }

        // The async IIFE allows this function to return synchronously.
        return (async (): Promise<Page> => {
            try {
                return await page;
            } catch (error) {
                return jet.dispatch(
                    makeErrorPageIntent({
                        // This allows the error page to pick the right platform
                        // and display the correct mesage (ex. "Page not found" for
                        // a 404)
                        error: error instanceof Error ? error : null,
                    }),
                );
            }
        })();
    }

    // NOTE: The use of page instead of pageWithRejectionErrorPage here is very
    //       intentional. Since pageWithRejectionErrorPage is reactive, it will
    //       be undefined in this initializer. This is intentionally not
    //       not derived (eg. defined as $: webNavigation = ...), since we only
    //       want to update it _after_ the page promise resolves (so the nav
    //       doesn't disappear on navigation). But then for SSR, there are no
    //       promises, so we need a sync value here so the nav renders, which
    //       is why we have the initializer.
    let webNavigation = page instanceof Promise ? null : page.webNavigation;
    $: {
        if (pageWithRejectionErrorPage instanceof Promise) {
            // Clientside once the new page resolves, update the navigation
            // (in case it changed)
            pageWithRejectionErrorPage.then((page: Page) => {
                webNavigation = page.webNavigation;
            });
        } else {
            // Sometimes clientside a promise is not passed to updateApp, so
            // we need to handle a WebRenderablePage (possible with a
            // different webNavigation).
            webNavigation = pageWithRejectionErrorPage.webNavigation;
        }
    }

    onMount(() => {
        //@ts-ignore
        window.__ASOTW = {
            version: BUILD,
        };
    });
</script>

<svelte:head>
    <meta name="version" content={BUILD} />
</svelte:head>

<Fonts {language} />

{#if import.meta.env.DEV}
    {#await import('~/components/ArtworkBreakpointLogger.svelte') then { default: ArtworkBreakpointLogger }}
        <ArtworkBreakpointLogger />
    {/await}
{/if}

<div class="app-container" data-testid="app-container">
    <div class="navigation-container">
        {#if webNavigation}
            <Navigation {webNavigation} />
        {:else}
            <NavigationSkeleton />
        {/if}
    </div>

    <div
        style="display: flex;
    position: relative;
    flex-direction: column;
        min-height: 100vh;
    "
    >
        <main class="page-container">
            <PageResolver page={pageWithRejectionErrorPage} {isFirstPage} />
        </main>

        <Footer />
    </div>
</div>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use '@amp/web-shared-styles/app/core/viewports' as *;
    @use '@amp/web-shared-styles/app/core/globalvars' as *;

    .app-container {
        min-height: 100vh;
        min-height: 100dvh;
        display: grid;
        grid-template-areas:
            'structure-header'
            'structure-main-section';
        grid-template-columns: minmax(0, 1fr);
        grid-gap: 0;
        grid-template-rows: 44px auto;

        @media (--sidebar-visible) {
            grid-template-rows: auto;
            grid-template-columns: 260px minmax(0, 1fr);
        }

        @media (--sidebar-large-visible) {
            grid-template-columns: $global-sidebar-width-large minmax(0, 1fr);
        }
    }

    .navigation-container {
        @media (--range-small-up) {
            height: 100vh;
            position: sticky;
            top: 0;
        }
    }

    .page-container {
        flex-grow: 1;
    }
</style>
