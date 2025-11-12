<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { buildSrc } from '@amp/web-app-components/src/components/Artwork/utils/srcset';
    import Item from '@amp/web-app-components/src/components/Navigation/Item.svelte';
    import ItemContent from '@amp/web-app-components/src/components/Navigation/ItemContent.svelte';

    const dispatch = createEventDispatcher();

    export let item: any;
    export let selected: boolean = false;
    export let translateFn: (key: string) => string;
    $$props; // lets the other props automatically passed to navigation item components enter without being delcared explicitly

    const itemClicked = (): void => {
        dispatch('selectItem', item);
    };

    $: backgroundImage = item.artwork
        ? buildSrc(
              item.artwork.template,
              {
                  crop: 'bb',
                  width: 40,
                  height: 40,
                  fileType: 'webp',
              },
              {},
          )
        : undefined;
</script>

<Item {item} {selected} {translateFn}>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <a
        href={item.url}
        class="navigation-item__link"
        role="button"
        aria-pressed={selected}
        on:click|preventDefault={itemClicked}
    >
        <ItemContent label={item.label}>
            <div
                slot="icon"
                aria-hidden={true}
                class="icon"
                style:--background-image={`url(${backgroundImage})`}
            />
        </ItemContent>
    </a>
</Item>

<style>
    .icon {
        display: flex;
        align-self: center;
        width: 20px;
        height: 20px;
        background: var(--keyColor);
        mask: var(--background-image) center / contain no-repeat;

        @media (--sidebar-visible) {
            width: 18px;
            height: 18px;
        }
    }
</style>
