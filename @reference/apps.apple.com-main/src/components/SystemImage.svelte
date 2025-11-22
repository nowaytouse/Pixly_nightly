<!--
@component
Renders an `Artwork` view model that references an SF Symbol through a  `systemimage://` or `resource://` template URL
-->
<script lang="ts" context="module">
    import type { Artwork } from '@jet-app/app-store/api/models';

    const systemImagePrefix = 'systemimage://';
    const resourcePrefix = 'resource://';

    type SystemImageTemplate = `${typeof systemImagePrefix}${string}`;
    type ResourceTemplate = `${typeof resourcePrefix}${string}`;

    /**
     * An {@linkcode Artwork} that references a system image
     */
    interface FullSystemImageArtwork extends Artwork {
        template: SystemImageTemplate | ResourceTemplate;
    }

    /**
     * The sub-set of {@linkcode FullSystemImageArtwork} required to render
     * the icon
     */
    type SystemImageArtwork = Pick<FullSystemImageArtwork, 'template'>;

    /**
     * Determine if some {@linkcode Artwork} represents a "system image"
     */
    export function isSystemImageArtwork(
        artwork: Artwork,
    ): artwork is FullSystemImageArtwork {
        return (
            artwork.template.startsWith(systemImagePrefix) ||
            artwork.template.startsWith(resourcePrefix)
        );
    }

    export function getIconNameFromTemplate(template: string) {
        return new URL(template).host;
    }
</script>

<script lang="ts">
    import SFSymbol from '~/components/SFSymbol.svelte';

    export let artwork: SystemImageArtwork;

    $: name = getIconNameFromTemplate(artwork.template);
</script>

<SFSymbol {name} />
