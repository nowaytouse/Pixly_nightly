<!--
@component
Component for rendering a `HeroCarouselItem` view-model from the App Store Client
-->
<script lang="ts">
    import type { HeroCarouselItem } from '@jet-app/app-store/api/models';

    import Hero from '~/components/hero/Hero.svelte';
    import HeroAppLockup from '~/components/hero/AppLockupDetail.svelte';
    import mediaQueries from '~/utils/media-queries';

    export let item: HeroCarouselItem;

    const {
        titleText,
        badgeText,
        overlayType,
        callToActionText,
        lockup: overlayLockup,
        clickAction,
        descriptionText,
    } = item.overlay || {};

    $: artwork = item.artwork || item.video?.preview;
    $: isXSmallViewport = $mediaQueries === 'xsmall';
    $: video = isXSmallViewport ? item.portraitVideo : item.video;
</script>

<Hero
    {artwork}
    {video}
    title={titleText}
    eyebrow={badgeText}
    action={clickAction}
    backgroundColor={item.backgroundColor}
    subtitle={descriptionText}
    isMediaDark={item.isMediaDark}
    collectionIcons={item.collectionIcons}
>
    <svelte:fragment slot="details" let:isPortraitLayout>
        {#if overlayLockup && overlayType === 'singleModule'}
            <HeroAppLockup lockup={overlayLockup} />
        {:else if callToActionText && !isPortraitLayout}
            <div class="button-container">
                <span class="get-button transparent">
                    {callToActionText}
                </span>
            </div>
        {/if}
    </svelte:fragment>
</Hero>

<style>
    .button-container {
        --get-button-font: var(--title-3-bold);
        margin-top: 16px;
        position: relative;
        z-index: 1;
    }
</style>
