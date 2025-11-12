<script lang="ts">
    import type { TodayCard } from '@jet-app/app-store/api/models';

    import Hero from '~/components/hero/Hero.svelte';
    import type { NamedProfile } from '~/config/components/artwork';
    import mediaQueries from '~/utils/media-queries';
    import { isRtl } from '~/utils/locale';

    export let item: TodayCard;

    let profile: NamedProfile;

    $: isXSmallViewport = $mediaQueries === 'xsmall';
    $: artwork = item.heroMedia?.artworks[0];
    $: video = isXSmallViewport ? null : item.heroMedia?.videos[0];
    $: ({ backgroundColor, clickAction, heading, inlineDescription, title } =
        item);
    $: profile = isXSmallViewport
        ? 'large-hero-story-card-portrait'
        : isRtl()
        ? 'large-hero-story-card-rtl'
        : 'large-hero-story-card';
</script>

<Hero
    {artwork}
    {backgroundColor}
    {title}
    {video}
    action={clickAction}
    eyebrow={heading}
    subtitle={inlineDescription}
    pinArtworkToVerticalMiddle={true}
    pinArtworkToHorizontalEnd={true}
    pinTextToVerticalStart={isRtl()}
    profileOverride={profile}
    isMediaDark={item.style !== 'white'}
/>
