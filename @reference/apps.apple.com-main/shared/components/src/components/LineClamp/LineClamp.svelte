<script lang="ts" context="module">
    // A single observer is shared for all LineClamp instances for better performance.
    // Using an observer also means recalculations are batched so layout only has to be
    // recalculated once regardless of the number of instances of this component.
    const resizeObserver =
        typeof window !== 'undefined' && window.ResizeObserver
            ? new window.ResizeObserver((entries) => {
                  for (const entry of entries) {
                      const contentHeight = Math.ceil(entry.contentRect.height);
                      const scrollHeight = Math.ceil(entry.target.scrollHeight);
                      const borderBoxHeight = Math.ceil(
                          entry.borderBoxSize[0].blockSize,
                      );

                      const style = getComputedStyle(entry.target);

                      const lineHeight = parseInt(
                          style.getPropertyValue('line-height'),
                      );
                      const multiline = contentHeight > lineHeight;
                      const multilineCount = contentHeight / lineHeight;
                      const truncated = scrollHeight > borderBoxHeight;

                      const event = new CustomEvent<LineClampResizeDetail>(
                          'lineClampResize',
                          {
                              detail: {
                                  multiline,
                                  multilineCount,
                                  truncated,
                              },
                          },
                      );
                      entry.target.dispatchEvent(event);
                  }
              })
            : null;
</script>

<script lang="ts">
    import { onMount, createEventDispatcher } from 'svelte';
    import { getRafQueue } from '@amp/web-app-components/src/utils/rafQueue';

    /*
     * Number of lines to clamp the container contents.
     */
    export let clamp: number = 1;

    /**
     * Whether the clamp container should be observed for multiline change events.
     *
     * Observed containers emit the `resize` event with event detail
     * { multiline: boolean, truncated: boolean }.
     *  - multiline (boolean): whether the container is more than one line tall
     *  - truncated (boolean): whether the text is truncated
     *
     * This can be used for conditional styling of other clamp containers which
     * may be allowed to expand if an adjacent container is only a single line.
     */
    export let observe: boolean = false;

    /*
     * Whether to allow focus indicators to overflow the container.
     *
     * Line clamping requires `overflow: hidden` in order to hide truncated contents.
     * However, this will also clip focus indicators of elements inside the clamped
     * container. Setting this to `true` allows focus indicators to overflow the
     * clamped container while still hiding truncated contents.
     *
     * The amount of overflow bleed defaults to the Sass variable `$focus-size`, but
     * can be adjusted using the CSS property `--overflowBleedSize`.
     */
    export let allowFocusOverflow: boolean = false;

    /**
     * Since slots are not able to be wrapped ( https://github.com/sveltejs/svelte/issues/5604)
     * We use this prop to determine if the badge should be rendered.
     */
    export let shouldRenderBadgeSlots: boolean = true;

    let clampElement: HTMLElement;

    let multiline: boolean = false;
    let truncated: boolean = false;

    if (observe && resizeObserver) {
        const dispatch = createEventDispatcher();
        const rafQueue = getRafQueue();

        onMount(() => {
            resizeObserver.observe(clampElement);
            clampElement.addEventListener(
                'lineClampResize',
                (e: CustomEvent<LineClampResizeDetail>) => {
                    dispatch('resize', e.detail);

                    // Multiline/truncation state is used for badge positioning
                    if ($$slots.badge && shouldRenderBadgeSlots) {
                        rafQueue.add(() => {
                            multiline = e.detail.multiline;
                            truncated = e.detail.truncated;
                        });
                    }
                },
            );

            return () => {
                resizeObserver.unobserve(clampElement);
            };
        });
    }
</script>

<!-- svelte-ignore a11y-unknown-role -->
<div
    class="multiline-clamp"
    class:multiline-clamp--overflow={allowFocusOverflow}
    class:multiline-clamp--multiline={multiline}
    class:multiline-clamp--truncated={truncated}
    class:multiline-clamp--with-badge={$$slots.badge && shouldRenderBadgeSlots}
    style="--mc-lineClamp: var(--defaultClampOverride, {clamp});"
    bind:this={clampElement}
    role="text"
