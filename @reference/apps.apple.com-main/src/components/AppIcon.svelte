<script lang="ts" context="module">
    import type { Artwork as JetArtworkType } from '@jet-app/app-store/api/models';
    import type { NamedProfile } from '~/config/components/artwork';

    export type AppIconProfile = Extract<
        NamedProfile,
        | 'app-icon'
        | 'app-icon-large'
        | 'app-icon-medium'
        | 'app-icon-small'
        | 'app-icon-xlarge'
        | 'app-icon-river'
        | 'brick-app-icon'
    >;

    export function doesAppIconNeedBorder(icon: JetArtworkType): boolean {
        const doesIconHaveTransparentBackground =
            icon.backgroundColor &&
            isNamedColor(icon.backgroundColor) &&
            icon.backgroundColor.name === 'clear';
        const isIconPrerendered =
            icon.style === 'roundedRectPrerendered' ||
            icon.style === 'roundPrerendered';
        const isIconUnadorned = icon.style === 'unadorned';

        return (
            !doesIconHaveTransparentBackground &&
            !isIconPrerendered &&
            !isIconUnadorned
        );
    }
</script>

<script lang="ts">
    import Artwork from '~/components/Artwork.svelte';
    import { ArtworkConfig } from '@amp/web-app-components/config/components/artwork';
    import { isNamedColor } from '~/utils/color';

    export let icon: JetArtworkType;
    export let profile: AppIconProfile = 'app-icon';
    export let fixedWidth: boolean = true;
    export let disableAutoCenter: boolean = false;
    export let withBorder: boolean = false;

    const profiles = ArtworkConfig.get().PROFILES;

    $: computedProfile = (
        icon.style === 'pill'
            ? `${profile}-pill`
            : icon.style === 'tvRect'
            ? `${profile}-tv-rect`
            : profile
    ) as NamedProfile;
    $: widthFromProfile = profiles?.get(computedProfile)?.[0] ?? 0;
    $: hasTransparentBackground =
        !!icon.backgroundColor &&
        isNamedColor(icon.backgroundColor) &&
        icon.backgroundColor.name === 'clear';
    $: needsBorder = withBorder || doesAppIconNeedBorder(icon);

    // These prerendered "Solarium" icons need to use higher than normal quality due to how their
    // rendering pipeline downscales/transforms sources.
    $: quality =
        icon.style &&
        ['roundedRectPrerendered', 'roundPrerendered'].includes(icon.style)
            ? 75
            : undefined;
</script>

<div
    class="app-icon"
    class:pill={icon.style === 'pill'}
    class:round={icon.style === 'round'}
    class:rounded-rect={icon.style === 'roundedRect'}
    class:tv-rect={icon.style === 'tvRect'}
    class:rounded-rect-prerendered={icon.style === 'roundedRectPrerendered'}
    class:round-prerendered={icon.style === 'roundPrerendered'}
    class:with-border={needsBorder}
    style={fixedWidth ? `--profileWidth: ${widthFromProfile}px` : ''}
>
    <Artwork
        {disableAutoCenter}
        {hasTransparentBackground}
        {quality}
        artwork={icon}
        profile={computedProfile}
        noShelfChevronAnchor={true}
    />
</div>

<style>
    .app-icon {
        aspect-ratio: 1 / 1;
        min-width: var(--profileWidth, auto);
    }

    .app-icon.pill {
        aspect-ratio: 4 / 3;

        /*
        Creates elliptical corners with horizontal radii at 50% of the width and vertical radii
        at 65% of the height, for a rounded, squished, pill-like effect
        */
        border-radius: 50% 50% 50% 50% / 65% 65% 65% 65%;
    }

    .app-icon.round {
        border-radius: 50%;
    }

    .app-icon.rounded-rect {
        border-radius: 23%;
    }

    .app-icon.tv-rect {
        aspect-ratio: 16/9;
        border-radius: 9% / 16%;
    }

    .app-icon.rounded-rect-prerendered {
        border-radius: 25%;
    }

    .app-icon.round-prerendered {
        border-radius: 50%;
    }

    .app-icon.with-border {
        box-shadow: 0 0 0 1px var(--systemQuaternary);
    }
</style>
