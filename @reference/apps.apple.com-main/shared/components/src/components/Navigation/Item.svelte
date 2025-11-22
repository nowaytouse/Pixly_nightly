<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import type { BaseNavigationItem } from '@amp/web-app-components/src/types';
    import allowDrag from '@amp/web-app-components/src/actions/allow-drag';
    import allowDrop, {
        type DropOptions,
    } from '@amp/web-app-components/src/actions/allow-drop';
    import ItemContent from './ItemContent.svelte';

    export let item: BaseNavigationItem;
    export let selected: boolean = false;
    export let isEditing: boolean = false;
    export let isChecked: boolean = false;
    export let translateFn: (key: string) => string;
    export let getDragData: (item: BaseNavigationItem) => any = null;
    export let dragEnabled: boolean | ((item: BaseNavigationItem) => boolean) =
        false;
    export let dropEnabled: boolean | ((item: BaseNavigationItem) => boolean) =
        false;
    export let dropTargets: DropOptions['targets'] = null;
    export let dropEffect: DataTransfer['dropEffect'] = null;
    export let effectAllowed: DataTransfer['effectAllowed'] = null;

    $: label = item.label ? item.label : translateFn(item.locKey);

    $: dragData = !!getDragData ? getDragData(item) : item;
    $: isDragEnabled =
        !!dragData &&
        (typeof dragEnabled === 'function' ? dragEnabled(item) : dragEnabled);
    $: isDropEnabled =
        typeof dropEnabled === 'function' ? dropEnabled(item) : dropEnabled;

    const dispatch = createEventDispatcher();

    function onChangeVisibility() {
        dispatch('visibilityChangeItem');
    }

    const itemClicked = (): void => {
        dispatch('selectItem', item);
    };
</script>

<!-- TODO: rdar://97308317 (Investigate svelte AX warnings in shared components) -->
<!-- svelte-ignore a11y-role-supports-aria-props -->
<li
    class="navigation-item navigation-item__{item.id.type}"
    class:navigation-item--selected={selected}
    class:is-editing={isEditing}
    class:drop-reset={!!dropEnabled}
    aria-selected={selected}
    data-testid="navigation-item"
    use:allowDrag={isDragEnabled &&
        !isEditing && {
            dragEnabled: true,
            dragData,
            usePlainDragImage: true,
            effectAllowed,
        }}
    use:allowDrop={isDropEnabled &&
        !isEditing && {
            dropEnabled: true,
            onDrop: (dropData) => dispatch('drop', dropData),
            targets: dropTargets,
            dropEffect,
        }}
>
    <slot>
        {#if isEditing}
            <label
                for={item.id.type}
                class="navigation-item__label"
                data-testid="navigation-item-editing"
            >
                <ItemContent icon={item.icon} {label}>
                    <input
                        class="navigation-item__checkbox"
                        data-testid="navigation-item-editing-checkbox"
                        type="checkbox"
                        id={item.id.type}
                        checked={isChecked}
                        on:change={onChangeVisibility}
                        slot="prefix"
                    />
                </ItemContent>
            </label>
        {:else}
            <a
                href={item.url}
                class="navigation-item__link"
                role="button"
                data-testid={item.id.resourceId || item.id.type}
                aria-pressed={selected}
                on:click|preventDefault={itemClicked}
            >
                <ItemContent icon={item.icon} {label} />
            </a>
        {/if}
    </slot>
</li>

<style lang="scss">
    @use 'amp/stylekit/core/mixins/overflow-bleed' as *;
    @use '@amp/web-shared-styles/app/core/globalvars' as *;

    .navigation-item {
        --linkHoverTextDecoration: none;
        border-radius: 6px;
        margin-bottom: 2px;
        padding: 4px;
        position: relative;

        &:last-child {
            margin-bottom: 1px;
        }

        &:not(.is-dragging) {
            &:global(.is-drag-over) {
                --drag-over-color: white;
                --navigation-item-text-color: var(--drag-over-color);
                --navigation-item-icon-color: var(--drag-over-color);
                background-color: var(--selectionColor);
            }

            &:global(.is-drag-over-top),
            &:global(.is-drag-over-bottom) {
                &::after {
                    content: '';
                    position: absolute;
                    background-color: var(--keyColor);
                    width: 100%;
                    height: $drag-over-focus-size;
                    inset-inline-start: 0;
                }
            }

            &:global(.is-drag-over-top) {
                &::after {
                    top: 0;
                    transform: translateY(calc(#{-$drag-over-focus-size} / 2));
                }
            }

            &:global(.is-drag-over-bottom) {
                &::after {
                    bottom: 0;
                    transform: translateY(calc(#{$drag-over-focus-size} / 2));
                }
            }
        }

        @media (--sidebar-visible) {
            height: 32px;

            &.navigation-item__radio {
                margin-bottom: 1px;
            }
        }
    }

    .navigation-item--selected {
        background-color: var(--navSidebarSelectedState);
    }

    .navigation-item__search {
        @media (--sidebar-visible) {
            display: none;
        }
    }

    .navigation-item__link {
        display: block;
        box-sizing: content-box;
        border-radius: inherit;

        @include overflow-bleed(3px);
    }

    .navigation-item__checkbox {
        accent-color: var(--keyColor);
        margin-inline-end: 5px;
    }
</style>