>
    <!--
        NOTE: Any elements slotted here *must* have `display: inline`,
        otherwise the clamping will not take effect!

        NOTE: In order for a multiline clamp with a badge to wrap correctly,
        there must be *no whitespace* between the text element and badge
        element. Otherwise, the badge will not "stick" to the last word, and
        can end up wrapping onto its own line.
    -->
    <span class="multiline-clamp__text"><slot /></span
    >{#if $$slots.badge && shouldRenderBadgeSlots}<span
            class="multiline-clamp__badge"><slot name="badge" /></span
        >{/if}
</div>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/helpers' as *;
    @use 'amp/stylekit/core/mixins/overflow-bleed' as *;
    @use 'amp/stylekit/core/mixins/line-clamp' as *;

    // Line Clamp
    //
    // PUBLIC CSS PROPS
    //
    // *cssprop {Number} --overflowBleedSize
    // *access public
    //   Size of overflow bleed used when component prop `allowFocusOverflow`
    //   is `true`.
    //
    // *cssprop {Number} --badgeSize
    // *access public
    //   Size of badge placed in component's `badge` slot, used for positioning
    //   when the line clamp overflows to multiple lines.
    //
    //
    // PRIVATE CSS PROPS
    //
    // *cssprop {Number} --mc-overflowBleedSize [var(--overflowBleedSize, 0)]
    // *access private
    //   Size of overflow bleed.
    //
    // *cssprop {Number} --mc-badgeSize [var(--badgeSize, 8px)]
    // *access private
    //   Size of badge placed in component's `badge` slot.
    //
    // *cssprop {Number} --mc-badgeSpacing [var(--mc-badgeSize) + var(--mc-overflowBleedSize)]
    // *access private
    //   Positioning helper to ensure badge wraps with text and doesn't
    //   get truncated.
    //
    // *cssprop {Number} --mc-lineClamp [1]
    // *access private
    //   Number of lines to clamp.
    //

    .multiline-clamp {
        --mc-overflowBleedSize: var(--overflowBleedSize, 0);
        --mc-badgeSize: var(--badgeSize, 8px);
        --mc-badgeSpacing: var(--mc-badgeSize);
        word-break: break-word; // Allow long words to be truncated

        @include line-clamp(var(--mc-lineClamp, 1));
    }

    .multiline-clamp--overflow {
        --mc-overflowBleedSize: var(--overflowBleedSize, #{$focus-size});
        --mc-badgeSpacing: calc(
            var(--mc-badgeSize) + var(--mc-overflowBleedSize)
        );

        // Clip overflow contents when unfocused in order to prevent content
        // that falls within the overflow padding box from being displayed.
        clip-path: inset(var(--mc-overflowBleedSize));

        // If container scrolls due to focus, keep focused item visible
        scroll-padding: var(--mc-overflowBleedSize);

        @include overflow-bleed(var(--mc-overflowBleedSize));

        &:focus-within {
            clip-path: none;
        }
    }

    .multiline-clamp--with-badge {
        &.multiline-clamp--truncated {
            position: relative;

            // Adjust padding at end of clamp container so badge doesn't overlap text
            padding-inline-end: var(--mc-badgeSpacing);
            z-index: var(--z-default);

            .multiline-clamp__badge {
                display: block;
                position: absolute;
                bottom: var(--mc-overflowBleedSize);
                inset-inline-end: var(--mc-overflowBleedSize);
                z-index: var(--z-default);
            }
        }

        // These styles on the text and badge create the effect of "sticking"
        // the badge to the last word, so the badge never wraps to a new line on
        // its own.
        .multiline-clamp__text {
            padding-inline-end: var(--mc-badgeSpacing);
        }

        .multiline-clamp__badge:not(:empty) {
            margin-inline-start: calc(-1 * var(--mc-badgeSpacing));
        }
    }
</style>
