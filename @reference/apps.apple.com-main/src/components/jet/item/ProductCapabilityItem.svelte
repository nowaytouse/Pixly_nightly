<script lang="ts">
    import {
        type ProductCapability,
        type ProductCapabilityType,
    } from '@jet-app/app-store/api/models';

    import LineClamp from '@amp/web-app-components/src/components/LineClamp/LineClamp.svelte';
    import LinkableTextItem from '~/components/jet/item/LinkableTextItem.svelte';

    type CapabilityIcons = Record<ProductCapabilityType, string | undefined>;

    const capabilityIcons: CapabilityIcons = {
        gameCenter: '/assets/images/supports/supports-GameCenter@2x.png',
        siri: '/assets/images/supports/supports-Siri@2x.png',
        wallet: '/assets/images/supports/supports-Wallet@2x.png',
        controllers: '/assets/images/supports/supports-GameController@2x.png',
        familySharing: '/assets/images/supports/supports-FamilySharing@2x.png',
        sharePlay: '/assets/images/supports/supports-Shareplay@2x.png',
        spatialControllers:
            '/assets/images/supports/supports-SpatialController@2x.png',
        safariExtensions: '/assets/images/supports/supports-Safari@2x.png',
    };

    export let item: ProductCapability;
</script>

<article>
    <div class="capability-icon-container">
        <img
            src={capabilityIcons[item.type]}
            class="capability-icon"
            alt=""
            aria-hidden="true"
        />
    </div>

    <div class="metadata-container">
        <LineClamp clamp={1}>
            <h3>{item.title}</h3>
        </LineClamp>

        <p>
            <LinkableTextItem item={item.caption} />
        </p>
    </div>
</article>

<style>
    article {
        display: flex;
        align-items: center;
    }

    .capability-icon-container {
        flex-shrink: 0;
        width: 48px;
        margin-inline-end: 16px;
    }

    .capability-icon {
        margin-top: 2px;
        min-width: 46px;
        height: 46px;
    }

    .metadata-container {
        margin-inline-end: 16px;
    }

    .metadata-container :global(a) {
        color: var(--keyColor);
    }

    h3 {
        color: var(--systemPrimary);
        font-size: 1em;
        margin-bottom: 1px;
    }

    p {
        color: var(--systemSecondary);
        font: var(--body-tall);
    }
</style>
