import { s as script } from "./vue-resize-C6uaZRhI.js";
import { O as ObserveVisibility } from "./vue-observe-visibility-C0vB_0Xs.js";
import { a5 as markRaw, am as shallowReactive, an as resolveComponent, ao as resolveDirective, Y as withDirectives, c as createElementBlock, o as openBlock, L as createCommentVNode, H as createBlock, R as createVNode, C as renderSlot, K as withCtx, M as Fragment, a6 as renderList, P as resolveDynamicComponent, D as mergeProps, ap as toHandlers, O as normalizeClass, W as normalizeStyle } from "./@vue-2iZ-QXnK.js";
var config = {
  itemsLimit: 1e3
};
var regex = /(auto|scroll)/;
function parents(node, ps) {
  if (node.parentNode === null) {
    return ps;
  }
  return parents(node.parentNode, ps.concat([node]));
}
var style = function style2(node, prop) {
  return getComputedStyle(node, null).getPropertyValue(prop);
};
var overflow = function overflow2(node) {
  return style(node, "overflow") + style(node, "overflow-y") + style(node, "overflow-x");
};
var scroll = function scroll2(node) {
  return regex.test(overflow(node));
};
function getScrollParent(node) {
  if (!(node instanceof HTMLElement || node instanceof SVGElement)) {
    return;
  }
  var ps = parents(node.parentNode, []);
  for (var i = 0; i < ps.length; i += 1) {
    if (scroll(ps[i])) {
      return ps[i];
    }
  }
  return document.scrollingElement || document.documentElement;
}
function _typeof(obj) {
  "@babel/helpers - typeof";
  return _typeof = "function" == typeof Symbol && "symbol" == typeof Symbol.iterator ? function(obj2) {
    return typeof obj2;
  } : function(obj2) {
    return obj2 && "function" == typeof Symbol && obj2.constructor === Symbol && obj2 !== Symbol.prototype ? "symbol" : typeof obj2;
  }, _typeof(obj);
}
var props = {
  items: {
    type: Array,
    required: true
  },
  keyField: {
    type: String,
    default: "id"
  },
  direction: {
    type: String,
    default: "vertical",
    validator: function validator(value) {
      return ["vertical", "horizontal"].includes(value);
    }
  },
  listTag: {
    type: String,
    default: "div"
  },
  itemTag: {
    type: String,
    default: "div"
  }
};
function simpleArray() {
  return this.items.length && _typeof(this.items[0]) !== "object";
}
var supportsPassive = false;
if (typeof window !== "undefined") {
  supportsPassive = false;
  try {
    var opts = Object.defineProperty({}, "passive", {
      get: function get() {
        supportsPassive = true;
      }
    });
    window.addEventListener("test", null, opts);
  } catch (e) {
  }
}
let uid = 0;
var script$2 = {
  name: "RecycleScroller",
  components: {
    ResizeObserver: script
  },
  directives: {
    ObserveVisibility
  },
  props: {
    ...props,
    itemSize: {
      type: Number,
      default: null
    },
    gridItems: {
      type: Number,
      default: void 0
    },
    itemSecondarySize: {
      type: Number,
      default: void 0
    },
    minItemSize: {
      type: [Number, String],
      default: null
    },
    sizeField: {
      type: String,
      default: "size"
    },
    typeField: {
      type: String,
      default: "type"
    },
    buffer: {
      type: Number,
      default: 200
    },
    pageMode: {
      type: Boolean,
      default: false
    },
    prerender: {
      type: Number,
      default: 0
    },
    emitUpdate: {
      type: Boolean,
      default: false
    },
    updateInterval: {
      type: Number,
      default: 0
    },
    skipHover: {
      type: Boolean,
      default: false
    },
    listTag: {
      type: String,
      default: "div"
    },
    itemTag: {
      type: String,
      default: "div"
    },
    listClass: {
      type: [String, Object, Array],
      default: ""
    },
    itemClass: {
      type: [String, Object, Array],
      default: ""
    }
  },
  emits: [
    "resize",
    "visible",
    "hidden",
    "update",
    "scroll-start",
    "scroll-end"
  ],
  data() {
    return {
      pool: [],
      totalSize: 0,
      ready: false,
      hoverKey: null
    };
  },
  computed: {
    sizes() {
      if (this.itemSize === null) {
        const sizes = {
          "-1": { accumulator: 0 }
        };
        const items = this.items;
        const field = this.sizeField;
        const minItemSize = this.minItemSize;
        let computedMinSize = 1e4;
        let accumulator = 0;
        let current;
        for (let i = 0, l = items.length; i < l; i++) {
          current = items[i][field] || minItemSize;
          if (current < computedMinSize) {
            computedMinSize = current;
          }
          accumulator += current;
          sizes[i] = { accumulator, size: current };
        }
        this.$_computedMinItemSize = computedMinSize;
        return sizes;
      }
      return [];
    },
    simpleArray,
    itemIndexByKey() {
      const { keyField, items } = this;
      const result = {};
      for (let i = 0, l = items.length; i < l; i++) {
        result[items[i][keyField]] = i;
      }
      return result;
    }
  },
  watch: {
    items() {
      this.updateVisibleItems(true);
    },
    pageMode() {
      this.applyPageMode();
      this.updateVisibleItems(false);
    },
    sizes: {
      handler() {
        this.updateVisibleItems(false);
      },
      deep: true
    },
    gridItems() {
      this.updateVisibleItems(true);
    },
    itemSecondarySize() {
      this.updateVisibleItems(true);
    }
  },
  created() {
    this.$_startIndex = 0;
    this.$_endIndex = 0;
    this.$_views = /* @__PURE__ */ new Map();
    this.$_unusedViews = /* @__PURE__ */ new Map();
    this.$_scrollDirty = false;
    this.$_lastUpdateScrollPosition = 0;
    if (this.prerender) {
      this.$_prerender = true;
      this.updateVisibleItems(false);
    }
    if (this.gridItems && !this.itemSize) {
      console.error("[vue-recycle-scroller] You must provide an itemSize when using gridItems");
    }
  },
  mounted() {
    this.applyPageMode();
    this.$nextTick(() => {
      this.$_prerender = false;
      this.updateVisibleItems(true);
      this.ready = true;
    });
  },
  activated() {
    const lastPosition = this.$_lastUpdateScrollPosition;
    if (typeof lastPosition === "number") {
      this.$nextTick(() => {
        this.scrollToPosition(lastPosition);
      });
    }
  },
  beforeUnmount() {
    this.removeListeners();
  },
  methods: {
    addView(pool, index, item, key, type) {
      const nr = markRaw({
        id: uid++,
        index,
        used: true,
        key,
        type
      });
      const view = shallowReactive({
        item,
        position: 0,
        nr
      });
      pool.push(view);
      return view;
    },
    unuseView(view, fake = false) {
      const unusedViews = this.$_unusedViews;
      const type = view.nr.type;
      let unusedPool = unusedViews.get(type);
      if (!unusedPool) {
        unusedPool = [];
        unusedViews.set(type, unusedPool);
      }
      unusedPool.push(view);
      if (!fake) {
        view.nr.used = false;
        view.position = -9999;
      }
    },
    handleResize() {
      this.$emit("resize");
      if (this.ready) this.updateVisibleItems(false);
    },
    handleScroll(event) {
      if (!this.$_scrollDirty) {
        this.$_scrollDirty = true;
        if (this.$_updateTimeout) return;
        const requestUpdate = () => requestAnimationFrame(() => {
          this.$_scrollDirty = false;
          const { continuous } = this.updateVisibleItems(false, true);
          if (!continuous) {
            clearTimeout(this.$_refreshTimout);
            this.$_refreshTimout = setTimeout(this.handleScroll, this.updateInterval + 100);
          }
        });
        requestUpdate();
        if (this.updateInterval) {
          this.$_updateTimeout = setTimeout(() => {
            this.$_updateTimeout = 0;
            if (this.$_scrollDirty) requestUpdate();
          }, this.updateInterval);
        }
      }
    },
    handleVisibilityChange(isVisible, entry) {
      if (this.ready) {
        if (isVisible || entry.boundingClientRect.width !== 0 || entry.boundingClientRect.height !== 0) {
          this.$emit("visible");
          requestAnimationFrame(() => {
            this.updateVisibleItems(false);
          });
        } else {
          this.$emit("hidden");
        }
      }
    },
    updateVisibleItems(checkItem, checkPositionDiff = false) {
      const itemSize = this.itemSize;
      const gridItems = this.gridItems || 1;
      const itemSecondarySize = this.itemSecondarySize || itemSize;
      const minItemSize = this.$_computedMinItemSize;
      const typeField = this.typeField;
      const keyField = this.simpleArray ? null : this.keyField;
      const items = this.items;
      const count = items.length;
      const sizes = this.sizes;
      const views = this.$_views;
      const unusedViews = this.$_unusedViews;
      const pool = this.pool;
      const itemIndexByKey = this.itemIndexByKey;
      let startIndex, endIndex;
      let totalSize;
      let visibleStartIndex, visibleEndIndex;
      if (!count) {
        startIndex = endIndex = visibleStartIndex = visibleEndIndex = totalSize = 0;
      } else if (this.$_prerender) {
        startIndex = visibleStartIndex = 0;
        endIndex = visibleEndIndex = Math.min(this.prerender, items.length);
        totalSize = null;
      } else {
        const scroll3 = this.getScroll();
        if (checkPositionDiff) {
          let positionDiff = scroll3.start - this.$_lastUpdateScrollPosition;
          if (positionDiff < 0) positionDiff = -positionDiff;
          if (itemSize === null && positionDiff < minItemSize || positionDiff < itemSize) {
            return {
              continuous: true
            };
          }
        }
        this.$_lastUpdateScrollPosition = scroll3.start;
        const buffer = this.buffer;
        scroll3.start -= buffer;
        scroll3.end += buffer;
        let beforeSize = 0;
        if (this.$refs.before) {
          beforeSize = this.$refs.before.scrollHeight;
          scroll3.start -= beforeSize;
        }
        if (this.$refs.after) {
          const afterSize = this.$refs.after.scrollHeight;
          scroll3.end += afterSize;
        }
        if (itemSize === null) {
          let h;
          let a = 0;
          let b = count - 1;
          let i = ~~(count / 2);
          let oldI;
          do {
            oldI = i;
            h = sizes[i].accumulator;
            if (h < scroll3.start) {
              a = i;
            } else if (i < count - 1 && sizes[i + 1].accumulator > scroll3.start) {
              b = i;
            }
            i = ~~((a + b) / 2);
          } while (i !== oldI);
          i < 0 && (i = 0);
          startIndex = i;
          totalSize = sizes[count - 1].accumulator;
          for (endIndex = i; endIndex < count && sizes[endIndex].accumulator < scroll3.end; endIndex++) ;
          if (endIndex === -1) {
            endIndex = items.length - 1;
          } else {
            endIndex++;
            endIndex > count && (endIndex = count);
          }
          for (visibleStartIndex = startIndex; visibleStartIndex < count && beforeSize + sizes[visibleStartIndex].accumulator < scroll3.start; visibleStartIndex++) ;
          for (visibleEndIndex = visibleStartIndex; visibleEndIndex < count && beforeSize + sizes[visibleEndIndex].accumulator < scroll3.end; visibleEndIndex++) ;
        } else {
          startIndex = ~~(scroll3.start / itemSize * gridItems);
          const remainer = startIndex % gridItems;
          startIndex -= remainer;
          endIndex = Math.ceil(scroll3.end / itemSize * gridItems);
          visibleStartIndex = Math.max(0, Math.floor((scroll3.start - beforeSize) / itemSize * gridItems));
          visibleEndIndex = Math.floor((scroll3.end - beforeSize) / itemSize * gridItems);
          startIndex < 0 && (startIndex = 0);
          endIndex > count && (endIndex = count);
          visibleStartIndex < 0 && (visibleStartIndex = 0);
          visibleEndIndex > count && (visibleEndIndex = count);
          totalSize = Math.ceil(count / gridItems) * itemSize;
        }
      }
      if (endIndex - startIndex > config.itemsLimit) {
        this.itemsLimitError();
      }
      this.totalSize = totalSize;
      let view;
      const continuous = startIndex <= this.$_endIndex && endIndex >= this.$_startIndex;
      if (continuous) {
        for (let i = 0, l = pool.length; i < l; i++) {
          view = pool[i];
          if (view.nr.used) {
            if (checkItem) {
              view.nr.index = itemIndexByKey[view.item[keyField]];
            }
            if (view.nr.index == null || view.nr.index < startIndex || view.nr.index >= endIndex) {
              this.unuseView(view);
            }
          }
        }
      }
      const unusedIndex = continuous ? null : /* @__PURE__ */ new Map();
      let item, type;
      let v;
      for (let i = startIndex; i < endIndex; i++) {
        item = items[i];
        const key = keyField ? item[keyField] : item;
        if (key == null) {
          throw new Error(`Key is ${key} on item (keyField is '${keyField}')`);
        }
        view = views.get(key);
        if (!itemSize && !sizes[i].size) {
          if (view) this.unuseView(view);
          continue;
        }
        type = item[typeField];
        let unusedPool = unusedViews.get(type);
        let newlyUsedView = false;
        if (!view) {
          if (continuous) {
            if (unusedPool && unusedPool.length) {
              view = unusedPool.pop();
            } else {
              view = this.addView(pool, i, item, key, type);
            }
          } else {
            v = unusedIndex.get(type) || 0;
            if (!unusedPool || v >= unusedPool.length) {
              view = this.addView(pool, i, item, key, type);
              this.unuseView(view, true);
              unusedPool = unusedViews.get(type);
            }
            view = unusedPool[v];
            unusedIndex.set(type, v + 1);
          }
          views.delete(view.nr.key);
          view.nr.used = true;
          view.nr.index = i;
          view.nr.key = key;
          view.nr.type = type;
          views.set(key, view);
          newlyUsedView = true;
        } else {
          if (!view.nr.used) {
            view.nr.used = true;
            newlyUsedView = true;
            if (unusedPool) {
              const index = unusedPool.indexOf(view);
              if (index !== -1) unusedPool.splice(index, 1);
            }
          }
        }
        view.item = item;
        if (newlyUsedView) {
          if (i === items.length - 1) this.$emit("scroll-end");
          if (i === 0) this.$emit("scroll-start");
        }
        if (itemSize === null) {
          view.position = sizes[i - 1].accumulator;
          view.offset = 0;
        } else {
          view.position = Math.floor(i / gridItems) * itemSize;
          view.offset = i % gridItems * itemSecondarySize;
        }
      }
      this.$_startIndex = startIndex;
      this.$_endIndex = endIndex;
      if (this.emitUpdate) this.$emit("update", startIndex, endIndex, visibleStartIndex, visibleEndIndex);
      clearTimeout(this.$_sortTimer);
      this.$_sortTimer = setTimeout(this.sortViews, this.updateInterval + 300);
      return {
        continuous
      };
    },
    getListenerTarget() {
      let target = getScrollParent(this.$el);
      if (window.document && (target === window.document.documentElement || target === window.document.body)) {
        target = window;
      }
      return target;
    },
    getScroll() {
      const { $el: el, direction } = this;
      const isVertical = direction === "vertical";
      let scrollState;
      if (this.pageMode) {
        const bounds = el.getBoundingClientRect();
        const boundsSize = isVertical ? bounds.height : bounds.width;
        let start = -(isVertical ? bounds.top : bounds.left);
        let size = isVertical ? window.innerHeight : window.innerWidth;
        if (start < 0) {
          size += start;
          start = 0;
        }
        if (start + size > boundsSize) {
          size = boundsSize - start;
        }
        scrollState = {
          start,
          end: start + size
        };
      } else if (isVertical) {
        scrollState = {
          start: el.scrollTop,
          end: el.scrollTop + el.clientHeight
        };
      } else {
        scrollState = {
          start: el.scrollLeft,
          end: el.scrollLeft + el.clientWidth
        };
      }
      return scrollState;
    },
    applyPageMode() {
      if (this.pageMode) {
        this.addListeners();
      } else {
        this.removeListeners();
      }
    },
    addListeners() {
      this.listenerTarget = this.getListenerTarget();
      this.listenerTarget.addEventListener("scroll", this.handleScroll, supportsPassive ? {
        passive: true
      } : false);
      this.listenerTarget.addEventListener("resize", this.handleResize);
    },
    removeListeners() {
      if (!this.listenerTarget) {
        return;
      }
      this.listenerTarget.removeEventListener("scroll", this.handleScroll);
      this.listenerTarget.removeEventListener("resize", this.handleResize);
      this.listenerTarget = null;
    },
    scrollToItem(index) {
      let scroll3;
      const gridItems = this.gridItems || 1;
      if (this.itemSize === null) {
        scroll3 = index > 0 ? this.sizes[index - 1].accumulator : 0;
      } else {
        scroll3 = Math.floor(index / gridItems) * this.itemSize;
      }
      this.scrollToPosition(scroll3);
    },
    scrollToPosition(position) {
      const direction = this.direction === "vertical" ? { scroll: "scrollTop", start: "top" } : { scroll: "scrollLeft", start: "left" };
      let viewport;
      let scrollDirection;
      let scrollDistance;
      if (this.pageMode) {
        const viewportEl = getScrollParent(this.$el);
        const scrollTop = viewportEl.tagName === "HTML" ? 0 : viewportEl[direction.scroll];
        const bounds = viewportEl.getBoundingClientRect();
        const scroller = this.$el.getBoundingClientRect();
        const scrollerPosition = scroller[direction.start] - bounds[direction.start];
        viewport = viewportEl;
        scrollDirection = direction.scroll;
        scrollDistance = position + scrollTop + scrollerPosition;
      } else {
        viewport = this.$el;
        scrollDirection = direction.scroll;
        scrollDistance = position;
      }
      viewport[scrollDirection] = scrollDistance;
    },
    itemsLimitError() {
      setTimeout(() => {
        console.log("It seems the scroller element isn't scrolling, so it tries to render all the items at once.", "Scroller:", this.$el);
        console.log("Make sure the scroller has a fixed height (or width) and 'overflow-y' (or 'overflow-x') set to 'auto' so it can scroll correctly and only render the items visible in the scroll viewport.");
      });
      throw new Error("Rendered items limit reached");
    },
    sortViews() {
      this.pool.sort((viewA, viewB) => viewA.nr.index - viewB.nr.index);
    }
  }
};
const _hoisted_1 = {
  key: 0,
  ref: "before",
  class: "vue-recycle-scroller__slot"
};
const _hoisted_2 = {
  key: 1,
  ref: "after",
  class: "vue-recycle-scroller__slot"
};
function render$1(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_ResizeObserver = resolveComponent("ResizeObserver");
  const _directive_observe_visibility = resolveDirective("observe-visibility");
  return withDirectives((openBlock(), createElementBlock(
    "div",
    {
      class: normalizeClass(["vue-recycle-scroller", {
        ready: $data.ready,
        "page-mode": $props.pageMode,
        [`direction-${_ctx.direction}`]: true
      }]),
      onScrollPassive: _cache[0] || (_cache[0] = (...args) => $options.handleScroll && $options.handleScroll(...args))
    },
    [
      _ctx.$slots.before ? (openBlock(), createElementBlock(
        "div",
        _hoisted_1,
        [
          renderSlot(_ctx.$slots, "before")
        ],
        512
        /* NEED_PATCH */
      )) : createCommentVNode("v-if", true),
      (openBlock(), createBlock(resolveDynamicComponent($props.listTag), {
        ref: "wrapper",
        style: normalizeStyle({ [_ctx.direction === "vertical" ? "minHeight" : "minWidth"]: $data.totalSize + "px" }),
        class: normalizeClass(["vue-recycle-scroller__item-wrapper", $props.listClass])
      }, {
        default: withCtx(() => [
          (openBlock(true), createElementBlock(
            Fragment,
            null,
            renderList($data.pool, (view) => {
              return openBlock(), createBlock(resolveDynamicComponent($props.itemTag), mergeProps({
                key: view.nr.id,
                style: $data.ready ? {
                  transform: `translate${_ctx.direction === "vertical" ? "Y" : "X"}(${view.position}px) translate${_ctx.direction === "vertical" ? "X" : "Y"}(${view.offset}px)`,
                  width: $props.gridItems ? `${_ctx.direction === "vertical" ? $props.itemSecondarySize || $props.itemSize : $props.itemSize}px` : void 0,
                  height: $props.gridItems ? `${_ctx.direction === "horizontal" ? $props.itemSecondarySize || $props.itemSize : $props.itemSize}px` : void 0
                } : null,
                class: ["vue-recycle-scroller__item-view", [
                  $props.itemClass,
                  {
                    hover: !$props.skipHover && $data.hoverKey === view.nr.key
                  }
                ]]
              }, toHandlers($props.skipHover ? {} : {
                mouseenter: () => {
                  $data.hoverKey = view.nr.key;
                },
                mouseleave: () => {
                  $data.hoverKey = null;
                }
              })), {
                default: withCtx(() => [
                  renderSlot(_ctx.$slots, "default", {
                    item: view.item,
                    index: view.nr.index,
                    active: view.nr.used
                  })
                ]),
                _: 2
                /* DYNAMIC */
              }, 1040, ["style", "class"]);
            }),
            128
            /* KEYED_FRAGMENT */
          )),
          renderSlot(_ctx.$slots, "empty")
        ]),
        _: 3
        /* FORWARDED */
      }, 8, ["style", "class"])),
      _ctx.$slots.after ? (openBlock(), createElementBlock(
        "div",
        _hoisted_2,
        [
          renderSlot(_ctx.$slots, "after")
        ],
        512
        /* NEED_PATCH */
      )) : createCommentVNode("v-if", true),
      createVNode(_component_ResizeObserver, { onNotify: $options.handleResize }, null, 8, ["onNotify"])
    ],
    34
    /* CLASS, HYDRATE_EVENTS */
  )), [
    [_directive_observe_visibility, $options.handleVisibilityChange]
  ]);
}
script$2.render = render$1;
script$2.__file = "src/components/RecycleScroller.vue";
export {
  script$2 as s
};
