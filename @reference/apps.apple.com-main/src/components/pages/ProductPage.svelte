<script lang="ts">
    import type { ShelfBasedProductPage } from '@jet-app/app-store/api/models';
    import { isFlowAction } from '@jet-app/app-store/api/models';
    import type { WebRenderablePage } from '@jet-app/app-store/api/models/web-renderable-page';

    import DefaultPage, {
        type DefaultPageRequirements,
    } from '~/components/pages/DefaultPage.svelte';
    import MarkerShelf from '~/components/jet/shelf/MarkerShelf.svelte';
    import ProductPageArcadeFooter from '~/components/ProductPageArcadeFooter.svelte';
    import { getProductPageShelvesWithExpandedMedia } from '~/utils/shelves';
    import { setAccessibilityLayoutContext } from '~/context/accessibility-layout';
    import { getJet } from '~/jet';
    import { isProductPageLinkShelf } from '~/components/jet/shelf/ProductPageLinkShelf.svelte';
    import { isEulaPageIntent } from '@jet-app/app-store/api/intents/eula-page-intent';
    export let page: ShelfBasedProductPage & WebRenderablePage;

    const jet = getJet();

    $: ({ presentationOptions, webNavigation } = page);

    $: shelves = getProductPageShelvesWithExpandedMedia(page);

    let defaultPageRequirements: DefaultPageRequirements;

    $: defaultPageRequirements = {
        shelves,
        presentationOptions,
        webNavigation,
    };

    // Set up accessibility layout context for neighbor shelf detection
    $: {
        setAccessibilityLayoutContext({ shelves });

        /**
         * We suppport "deep linking" to the product page with the License Agreement modal open by
         * default, based on the presence of the `lic` query parameter. No other modals support
         * opening via deep link, which is why there isn't a more robust solution for this use case.
         * Instead, we are just firing off the click action from the license agreement shelf.
         */
        if (page.canonicalURL) {
            const canonicalUrl = new URL(page.canonicalURL);
            const hasLic = canonicalUrl.searchParams.has('lic');

            if (hasLic && shelves) {
                const eulaItem = shelves
                    .find(isProductPageLinkShelf)
                    ?.items.find(
                        ({ clickAction }) =>
                            isFlowAction(clickAction) &&
                            clickAction.destination &&
                            isEulaPageIntent(clickAction.destination),
                    );

                if (eulaItem) {
                    jet.perform(eulaItem.clickAction);
                }
            }
        }
    }

    // TODO: replace with `supportsArcade` from Jet
    // rdar://143706610 (Support `supportsArcade` attribute)
    $: supportsArcade =
        page.lockup.offerDisplayProperties?.offerType === 'arcadeApp';
</script>

<DefaultPage page={defaultPageRequirements}>
    <svelte:fragment slot="marker-shelf" let:shelf>
        <MarkerShelf {shelf} {page} />
    </svelte:fragment>
</DefaultPage>

{#if supportsArcade}
    <ProductPageArcadeFooter />
{/if}
