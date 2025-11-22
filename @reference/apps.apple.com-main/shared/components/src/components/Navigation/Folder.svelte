<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import type { Writable } from 'svelte/store';
    import type {
        NavigationId,
        BaseNavigationItem,
    } from '@amp/web-app-components/src/types';
    import {
        isSameTab,
        getItemComponent,
    } from '@amp/web-app-components/src/components/Navigation/utils';
    import allowDrag from '@amp/web-app-components/src/actions/allow-drag';
    import allowDrop from '@amp/web-app-components/src/actions/allow-drop';
    import { subscribeFolderOpenState } from '@amp/web-app-components/src/stores/navigation-folders-open';
    import ItemContent from './ItemContent.svelte';

    const FOLDER_EXPAND_DELAY = 1000;
    const dispatch = createEventDispatcher();

    export let item: BaseNavigationItem;
    export let isEditing: boolean = false;
    export let currentTab: Writable<NavigationId | null>;
    export let translateFn: (key: string) => string;
    export let getItemDragData: (item: BaseNavigationItem) => any = null;
    export let itemDragEnabled:
        | boolean
        | ((item: BaseNavigationItem) => boolean) = false;
    export let itemDropEnabled:
        | boolean
        | ((item: BaseNavigationItem) => boolean) = false;

    let delayedExpandTimeoutId: ReturnType<typeof setTimeout>;
    $: itemId = item.id.resourceId;
    $: children = item.children;
    $: hasChildren = children?.length > 0;
    $: label = item.label ? item.label : translateFn(item.locKey);
    $: isExpanded = subscribeFolderOpenState(itemId);
    $: dragData = !!getItemDragData ? getItemDragData(item) : item;
    $: isDragEnabled =
        !!dragData &&
        (typeof itemDragEnabled === 'function'
            ? itemDragEnabled(item)
            : itemDragEnabled);
    $: isDropEnabled =
        typeof itemDropEnabled === 'function'
            ? itemDropEnabled(item)
            : itemDropEnabled;

    const toggleExpand = (): void => {
        if (hasChildren) {
            isExpanded.set(!$isExpanded);
        }
    };

    const handleKeydown = (e: KeyboardEvent) => {
        switch (e.key) {
            case 'Enter':
                toggleExpand();
                break;

            case 'ArrowRight':
                if (hasChildren && !$isExpanded) {
                    isExpanded.set(true);
                    e.preventDefault();
                    e.stopPropagation();
                }
                break;

            case 'ArrowLeft':
                if (hasChildren && $isExpanded) {
                    isExpanded.set(false);
                    e.preventDefault();
                    e.stopPropagation();
                }
                break;
        }
    };

    // Due to dragleave events being fired when dragging over child elements,
    // we need to maintain a count of the number of elements we have entered
    // within the folder to know when we have actually left the element. When
    // enteredCount reaches 0, we know that we have finally left the outermost
    // element.
    //
    // rdar://118572702 (Use event.relatedTarget to handle dragging playlists over folders)
    // A more elegant solution could leverage event.relatedTarget to ignore
    // dragleave events from children, but there is a Safari bug where
    // relatedTarget is always null.

    let enteredCount = 0;

    const delayedExpand = (): void => {
        enteredCount++;

        if (!$isExpanded && !delayedExpandTimeoutId) {
            delayedExpandTimeoutId = setTimeout(() => {
                isExpanded.set(true);
                delayedExpandTimeoutId = null;
            }, FOLDER_EXPAND_DELAY);
        }
    };

    const cancelDelayedExpand = (): void => {
        enteredCount--;

        if (enteredCount === 0 && delayedExpandTimeoutId) {
            clearTimeout(delayedExpandTimeoutId);
            delayedExpandTimeoutId = null;
        }
    };
</script>

<!-- svelte-ignore a11y-role-has-required-aria-props -->
<li
    class="navigation-item navigation-item__folder"
    data-testid="navigation-item__{item.id.type}"
    class:navigation-item__folder--has-children={children}
    class:folder-open={$isExpanded}
    aria-expanded={$isExpanded}
    role="treeitem"
    tabindex="-1"
    on:dragenter|capture|preventDefault={delayedExpand}
    on:dragleave|capture|preventDefault={cancelDelayedExpand}
    on:keydown|self={handleKeydown}
