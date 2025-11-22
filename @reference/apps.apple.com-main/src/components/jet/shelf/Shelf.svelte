<script lang="ts">
    import type { Shelf } from '@jet-app/app-store/api/models';

    // Components for specific types of `Shelf`
    import AccessibilityHeaderShelf, {
        isAccessibilityHeaderShelf,
    } from '~/components/jet/shelf/AccessibilityHeaderShelf.svelte';
    import AccessibilityFeaturesShelf, {
        isAccessibilityFeaturesShelf,
    } from '~/components/jet/shelf/AccessibilityFeaturesShelf.svelte';
    import AccessibilityDeveloperLinkShelf, {
        isAccessibilityDeveloperLinkShelf,
    } from './AccessibilityDeveloperLinkShelf.svelte';
    import ActionShelf, {
        isActionShelf,
    } from '~/components/jet/shelf/ActionShelf.svelte';
    import AnnotationShelf, {
        isAnnotationShelf,
    } from '~/components/jet/shelf/AnnotationShelf.svelte';
    import AppEventDetailShelf, {
        isAppEventDetailShelf,
    } from '~/components/jet/shelf/AppEventDetailShelf.svelte';
    import AppPromotionShelf, {
        isAppPromotionShelf,
    } from '~/components/jet/shelf/AppPromotionShelf.svelte';
    import AppShowcaseShelf, {
        isAppShowcaseShelf,
    } from '~/components/jet/shelf/AppShowcaseShelf.svelte';
    import AppTrailerLockupShelf, {
        isAppTrailerLockupShelf,
    } from '~/components/jet/shelf/AppTrailerLockupShelf.svelte';
    import ArcadeFooterShelf, {
        isArcadeFooterShelf,
    } from '~/components/jet/shelf/ArcadeFooterShelf.svelte';
    import { isBannerShelf } from '~/components/jet/shelf/BannerShelf.svelte';
    import BrickShelf, {
        isBrickShelf,
    } from '~/components/jet/shelf/BrickShelf.svelte';
    import CategoryBrickShelf, {
        isCategoryBrickShelf,
    } from '~/components/jet/shelf/CategoryBrickShelf.svelte';
    import EditorialCardShelf, {
        isEditorialCardShelf,
    } from '~/components/jet/shelf/EditorialCardShelf.svelte';
    import EditorialLinkShelf, {
        isEditorialLinkShelf,
    } from '~/components/jet/shelf/EditorialLinkShelf.svelte';
    import FramedArtworkShelf, {
        isFramedArtworkShelf,
    } from '~/components/jet/shelf/FramedArtworkShelf.svelte';
    import FramedVideoShelf, {
        isFramedVideoShelf,
    } from '~/components/jet/shelf/FramedVideoShelf.svelte';
    import HeroCarouselShelf, {
        isHeroCarouselShelf,
    } from '~/components/jet/shelf/HeroCarouselShelf.svelte';
    import HorizontalRuleShelf, {
        isHorizontalRuleShelf,
    } from '~/components/jet/shelf/HorizontalRuleShelf.svelte';
    import InAppPurchaseLockupShelf, {
        isInAppPurchaseLockupShelf,
    } from '~/components/jet/shelf/InAppPurchaseLockupShelf.svelte';
    import LargeHeroBreakoutShelf, {
        isLargeHeroBreakoutShelf,
    } from '~/components/jet/shelf/LargeHeroBreakoutShelf.svelte';
    import LargeBrickShelf, {
        isLargeBrickShelf,
    } from '~/components/jet/shelf/LargeBrickShelf.svelte';
    import LargeImageLockupShelf, {
        isLargeImageLockupShelf,
    } from '~/components/jet/shelf/LargeImageLockupShelf.svelte';
    import LargeLockupShelf, {
        isLargeLockupShelf,
    } from '~/components/jet/shelf/LargeLockupShelf.svelte';
    import LargeStoryCardShelf, {
        isLargeStoryCardShelf,
    } from '~/components/jet/shelf/LargeStoryCardShelf.svelte';
    import LinkableTextShelf, {
        isLinkableTextShelf,
    } from '~/components/jet/shelf/LinkableTextShelf.svelte';
    import {
        isMarkerShelf,
        type MarkerShelf as MarkerShelfModel,
    } from '~/components/jet/shelf/MarkerShelf.svelte';
    import MediumImageLockupShelf, {
        isMediumImageLockupShelf,
    } from '~/components/jet/shelf/MediumImageLockupShelf.svelte';
    import MediumLockupShelf, {
        isMediumLockupShelf,
    } from '~/components/jet/shelf/MediumLockupShelf.svelte';
    import MediumStoryCardShelf, {
        isMediumStoryCardShelf,
    } from '~/components/jet/shelf/MediumStoryCardShelf.svelte';
    import ProductBadgeShelf, {
        isProductBadgeShelf,
    } from '~/components/jet/shelf/ProductBadgeShelf.svelte';
    import PageHeaderShelf, {
        isPageHeaderShelf,
    } from '~/components/jet/shelf/PageHeaderShelf.svelte';
    import ParagraphShelf, {
        isParagraphShelf,
    } from '~/components/jet/shelf/ParagraphShelf.svelte';
    import PosterLockupShelf, {
        isPosterLockupShelf,
    } from '~/components/jet/shelf/PosterLockupShelf.svelte';
    import ProductMediaShelf, {
        isProductMediaShelf,
    } from '~/components/jet/shelf/ProductMediaShelf.svelte';
    import ProductDescriptionShelf, {
        isProductDescriptionShelf,
    } from '~/components/jet/shelf/ProductDescriptionShelf.svelte';
    import ProductRatingsShelf, {
        isProductRatingsShelf,
    } from '~/components/jet/shelf/ProductRatingsShelf.svelte';
    import ProductReviewShelf, {
        isProductReviewShelf,
    } from '~/components/jet/shelf/ProductReviewShelf.svelte';
    import RibbonBarShelf, {
        isRibbonBarShelf,
    } from '~/components/jet/shelf/RibbonBarShelf.svelte';
    import SearchLinkShelf, {
        isSearchLinkShelf,
    } from '~/components/jet/shelf/SearchLinkShelf.svelte';
    import SearchResultShelf, {
        isSearchResultShelf,
    } from '~/components/jet/shelf/SearchResultShelf.svelte';
    import SmallBreakoutShelf, {
        isSmallBreakoutShelf,
    } from '~/components/jet/shelf/SmallBreakoutShelf.svelte';
    import SmallBrickShelf, {
        isSmallBrickShelf,
    } from '~/components/jet/shelf/SmallBrickShelf.svelte';
    import SmallLockupShelf, {
        isSmallLockupShelf,
    } from '~/components/jet/shelf/SmallLockupShelf.svelte';
    import SmallStoryCardShelf, {
        isSmallStoryCardShelf,
    } from '~/components/jet/shelf/SmallStoryCardShelf.svelte';
    import PrivacyHeaderShelf, {
        isPrivacyHeaderShelf,
    } from '~/components/jet/shelf/PrivacyHeaderShelf.svelte';
    import PrivacyFooterShelf, {
        isPrivacyFooterShelf,
    } from '~/components/jet/shelf/PrivacyFooterShelf.svelte';
    import PrivacyTypeShelf, {
        isPrivacyTypeShelf,
    } from '~/components/jet/shelf/PrivacyTypeShelf.svelte';
    import ProductCapabilityShelf, {
        isProductCapabilityShelf,
    } from '~/components/jet/shelf/ProductCapabilityShelf.svelte';
    import ProductPageLinkShelf, {
        isProductPageLinkShelf,
    } from './ProductPageLinkShelf.svelte';
    import QuoteShelf, {
        isQuoteShelf,
    } from '~/components/jet/shelf/QuoteShelf.svelte';
    import ReviewsContainerShelf, {
        isReviewsContainerShelf,
    } from '~/components/jet/shelf/ReviewsContainerShelf.svelte';
    import ReviewsShelf, {
        isReviewsShelf,
    } from '~/components/jet/shelf/ReviewsShelf.svelte';
    import TitledParagraphShelf, {
        isTitledParagraphShelf,
    } from '~/components/jet/shelf/TitledParagraphShelf.svelte';
    import TodayCardShelf, {
        isTodayCardShelf,
    } from '~/components/jet/shelf/TodayCardShelf.svelte';
    import UberShelf, {
        isUberShelf,
    } from '~/components/jet/shelf//UberShelf.svelte';
    import FallbackShelf, {
        isFallbackShelf,
    } from '~/components/jet/shelf/FallbackShelf.svelte';

    interface $$Slots {
        /**
         * If the `shelf` is recognized to be a {@linkcode MarkerShelfModel}, this
         * slot is rendered with the `shelf` as data rather than rendering the
         * shelf directly.
         *
         * This is done because "marker" shelves need the whole "page" definition to
         * be rendered, which is not available at this level of the UI. Rather than
         * having to pass that data down to this level, we yield rendering back to
         * the "parent" component that can provide that data directly.
         */
        'marker-shelf': {
            shelf: MarkerShelfModel;
        };
    }

    export let shelf: Shelf;
