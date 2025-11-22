<!--
@component
Renders a supported "SF Symbol" from the icons available in `~/sf-symbols`
-->
<script lang="ts" context="module">
    import type { ComponentType } from 'svelte';

    const iconComponents = import.meta.glob('~/sf-symbols/*.svg', {
        eager: true,
        import: 'default',
    });

    const iconNameToComponent: Record<string, ComponentType | undefined> =
        Object.fromEntries(
            Object.entries(iconComponents).map(
                ([fullPathToIcon, iconComponent]) => {
                    const iconName = fullPathToIcon
                        .replace('/src/sf-symbols/', '')
                        .replace('.svg', '');

                    return [iconName, iconComponent as ComponentType];
                },
            ),
        );

    /**
     * The list of all supported icons
     *
     * This is exposed only for testing/Storybook purposes
     */
    export const __iconNames = Object.keys(iconNameToComponent);

    export function getIconComponentByName(iconName: string) {
        return iconNameToComponent[iconName];
    }
</script>

<script lang="ts">
    /**
     * The name of the SF Symbol to render
     *
     * Must match the name of an `.svg` file in `~/sf-symbols`. If a file with a matching
     * name does not exist, nothing will be rendered
     */
    export let name: string;
    export let ariaHidden: boolean = false;

    $: icon = getIconComponentByName(name);
</script>

<svelte:component this={icon} aria-hidden={ariaHidden ? 'true' : 'false'} />
