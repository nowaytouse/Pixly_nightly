<script lang="ts">
    import type { EditorialCard } from '@jet-app/app-store/api/models';

    import Hero from '~/components/hero/Hero.svelte';
    import AppEventDate from '~/components/AppEventDate.svelte';
    import AppLockupDetail from '~/components/hero/AppLockupDetail.svelte';
    import mediaQueries from '~/utils/media-queries';
    import { isRtl } from '~/utils/locale';

    export let item: EditorialCard;

    $: isPortraitLayout = $mediaQueries === 'xsmall';
</script>

<Hero
    action={item.clickAction}
    artwork={item.artwork}
    subtitle={item.subtitle}
    title={item.title}
    pinArtworkToHorizontalEnd={true}
    backgroundColor={item.artwork?.backgroundColor}
    isMediaDark={item.mediaOverlayStyle === 'dark'}
    profileOverride={isPortraitLayout ? 'large-hero-portrait-iphone' : null}
>
    <svelte:fragment slot="eyebrow">
        {#if item.appEventFormattedDates}
            <AppEventDate formattedDates={item.appEventFormattedDates} />
        {:else}
            {item.caption}
        {/if}
    </svelte:fragment>

    <svelte:fragment slot="details">
        {#if item.lockup}
            <AppLockupDetail
                lockup={item.lockup}
                isOnDarkBackground={item.mediaOverlayStyle === 'dark'}
            />
        {/if}
    </svelte:fragment>
</Hero>
