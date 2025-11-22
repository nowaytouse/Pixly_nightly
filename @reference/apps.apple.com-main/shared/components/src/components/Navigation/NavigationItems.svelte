<script lang="ts">
    import { createEventDispatcher, onMount } from 'svelte';
    import type { Writable } from 'svelte/store';
    import type { NavigationId } from '@amp/web-app-components/src/types';
    import { menuIsExpanded } from '@amp/web-app-components/src/components/Navigation/store/menu-state';
    import type { NavigationItem } from '@amp/web-app-components/src/components/Navigation/types';
    import {
        isSameTab,
        getItemComponent,
    } from '@amp/web-app-components/src/components/Navigation/utils';
    import Folder from './Folder.svelte';
    import { shouldShowNavigationItem } from '@amp/web-app-components/src/utils/should-show-navigation-item';
    import allowDrop from '@amp/web-app-components/src/actions/allow-drop';
    import { listKeyboardAccess } from '@amp/web-app-components/src/actions/list-keyboard-access';

    let isEditing = false;

    /**
     * The local storage key with the prefs of what library items to be visible
     */
    export let visibilityPreferencesKey: string | null = null;

    /**
     * The navigation tabs to display.
     */
    export let items: NavigationItem[];

    /**
     * The type of navigation item to display
     */
    export let type: string | null = null;

    /**
     * Retrieve UI translations for a given localization key.
     */
    export let translateFn: (key: string) => string;

    /**
     * The navigation title header -- this appears right over the items.
     */
    export let header: string | null;

    /**
     * The store containing the currently selected tab.
     */
    export let currentTab: Writable<NavigationId | null>;

    /**
     * Boolean or method to indicate if it allows drop on header
     */
    export let headerDropEnabled: boolean | ((type: string) => boolean) = false;

    /**
     * Optional function to map item to drag data
     */
    export let getItemDragData: (item: NavigationItem) => any = null;

    /**
     * Boolean or method to indicate if it allows dragging an item
     */
    export let itemDragEnabled: boolean | ((item: NavigationItem) => boolean) =
        false;

    /**
     * Boolean or method to indicate if it allows drop on an item
     */
    export let itemDropEnabled: boolean | ((item: NavigationItem) => boolean) =
        false;

    export let listGroupElement: HTMLElement = null;

    const dispatch = createEventDispatcher();

    const setCurrentActiveItem = (event: CustomEvent<{ id: NavigationId }>) => {
        currentTab.set(event.detail.id);

        // Always immediately close the menu (in XS breakpoint)
        menuIsExpanded.set(false);

        dispatch('menuItemClick', event.detail);
    };

    $: ariaRole = items.find((item) => item?.children) ? 'tree' : null;
    $: containingClassName = type ? `navigation-items--${type}` : '';
    $: isHeaderDropEnabled =
        typeof headerDropEnabled === 'function'
            ? headerDropEnabled(type)
            : headerDropEnabled;

    function toggleEdit() {
        isEditing = !isEditing;
    }

    let data = {};

    function visibilityChangeItem(storageKey: string) {
        const currentSetting = data[storageKey];
        data = { ...data, [storageKey]: !currentSetting };
        localStorage.setItem(visibilityPreferencesKey, JSON.stringify(data));
    }

    function displayOptions() {
        const current = localStorage?.getItem(visibilityPreferencesKey);

        if (current) {
            data = JSON.parse(current);
        } else {
            data = Object.fromEntries(
                items.map(({ storageKey }) => [storageKey, true]),
            );
            localStorage?.setItem(
                visibilityPreferencesKey,
                JSON.stringify(data),
            );
        }
    }

    onMount(() => {
        if (visibilityPreferencesKey) {
            displayOptions();
        }
    });
</script>

<div
    data-testid={`navigation-items-${type}`}
    class={`navigation-items ${containingClassName}`}
