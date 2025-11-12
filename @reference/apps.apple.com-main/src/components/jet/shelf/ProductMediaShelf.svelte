<script lang="ts" context="module">
    import type {
        Shelf,
        ProductMedia,
        AppPlatform,
        MediaType,
        MediaPlatform,
    } from '@jet-app/app-store/api/models';

    interface ProductMediaShelf extends Shelf, ProductMedia {
        items: ProductMedia['items'];
        expandedMedia?: ProductMediaShelf[];
    }

    export function isProductMediaShelf(
        shelf: Shelf,
    ): shelf is ProductMediaShelf {
        const { contentType, items } = shelf;
        return contentType === 'productMediaItem' && Array.isArray(items);
    }

    const platformToIconNameMap: Record<AppPlatform, string> = {
        phone: 'iphone.gen2',
        pad: 'ipad.gen2',
        tv: 'tv',
        watch: 'applewatch',
        mac: 'macbook.gen2',
        messages: 'message',
        vision: 'visionpro',
    };

    const platformToDescriptionMap: Record<AppPlatform, string> = {
        phone: 'AppStore.AppPlatform.Phone',
        pad: 'AppStore.AppPlatform.Pad',
        tv: 'AppStore.AppPlatform.TV',
        watch: 'AppStore.AppPlatform.Watch',
        mac: 'AppStore.AppPlatform.Mac',
        messages: 'AppStore.AppPlatform.Messages',
        vision: 'AppStore.AppPlatform.Vision',
    };
</script>

<script lang="ts">
    import ShelfWrapper from '~/components/Shelf/Wrapper.svelte';
    import ShelfItemLayout from '~/components/ShelfItemLayout.svelte';
    import ProductMediaVisionItem from '~/components/jet/item/ProductMedia/ProductMediaVisionItem.svelte';
    import ProductMediaPhoneItem from '~/components/jet/item/ProductMedia/ProductMediaPhoneItem.svelte';
    import ProductMediaMacItem from '~/components/jet/item/ProductMedia/ProductMediaMacItem.svelte';
    import ProductMediaPadItem from '~/components/jet/item/ProductMedia/ProductMediaPadItem.svelte';
    import ProductMediaWatchItem from '~/components/jet/item/ProductMedia/ProductMediaWatchItem.svelte';
    import ProductMediaTVItem from '~/components/jet/item/ProductMedia/ProductMediaTVItem.svelte';
    import ChevronDown from '~/sf-symbols/chevron.down.svg';
    import SFSymbol from '~/components/SFSymbol.svelte';
    import { getI18n } from '~/stores/i18n';
    import { slide } from 'svelte/transition';
    import { getJet } from '~/jet';

    export let shelf: ProductMediaShelf;
    export let isExpandedMedia: boolean = false;

    const i18n = getI18n();
    const jet = getJet();

    let appPlatform: AppPlatform | undefined;
    let allPlatforms: MediaPlatform[] | undefined;
    let mediaType: MediaType | undefined;
    let hasPortraitMedia: boolean = false;
    let shouldDisplayExpandedMedia: boolean = false;

    $: {
        if (shelf.contentsMetadata.type === 'productMedia') {
            ({ hasPortraitMedia, allPlatforms } = shelf.contentsMetadata);
            ({ appPlatform, mediaType } = shelf.contentsMetadata.platform);
        }
    }

    $: allPlatformsDescription = allPlatforms
        ?.map(({ appPlatform }) =>
            $i18n.t(platformToDescriptionMap[appPlatform]),
        )
        ?.join($i18n.t('AppStore.AppPlatform.Component.Separator'));

    $: shouldShowPlatform =
        isExpandedMedia ||
        shouldDisplayExpandedMedia ||
        allPlatforms?.length === 1;

    const displayExpandedMedia = () => {
        shouldDisplayExpandedMedia = true;
        jet.recordCustomMetricsEvent({
            eventType: 'click',
            actionDetails: { type: 'platformSelect' },
            targetType: 'button',
            targetId: 'productMediaShelf',
        });
    };
</script>

