<script lang="ts">
    import type {
        ArcadeFooter,
        Artwork,
        ImpressionableArtwork,
    } from '@jet-app/app-store/api/models';
    import { unwrapOptional as unwrap } from '@jet/environment/types/optional';

    import AppleArcadeLogo from '~/components/icons/AppleArcadeLogo.svg';
    import AppIconRiver from '~/components/AppIconRiver.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';

    export let item: ArcadeFooter;

    $: action = unwrap(item.buttonAction);

    function isImpressionableArtwork(
        item: ImpressionableArtwork | Artwork,
    ): item is ImpressionableArtwork {
        return 'art' in item;
    }

    // Sometimes data used to render an app icon is directly on `icon` but other times, in the case
    // of `ImpressionableArtwork`, it's on `icon.art`. Here we are plucking the data no matter where it is.
    const icons = (item.icons ?? []).map((icon) =>
        isImpressionableArtwork(icon) ? icon.art : icon,
    );
</script>

<LinkWrapper {action}>
    <article>
        {#if icons.length}
            <AppIconRiver {icons} />
        {/if}

        <div class="metadata-container">
            <div class="logo-container">
                <AppleArcadeLogo />
            </div>

            <button class="get-button gray">
                {action.title}
            </button>
        </div>
    </article>
</LinkWrapper>

<style>
    article {
        --app-icon-river-speed: 120s;
        display: flex;
        overflow: hidden;
        flex-flow: column;
        padding: 20px 0 30px;
        margin-bottom: 20px;
        text-align: center;
        border-radius: var(--global-border-radius-large);
        background: var(--footerBg);

        @media (--range-small-down) {
            --app-icon-river-icon-width: 88px;
        }

        @media (--range-medium-up) {
            --get-button-font: var(--title-3-emphasized);
        }
    }

    .metadata-container {
        display: flex;
        align-items: center;
        flex-flow: column;
        gap: 20px;
    }

    .logo-container {
        width: 128px;

        @media (--range-small-down) {
            width: 88px;
        }
    }
</style>