>
    {#if header}
        <div
            aria-hidden="true"
            class="navigation-items__header"
            class:drop-reset={isHeaderDropEnabled}
            data-testid={`navigation-items-header`}
            use:allowDrop={isHeaderDropEnabled &&
                !isEditing && {
                    dropEnabled: true,
                    onDrop: (dropData) =>
                        dispatch('dropOnHeader', { type, dropData }),
                }}
        >
            <span>
                {header}
            </span>
            {#if visibilityPreferencesKey}
                <button
                    data-testid="navigation-items__toggler"
                    on:click={toggleEdit}
                    class="edit-toggle-button"
                    class:is-editing={isEditing}
                >
                    {#if isEditing}
                        <span data-testid="navigation-items__editing-done"
                            >{translateFn('AMP.Shared.Done')}</span
                        >
                    {:else}
                        <span data-testid="navigation-items__editing-edit"
                            >{translateFn('AMP.Shared.Edit')}</span
                        >
                    {/if}
                </button>
            {/if}
        </div>
    {/if}

    <ul
        role={ariaRole}
        aria-label={header}
        class="navigation-items__list"
        use:listKeyboardAccess={{
            listItemClassNames:
                'navigation-item__link, navigation-item__folder, click-action',
            isRoving: true,
            listGroupElement: listGroupElement,
        }}
    >
        {#each items as item (item.id)}
            {#if item.id.type === 'folder'}
                <Folder
                    item={{ ...item }}
                    {isEditing}
                    {currentTab}
                    {translateFn}
                    {getItemDragData}
                    {itemDragEnabled}
                    {itemDropEnabled}
                    on:selectItem={setCurrentActiveItem}
                    on:dropOnItem
                />
            {:else if shouldShowNavigationItem(visibilityPreferencesKey, isEditing, data, item.storageKey)}
                <svelte:component
                    this={getItemComponent(item)}
                    {item}
                    selected={isSameTab(item.id, $currentTab)}
                    on:selectItem={setCurrentActiveItem}
                    isChecked={data && data[item.storageKey]}
                    {isEditing}
                    {translateFn}
                    getDragData={getItemDragData}
                    dragEnabled={itemDragEnabled}
                    dropEnabled={itemDropEnabled}
                    on:drop={({ detail: dropData }) =>
                        dispatch('dropOnItem', { item, dropData })}
                    on:visibilityChangeItem={() =>
                        visibilityChangeItem(item.storageKey)}
                />
            {/if}
        {/each}
    </ul>
</div>

<style lang="scss">
    @use '@amp/web-shared-styles/app/core/globalvars' as *;
    @use 'amp/stylekit/core/mixins/overflow-bleed' as *;

    .navigation-items {
        grid-area: navigation-items;
        padding-top: 7px;
    }

    .navigation-items--primary {
        padding-top: 9px;
    }

    .navigation-items--library {
        grid-area: library-navigation-items;
    }

    .navigation-items--personalized {
        grid-area: personalized-navigation-items;
    }

    .navigation-items__header {
        color: var(--systemSecondary);
        padding: 15px 26px 3px;
        display: flex;
        justify-content: space-between;
        font: var(--body-emphasized);

        @media (--sidebar-visible) {
            margin: 0 20px -3px;
            padding: 4px 6px;
            border-radius: 6px;
            font: var(--footnote-emphasized);
        }

        &:global(.is-drag-over) {
            --drag-over-color: white;
            color: var(--drag-over-color);
            background-color: var(--selectionColor);
        }
    }

    .edit-toggle-button {
        color: var(--systemPrimary);

        @media (--sidebar-visible) {
            opacity: 0;
            transition: var(--global-transition);

            &:focus {
                opacity: 1;
            }
        }
    }

    .edit-toggle-button.is-editing,
    .navigation-items__header:hover .edit-toggle-button {
        opacity: 1;
    }

    .navigation-items__list {
        font: var(--title-2);
        padding: 3px 26px;

        @media (--sidebar-visible) {
            font: var(--title-3);
            padding: 0 $web-navigation-inline-padding 9px;
        }
    }
</style>