<ShelfWrapper {shelf} withBottomPadding={!shelf.expandedMedia}>
    {#if appPlatform === 'vision'}
        <ShelfItemLayout {shelf} gridType="ScreenshotVision" let:item>
            <ProductMediaVisionItem {item} />
        </ShelfItemLayout>
    {:else if appPlatform === 'phone' || appPlatform === 'messages'}
        <ShelfItemLayout
            {shelf}
            gridType={hasPortraitMedia ? 'ScreenshotPhone' : 'ScreenshotLarge'}
            let:item
        >
            <ProductMediaPhoneItem {item} {hasPortraitMedia} {mediaType} />
        </ShelfItemLayout>
    {:else if appPlatform === 'pad'}
        <ShelfItemLayout
            {shelf}
            gridType={hasPortraitMedia ? 'ScreenshotPad' : 'ScreenshotLarge'}
            let:item
        >
            <ProductMediaPadItem {item} {hasPortraitMedia} {mediaType} />
        </ShelfItemLayout>
    {:else if appPlatform === 'mac'}
        <ShelfItemLayout {shelf} gridType="ScreenshotLarge" let:item>
            <ProductMediaMacItem {item} />
        </ShelfItemLayout>
    {:else if appPlatform === 'tv'}
        <ShelfItemLayout {shelf} gridType="ScreenshotLarge" let:item>
            <ProductMediaTVItem {item} />
        </ShelfItemLayout>
    {:else if appPlatform === 'watch'}
        <ShelfItemLayout {shelf} gridType="ScreenshotPhone" let:item>
            <ProductMediaWatchItem {item} {mediaType} />
        </ShelfItemLayout>
    {:else}
        <ShelfItemLayout {shelf} gridType="A" let:item>
            <ProductMediaPhoneItem {item} {hasPortraitMedia} {mediaType} />
        </ShelfItemLayout>
    {/if}

    {#if appPlatform && shouldShowPlatform}
        <div class="platform-description">
            <div class="icon" aria-hidden="true">
                <SFSymbol name={platformToIconNameMap[appPlatform]} />
            </div>
            <div class="platform-label">
                {$i18n.t(platformToDescriptionMap[appPlatform])}
            </div>
        </div>
    {/if}
</ShelfWrapper>

{#if shelf.expandedMedia && allPlatforms && allPlatforms.length > 1}
    <div class="expanded-media">
        {#if !shouldDisplayExpandedMedia}
            <button
                class="expanded-media-header"
                on:click={displayExpandedMedia}
            >
                <div class="all-platforms">
                    <div class="all-platforms-icons">
                        {#each allPlatforms as platform}
                            <div class="icon" aria-hidden="true">
                                <SFSymbol
                                    name={platformToIconNameMap[
                                        platform.appPlatform
                                    ]}
                                />
                            </div>
                        {/each}
                    </div>
                    <div class="all-platforms-names">
                        {allPlatformsDescription}
                    </div>
                </div>
                <div class="chevron-container icon" aria-hidden="true">
                    <ChevronDown />
                </div>
            </button>
        {/if}
        {#if shouldDisplayExpandedMedia}
            <div class="expanded-media-content" transition:slide>
                {#each shelf.expandedMedia as expandedMediaShelf}
                    <svelte:self
                        shelf={expandedMediaShelf}
                        isExpandedMedia={true}
                    />
                {/each}
            </div>
        {/if}
    </div>
{/if}

{#if !isExpandedMedia}
    <div class="divider" />
{/if}

<style>
    .expanded-media {
        margin: 15px 0;
    }

    .expanded-media-header {
        width: 100%;
        padding-inline: var(--bodyGutter);
        display: inline-flex;
        align-items: center;
        justify-content: space-between;
    }

    .platform-description {
        display: inline-flex;
        align-items: center;
        font: var(--body-reduced-semibold);
        color: var(--systemSecondary);
        margin-top: 15px;
        gap: 10px;
        margin-inline: var(--bodyGutter);
    }

    .all-platforms {
        display: inline-flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 10px;
        color: var(--systemSecondary);
    }

    .all-platforms-icons {
        display: inline-flex;
        gap: 10px;
    }

    .all-platforms-names {
        font: var(--body-reduced-semibold);
    }

    .icon :global(svg) {
        overflow: visible;
        height: 16px;
        max-width: 25px;
        fill: var(--systemSecondary);
        position: relative;
        display: flex;
    }

    .divider {
        margin: 10px var(--bodyGutter);
        border-bottom: 1px solid var(--systemGray4);
    }

    .chevron-container {
        top: 2px;
    }

    .expanded-media-content :global(.shelf:last-of-type) {
        padding-bottom: 0;
    }

    .expanded-media-header .all-platforms,
    .expanded-media-header .chevron-container :global(svg) {
        transition-duration: 210ms;
        transition-timing-function: ease-out;
        transition-property: color, fill;
    }

    .expanded-media-header:hover .all-platforms,
    .expanded-media-header:hover .chevron-container :global(svg) {
        color: var(--systemPrimary);
        fill: var(--systemPrimary);
    }
</style>