>
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <span
        class="navigation-item__folder-label"
        class:drop-reset={!!isDropEnabled}
        data-testid={itemId}
        on:click|preventDefault={toggleExpand}
        use:allowDrag={isDragEnabled && {
            dragEnabled: true,
            dragData,
            usePlainDragImage: true,
        }}
        use:allowDrop={isDropEnabled && {
            dropEnabled: true,
            onDrop: (dropData) => dispatch('dropOnItem', { item, dropData }),
        }}
    >
        {#if hasChildren}
            <span
                data-testid="folder-arrow-indicator"
                class="folder-arrow-indicator"
                role="presentation"
            />
        {/if}
        <ItemContent icon={item.icon} {label} />
    </span>
    {#if hasChildren && $isExpanded}
        <ul class="navigation-item__folder-list">
            {#each children as child}
                {#if child.id.type === 'folder'}
                    <svelte:self
                        item={child}
                        {currentTab}
                        {getItemDragData}
                        {itemDragEnabled}
                        {itemDropEnabled}
                        {translateFn}
                        {isEditing}
                        on:selectItem
                        on:dropOnItem
                    />
                {:else}
                    <svelte:component
                        this={getItemComponent(child)}
                        item={child}
                        selected={isSameTab(child.id, $currentTab)}
                        {translateFn}
                        {isEditing}
                        getDragData={getItemDragData}
                        dragEnabled={itemDragEnabled}
                        dropEnabled={itemDropEnabled}
                        on:selectItem
                        on:drop={({ detail: dropData }) =>
                            dispatch('dropOnItem', { item: child, dropData })}
                    />
                {/if}
            {/each}
        </ul>
    {/if}
</li>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/locale' as *;
    @use 'amp/stylekit/core/mixins/line-clamp' as *;
    @use 'amp/stylekit/core/mixins/overflow-bleed' as *;

    $menuicon-folder-transition: 0.3s transform ease;

    .navigation-item__folder {
        --linkHoverTextDecoration: none;
        border-radius: 6px;
        margin-bottom: 2px;
        padding: 4px;
        position: relative;

        @media (--sidebar-visible) {
            height: 32px;
        }

        &.folder-open {
            margin-bottom: 0;
            padding-bottom: 0;
        }
    }

    .navigation-item__folder--has-children {
        height: auto;
    }

    .navigation-item__folder-label {
        border-radius: 6px;
        box-sizing: content-box;
        display: flex;
        align-items: center;

        @include overflow-bleed(3px);

        .navigation-item__folder--has-children & {
            cursor: pointer;
        }

        &:global(.is-drag-over) {
            --drag-over-color: white;
            --navigation-item-text-color: var(--drag-over-color);
            --navigation-item-icon-color: var(--drag-over-color);
            background-color: var(--selectionColor);
        }
    }

    .navigation-item__folder-list {
        margin-inline-start: 8px;
        margin-top: 4px;
    }

    .folder-arrow-indicator::before {
        content: '';
        width: 0;
        height: 0;
        display: inline-block;
        position: absolute;
        top: 16px;
        border-style: solid;
        border-top-width: 4px;
        border-top-color: transparent;
        border-bottom-width: 4px;
        border-bottom-color: transparent;
        transform: rotate(0deg);
        transition: $menuicon-folder-transition;
        border-inline-end-width: 0;
        border-inline-end-color: transparent;
        border-inline-start-width: 6px;
        border-inline-start-color: var(--systemTertiary);
        inset-inline-start: -12px;

        .folder-open & {
            transform: rotate(90deg);

            @include rtl {
                transform: rotate(-90deg);
            }
        }

        @media (--sidebar-visible) {
            top: 12px;
        }

        @media (prefers-reduced-motion: reduce) {
            transition: none;
        }
    }
</style>