</script>

<!--
@component
Render a generic `Shelf`

This component is responsible for rendering any kind of `Shelf` that
the App Store is capable of rendering. It primarily does this by trying
to narrow the generic `Shelf` down to a more-specific type and then
rendering a component specifically made for it
-->

{#if isAccessibilityHeaderShelf(shelf)}
    <AccessibilityHeaderShelf {shelf} />
{:else if isAccessibilityFeaturesShelf(shelf)}
    <AccessibilityFeaturesShelf {shelf} />
{:else if isAccessibilityDeveloperLinkShelf(shelf)}
    <AccessibilityDeveloperLinkShelf {shelf} />
{:else if isActionShelf(shelf)}
    <ActionShelf {shelf} />
{:else if isAnnotationShelf(shelf)}
    <AnnotationShelf {shelf} />
{:else if isAppEventDetailShelf(shelf)}
    <AppEventDetailShelf {shelf} />
{:else if isAppPromotionShelf(shelf)}
    <AppPromotionShelf {shelf} />
{:else if isAppShowcaseShelf(shelf)}
    <AppShowcaseShelf {shelf} />
{:else if isAppTrailerLockupShelf(shelf)}
    <AppTrailerLockupShelf {shelf} />
{:else if isArcadeFooterShelf(shelf)}
    <ArcadeFooterShelf {shelf} />
{:else if isBannerShelf(shelf)}
    <!-- a no-op until we determine if we actually want to support these banners -->
    <!-- <BannerShelf {shelf} /> -->
{:else if isBrickShelf(shelf)}
    <BrickShelf {shelf} />
{:else if isCategoryBrickShelf(shelf)}
    <CategoryBrickShelf {shelf} />
{:else if isEditorialCardShelf(shelf)}
    <EditorialCardShelf {shelf} />
{:else if isEditorialLinkShelf(shelf)}
    <EditorialLinkShelf {shelf} />
{:else if isFramedArtworkShelf(shelf)}
    <FramedArtworkShelf {shelf} />
{:else if isFramedVideoShelf(shelf)}
    <FramedVideoShelf {shelf} />
{:else if isHeroCarouselShelf(shelf)}
    <HeroCarouselShelf {shelf} />
{:else if isHorizontalRuleShelf(shelf)}
    <HorizontalRuleShelf {shelf} />
{:else if isInAppPurchaseLockupShelf(shelf)}
    <InAppPurchaseLockupShelf {shelf} />
{:else if isLargeHeroBreakoutShelf(shelf)}
    <LargeHeroBreakoutShelf {shelf} />
{:else if isLargeBrickShelf(shelf)}
    <LargeBrickShelf {shelf} />
{:else if isLargeImageLockupShelf(shelf)}
    <LargeImageLockupShelf {shelf} />
{:else if isLargeLockupShelf(shelf)}
    <LargeLockupShelf {shelf} />
{:else if isLargeStoryCardShelf(shelf)}
    <LargeStoryCardShelf {shelf} />
{:else if isLinkableTextShelf(shelf)}
    <LinkableTextShelf {shelf} />
{:else if isProductDescriptionShelf(shelf)}
    <ProductDescriptionShelf {shelf} />
{:else if isMediumImageLockupShelf(shelf)}
    <MediumImageLockupShelf {shelf} />
{:else if isMediumLockupShelf(shelf)}
    <MediumLockupShelf {shelf} />
{:else if isMediumStoryCardShelf(shelf)}
    <MediumStoryCardShelf {shelf} />
{:else if isPosterLockupShelf(shelf)}
    <PosterLockupShelf {shelf} />
{:else if isProductBadgeShelf(shelf)}
    <ProductBadgeShelf {shelf} />
{:else if isPageHeaderShelf(shelf)}
    <PageHeaderShelf {shelf} />
{:else if isParagraphShelf(shelf)}
    <ParagraphShelf {shelf} />
{:else if isPrivacyHeaderShelf(shelf)}
    <PrivacyHeaderShelf {shelf} />
{:else if isPrivacyFooterShelf(shelf)}
    <PrivacyFooterShelf {shelf} />
{:else if isPrivacyTypeShelf(shelf)}
    <PrivacyTypeShelf {shelf} />
{:else if isProductMediaShelf(shelf)}
    <ProductMediaShelf {shelf} />
{:else if isProductRatingsShelf(shelf)}
    <ProductRatingsShelf {shelf} />
{:else if isProductReviewShelf(shelf)}
    <ProductReviewShelf {shelf} />
{:else if isRibbonBarShelf(shelf)}
    <RibbonBarShelf {shelf} />
{:else if isSearchLinkShelf(shelf)}
    <SearchLinkShelf {shelf} />
{:else if isSearchResultShelf(shelf)}
    <SearchResultShelf {shelf} />
{:else if isSmallBreakoutShelf(shelf)}
    <SmallBreakoutShelf {shelf} />
{:else if isSmallBrickShelf(shelf)}
    <SmallBrickShelf {shelf} />
{:else if isSmallStoryCardShelf(shelf)}
    <SmallStoryCardShelf {shelf} />
{:else if isSmallLockupShelf(shelf)}
    <SmallLockupShelf {shelf} />
{:else if isProductCapabilityShelf(shelf)}
    <ProductCapabilityShelf {shelf} />
{:else if isProductPageLinkShelf(shelf)}
    <ProductPageLinkShelf {shelf} />
{:else if isQuoteShelf(shelf)}
    <QuoteShelf {shelf} />
{:else if isReviewsContainerShelf(shelf)}
    <ReviewsContainerShelf {shelf} />
{:else if isReviewsShelf(shelf)}
    <ReviewsShelf {shelf} />
{:else if isTodayCardShelf(shelf)}
    <TodayCardShelf {shelf} />
{:else if isTitledParagraphShelf(shelf)}
    <TitledParagraphShelf {shelf} />
{:else if isUberShelf(shelf)}
    <UberShelf {shelf} />
{:else if isMarkerShelf(shelf)}
    <slot name="marker-shelf" {shelf} />
{:else if isFallbackShelf(shelf)}
    <FallbackShelf {shelf} />
{/if}
