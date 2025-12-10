import { c as createElementBlock, o as openBlock, C as renderSlot, r as ref, v as watchEffect, b as computed, R as createVNode, K as withCtx, a as createBaseVNode, O as normalizeClass, M as Fragment, a6 as renderList, u as unref, W as normalizeStyle, ad as createTextVNode, V as toDisplayString, j as reactive, E as onBeforeUnmount, n as getCurrentInstance, ao as resolveDirective, Y as withDirectives, L as createCommentVNode, p as inject, w as watch, _ as vShow, m as onMounted, a1 as onUnmounted, H as createBlock, X as Transition, T as Teleport, D as mergeProps, aq as vModelText, G as provide, F as nextTick, a5 as markRaw, s as shallowRef, al as createApp } from "./@vue-2iZ-QXnK.js";
import { E as ElIcon, a as ElEmpty, b as ElButton, c as ElDialog } from "./element-plus-DNGnGs48.js";
import { s as script$2 } from "./vue-virtual-scroller-CzXq0kCz.js";
import { c as clsx } from "./clsx-DgYk2OaC.js";
import { t as twMerge } from "./tailwind-merge-BfvC9NjT.js";
import { P as Primitive, u as useForwardPropsEmits, C as ComboboxRoot_default, a as useForwardProps, b as ComboboxAnchor_default, c as ComboboxEmpty_default, d as ComboboxLabel_default, e as ComboboxGroup_default, f as ComboboxInput_default, g as ComboboxItem_default, h as ComboboxItemIndicator_default, i as ComboboxContent_default, j as ComboboxPortal_default, k as ComboboxTrigger_default, l as useFilter, L as ListboxRoot_default, m as useId, n as ListboxGroupLabel_default, o as ListboxGroup_default, p as ListboxFilter_default, q as ListboxItem_default, r as ListboxContent_default, S as Separator_default, s as createContext, t as ContextMenuRoot_default, v as ContextMenuContent_default, w as ContextMenuPortal_default, x as ContextMenuItem_default, y as ContextMenuSeparator_default, z as ContextMenuTrigger_default } from "./reka-ui-wCciQd2l.js";
import { c as cva } from "./class-variance-authority-BHpUy2Ix.js";
import { r as reactiveOmit, u as useCurrentElement } from "./@vueuse-QEM31eOG.js";
import { S as Search, C as ChevronsUpDown, a as Check } from "./lucide-vue-next-BdfLbw8e.js";
import { p as plugin } from "./vue-tippy-ukGDgxUb.js";
import { V as VueMousetrapPlugin } from "./vue-mousetrap-QULKQ58U.js";
/* empty css                  */
import "./@element-plus-GnDUbe_i.js";
import "./@ctrl-CUqN8X7N.js";
import "./lodash-es-CWZcb097.js";
import "./vue-resize-C6uaZRhI.js";
import "./vue-observe-visibility-C0vB_0Xs.js";
import "./ohash-FNsp9I0d.js";
import "./aria-hidden-DPa16MWu.js";
import "./defu-CyM4_ujn.js";
import "./@floating-ui-VZOTgQQ6.js";
import "./mousetrap-CjyoXO91.js";
import "./dayjs-CUmg6egw.js";
(function polyfill() {
  const relList = document.createElement("link").relList;
  if (relList && relList.supports && relList.supports("modulepreload")) {
    return;
  }
  for (const link of document.querySelectorAll('link[rel="modulepreload"]')) {
    processPreload(link);
  }
  new MutationObserver((mutations) => {
    for (const mutation of mutations) {
      if (mutation.type !== "childList") {
        continue;
      }
      for (const node of mutation.addedNodes) {
        if (node.tagName === "LINK" && node.rel === "modulepreload")
          processPreload(node);
      }
    }
  }).observe(document, { childList: true, subtree: true });
  function getFetchOpts(link) {
    const fetchOpts = {};
    if (link.integrity) fetchOpts.integrity = link.integrity;
    if (link.referrerPolicy) fetchOpts.referrerPolicy = link.referrerPolicy;
    if (link.crossOrigin === "use-credentials")
      fetchOpts.credentials = "include";
    else if (link.crossOrigin === "anonymous") fetchOpts.credentials = "omit";
    else fetchOpts.credentials = "same-origin";
    return fetchOpts;
  }
  function processPreload(link) {
    if (link.ep)
      return;
    link.ep = true;
    const fetchOpts = getFetchOpts(link);
    fetch(link.href, fetchOpts);
  }
})();
const _export_sfc = (sfc, props) => {
  const target = sfc.__vccOpts || sfc;
  for (const [key, val] of props) {
    target[key] = val;
  }
  return target;
};
const _sfc_main$N = {};
const _hoisted_1$u = { class: "body-vue" };
function _sfc_render(_ctx, _cache) {
  return openBlock(), createElementBlock("div", _hoisted_1$u, [
    renderSlot(_ctx.$slots, "default")
  ]);
}
const __unplugin_components_1 = /* @__PURE__ */ _export_sfc(_sfc_main$N, [["render", _sfc_render]]);
const _hoisted_1$t = { class: "v-table" };
const _sfc_main$M = {
  __name: "TableVue",
  props: {
    header: {
      type: Object,
      default(rawProps) {
        return {
          name: "",
          minWidth: null,
          maxWidth: null,
          align: "start",
          fill: false,
          line: false
        };
      },
      required: true
    },
    data: { type: Array, required: true },
    autoScrollDelay: { type: Number, default: 3 }
  },
  setup(__props, { expose: __expose }) {
    const props = __props;
    let thead = [];
    const list = ref([]);
    Object.entries(props.header).forEach((item) => {
      const [key, value] = item;
      const width = measureText(value.name).width;
      thead.push({
        key,
        ...value,
        width: `${width}px`,
        minWidth: value.minWidth ? `${value.minWidth}px` : "auto",
        maxWidth: value.maxWidth ? `${value.maxWidth}px` : "auto"
      });
    });
    function measureText(pText) {
      let div = document.createElement("div");
      document.body.appendChild(div);
      div.style.position = "absolute";
      div.style.left = -1e3;
      div.style.top = -1e3;
      div.style.fontSize = "14px";
      div.textContent = pText;
      let result = {
        width: div.clientWidth,
        height: div.clientHeight
      };
      document.body.removeChild(div);
      div = null;
      return result;
    }
    watchEffect(() => {
      list.value = props.data.map((value, index) => {
        return { id: index, data: value };
      });
    });
    const scroller = ref(null);
    const visibleLength = computed(() => 5);
    const currentDelay = ref(0);
    const onWheel = () => {
      currentDelay.value = props.autoScrollDelay;
    };
    setInterval(() => {
      if (currentDelay.value > 0) {
        currentDelay.value--;
      }
    }, 1e3);
    __expose({
      scrollToIndex(index) {
        if (index < visibleLength.value || index >= props.data.length) return;
        if (currentDelay.value == 0) {
          scroller.value.scrollToItem(index - visibleLength.value);
        }
      }
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$t, [
        createVNode(unref(script$2), {
          class: "v-tbody",
          items: list.value,
          "item-size": 41,
          buffer: 200,
          ref_key: "scroller",
          ref: scroller,
          "key-field": "id",
          onWheel,
          "emit-update": false
        }, {
          default: withCtx(({ item, index }) => [
            createBaseVNode("div", {
              class: normalizeClass(["v-tr", `task-row-${item.data.status || ""}`])
            }, [
              (openBlock(true), createElementBlock(Fragment, null, renderList(unref(thead), (value) => {
                return openBlock(), createElementBlock("div", {
                  class: normalizeClass(["v-td", {
                    fill: value.fill,
                    striped: index % 2 == 1
                  }]),
                  style: normalizeStyle({
                    minWidth: value.minWidth,
                    maxWidth: value.maxWidth,
                    justifyContent: value.align,
                    padding: value.padding
                  }),
                  key: value
                }, [
                  renderSlot(_ctx.$slots, value.key, {
                    row: item.data,
                    index
                  }, () => [
                    createTextVNode(toDisplayString(item.data[value.key]), 1)
                  ], true)
                ], 6);
              }), 128))
            ], 2)
          ]),
          _: 3
        }, 8, ["items"])
      ]);
    };
  }
};
const TableVue = /* @__PURE__ */ _export_sfc(_sfc_main$M, [["__scopeId", "data-v-7ebf6a6f"]]);
const globalState = reactive({
  visible: false,
  imageUrl: "",
  alt: "",
  mouseX: 0,
  mouseY: 0
});
class HoverTimerManager {
  constructor() {
    this.timers = /* @__PURE__ */ new Map();
  }
  set(key, callback, delay = 250) {
    this.clear(key);
    const timer = setTimeout(callback, delay);
    this.timers.set(key, timer);
    return timer;
  }
  clear(key) {
    const timer = this.timers.get(key);
    if (timer) {
      clearTimeout(timer);
      this.timers.delete(key);
    }
  }
  clearAll() {
    this.timers.forEach((timer) => clearTimeout(timer));
    this.timers.clear();
  }
}
const timerManager = new HoverTimerManager();
function useHoverPreview() {
  const updateState = (updates) => {
    Object.assign(globalState, updates);
  };
  const showPreview = (imageUrl, alt = "", mouseX = 0, mouseY = 0) => {
    updateState({
      visible: true,
      imageUrl,
      alt,
      mouseX,
      mouseY
    });
  };
  const hidePreview = () => {
    updateState({ visible: false });
    timerManager.clearAll();
  };
  const updateMousePosition = (mouseX, mouseY) => {
    if (globalState.visible) {
      updateState({ mouseX, mouseY });
    }
  };
  const createHoverHandler = (getImageData) => {
    const componentId = Symbol("hover-component");
    const handleMouseEnter = (event) => {
      const { imageUrl, alt } = getImageData();
      if (!imageUrl) return;
      const mouseX = event.clientX;
      const mouseY = event.clientY;
      timerManager.set(componentId, () => {
        showPreview(imageUrl, alt, mouseX, mouseY);
      });
    };
    const handleMouseLeave = () => {
      timerManager.clear(componentId);
      hidePreview();
    };
    const handleMouseMove = (event) => {
      updateMousePosition(event.clientX, event.clientY);
    };
    onBeforeUnmount(() => {
      timerManager.clear(componentId);
    });
    return {
      handleMouseEnter,
      handleMouseLeave,
      handleMouseMove
    };
  };
  return {
    // 狀態 (只讀)
    globalHoverPreview: globalState,
    // 低級 API (供全局使用)
    showPreview,
    hidePreview,
    updateMousePosition,
    // 高級 API (供組件使用)
    createHoverHandler
  };
}
const _hoisted_1$s = ["src", "alt"];
const _hoisted_2$k = {
  key: 1,
  class: "thumbnail-placeholder show-placeholder"
};
const _sfc_main$L = {
  __name: "TaskThumbnail",
  props: {
    task: {
      type: Object,
      required: true
    }
  },
  setup(__props) {
    const props = __props;
    const showPlaceholder = ref(false);
    const { createHoverHandler } = useHoverPreview();
    const { handleMouseEnter, handleMouseLeave, handleMouseMove } = createHoverHandler(() => {
      var _a;
      return {
        imageUrl: props.task.thumbnailUrl && !showPlaceholder.value ? props.task.thumbnailUrl : "",
        alt: ((_a = props.task.item) == null ? void 0 : _a.name) || ""
      };
    });
    const handleThumbnailError = (event) => {
      var _a, _b;
      showPlaceholder.value = true;
      (_b = (_a = props.task).handleThumbnailError) == null ? void 0 : _b.call(_a, event);
    };
    return (_ctx, _cache) => {
      var _a;
      const _component_el_icon = ElIcon;
      return openBlock(), createElementBlock("div", {
        class: "thumbnail-wrapper",
        onMouseenter: _cache[0] || (_cache[0] = (...args) => unref(handleMouseEnter) && unref(handleMouseEnter)(...args)),
        onMouseleave: _cache[1] || (_cache[1] = (...args) => unref(handleMouseLeave) && unref(handleMouseLeave)(...args)),
        onMousemove: _cache[2] || (_cache[2] = (...args) => unref(handleMouseMove) && unref(handleMouseMove)(...args))
      }, [
        __props.task.thumbnailUrl && !showPlaceholder.value ? (openBlock(), createElementBlock("img", {
          key: 0,
          src: __props.task.thumbnailUrl,
          alt: (_a = __props.task.item) == null ? void 0 : _a.name,
          class: "task-thumbnail",
          style: normalizeStyle(__props.task.thumbnailStyle),
          onError: handleThumbnailError
        }, null, 44, _hoisted_1$s)) : (openBlock(), createElementBlock("div", _hoisted_2$k, [
          createVNode(_component_el_icon, { class: "icon-image" })
        ]))
      ], 32);
    };
  }
};
const TaskThumbnail = /* @__PURE__ */ _export_sfc(_sfc_main$L, [["__scopeId", "data-v-405a4795"]]);
const _hoisted_1$r = { class: "task-actions" };
const _sfc_main$K = {
  __name: "TaskActions",
  props: {
    task: {
      type: Object,
      required: true
    }
  },
  emits: ["remove"],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emit = __emit;
    const { proxy } = getCurrentInstance();
    const $translate = proxy.$translate;
    const removeTooltip = $translate("main.taskList.remove");
    const handleRemove = () => {
      emit("remove", props.task.id);
    };
    return (_ctx, _cache) => {
      const _component_el_icon = ElIcon;
      const _directive_tippy = resolveDirective("tippy");
      return openBlock(), createElementBlock("div", _hoisted_1$r, [
        withDirectives(createVNode(_component_el_icon, {
          class: "icon icon-trash remove",
          onClick: handleRemove
        }, null, 512), [
          [_directive_tippy, {
            content: unref(removeTooltip),
            placement: "left",
            delay: [200, 0],
            duration: [150, 0]
          }]
        ])
      ]);
    };
  }
};
const TaskActions = /* @__PURE__ */ _export_sfc(_sfc_main$K, [["__scopeId", "data-v-56b3ef08"]]);
const _hoisted_1$q = { class: "dimensions-cell" };
const _hoisted_2$j = { class: "dimension-original-item" };
const _hoisted_3$c = { class: "dimension-item-width" };
const _hoisted_4$9 = { class: "dimension-item-height" };
const _hoisted_5$6 = { class: "dimension-export-item" };
const _hoisted_6$5 = { class: "dimension-item-width" };
const _hoisted_7$4 = { class: "dimension-item-height" };
const _hoisted_8$3 = {
  key: 1,
  class: "dimension-original-item empty"
};
const _sfc_main$J = {
  __name: "DimensionDisplay",
  props: {
    task: {
      type: Object,
      required: true
    }
  },
  setup(__props) {
    return (_ctx, _cache) => {
      const _component_el_icon = ElIcon;
      return openBlock(), createElementBlock("div", _hoisted_1$q, [
        !__props.task.isError && (__props.task.dimensionsDisplay.original.width && __props.task.dimensionsDisplay.original.height) ? (openBlock(), createElementBlock(Fragment, { key: 0 }, [
          createBaseVNode("div", _hoisted_2$j, [
            createBaseVNode("span", _hoisted_3$c, toDisplayString(__props.task.dimensionsDisplay.original.width), 1),
            _cache[0] || (_cache[0] = createBaseVNode("span", { class: "dimension-item-separator" }, "×", -1)),
            createBaseVNode("span", _hoisted_4$9, toDisplayString(__props.task.dimensionsDisplay.original.height), 1)
          ]),
          __props.task.showDimensionArrow ? (openBlock(), createElementBlock(Fragment, { key: 0 }, [
            createVNode(_component_el_icon, { class: "icon-arrow" }),
            createBaseVNode("span", _hoisted_5$6, [
              createBaseVNode("span", _hoisted_6$5, toDisplayString(__props.task.dimensionsDisplay.export.width), 1),
              _cache[1] || (_cache[1] = createBaseVNode("span", { class: "dimension-item-separator" }, "×", -1)),
              createBaseVNode("span", _hoisted_7$4, toDisplayString(__props.task.dimensionsDisplay.export.height), 1)
            ])
          ], 64)) : createCommentVNode("", true)
        ], 64)) : (openBlock(), createElementBlock("span", _hoisted_8$3, "-"))
      ]);
    };
  }
};
const DimensionDisplay = /* @__PURE__ */ _export_sfc(_sfc_main$J, [["__scopeId", "data-v-36802330"]]);
const scaleProportional = (width, height, ratio) => [
  Math.round(width * ratio),
  Math.round(height * ratio)
];
const safeRatio = (target, current) => current > 0 ? target / current : 1;
const RESIZE_STRATEGIES = {
  maxSide: (w, h, target) => {
    const maxDim = Math.max(w, h);
    return maxDim > target ? scaleProportional(w, h, safeRatio(target, maxDim)) : [w, h];
  },
  minSide: (w, h, target) => {
    const minDim = Math.min(w, h);
    return minDim > target ? scaleProportional(w, h, safeRatio(target, minDim)) : [w, h];
  },
  maxWidth: (w, h, target) => {
    return w > target ? [target, Math.floor(h * safeRatio(target, w))] : [w, h];
  },
  maxHeight: (w, h, target) => {
    return h > target ? [Math.floor(w * safeRatio(target, h)), target] : [w, h];
  },
  minWidth: (w, h, target) => {
    return w < target ? [target, Math.floor(h * safeRatio(target, w))] : [w, h];
  },
  minHeight: (w, h, target) => {
    return h < target ? [Math.floor(w * safeRatio(target, h)), target] : [w, h];
  },
  original: (w, h) => [w, h]
};
const calculateDimensions = (width, height, sizeType = "original", targetSize = 900) => {
  if (!width || !height || width <= 0 || height <= 0) {
    return [width || 0, height || 0];
  }
  const strategy = RESIZE_STRATEGIES[sizeType] || RESIZE_STRATEGIES.original;
  return strategy(width, height, parseInt(targetSize) || 900);
};
const calculateItemDimensions = (item, sizeType, targetSize) => {
  if (!(item == null ? void 0 : item.width) || !(item == null ? void 0 : item.height)) return null;
  const [width, height] = calculateDimensions(item.width, item.height, sizeType, targetSize);
  return { width, height };
};
class DataHistoryManager {
  /**
   * @param {string} [data] - 可選擇性初始化時設定的原始數據
   */
  constructor(data = null) {
    this.original = data;
    this.current = data;
    this.undoStack = [];
    this.redoStack = [];
  }
  /**
   * 設定原始數據
   *
   * @param {string} data - 原始數據
   */
  setOriginal(data) {
    this.original = data;
    this.current = data;
    this.undoStack = [];
    this.redoStack = [];
  }
  /**
   * 獲取當前數據
   *
   * @return {string} 當前數據
   */
  get view() {
    return this.current;
  }
  /**
   * 添加新數據
   *
   * @param {string} data - 新數據
   */
  add(data) {
    if (!data) return;
    if (this.original === null) {
      this.setOriginal(data);
    } else {
      this.undoStack.push(this.current);
      this.current = data;
      this.redoStack = [];
    }
  }
  /**
   * 撤銷操作
   */
  undo() {
    if (this.undoStack.length > 0) {
      this.redoStack.push(this.current);
      this.current = this.undoStack.pop();
    } else {
      console.warn("No more undos available");
    }
  }
  /**
   * 重做操作
   */
  redo() {
    if (this.redoStack.length > 0) {
      this.undoStack.push(this.current);
      this.current = this.redoStack.pop();
    } else {
      console.warn("No more redos available");
    }
  }
  /**
   * 重置
   */
  reset() {
    this.original = null;
    this.current = null;
    this.undoStack = [];
    this.redoStack = [];
  }
}
class TaskDeduplicator {
  constructor() {
    this.taskIds = /* @__PURE__ */ new Set();
    this.taskMap = /* @__PURE__ */ new Map();
    this.memoryPressure = 0;
  }
  /**
   * 過濾新項目，移除重複
   * @param {Array} items - 待檢查的項目陣列
   * @returns {Array} 去除重複後的新項目
   */
  filterNewItems(items) {
    const newItems = items.filter((item) => !this.taskIds.has(item.id));
    return newItems;
  }
  /**
   * 添加任務到映射
   * @param {Object} task - 任務物件
   */
  addTask(task) {
    this.taskMap.set(task.id, task);
  }
  /**
   * 移除任務
   * @param {string} taskId - 任務 ID
   */
  removeTask(taskId) {
    if (this.taskIds.has(taskId)) {
      this.taskIds.delete(taskId);
      this.taskMap.delete(taskId);
    }
  }
  /**
   * 檢查任務是否存在
   * @param {string} taskId - 任務 ID
   * @returns {boolean}
   */
  hasTask(taskId) {
    return this.taskIds.has(taskId);
  }
  /**
   * 獲取任務
   * @param {string} taskId - 任務 ID
   * @returns {Object|null}
   */
  getTask(taskId) {
    return this.taskMap.get(taskId) || null;
  }
  /**
   * 清空所有任務
   */
  clear() {
    this.taskIds.clear();
    this.taskMap.clear();
    this.memoryPressure = 0;
  }
  /**
   * 估算項目記憶體大小
   * @param {Object} item - 項目物件
   * @returns {number} 估算的位元組數
   */
  estimateItemSize(item) {
    const str = JSON.stringify(item);
    return str.length * 2;
  }
  /**
   * 獲取統計資訊
   * @returns {Object}
   */
  getStats() {
    return {
      totalTasks: this.taskIds.size,
      memoryPressure: this.memoryPressure,
      memoryPressureMB: Math.round(this.memoryPressure / (1024 * 1024) * 100) / 100
    };
  }
}
class TaskBatcher {
  constructor(options = {}) {
    this.deduplicator = new TaskDeduplicator();
    this.options = {
      batchSize: 200,
      maxMemoryMB: 100,
      // 最大記憶體使用量 (MB)
      autoTune: true,
      // 自動調整批次大小
      ...options
    };
    this.currentBatchSize = this.options.batchSize;
    this.performanceMetrics = {
      avgBatchTime: 0,
      totalBatches: 0,
      memoryPeak: 0
    };
  }
  /**
   * 處理大量項目的分批載入
   * @param {Array} items - 要處理的項目陣列
   * @param {Function} processor - 處理函數
   * @param {Object} options - 處理選項
   * @returns {Promise<Array>} 處理結果
   */
  async processBatch(items, processor, options = {}) {
    const { onProgress = () => {
    }, onBatchComplete = () => {
    }, batchDelay = 16 } = options;
    const uniqueItems = this.deduplicator.filterNewItems(items);
    console.log(
      `TaskBatcher: Processing ${uniqueItems.length} unique items (${items.length - uniqueItems.length} duplicates filtered)`
    );
    if (uniqueItems.length === 0) {
      return [];
    }
    if (this.options.autoTune) {
      this.adjustBatchSize();
    }
    const batches = this.createBatches(uniqueItems, this.currentBatchSize);
    const results = [];
    for (let i = 0; i < batches.length; i++) {
      const batchStartTime = Date.now();
      const batch = batches[i];
      const batchResults = [];
      for (const item of batch) {
        try {
          const result = await processor(item);
          if (result) {
            batchResults.push(result);
            this.deduplicator.addTask(result);
          }
        } catch (error) {
          console.warn(`TaskBatcher: Error processing item ${item.id}:`, error);
        }
      }
      results.push(...batchResults);
      const batchTime = Date.now() - batchStartTime;
      this.updatePerformanceMetrics(batchTime);
      onProgress({
        batchIndex: i + 1,
        totalBatches: batches.length,
        batchSize: batch.length,
        batchTime,
        memoryStats: this.deduplicator.getStats(),
        currentBatchSize: this.currentBatchSize
      });
      onBatchComplete(i, batchResults);
      if (this.checkMemoryPressure()) {
        console.warn("TaskBatcher: High memory pressure detected, triggering cleanup");
        await this.performMemoryCleanup();
      }
      if (i < batches.length - 1) {
        await new Promise((resolve) => setTimeout(resolve, batchDelay));
      }
    }
    console.log(
      `TaskBatcher: Completed processing ${results.length} items in ${batches.length} batches`
    );
    return results;
  }
  /**
   * 創建批次陣列
   * @param {Array} items - 項目陣列
   * @param {number} batchSize - 批次大小
   * @returns {Array<Array>} 批次陣列
   */
  createBatches(items, batchSize) {
    const batches = [];
    for (let i = 0; i < items.length; i += batchSize) {
      batches.push(items.slice(i, i + batchSize));
    }
    return batches;
  }
  /**
   * 根據效能調整批次大小
   */
  adjustBatchSize() {
    const stats = this.deduplicator.getStats();
    const memoryUsageMB = stats.memoryPressureMB;
    if (memoryUsageMB > this.options.maxMemoryMB * 0.8) {
      this.currentBatchSize = Math.max(50, Math.floor(this.currentBatchSize * 0.8));
    } else if (memoryUsageMB < this.options.maxMemoryMB * 0.3 && this.performanceMetrics.avgBatchTime < 50) {
      this.currentBatchSize = Math.min(500, Math.floor(this.currentBatchSize * 1.2));
    }
    console.log(
      `TaskBatcher: Adjusted batch size to ${this.currentBatchSize} (Memory: ${memoryUsageMB}MB)`
    );
  }
  /**
   * 更新效能指標
   * @param {number} batchTime - 批次處理時間
   */
  updatePerformanceMetrics(batchTime) {
    this.performanceMetrics.totalBatches++;
    const total = this.performanceMetrics.avgBatchTime * (this.performanceMetrics.totalBatches - 1) + batchTime;
    this.performanceMetrics.avgBatchTime = total / this.performanceMetrics.totalBatches;
    const currentMemory = this.deduplicator.getStats().memoryPressureMB;
    if (currentMemory > this.performanceMetrics.memoryPeak) {
      this.performanceMetrics.memoryPeak = currentMemory;
    }
  }
  /**
   * 檢查記憶體壓力
   * @returns {boolean}
   */
  checkMemoryPressure() {
    const stats = this.deduplicator.getStats();
    return stats.memoryPressureMB > this.options.maxMemoryMB;
  }
  /**
   * 執行記憶體清理
   * @returns {Promise}
   */
  async performMemoryCleanup() {
    if (global.gc) {
      global.gc();
    }
    await new Promise((resolve) => setTimeout(resolve, 0));
    console.log("TaskBatcher: Memory cleanup performed");
  }
  /**
   * 獲取效能統計
   * @returns {Object}
   */
  getPerformanceStats() {
    return {
      ...this.performanceMetrics,
      currentBatchSize: this.currentBatchSize,
      dedupStats: this.deduplicator.getStats()
    };
  }
  /**
   * 重置批次處理器
   */
  reset() {
    this.deduplicator.clear();
    this.currentBatchSize = this.options.batchSize;
    this.performanceMetrics = {
      avgBatchTime: 0,
      totalBatches: 0,
      memoryPeak: 0
    };
  }
}
class FileNameAllocator {
  constructor() {
    this.nameMap = /* @__PURE__ */ new Map();
    this.usedNames = /* @__PURE__ */ new Map();
  }
  /**
   * 為任務批次預分配檔名
   * @param {Array} tasks - 任務陣列 [{id, fileName, format}...]
   * @param {Object} options - 選項 {nameType: 'original'|'custom', customName, startNumber}
   * @returns {Array} [{id: taskId, exportName: 'XXX'}, ...]
   */
  allocateFileNames(tasks, options = {}) {
    const results = [];
    const { nameType = "original", customName = "file", startNumber = 1 } = options;
    tasks.forEach((task, index) => {
      let baseName;
      if (nameType === "original") {
        baseName = task.fileName || task.name || "unnamed";
      } else {
        const currentNumber = startNumber + index;
        baseName = `${customName} ${currentNumber}`;
      }
      const extension = task.format || task.ext || "jpg";
      const baseKey = `${baseName}.${extension}`;
      if (!this.usedNames.has(baseKey)) {
        this.usedNames.set(baseKey, /* @__PURE__ */ new Set());
      }
      const usedSet = this.usedNames.get(baseKey);
      let finalName = baseName;
      let fullName = `${finalName}.${extension}`;
      if (usedSet.has(fullName)) {
        let counter = 1;
        do {
          finalName = `${baseName}(${counter})`;
          fullName = `${finalName}.${extension}`;
          counter++;
        } while (usedSet.has(fullName) && counter < 1e3);
      }
      usedSet.add(fullName);
      this.nameMap.set(task.id, finalName);
      results.push({
        id: task.id,
        exportName: finalName,
        originalName: baseName,
        extension
      });
    });
    return results;
  }
  /**
   * 獲取任務的分配名稱
   * @param {string} taskId
   * @returns {string|null}
   */
  getExportName(taskId) {
    return this.nameMap.get(taskId) || null;
  }
  /**
   * 清空所有分配
   */
  clear() {
    this.nameMap.clear();
    this.usedNames.clear();
  }
  /**
   * 獲取統計資訊
   * @returns {Object}
   */
  getStats() {
    let totalAllocated = 0;
    let duplicateCount = 0;
    this.usedNames.forEach((usedSet, baseKey) => {
      totalAllocated += usedSet.size;
      if (usedSet.size > 1) {
        duplicateCount += usedSet.size - 1;
      }
    });
    return {
      totalAllocated,
      uniqueBaseNames: this.usedNames.size,
      duplicateCount,
      taskCount: this.nameMap.size
    };
  }
}
function createLightTask(item) {
  return {
    // 核心屬性 - 保留 id 方便快速存取
    id: item.id,
    item,
    // 保存完整的 item 物件
    // 轉換相關屬性
    status: "waiting",
    newFormat: "",
    convertedSize: 0,
    thumbnailUrl: item.thumbnailURL,
    // 錯誤相關屬性
    error: null,
    // 時間戳記
    createdAt: Date.now(),
    updatedAt: Date.now()
  };
}
class TaskObjectPool {
  constructor(initialSize = 100) {
    this.pool = [];
    this.inUse = /* @__PURE__ */ new Set();
    for (let i = 0; i < initialSize; i++) {
      this.pool.push(this.createEmptyTask());
    }
  }
  /**
   * 從池中獲取任務物件
   * @param {Object} item - Eagle 項目物件
   * @returns {Object} 任務物件
   */
  acquire(item) {
    let task;
    if (this.pool.length > 0) {
      task = this.pool.pop();
      this.resetTask(task, item);
    } else {
      task = createLightTask(item);
    }
    this.inUse.add(task);
    return task;
  }
  /**
   * 將任務物件返回池中
   * @param {Object} task - 任務物件
   */
  release(task) {
    if (this.inUse.has(task)) {
      this.inUse.delete(task);
      this.pool.push(task);
    }
  }
  /**
   * 建立空的任務物件
   * @returns {Object}
   */
  createEmptyTask() {
    return {
      id: "",
      item: null,
      status: "waiting",
      newFormat: "",
      convertedSize: 0,
      thumbnailUrl: "",
      error: null,
      createdAt: 0,
      updatedAt: 0
    };
  }
  /**
   * 重置任務物件
   * @param {Object} task - 任務物件
   * @param {Object} item - Eagle 項目物件
   */
  resetTask(task, item) {
    task.id = item.id;
    task.item = item;
    task.status = "waiting";
    task.newFormat = "";
    task.convertedSize = 0;
    task.thumbnailUrl = item.thumbnailURL;
    task.error = null;
    task.createdAt = Date.now();
    task.updatedAt = Date.now();
  }
  /**
   * 獲取池統計
   * @returns {Object}
   */
  getStats() {
    return {
      poolSize: this.pool.length,
      inUse: this.inUse.size,
      total: this.pool.length + this.inUse.size
    };
  }
}
const taskBatcher = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  FileNameAllocator,
  TaskBatcher,
  TaskDeduplicator,
  TaskObjectPool,
  createLightTask
}, Symbol.toStringTag, { value: "Module" }));
const fileConverter = require(`${__dirname}/modules/fileConverter`);
class Main {
  constructor() {
    this.isLoading = false;
    this.suppertedFileTypes = [
      "bmp",
      "exr",
      "gif",
      "hdr",
      "heic",
      "heif",
      "hif",
      "ico",
      "jpeg",
      "jpg",
      "jfif",
      "png",
      "svg",
      "tga",
      "tif",
      "tiff",
      "webp",
      "avif",
      "insp",
      "jxl",
      "jpe",
      "dds",
      "mp4",
      "webm",
      "mov",
      "m4v",
      "mkv"
    ];
    this.isInputEmpty = false;
    this.isInputError = false;
    this.errorType = "";
    this.errorDescription = "";
    this.item_id = null;
    this.brushSize = 50;
    this.status = "";
    this.imageHistoryManager = new DataHistoryManager();
    this.localStorageKey = "";
    this.localStorageSetting = {};
    this.items = null;
    this.taskManager = null;
  }
  // 新增方法以注入 taskManager
  setTaskManager(taskManager) {
    this.taskManager = taskManager;
  }
  /**
   * 生成唯一的檔案名稱（避免檔名衝突）
   * @param {string} outputDir 輸出目錄
   * @param {string} fileName 基礎檔名（不含副檔名）
   * @param {string} extension 副檔名
   * @returns {Promise<string>} 唯一的檔名（不含副檔名）
   */
  async generateUniqueFileName(outputDir, fileName, extension) {
    const fs2 = require("fs").promises;
    const path2 = require("path");
    let testFileName = fileName;
    let counter = 0;
    while (true) {
      const testPath = path2.join(outputDir, `${testFileName}.${extension}`);
      try {
        await fs2.access(testPath);
        if (counter === 0) {
          testFileName = `${fileName} copy`;
        } else {
          testFileName = `${fileName} copy ${counter}`;
        }
        counter++;
      } catch {
        return testFileName;
      }
    }
  }
  async convertBatch(items, dest, settings) {
    var _a, _b, _c, _d, _e, _f;
    try {
      this.status = "processing";
      console.time("batchProcess");
      eagle.log.info(`start batch converting with settings:`, settings);
      this.currentConversionTask = fileConverter;
      const validItems = items.filter((item) => {
        const isSupported = this.suppertedFileTypes.includes(item.ext.toLowerCase());
        if (!isSupported) {
          console.warn(
            `[Main.convertBatch] Unsupported format: ${item.ext} for ${item.name}`
          );
          if (this.taskManager) {
            this.taskManager.updateTaskStatus(item.id, "warn", 0, "Unsupported format");
          }
          return false;
        }
        return true;
      });
      if (validItems.length === 0) {
        console.log("[Main.convertBatch] No valid items to convert");
        (_b = (_a = settings.callbacks) == null ? void 0 : _a.onComplete) == null ? void 0 : _b.call(_a, {
          successfulTasks: [],
          failedTasks: items.map((item) => ({
            src: item.filePath,
            error: i18next.t("status.unsupportedFormat"),
            itemId: item.id
          })),
          statistics: {
            total: items.length,
            succeeded: 0,
            failed: items.length,
            skipped: 0
          }
        });
        return;
      }
      if (this.taskManager) {
        validItems.forEach((item) => {
          this.taskManager.updateTaskStatus(item.id, "processing", 0);
        });
      }
      const fileNameAllocator = new FileNameAllocator();
      const tasksForAllocation = validItems.map((item) => ({
        id: item.id,
        fileName: item.name || item.filePath.split("/").pop().split("\\").pop().split(".").shift(),
        name: item.name,
        format: settings.format === "original" ? item.ext : settings.format,
        ext: item.ext
      }));
      const allocatedNames = fileNameAllocator.allocateFileNames(tasksForAllocation, {
        nameType: settings.nameType,
        customName: settings.newFileName,
        startNumber: parseInt(settings.startNumber) || 1
      });
      const nameMap = /* @__PURE__ */ new Map();
      allocatedNames.forEach((allocation) => {
        nameMap.set(allocation.id, allocation.exportName);
      });
      console.log("[Main.convertBatch] File names allocated:", fileNameAllocator.getStats());
      if (settings.keepBothMode || settings.runtimeConflictAction === "keepBoth") {
        console.log("[Main.convertBatch] KeepBoth mode detected, checking for conflicts...");
        for (const item of validItems) {
          const originalName = nameMap.get(item.id);
          if (originalName) {
            const format = settings.format === "original" ? item.ext : settings.format;
            const uniqueName = await this.generateUniqueFileName(dest, originalName, format);
            if (uniqueName !== originalName) {
              console.log(`[Main.convertBatch] Conflict detected for "${originalName}", using "${uniqueName}"`);
              nameMap.set(item.id, uniqueName);
            }
          }
        }
      }
      const tasks = validItems.map((item, index) => {
        const fileName = nameMap.get(item.id) || `unnamed_${index}`;
        return {
          src: item.filePath,
          itemId: item.id,
          // 保留 ID 以便後續查找
          options: {
            output: dest,
            format: settings.format === "original" ? item.filePath.split(".").pop() : settings.format,
            quality: settings.quality,
            codec: settings.codec,
            animatedFps: settings.animatedFps,
            // 新增：傳遞動畫 FPS 參數
            sizeType: settings.sizeType,
            sizeValue: settings.sizeValue,
            fileName,
            keepBothMode: settings.keepBothMode,
            // 新增：傳遞 keepBoth 標記
            runtimeConflictAction: settings.runtimeConflictAction,
            // 新增：傳遞執行時衝突處理動作
            usePreallocatedName: true
            // 標記使用預分配的檔名
            // isReplaceMode: settings.isReplaceMode, // 傳遞替換模式標記
          }
        };
      });
      let completedTasks = 0;
      const result = await fileConverter.convert(tasks, {
        onTaskComplete: async (taskResult) => {
          var _a2, _b2;
          try {
            if (this.taskManager) {
              const item = validItems.find((i) => i.filePath === taskResult.src);
              if (item) {
                const status = taskResult.success ? "success" : "failed";
                this.taskManager.updateTaskStatus(
                  item.id,
                  status,
                  taskResult.success ? 100 : 0,
                  taskResult.error
                );
                if (taskResult.success && taskResult.outputPath) {
                  const exportInfo = {
                    outputPath: taskResult.outputPath,
                    newFileName: taskResult.actualFileName
                    // 新增：導出的實際檔名
                  };
                  try {
                    if (item.width && item.height) {
                      exportInfo.originalDimensions = {
                        width: item.width,
                        height: item.height
                      };
                    }
                  } catch (error) {
                    console.warn(i18next.t("console.warnings.failedToGetDimensions"), error);
                  }
                  try {
                    const fs2 = require("fs");
                    const stats = await fs2.promises.stat(taskResult.outputPath);
                    exportInfo.convertedSize = stats.size;
                    const exportDimensions = await this.getImageDimensions(
                      taskResult.outputPath
                    );
                    if (exportDimensions) {
                      exportInfo.exportDimensions = exportDimensions;
                    }
                  } catch (error) {
                    console.warn(i18next.t("console.warnings.failedToGetExportInfo"), error);
                  }
                  this.taskManager.updateTaskExportInfo(item.id, exportInfo);
                }
                if (settings.isReplaceMode && taskResult.replacedInEagle) {
                  console.log(
                    "[Main.onTaskComplete] Updating task for replace mode:",
                    {
                      taskId: item.id,
                      oldPath: item.filePath,
                      newPath: taskResult.outputPath
                    }
                  );
                  const path2 = require("path");
                  const newFilePath = taskResult.outputPath;
                  const newExt = path2.extname(newFilePath).slice(1);
                  this.taskManager.updateTaskItemInfo(item.id, {
                    filePath: newFilePath,
                    ext: newExt
                  });
                  console.log("[Main.onTaskComplete] Task updated successfully");
                }
              } else {
                console.warn(
                  "[Main.onTaskComplete] Task item not found:",
                  taskResult.src
                );
              }
            } else {
              console.error(i18next.t("console.errors.taskComplete", { message: taskResult.error }), {
                itemId: taskResult.itemId,
                error: taskResult.error,
                src: taskResult.src
              });
            }
            completedTasks++;
            const enhancedTaskResult = {
              ...taskResult,
              currentFrame: completedTasks,
              totalFrames: validItems.length
            };
            (_b2 = (_a2 = settings.callbacks) == null ? void 0 : _a2.onTaskComplete) == null ? void 0 : _b2.call(_a2, enhancedTaskResult);
          } catch (error) {
            console.error(i18next.t("console.errors.taskCompleteError", { error: error.message }), {
              error: error.message,
              stack: error.stack,
              result: taskResult,
              settings: { isReplaceMode: settings.isReplaceMode }
            });
            throw error;
          }
        },
        onProgress: (_c = settings.callbacks) == null ? void 0 : _c.onProgress,
        onError: (_d = settings.callbacks) == null ? void 0 : _d.onError,
        onComplete: (_e = settings.callbacks) == null ? void 0 : _e.onComplete,
        onCancelled: (_f = settings.callbacks) == null ? void 0 : _f.onCancelled
      });
      eagle.log.info(`end batch converting`);
      console.timeEnd("batchProcess");
      this.status = "success";
      return result;
    } catch (error) {
      if (this.taskManager) {
        items.forEach((item) => {
          const task = this.taskManager.getTask(item.id);
          if (task && task.status === "processing") {
            this.taskManager.updateTaskStatus(item.id, "failed", 0, error.message);
          }
        });
      }
      eagle.log.error(error);
      this.status = "error";
      throw error;
    } finally {
      this.status = "";
      this.currentConversionTask = null;
    }
  }
  /**
   * Cancel current batch conversion
   */
  cancelConversion() {
    if (this.currentConversionTask) {
      console.log("[Main] Cancelling current conversion task");
      this.currentConversionTask.cancel();
      this.currentConversionTask = null;
      return true;
    }
    this.status = "cancelled";
    return false;
  }
  // 獲取圖片尺寸的輔助方法
  async getImageDimensions(filePath) {
    try {
      const path2 = require("path");
      const ext = path2.extname(filePath).toLowerCase().slice(1);
      const imageFormats = ["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg"];
      if (imageFormats.includes(ext)) {
        return new Promise((resolve) => {
          const img = new Image();
          img.onload = () => {
            resolve({ width: img.width, height: img.height });
          };
          img.onerror = () => {
            resolve(null);
          };
          img.src = filePath;
        });
      }
      if (eagle.ffmpeg && eagle.ffmpeg.ffprobe) {
        return new Promise((resolve) => {
          eagle.ffmpeg.ffprobe(filePath, (err, metadata) => {
            if (err || !metadata || !metadata.streams) {
              resolve(null);
              return;
            }
            const videoStream = metadata.streams.find((s) => s.codec_type === "video");
            if (videoStream && videoStream.width && videoStream.height) {
              resolve({ width: videoStream.width, height: videoStream.height });
            } else {
              resolve(null);
            }
          });
        });
      }
      return null;
    } catch (error) {
      console.error(i18next.t("console.errors.gettingImageDimensions", { error: error.message }));
      return null;
    }
  }
}
class FormatValidator {
  constructor() {
    const mainInstance = new Main();
    this.supportedFormats = mainInstance.suppertedFileTypes || [
      "bmp",
      "exr",
      "gif",
      "hdr",
      "heic",
      "heif",
      "hif",
      "ico",
      "jpeg",
      "jpg",
      "jfif",
      "png",
      "svg",
      "tga",
      "tif",
      "tiff",
      "webp",
      "avif",
      "insp",
      "jxl",
      "jpe",
      "dds",
      "mp4",
      "webm",
      "mov",
      "m4v",
      "mkv"
    ];
    this.supportedExportFormats = [
      "jpg",
      "png",
      "bmp",
      "gif",
      "tif",
      "tiff",
      "ico",
      "webp",
      "avif",
      "hdr",
      "exr",
      "tga",
      "mp4",
      "webm"
    ];
    this.videoFormats = ["mp4", "mov", "webm", "mkv", "m4v"];
    this.imageFormats = this.supportedFormats.filter(
      (format) => !this.videoFormats.includes(format)
    );
    this.errorMessages = {
      fileNotSupported: "error.fileNotSupported",
      formatInvalid: "error.formatInvalid"
    };
  }
  /**
   * 檢查檔案格式是否支援
   * @param {string} extension - 檔案副檔名 (可包含或不包含點號)
   * @returns {boolean} 是否支援該格式
   */
  isFormatSupported(extension) {
    if (!extension) return false;
    const normalizedExt = extension.toLowerCase().replace(/^\./, "");
    return this.supportedFormats.includes(normalizedExt);
  }
  /**
   * 檢查是否為影片格式
   * @param {string} extension - 檔案副檔名
   * @returns {boolean} 是否為影片格式
   */
  isVideoFormat(extension) {
    if (!extension) return false;
    const normalizedExt = extension.toLowerCase().replace(/^\./, "");
    return this.videoFormats.includes(normalizedExt);
  }
  /**
   * 檢查是否為圖片格式
   * @param {string} extension - 檔案副檔名
   * @returns {boolean} 是否為圖片格式
   */
  isImageFormat(extension) {
    if (!extension) return false;
    const normalizedExt = extension.toLowerCase().replace(/^\./, "");
    return this.imageFormats.includes(normalizedExt);
  }
  /**
   * 獲取格式不支援的錯誤信息鍵值
   * @returns {string} i18n 錯誤信息鍵值
   */
  getUnsupportedFormatError() {
    return this.errorMessages.fileNotSupported;
  }
  /**
   * 獲取支援的格式列表
   * @returns {Array<string>} 支援的格式陣列
   */
  getSupportedFormats() {
    return [...this.supportedFormats];
  }
  /**
   * 靜態方法：快速檢查格式支援 (便於直接調用)
   * @param {string} extension - 檔案副檔名
   * @returns {boolean} 是否支援該格式
   */
  static isSupported(extension) {
    const validator = new FormatValidator();
    return validator.isFormatSupported(extension);
  }
  /**
   * 驗證任務物件的格式支援狀態
   * @param {Object} item - 任務項目物件
   * @param {string} targetFormat - 目標導出格式 (可選)
   * @returns {Object} 驗證結果 { isSupported, errorType, errorMessage, status }
   */
  validateTaskFormat(item, targetFormat = null) {
    if (!item || !item.ext) {
      return {
        isSupported: false,
        status: "error",
        errorType: "format",
        errorMessage: this.errorMessages.formatInvalid
      };
    }
    const isSupported = this.isFormatSupported(item.ext);
    if (!isSupported) {
      return {
        isSupported: false,
        status: "error",
        errorType: "format",
        errorMessage: this.errorMessages.fileNotSupported
      };
    }
    if (targetFormat) {
      const sourceIsVideo = this.isVideoFormat(item.ext);
      const targetIsImage = this.isImageFormat(targetFormat);
      if (sourceIsVideo && targetIsImage) {
        return {
          isSupported: false,
          status: "warn",
          errorType: "format",
          errorMessage: this.errorMessages.fileNotSupported
        };
      }
      targetFormat = targetFormat === "original" ? item.ext : targetFormat;
      if (!this.supportedExportFormats.includes(targetFormat)) {
        return {
          isSupported: false,
          status: "warn",
          errorType: "format",
          errorMessage: this.errorMessages.fileNotSupported
        };
      }
    }
    return {
      isSupported: true,
      status: "ok",
      errorType: null,
      errorMessage: null
    };
  }
  /**
   * 獲取格式驗證統計資訊
   * @param {Array} items - 項目陣列
   * @param {string} targetFormat - 目標導出格式 (可選)
   * @returns {Object} 統計資訊
   */
  getValidationStats(items, targetFormat = null) {
    if (!Array.isArray(items)) return { supported: 0, unsupported: 0, warnings: 0, total: 0 };
    const stats = items.reduce(
      (acc, item) => {
        const validation = this.validateTaskFormat(item, targetFormat);
        if (validation.status === "ok") {
          acc.supported++;
        } else if (validation.status === "warn") {
          acc.warnings++;
        } else {
          acc.unsupported++;
        }
        acc.total++;
        return acc;
      },
      { supported: 0, unsupported: 0, warnings: 0, total: 0 }
    );
    return stats;
  }
}
const formatValidator = new FormatValidator();
class TaskValidator {
  constructor() {
    this.sizeValidationStrategies = {
      // WebP/AVIF：檢查單邊超限
      webp: this._createSideLimitStrategy(16384),
      avif: this._createSideLimitStrategy(16384),
      // ICO：不判斷，因為超過我會幫她導出 256x256
      ico: () => ({ isValid: true }),
      // 其他格式：檢查總像素超限  
      default: this._createPixelLimitStrategy(16384)
    };
    this.ERROR_TYPES = {
      FORMAT: "format",
      SIZE: "size",
      ANIMATION: "animation",
      INVALID: "invalid"
    };
  }
  /**
   * 創建單邊限制策略
   * @private
   */
  _createSideLimitStrategy(maxSide) {
    return (width, height) => {
      if (width > maxSide || height > maxSide) {
        return {
          isValid: false,
          exceedsWidth: width > maxSide,
          exceedsHeight: height > maxSide,
          limit: { maxWidth: maxSide, maxHeight: maxSide }
        };
      }
      return { isValid: true };
    };
  }
  /**
   * 創建總像素限制策略
   * @private
   */
  _createPixelLimitStrategy(maxSide) {
    const maxPixels = maxSide * maxSide;
    return (width, height) => {
      const totalPixels = width * height;
      if (totalPixels > maxPixels) {
        return {
          isValid: false,
          exceedsWidth: false,
          exceedsHeight: false,
          limit: { maxPixels }
        };
      }
      return { isValid: true };
    };
  }
  /**
   * 完整驗證任務項目（格式 + 尺寸）
   * @param {Object} item - Eagle 項目物件
   * @param {Object} exportSettings - 導出設定
   * @returns {Object} 完整驗證結果
   */
  validateTask(item, exportSettings) {
    const formatValidation = this._validateFormat(item, exportSettings.format);
    if (!formatValidation.isValid) {
      return {
        isValid: false,
        errorType: formatValidation.errorType,
        errorMessage: formatValidation.errorMessage,
        validationDetails: {
          format: formatValidation,
          size: { isValid: true },
          // 格式不支援時跳過尺寸檢查
          animation: { isValid: true }
          // 格式不支援時跳過動畫檢查
        }
      };
    }
    const sizeValidation = this._validateSize(item, exportSettings);
    const isValid = formatValidation.isValid && sizeValidation.isValid;
    const errorInfo = !sizeValidation.isValid ? sizeValidation : formatValidation;
    return {
      isValid,
      errorType: errorInfo.errorType,
      errorMessage: errorInfo.errorMessage,
      validationDetails: {
        format: formatValidation,
        size: sizeValidation
      }
    };
  }
  /**
   * 批次驗證多個任務
   * @param {Array} items - 項目陣列
   * @param {Object} exportSettings - 導出設定
   * @returns {Array} 驗證結果陣列
   */
  validateTasks(items, exportSettings) {
    if (!Array.isArray(items)) return [];
    return items.map((item) => ({
      itemId: item.id,
      validation: this.validateTask(item, exportSettings)
    }));
  }
  /**
   * 檢查設定變更是否需要重新驗證
   * @param {Object} oldSettings - 舊設定
   * @param {Object} newSettings - 新設定  
   * @returns {boolean} 是否需要重新驗證
   */
  needsRevalidation(oldSettings, newSettings) {
    const criticalSettings = ["format", "sizeType", "sizeValue"];
    return criticalSettings.some(
      (key) => (oldSettings == null ? void 0 : oldSettings[key]) !== (newSettings == null ? void 0 : newSettings[key])
    );
  }
  /**
   * 獲取驗證統計
   * @param {Array} validationResults - 驗證結果陣列
   * @returns {Object} 統計信息
   */
  getValidationStats(validationResults) {
    if (!Array.isArray(validationResults)) return { valid: 0, invalid: 0, total: 0 };
    const stats = validationResults.reduce((acc, result) => {
      const validation = result.validation || result;
      if (validation.isValid) {
        acc.valid++;
      } else {
        acc.invalid++;
        acc.errorTypes[validation.errorType] = (acc.errorTypes[validation.errorType] || 0) + 1;
      }
      acc.total++;
      return acc;
    }, {
      valid: 0,
      invalid: 0,
      total: 0,
      errorTypes: {}
    });
    return {
      ...stats,
      validPercentage: stats.total > 0 ? Math.round(stats.valid / stats.total * 100) : 0,
      invalidPercentage: stats.total > 0 ? Math.round(stats.invalid / stats.total * 100) : 0
    };
  }
  // === 私有方法 ===
  /**
   * 驗證檔案格式
   * @private
   */
  _validateFormat(item, targetFormat = null) {
    const validation = formatValidator.validateTaskFormat(item, targetFormat);
    return {
      isValid: validation.isSupported,
      errorType: validation.errorType,
      errorMessage: validation.errorMessage
    };
  }
  /**
   * 驗證尺寸限制
   * @private  
   */
  _validateSize(item, exportSettings) {
    var _a;
    if (!item || !exportSettings) {
      return { isValid: true };
    }
    const exportDimensions = calculateItemDimensions(
      item,
      exportSettings.sizeType,
      exportSettings.sizeValue
    );
    if (!exportDimensions || !exportDimensions.width && !exportDimensions.height) {
      return { isValid: true };
    }
    const targetFormat = (_a = exportSettings.format) == null ? void 0 : _a.toLowerCase();
    const strategy = this.sizeValidationStrategies[targetFormat] || this.sizeValidationStrategies.default;
    const result = strategy(exportDimensions.width, exportDimensions.height);
    if (!result.isValid) {
      return {
        isValid: false,
        errorType: this.ERROR_TYPES.SIZE,
        errorMessage: "main.status.imageTooLarge",
        sizeInfo: {
          current: exportDimensions,
          limit: result.limit,
          exceeds: { width: result.exceedsWidth, height: result.exceedsHeight }
        }
      };
    }
    return { isValid: true };
  }
}
const taskValidator = new TaskValidator();
class TaskItem {
  constructor(rawTask, exportSettings, index = 0, translate = null) {
    this._raw = rawTask;
    this._settings = exportSettings;
    this._index = index;
    this._translate = translate;
    this.id = rawTask.id;
    this.item = rawTask.item;
    this.status = rawTask.status;
    this.exportPath = rawTask.exportPath;
    this.newFileName = rawTask.newFileName;
    this.newFormat = rawTask.newFormat;
    this.thumbnailUrl = rawTask.thumbnailUrl;
    this.handleNameClick = this.handleNameClick.bind(this);
    this.handleThumbnailError = this.handleThumbnailError.bind(this);
  }
  // Status computations - eliminate scattered checks
  get isError() {
    return this.status && ["warn", "failed"].includes(this.status);
  }
  get isSuccess() {
    return this.status === "success" && this.exportPath;
  }
  get isWaiting() {
    return this.status === "waiting";
  }
  // Status display properties
  get statusClass() {
    return [`icon-${this.status}`, "status"];
  }
  get statusMessage() {
    if (!this._translate) return "";
    if (this.status === "warn" || this.status === "failed") {
      return this._translate(this._raw.errorMessage);
    }
    return this._translate(`main.taskList.statusLabels.${this.status}`);
  }
  // Name display logic - consolidated from original displayName function
  get displayName() {
    var _a, _b, _c;
    const isError = this.isError;
    const useCustomName = ((_a = this._settings) == null ? void 0 : _a.nameType) === "custom";
    if (!isError && useCustomName) {
      return `${this._settings.newFileName}${this._settings.startNumber + this._index}`;
    }
    return isError ? (_b = this.item) == null ? void 0 : _b.name : this.newFileName || ((_c = this.item) == null ? void 0 : _c.name);
  }
  get nameClass() {
    return { "filename-link": this.isSuccess };
  }
  // Thumbnail properties
  get thumbnailStyle() {
    var _a, _b;
    if (!((_a = this.item) == null ? void 0 : _a.width) || !((_b = this.item) == null ? void 0 : _b.height)) return {};
    return { "aspect-ratio": `${this.item.width}/${this.item.height}` };
  }
  get showThumbnailPlaceholder() {
    return !this.thumbnailUrl;
  }
  // Format display logic
  get originalFormatText() {
    var _a;
    return this.formatText((_a = this.item) == null ? void 0 : _a.ext);
  }
  get newFormatText() {
    var _a, _b, _c;
    if (this.isError) return "-";
    const targetFormat = ((_b = (_a = this._settings) == null ? void 0 : _a.format) == null ? void 0 : _b.toLowerCase()) || this.newFormat;
    return targetFormat === "original" ? this.formatText((_c = this.item) == null ? void 0 : _c.ext) : this.formatText(targetFormat);
  }
  get formatStyle() {
    return {
      color: this.isError ? "var(--color-warning)" : ""
    };
  }
  get formatMessage() {
    if (!this.isError || !this._translate) return "";
    return this.status === "warn" ? this._translate("error.fileNotSupported") : this._translate(`main.taskList.statusLabels.${this.status}`);
  }
  get newFormatClass() {
    var _a, _b, _c, _d, _e;
    const classes = [];
    if (this.isError) classes.push("empty");
    const originalFormat = (_b = (_a = this.item) == null ? void 0 : _a.ext) == null ? void 0 : _b.toLowerCase();
    const targetFormat = ((_d = (_c = this._settings) == null ? void 0 : _c.format) == null ? void 0 : _d.toLowerCase()) || ((_e = this.newFormat) == null ? void 0 : _e.toLowerCase());
    if (targetFormat !== "original" && originalFormat && targetFormat && originalFormat !== targetFormat) {
      classes.push("format-changed");
    }
    return classes;
  }
  get showFormatArrow() {
    var _a;
    return ((_a = this._settings) == null ? void 0 : _a.format) !== "original";
  }
  // Dimensions logic
  get exportDimensions() {
    var _a, _b;
    return calculateItemDimensions(
      this.item,
      (_a = this._settings) == null ? void 0 : _a.sizeType,
      (_b = this._settings) == null ? void 0 : _b.sizeValue
    );
  }
  get showDimensionArrow() {
    var _a;
    return ((_a = this._settings) == null ? void 0 : _a.sizeType) !== "original";
  }
  get dimensionsDisplay() {
    if (this.isError) {
      return { original: "-", export: "-" };
    }
    const original = this.item ? {
      width: this.item.width,
      height: this.item.height
    } : { width: "-", height: "-" };
    const exportDims = this.exportDimensions;
    return {
      original,
      export: exportDims
    };
  }
  // Helper methods
  formatText(ext) {
    if (!ext || !this._translate) return "";
    const key = `main.itemExportFormat.format-options.${ext.toLowerCase()}`;
    const translation = this._translate(key);
    return translation === key ? ext.toUpperCase() : translation;
  }
  // Action handlers - methods, not getters (actions vs display)
  async handleNameClick() {
    if (this.status !== "success" || !this.exportPath) return;
    try {
      await eagle.shell.showItemInFolder(this.exportPath);
    } catch (error) {
      console.error("Failed to show item in folder:", error);
    }
  }
  handleThumbnailError(event) {
    console.warn("Thumbnail failed to load:", event.target.src);
    return { showPlaceholder: true };
  }
  // Static factory method for batch creation
  static fromRawTasks(rawTasks, exportSettings, translate) {
    return rawTasks.map(
      (task, index) => new TaskItem(task, exportSettings, index, translate)
    );
  }
  // 驗證方法 - 使用統一的驗證器
  validate(exportSettings) {
    return taskValidator.validateTask(this.item, exportSettings);
  }
  // 檢查是否需要重新驗證
  needsRevalidation(oldSettings, newSettings) {
    return taskValidator.needsRevalidation(oldSettings, newSettings);
  }
  // Debug info - useful for development
  toDebugString() {
    return `TaskItem(${this.id}, ${this.status}, ${this.displayName})`;
  }
}
const _hoisted_1$p = ["onClick"];
const _hoisted_2$i = { class: "format-cell" };
const _sfc_main$I = {
  __name: "TaskListVue",
  setup(__props, { expose: __expose }) {
    const main = inject("main");
    const exportSettings = inject("exportSettings");
    const taskManager = inject("taskManager");
    const { proxy } = getCurrentInstance();
    const $translate = proxy.$translate;
    const tableHeaders = {
      status: { name: "", minWidth: 36 },
      thumbnail: { name: "", minWidth: 32 },
      name: { name: "", minWidth: 200, fill: true, line: true, padding: "8px" },
      format: { name: "", minWidth: 120, line: true, padding: "8px" },
      dimensions: { name: "", minWidth: 240, line: true, padding: "8px" },
      remove: { name: "", minWidth: 36, align: "center" }
    };
    const displayTasks = computed(
      () => TaskItem.fromRawTasks(taskManager.getFormattedTasks.value, exportSettings.value, $translate)
    );
    const hasAnyTasks = computed(
      () => displayTasks.value.length > 0 || taskManager.isLoadingTasks.value
    );
    const isEmpty = computed(
      () => displayTasks.value.length === 0 && !taskManager.isLoadingTasks.value
    );
    const addNewItems = (items) => {
      if (!(items == null ? void 0 : items.length)) return;
      taskManager.addTasksProgressively(items, {
        batchDelay: 16
        // 60fps
      });
    };
    watch(
      () => main == null ? void 0 : main.items,
      () => {
        addNewItems(main.items);
      },
      { immediate: true }
    );
    __expose({
      // Direct access to task manager methods
      tasks: taskManager.tasks,
      taskStats: taskManager.taskStats,
      clearAllTasks: taskManager.clearAllTasks,
      clearCompletedTasks: taskManager.clearCompletedTasks,
      removeTask: taskManager.removeTask,
      resetTaskStatus: taskManager.resetTaskStatus
    });
    return (_ctx, _cache) => {
      const _component_el_icon = ElIcon;
      const _component_el_empty = ElEmpty;
      const _directive_tippy = resolveDirective("tippy");
      return openBlock(), createElementBlock(Fragment, null, [
        withDirectives(createVNode(TableVue, {
          header: tableHeaders,
          data: displayTasks.value,
          class: "v-table"
        }, {
          status: withCtx(({ row }) => [
            withDirectives(createVNode(_component_el_icon, {
              class: normalizeClass(row.statusClass)
            }, null, 8, ["class"]), [
              [_directive_tippy, {
                content: row.statusMessage,
                delay: [200, 0],
                duration: [150, 0]
              }]
            ])
          ]),
          thumbnail: withCtx(({ row }) => [
            createVNode(TaskThumbnail, { task: row }, null, 8, ["task"])
          ]),
          name: withCtx(({ row }) => [
            createBaseVNode("span", {
              class: normalizeClass(["text-ellipsis filename-text", row.nameClass]),
              onClick: row.handleNameClick
            }, toDisplayString(row.displayName), 11, _hoisted_1$p)
          ]),
          format: withCtx(({ row }) => [
            createBaseVNode("div", _hoisted_2$i, [
              withDirectives((openBlock(), createElementBlock("span", {
                class: "original-format",
                style: normalizeStyle(row.formatStyle)
              }, [
                createTextVNode(toDisplayString(row.originalFormatText), 1)
              ], 4)), [
                [_directive_tippy, {
                  content: row.formatMessage,
                  show: row.isError,
                  delay: [200, 0],
                  duration: [150, 0]
                }]
              ]),
              row.showFormatArrow ? (openBlock(), createElementBlock(Fragment, { key: 0 }, [
                createVNode(_component_el_icon, { class: "icon-arrow" }),
                createBaseVNode("span", {
                  class: normalizeClass(["new-format", row.newFormatClass])
                }, toDisplayString(row.newFormatText), 3)
              ], 64)) : createCommentVNode("", true)
            ])
          ]),
          dimensions: withCtx(({ row }) => [
            createVNode(DimensionDisplay, { task: row }, null, 8, ["task"])
          ]),
          remove: withCtx(({ row }) => [
            createVNode(TaskActions, {
              task: row,
              onRemove: unref(taskManager).removeTask
            }, null, 8, ["task", "onRemove"])
          ]),
          _: 1
        }, 8, ["data"]), [
          [vShow, hasAnyTasks.value]
        ]),
        withDirectives(createVNode(_component_el_empty, {
          description: unref($translate)("main.taskList.emptyText"),
          "image-size": 128
        }, {
          image: withCtx(() => [
            createVNode(_component_el_icon, { class: "icon icon-empty" })
          ]),
          default: withCtx(() => [
            createTextVNode(" " + toDisplayString(unref($translate)("main.taskList.emptyText")), 1)
          ]),
          _: 1
        }, 8, ["description"]), [
          [vShow, isEmpty.value]
        ])
      ], 64);
    };
  }
};
const TaskListVue = /* @__PURE__ */ _export_sfc(_sfc_main$I, [["__scopeId", "data-v-24bb6985"]]);
function useTabNavigation(containerRef) {
  const focusableElements = ref([]);
  const currentFocusIndex = ref(-1);
  const FOCUSABLE_SELECTOR = `
    button:not([disabled]):not([tabindex="-1"]),
    input:not([disabled]):not([tabindex="-1"]),
    select:not([disabled]):not([tabindex="-1"]),
    textarea:not([disabled]):not([tabindex="-1"]),
    [tabindex]:not([tabindex="-1"]):not([disabled])
  `.trim();
  const getFocusableElements = () => {
    if (!containerRef.value) return [];
    const elements = Array.from(containerRef.value.querySelectorAll(FOCUSABLE_SELECTOR));
    return elements.sort((a, b) => {
      const aIndex = parseInt(a.getAttribute("tabindex") || "0");
      const bIndex = parseInt(b.getAttribute("tabindex") || "0");
      if (aIndex > 0 && bIndex > 0) {
        return aIndex - bIndex;
      }
      if (aIndex > 0) return -1;
      if (bIndex > 0) return 1;
      return a.compareDocumentPosition(b) & 2 ? 1 : -1;
    });
  };
  const updateFocusableElements = () => {
    focusableElements.value = getFocusableElements();
  };
  const handleTabKey = (event) => {
    var _a, _b;
    const elements = getFocusableElements();
    if (elements.length === 0) return;
    event.preventDefault();
    event.stopPropagation();
    const activeElement = document.activeElement;
    let currentIndex = elements.findIndex((el) => el === activeElement || el.contains(activeElement));
    if (currentIndex === -1) {
      if (event.shiftKey) {
        currentIndex = elements.length - 1;
      } else {
        currentIndex = 0;
      }
      currentFocusIndex.value = currentIndex;
      (_a = elements[currentIndex]) == null ? void 0 : _a.focus();
      return;
    }
    if (event.shiftKey) {
      currentIndex = currentIndex <= 0 ? elements.length - 1 : currentIndex - 1;
    } else {
      currentIndex = (currentIndex + 1) % elements.length;
    }
    currentFocusIndex.value = currentIndex;
    (_b = elements[currentIndex]) == null ? void 0 : _b.focus();
  };
  const focusFirst = () => {
    const elements = getFocusableElements();
    if (elements.length > 0) {
      elements[0].focus();
      currentFocusIndex.value = 0;
    }
  };
  const focusLast = () => {
    const elements = getFocusableElements();
    if (elements.length > 0) {
      elements[elements.length - 1].focus();
      currentFocusIndex.value = elements.length - 1;
    }
  };
  const trapFocus = () => {
    var _a;
    const elements = getFocusableElements();
    if (elements.length === 0) return;
    const firstElement = elements[0];
    elements[elements.length - 1];
    if (!((_a = containerRef.value) == null ? void 0 : _a.contains(document.activeElement))) {
      firstElement == null ? void 0 : firstElement.focus();
    }
  };
  let observer = null;
  onMounted(() => {
    updateFocusableElements();
    if (containerRef.value) {
      observer = new MutationObserver(() => {
        updateFocusableElements();
      });
      observer.observe(containerRef.value, {
        childList: true,
        subtree: true,
        attributes: true,
        attributeFilter: ["disabled", "tabindex"]
      });
    }
  });
  onUnmounted(() => {
    if (observer) {
      observer.disconnect();
      observer = null;
    }
  });
  return {
    handleTabKey,
    getFocusableElements,
    updateFocusableElements,
    currentFocusIndex,
    focusableElements,
    focusFirst,
    focusLast,
    trapFocus
  };
}
const _imports_0 = "" + new URL("../../logo.png", import.meta.url).href;
const _hoisted_1$o = {
  key: 0,
  class: "custom-dialog-wrapper"
};
const _sfc_main$H = {
  __name: "CustomDialogContent",
  props: {
    modelValue: {
      type: Boolean,
      default: false
    },
    container: {
      type: [HTMLElement, String, null],
      default: null
    },
    class: {
      type: String,
      default: ""
    }
  },
  emits: ["update:modelValue"],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emit = __emit;
    const isOpen = computed({
      get: () => props.modelValue,
      set: (val) => emit("update:modelValue", val)
    });
    const closeDialog = inject("closeDialog", null);
    const handleClickOutside = (e) => {
      if (e.target.classList.contains("custom-dialog-overlay")) {
        closeDialog();
      }
    };
    return (_ctx, _cache) => {
      return openBlock(), createBlock(Teleport, {
        to: __props.container || "body"
      }, [
        createVNode(Transition, { name: "dialog-wrapper" }, {
          default: withCtx(() => [
            isOpen.value ? (openBlock(), createElementBlock("div", _hoisted_1$o, [
              createBaseVNode("div", {
                class: "custom-dialog-overlay",
                onClick: handleClickOutside
              }),
              createBaseVNode("div", {
                class: normalizeClass(["custom-dialog-content", props.class])
              }, [
                renderSlot(_ctx.$slots, "default", {}, void 0, true)
              ], 2)
            ])) : createCommentVNode("", true)
          ]),
          _: 3
        })
      ], 8, ["to"]);
    };
  }
};
const CustomDialogContent = /* @__PURE__ */ _export_sfc(_sfc_main$H, [["__scopeId", "data-v-c49a1995"]]);
function cn(...inputs) {
  return twMerge(clsx(inputs));
}
const _sfc_main$G = {
  __name: "Button",
  props: {
    variant: { type: null, required: false },
    size: { type: null, required: false },
    class: { type: null, required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false, default: "button" }
  },
  setup(__props) {
    const props = __props;
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(Primitive), {
        "data-slot": "button",
        as: __props.as,
        "as-child": __props.asChild,
        class: normalizeClass(unref(cn)(unref(buttonVariants)({ variant: __props.variant, size: __props.size }), props.class))
      }, {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 8, ["as", "as-child", "class"]);
    };
  }
};
const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground hover:bg-primary/90",
        destructive: "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
        outline: "border bg-background hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50",
        secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ghost: "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
        link: "text-primary underline-offset-4 hover:underline"
      },
      size: {
        default: "h-9 px-4 py-2 has-[>svg]:px-3",
        sm: "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5",
        lg: "h-10 rounded-md px-6 has-[>svg]:px-4",
        icon: "size-9"
      }
    },
    defaultVariants: {
      variant: "default",
      size: "default"
    }
  }
);
const _sfc_main$F = {
  __name: "Combobox",
  props: {
    open: { type: Boolean, required: false },
    defaultOpen: { type: Boolean, required: false },
    resetSearchTermOnBlur: { type: Boolean, required: false },
    resetSearchTermOnSelect: { type: Boolean, required: false },
    openOnFocus: { type: Boolean, required: false },
    openOnClick: { type: Boolean, required: false },
    ignoreFilter: { type: Boolean, required: false },
    modelValue: { type: null, required: false },
    defaultValue: { type: null, required: false },
    multiple: { type: Boolean, required: false },
    dir: { type: String, required: false },
    disabled: { type: Boolean, required: false },
    highlightOnHover: { type: Boolean, required: false },
    by: { type: [String, Function], required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    name: { type: String, required: false },
    required: { type: Boolean, required: false }
  },
  emits: ["update:modelValue", "highlight", "update:open"],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emits = __emit;
    const forwarded = useForwardPropsEmits(props, emits);
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ComboboxRoot_default), mergeProps({ "data-slot": "combobox" }, unref(forwarded)), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16);
    };
  }
};
const _sfc_main$E = {
  __name: "ComboboxAnchor",
  props: {
    reference: { type: null, required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false }
  },
  setup(__props) {
    const props = __props;
    const delegatedProps = reactiveOmit(props, "class");
    const forwarded = useForwardProps(delegatedProps);
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ComboboxAnchor_default), mergeProps({ "data-slot": "combobox-anchor" }, unref(forwarded), {
        class: unref(cn)(props.class),
        style: { "height": "100%" }
      }), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16, ["class"]);
    };
  }
};
const _sfc_main$D = {
  __name: "ComboboxEmpty",
  props: {
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false }
  },
  setup(__props) {
    const props = __props;
    const delegatedProps = reactiveOmit(props, "class");
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ComboboxEmpty_default), mergeProps({ "data-slot": "combobox-empty" }, unref(delegatedProps), {
        class: unref(cn)("py-6 text-center text-sm", props.class)
      }), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16, ["class"]);
    };
  }
};
const _sfc_main$C = {
  __name: "ComboboxGroup",
  props: {
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false },
    heading: { type: String, required: false }
  },
  setup(__props) {
    const props = __props;
    const delegatedProps = reactiveOmit(props, "class");
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ComboboxGroup_default), mergeProps({ "data-slot": "combobox-group" }, unref(delegatedProps), {
        class: unref(cn)("overflow-hidden p-1 text-foreground", props.class)
      }), {
        default: withCtx(() => [
          __props.heading ? (openBlock(), createBlock(unref(ComboboxLabel_default), {
            key: 0,
            class: "px-2 py-1.5 text-xs font-medium text-muted-foreground"
          }, {
            default: withCtx(() => [
              createTextVNode(toDisplayString(__props.heading), 1)
            ]),
            _: 1
          })) : createCommentVNode("", true),
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16, ["class"]);
    };
  }
};
const _hoisted_1$n = {
  "data-slot": "command-input-wrapper",
  class: "flex h-9 items-center gap-2",
  style: { "height": "inherit", "padding-left": "10px" }
};
const _sfc_main$B = /* @__PURE__ */ Object.assign({
  inheritAttrs: false
}, {
  __name: "ComboboxInput",
  props: {
    displayValue: { type: Function, required: false },
    modelValue: { type: String, required: false },
    autoFocus: { type: Boolean, required: false },
    disabled: { type: Boolean, required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false },
    showSearchIcon: { type: Boolean, default: true }
  },
  emits: ["update:modelValue"],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emits = __emit;
    const delegatedProps = reactiveOmit(props, "class");
    const forwarded = useForwardPropsEmits(delegatedProps, emits);
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$n, [
        props.showSearchIcon ? (openBlock(), createBlock(unref(Search), {
          key: 0,
          class: "size-4 shrink-0 opacity-50"
        })) : createCommentVNode("", true),
        createVNode(unref(ComboboxInput_default), mergeProps({
          "data-slot": "command-input",
          class: unref(cn)(
            "placeholder:text-muted-foreground flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50",
            props.class
          )
        }, { ...unref(forwarded), ..._ctx.$attrs }), {
          default: withCtx(() => [
            renderSlot(_ctx.$slots, "default")
          ]),
          _: 3
        }, 16, ["class"])
      ]);
    };
  }
});
const _sfc_main$A = {
  __name: "ComboboxItem",
  props: {
    textValue: { type: String, required: false },
    value: { type: null, required: true },
    disabled: { type: Boolean, required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false }
  },
  emits: ["select"],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emits = __emit;
    const delegatedProps = reactiveOmit(props, "class");
    const forwarded = useForwardPropsEmits(delegatedProps, emits);
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ComboboxItem_default), mergeProps({ "data-slot": "combobox-item" }, unref(forwarded), {
        class: unref(cn)(
          `data-[highlighted]:bg-[var(--color-bg-hover)] data-[highlighted]:text-[var(--color-text-primary)] [&_svg:not([class*='text-'])]:text-muted-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4`,
          props.class
        )
      }), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16, ["class"]);
    };
  }
};
const _sfc_main$z = {
  __name: "ComboboxItemIndicator",
  props: {
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false }
  },
  setup(__props) {
    const props = __props;
    const delegatedProps = reactiveOmit(props, "class");
    const forwarded = useForwardProps(delegatedProps);
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ComboboxItemIndicator_default), mergeProps({ "data-slot": "combobox-item-indicator" }, unref(forwarded), {
        class: unref(cn)("ml-auto", props.class)
      }), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16, ["class"]);
    };
  }
};
const _sfc_main$y = {
  __name: "ComboboxList",
  props: {
    forceMount: { type: Boolean, required: false },
    position: { type: String, required: false, default: "popper" },
    bodyLock: { type: Boolean, required: false },
    side: { type: null, required: false },
    sideOffset: { type: Number, required: false, default: 4 },
    sideFlip: { type: Boolean, required: false },
    align: { type: null, required: false, default: "center" },
    alignOffset: { type: Number, required: false },
    alignFlip: { type: Boolean, required: false },
    avoidCollisions: { type: Boolean, required: false },
    collisionBoundary: { type: null, required: false },
    collisionPadding: { type: [Number, Object], required: false },
    arrowPadding: { type: Number, required: false },
    sticky: { type: String, required: false },
    hideWhenDetached: { type: Boolean, required: false },
    positionStrategy: { type: String, required: false },
    updatePositionStrategy: { type: String, required: false },
    disableUpdateOnLayoutShift: { type: Boolean, required: false },
    prioritizePosition: { type: Boolean, required: false },
    reference: { type: null, required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    disableOutsidePointerEvents: { type: Boolean, required: false },
    class: { type: null, required: false }
  },
  emits: [
    "escapeKeyDown",
    "pointerDownOutside",
    "focusOutside",
    "interactOutside"
  ],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emits = __emit;
    const delegatedProps = reactiveOmit(props, "class");
    const forwarded = useForwardPropsEmits(delegatedProps, emits);
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ComboboxPortal_default), null, {
        default: withCtx(() => [
          createVNode(unref(ComboboxContent_default), mergeProps({ "data-slot": "combobox-list" }, unref(forwarded), {
            class: unref(cn)(
              "z-50 rounded-md border bg-popover text-popover-foreground origin-(--reka-combobox-content-transform-origin) shadow-md outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
              props.class
            )
          }), {
            default: withCtx(() => [
              renderSlot(_ctx.$slots, "default", {}, void 0, true)
            ]),
            _: 3
          }, 16, ["class"])
        ]),
        _: 3
      });
    };
  }
};
const ComboboxList = /* @__PURE__ */ _export_sfc(_sfc_main$y, [["__scopeId", "data-v-8b09f78e"]]);
const _hoisted_1$m = {
  class: "relative w-full max-w-sm items-center",
  style: { "height": "inherit" }
};
const _hoisted_2$h = { key: 0 };
const _hoisted_3$b = {
  key: 1,
  class: "placeholder"
};
const _hoisted_4$8 = { class: "search-input-wrapper sticky top-0 z-10 bg-popover border-b border-border px-1 py-1" };
const _hoisted_5$5 = { class: "scrollable-content overflow-y-auto" };
const _hoisted_6$4 = { class: "pl-7 pr-4" };
const _hoisted_7$3 = { class: "pl-7 pr-4" };
const _sfc_main$x = {
  __name: "ComboBoxVue",
  props: {
    modelValue: {
      type: [String, Number, Object],
      default: null
    },
    options: {
      type: Array,
      required: true
    },
    placeholder: {
      type: String,
      default: ""
    },
    translatePrefix: {
      type: String,
      default: ""
    },
    tabindex: {
      type: [Number, String],
      default: 0
    }
  },
  emits: ["update:modelValue", "visible-change"],
  setup(__props, { emit: __emit }) {
    const { proxy } = getCurrentInstance();
    const $translate = proxy == null ? void 0 : proxy.$translate;
    const props = __props;
    const emit = __emit;
    const selectedValue = ref(props.modelValue);
    const searchTerm = ref("");
    watch(() => props.modelValue, (newValue) => {
      selectedValue.value = newValue;
    });
    const isOpen = ref(false);
    const handleOpenChange = (open) => {
      isOpen.value = open;
      if (open) {
        searchTerm.value = "";
      }
      emit("visible-change", open);
    };
    const handleKeyDown = (event) => {
      if (event.key === "ArrowDown" && !isOpen.value) {
        event.preventDefault();
        isOpen.value = true;
      }
    };
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(_sfc_main$F), {
        modelValue: selectedValue.value,
        "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => selectedValue.value = $event),
        "search-term": searchTerm.value,
        "onUpdate:searchTerm": _cache[1] || (_cache[1] = ($event) => searchTerm.value = $event),
        open: isOpen.value,
        "onUpdate:open": [
          _cache[2] || (_cache[2] = ($event) => isOpen.value = $event),
          handleOpenChange
        ]
      }, {
        default: withCtx(() => [
          createVNode(unref(_sfc_main$E), { "as-child": "" }, {
            default: withCtx(() => [
              createBaseVNode("div", _hoisted_1$m, [
                createVNode(unref(ComboboxTrigger_default), {
                  "as-child": "",
                  tabindex: __props.tabindex
                }, {
                  default: withCtx(() => [
                    createVNode(unref(_sfc_main$G), {
                      variant: "outline",
                      class: normalizeClass(["w-full justify-between combobox-button text-left", { "is-active": isOpen.value }]),
                      onKeydown: handleKeyDown
                    }, {
                      default: withCtx(() => [
                        selectedValue.value ? (openBlock(), createElementBlock("span", _hoisted_2$h, toDisplayString(unref($translate)(__props.translatePrefix + selectedValue.value)), 1)) : (openBlock(), createElementBlock("span", _hoisted_3$b, toDisplayString(__props.placeholder), 1)),
                        createVNode(unref(ChevronsUpDown), { class: "h-4 w-4 shrink-0 opacity-50" })
                      ]),
                      _: 1
                    }, 8, ["class"])
                  ]),
                  _: 1
                }, 8, ["tabindex"])
              ])
            ]),
            _: 1
          }),
          createVNode(unref(ComboboxList), {
            align: "end",
            "side-offset": "1",
            class: "w-max min-w-[180px] combobox-list combobox-list-container"
          }, {
            default: withCtx(() => [
              createBaseVNode("div", _hoisted_4$8, [
                createVNode(unref(_sfc_main$B), {
                  class: "focus-visible:ring-0 focus-visible:ring-offset-0 border-0 rounded-none h-10 w-full search-input",
                  placeholder: unref($translate)("main.SettingSidebar.combobox.searchPlaceholder"),
                  "display-value": () => ""
                }, null, 8, ["placeholder"])
              ]),
              createBaseVNode("div", _hoisted_5$5, [
                createVNode(unref(_sfc_main$D), null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(unref($translate)("main.SettingSidebar.combobox.noResults")), 1)
                  ]),
                  _: 1
                }),
                Array.isArray(__props.options) && __props.options.length > 0 && typeof __props.options[0] === "object" && __props.options[0].label ? (openBlock(true), createElementBlock(Fragment, { key: 0 }, renderList(__props.options, (group) => {
                  return openBlock(), createBlock(unref(_sfc_main$C), {
                    key: group.label,
                    class: "combobox-group"
                  }, {
                    default: withCtx(() => [
                      (openBlock(true), createElementBlock(Fragment, null, renderList(group.options, (option) => {
                        return openBlock(), createBlock(unref(_sfc_main$A), {
                          key: option,
                          value: option,
                          onSelect: () => {
                            selectedValue.value = option;
                            emit("update:modelValue", option);
                          },
                          class: "relative flex items-center whitespace-nowrap"
                        }, {
                          default: withCtx(() => [
                            createVNode(unref(_sfc_main$z), { class: "absolute left-2" }, {
                              default: withCtx(() => [
                                createVNode(unref(Check), {
                                  class: normalizeClass(unref(cn)("h-4 w-4"))
                                }, null, 8, ["class"])
                              ]),
                              _: 1
                            }),
                            createBaseVNode("span", _hoisted_6$4, toDisplayString(unref($translate)(__props.translatePrefix + option)), 1)
                          ]),
                          _: 2
                        }, 1032, ["value", "onSelect"]);
                      }), 128))
                    ]),
                    _: 2
                  }, 1024);
                }), 128)) : (openBlock(), createBlock(unref(_sfc_main$C), {
                  key: 1,
                  class: "combobox-group"
                }, {
                  default: withCtx(() => [
                    (openBlock(true), createElementBlock(Fragment, null, renderList(__props.options, (option) => {
                      return openBlock(), createBlock(unref(_sfc_main$A), {
                        key: option,
                        value: option,
                        onSelect: () => {
                          selectedValue.value = option;
                          emit("update:modelValue", option);
                        },
                        class: "relative flex items-center whitespace-nowrap"
                      }, {
                        default: withCtx(() => [
                          createVNode(unref(_sfc_main$z), { class: "absolute left-2" }, {
                            default: withCtx(() => [
                              createVNode(unref(Check), {
                                class: normalizeClass(unref(cn)("h-4 w-4"))
                              }, null, 8, ["class"])
                            ]),
                            _: 1
                          }),
                          createBaseVNode("span", _hoisted_7$3, toDisplayString(unref($translate)(__props.translatePrefix + option)), 1)
                        ]),
                        _: 2
                      }, 1032, ["value", "onSelect"]);
                    }), 128))
                  ]),
                  _: 1
                }))
              ])
            ]),
            _: 1
          })
        ]),
        _: 1
      }, 8, ["modelValue", "search-term", "open"]);
    };
  }
};
const ComboBoxVue = /* @__PURE__ */ _export_sfc(_sfc_main$x, [["__scopeId", "data-v-7b594bd9"]]);
const _hoisted_1$l = { class: "preset-settings" };
const _hoisted_2$g = { class: "settings-header" };
const _hoisted_3$a = { class: "header-title" };
const _hoisted_4$7 = { class: "settings-content" };
const _hoisted_5$4 = { class: "settings-option" };
const _hoisted_6$3 = ["placeholder"];
const _hoisted_7$2 = { class: "settings-option" };
const _hoisted_8$2 = {
  key: 0,
  class: "settings-option"
};
const _hoisted_9$1 = {
  key: 1,
  class: "settings-option"
};
const _hoisted_10$1 = {
  key: 2,
  class: "settings-option"
};
const _hoisted_11 = { class: "settings-option" };
const _hoisted_12 = {
  key: 3,
  class: "settings-option"
};
const _hoisted_13 = { class: "size-input-wrapper" };
const _hoisted_14 = ["value", "aria-label"];
const _hoisted_15 = { class: "unit" };
const _sfc_main$w = {
  __name: "presetSettingsVue",
  setup(__props) {
    const main = inject("main");
    const closeCreatePanel = inject("closeCreatePanel");
    const editMode = inject("editMode", ref(false));
    const editingPreset = inject("editingPreset", ref(null));
    const editingIndex = inject("editingIndex", ref(-1));
    const presetName = ref("");
    const format = ref("");
    const formatOptions = ref([
      {
        label: "original",
        options: ["original"]
      },
      {
        label: "common",
        options: ["jpg", "png", "bmp", "gif", "tif", "ico"]
      },
      {
        label: "nextGen",
        options: ["webp", "avif"]
      },
      {
        label: "other",
        options: ["hdr", "exr", "tga"]
      },
      {
        label: "video",
        options: ["mp4", "webm"]
      }
    ]);
    const dropdownStates = ref({
      format: false,
      quality: false,
      size: false,
      fps: false,
      // FPS 下拉選單狀態
      codec: false
      // 新增 Codec 下拉選單狀態
    });
    const quality = ref();
    const generateQualityOptions2 = () => {
      const options = [];
      for (let i = 100; i > 0; i -= 5) {
        options.push(i);
      }
      return options;
    };
    const qualityOptions = ref(generateQualityOptions2());
    const sizeType = ref();
    const sizeValue = ref(900);
    const sizeOptions = ref([
      "original",
      "maxWidth",
      "maxHeight",
      "minWidth",
      "minHeight",
      "maxSide",
      "minSide"
    ]);
    const animatedFps = ref(30);
    const fpsOptions = ref([10, 15, 24, 30, 60]);
    const codec = ref("");
    const codecOptions = ref([
      {
        label: "mp4",
        options: ["h264", "h265"]
      },
      {
        label: "webm",
        options: ["vp8", "vp9"]
      }
    ]);
    const updateDropdownState = (key, visible) => {
      dropdownStates.value[key] = visible;
    };
    const showQualityOption = computed(() => {
      const qualityFormats = ["jpg", "jpeg", "webp", "avif", "jxl", "mp4", "webm"];
      return qualityFormats.includes(format.value);
    });
    const showFpsOption = computed(() => {
      const animatedFormats = ["gif", "webp", "mp4", "webm"];
      return animatedFormats.includes(format.value);
    });
    const showCodecOption = computed(() => {
      const codecFormats = ["mp4", "webm"];
      return codecFormats.includes(format.value);
    });
    const getCodecOptions = () => {
      var _a;
      return ((_a = codecOptions.value.find((option) => option.label === format.value)) == null ? void 0 : _a.options) || [];
    };
    const codecOptionsList = computed(() => {
      return getCodecOptions();
    });
    const getDefaultCodec = () => {
      const options = getCodecOptions();
      if (options.includes(codec.value)) {
        return codec.value;
      }
      codec.value = options[0] || "";
      return codec.value;
    };
    const defaultCodec = computed(() => {
      return getDefaultCodec();
    });
    const showSizeInput = computed(() => {
      return sizeType.value !== "original" && sizeType.value !== "";
    });
    const validateNumberValue = (event, fieldType) => {
      const currentValue = event.target.value;
      const numValue = Number(currentValue);
      if (!isNaN(numValue) && currentValue !== "") {
        {
          sizeValue.value = Math.max(1, Math.floor(numValue));
        }
      } else {
        {
          event.target.value = sizeValue.value;
        }
      }
    };
    const initializeFormValues = () => {
      if (editMode.value && editingPreset.value) {
        console.log("初始化編輯表單值:", editingPreset.value);
        presetName.value = editingPreset.value.name || "";
        format.value = editingPreset.value.format || "";
        quality.value = editingPreset.value.quality || "";
        sizeType.value = editingPreset.value.sizeType || "";
        sizeValue.value = editingPreset.value.sizeValue || "";
        animatedFps.value = editingPreset.value.animatedFps || "";
        codec.value = editingPreset.value.codec || "";
      } else {
        presetName.value = "";
        format.value = "";
        quality.value = "";
        sizeType.value = "";
        sizeValue.value = 900;
        animatedFps.value = 30;
        codec.value = "";
      }
    };
    onMounted(() => {
      initializeFormValues();
    });
    watch([editMode, editingPreset], () => {
      console.log("編輯模式變化:", editMode.value, editingPreset.value);
      initializeFormValues();
    }, { immediate: true });
    const handleCreatePreset = () => {
      if (!presetName.value) {
        console.warn("預設名稱不能為空");
        return;
      }
      const existingSettings = JSON.parse(localStorage.getItem(main.localStorageKey) || "{}");
      if (!existingSettings.preset) {
        existingSettings.preset = [];
      }
      const presetData = {
        id: editMode.value ? editingPreset.value.id : Date.now().toString(),
        name: presetName.value,
        format: format.value,
        quality: quality.value,
        sizeType: sizeType.value,
        sizeValue: sizeValue.value,
        animatedFps: animatedFps.value,
        codec: codec.value,
        createdAt: editMode.value ? editingPreset.value.createdAt : (/* @__PURE__ */ new Date()).toISOString()
      };
      if (editMode.value) {
        existingSettings.preset[editingIndex.value] = presetData;
      } else {
        existingSettings.preset.push(presetData);
      }
      localStorage.setItem(main.localStorageKey, JSON.stringify(existingSettings));
      main.localStorageSetting = existingSettings;
      presetName.value = "";
      console.log(editMode.value ? "預設參數已更新:" : "預設參數已儲存:", presetData);
      if (closeCreatePanel) {
        closeCreatePanel();
      }
    };
    const validateForm = () => {
      const validateQuality = showQualityOption.value ? quality.value : true;
      const validateFps = showFpsOption.value ? animatedFps.value : true;
      const validateCodec = showCodecOption.value ? codec.value : true;
      return presetName.value && format.value && validateQuality && validateFps && validateCodec && sizeType.value;
    };
    watch(() => format.value, () => {
      if (showCodecOption.value) {
        codec.value = getCodecOptions()[0] || "";
      }
    });
    return (_ctx, _cache) => {
      const _component_el_icon = ElIcon;
      const _component_el_button = ElButton;
      return openBlock(), createElementBlock("div", _hoisted_1$l, [
        createBaseVNode("div", _hoisted_2$g, [
          createBaseVNode("span", _hoisted_3$a, toDisplayString(_ctx.$translate("main.SettingSidebar.preset.settings.title")), 1),
          createBaseVNode("button", {
            class: "header-button",
            onClick: _cache[0] || (_cache[0] = (...args) => unref(closeCreatePanel) && unref(closeCreatePanel)(...args)),
            tabindex: "99"
          }, [
            createVNode(_component_el_icon, { class: "icon icon-close" })
          ])
        ]),
        createBaseVNode("div", _hoisted_4$7, [
          createBaseVNode("div", _hoisted_5$4, [
            createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.preset.settings.nameLabel")), 1),
            withDirectives(createBaseVNode("input", {
              type: "text",
              class: "settings-input settings-size-input",
              tabindex: 5,
              "onUpdate:modelValue": _cache[1] || (_cache[1] = ($event) => presetName.value = $event),
              placeholder: _ctx.$translate("main.SettingSidebar.preset.settings.namePlaceholder")
            }, null, 8, _hoisted_6$3), [
              [vModelText, presetName.value]
            ])
          ]),
          createBaseVNode("div", _hoisted_7$2, [
            createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.format.label")), 1),
            createVNode(ComboBoxVue, {
              modelValue: format.value,
              "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => format.value = $event),
              options: formatOptions.value,
              placeholder: _ctx.$translate("main.SettingSidebar.preset.settings.selectPlaceholder"),
              "translate-prefix": "main.SettingSidebar.format.format-options.",
              class: "settings-combobox",
              onVisibleChange: _cache[3] || (_cache[3] = (visible) => updateDropdownState("format", visible)),
              tabindex: 1,
              "aria-label": _ctx.$translate("main.SettingSidebar.format.label"),
              required: ""
            }, null, 8, ["modelValue", "options", "placeholder", "aria-label"])
          ]),
          showQualityOption.value ? (openBlock(), createElementBlock("div", _hoisted_8$2, [
            createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.quality.label")), 1),
            createVNode(ComboBoxVue, {
              modelValue: quality.value,
              "onUpdate:modelValue": _cache[4] || (_cache[4] = ($event) => quality.value = $event),
              options: qualityOptions.value,
              class: normalizeClass(["settings-combobox", dropdownStates.value.quality ? "is-active" : ""]),
              placeholder: _ctx.$translate("main.SettingSidebar.preset.settings.selectPlaceholder"),
              onVisibleChange: _cache[5] || (_cache[5] = (visible) => updateDropdownState("quality", visible)),
              tabindex: 2,
              "aria-label": _ctx.$translate("main.SettingSidebar.quality.label")
            }, null, 8, ["modelValue", "options", "placeholder", "class", "aria-label"])
          ])) : createCommentVNode("", true),
          showFpsOption.value ? (openBlock(), createElementBlock("div", _hoisted_9$1, [
            createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.fps.label")), 1),
            createVNode(ComboBoxVue, {
              modelValue: animatedFps.value,
              "onUpdate:modelValue": _cache[6] || (_cache[6] = ($event) => animatedFps.value = $event),
              options: fpsOptions.value,
              class: normalizeClass(["settings-combobox", dropdownStates.value.fps ? "is-active" : ""]),
              placeholder: _ctx.$translate("main.SettingSidebar.preset.settings.selectPlaceholder"),
              onVisibleChange: _cache[7] || (_cache[7] = (visible) => updateDropdownState("fps", visible)),
              tabindex: 3
            }, null, 8, ["modelValue", "options", "placeholder", "class"])
          ])) : createCommentVNode("", true),
          showCodecOption.value ? (openBlock(), createElementBlock("div", _hoisted_10$1, [
            createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.codec.label")), 1),
            createVNode(ComboBoxVue, {
              "model-value": defaultCodec.value,
              "onUpdate:modelValue": _cache[8] || (_cache[8] = ($event) => codec.value = $event),
              options: codecOptionsList.value,
              class: normalizeClass(["settings-combobox", dropdownStates.value.codec ? "is-active" : ""]),
              placeholder: _ctx.$translate("main.SettingSidebar.preset.settings.selectPlaceholder"),
              "translate-prefix": "main.SettingSidebar.codec.codec-options.",
              onVisibleChange: _cache[9] || (_cache[9] = (visible) => updateDropdownState("codec", visible)),
              tabindex: 4
            }, null, 8, ["model-value", "options", "placeholder", "class"])
          ])) : createCommentVNode("", true),
          createBaseVNode("div", _hoisted_11, [
            createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.size.label")), 1),
            createVNode(ComboBoxVue, {
              modelValue: sizeType.value,
              "onUpdate:modelValue": _cache[10] || (_cache[10] = ($event) => sizeType.value = $event),
              options: sizeOptions.value,
              "translate-prefix": "main.SettingSidebar.size.size-options.",
              class: normalizeClass(["settings-combobox", dropdownStates.value.size ? "is-active" : ""]),
              placeholder: _ctx.$translate("main.SettingSidebar.preset.settings.selectPlaceholder"),
              onVisibleChange: _cache[11] || (_cache[11] = (visible) => updateDropdownState("size", visible)),
              tabindex: 4,
              "aria-label": _ctx.$translate("main.SettingSidebar.size.label")
            }, null, 8, ["modelValue", "options", "placeholder", "class", "aria-label"])
          ]),
          showSizeInput.value ? (openBlock(), createElementBlock("div", _hoisted_12, [
            createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.size.size-options." + sizeType.value)), 1),
            createBaseVNode("div", _hoisted_13, [
              createBaseVNode("input", {
                type: "text",
                value: sizeValue.value,
                class: "settings-input settings-size-input",
                onKeydown: _cache[12] || (_cache[12] = (e) => ["e", "E", "+", "-"].includes(e.key) && e.preventDefault()),
                onBlur: _cache[13] || (_cache[13] = ($event) => validateNumberValue($event)),
                tabindex: 5,
                "aria-label": _ctx.$translate("main.SettingSidebar.size.size-options." + sizeType.value)
              }, null, 40, _hoisted_14),
              createBaseVNode("span", _hoisted_15, toDisplayString(_ctx.$translate("main.SettingSidebar.preset.settings.pixelUnit")), 1)
            ])
          ])) : createCommentVNode("", true),
          createVNode(_component_el_button, {
            type: "primary",
            onClick: handleCreatePreset,
            tabindex: 9,
            class: "create-preset-button",
            disabled: !validateForm()
          }, {
            default: withCtx(() => [
              createTextVNode(toDisplayString(unref(editMode) ? _ctx.$translate("main.SettingSidebar.preset.settings.updatePreset") : _ctx.$translate("main.SettingSidebar.preset.settings.createPreset")), 1)
            ]),
            _: 1
          }, 8, ["disabled"])
        ])
      ]);
    };
  }
};
const _sfc_main$v = {
  __name: "Command",
  props: {
    modelValue: { type: null, required: false, default: "" },
    defaultValue: { type: null, required: false },
    multiple: { type: Boolean, required: false },
    orientation: { type: String, required: false },
    dir: { type: String, required: false },
    disabled: { type: Boolean, required: false },
    selectionBehavior: { type: String, required: false },
    highlightOnHover: { type: Boolean, required: false },
    by: { type: [String, Function], required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    name: { type: String, required: false },
    required: { type: Boolean, required: false },
    class: { type: null, required: false }
  },
  emits: [
    "update:modelValue",
    "highlight",
    "entryFocus",
    "leave"
  ],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emits = __emit;
    const delegatedProps = reactiveOmit(props, "class");
    const forwarded = useForwardPropsEmits(delegatedProps, emits);
    const allItems = ref(/* @__PURE__ */ new Map());
    const allGroups = ref(/* @__PURE__ */ new Map());
    const { contains } = useFilter({ sensitivity: "base" });
    const filterState = reactive({
      search: "",
      filtered: {
        /** The count of all visible items. */
        count: 0,
        /** Map from visible item id to its search score. */
        items: /* @__PURE__ */ new Map(),
        /** Set of groups with at least one visible item. */
        groups: /* @__PURE__ */ new Set()
      }
    });
    function filterItems() {
      if (!filterState.search) {
        filterState.filtered.count = allItems.value.size;
        return;
      }
      filterState.filtered.groups = /* @__PURE__ */ new Set();
      let itemCount = 0;
      for (const [id, value] of allItems.value) {
        const score = contains(value, filterState.search);
        filterState.filtered.items.set(id, score ? 1 : 0);
        if (score) itemCount++;
      }
      for (const [groupId, group] of allGroups.value) {
        for (const itemId of group) {
          if (filterState.filtered.items.get(itemId) > 0) {
            filterState.filtered.groups.add(groupId);
            break;
          }
        }
      }
      filterState.filtered.count = itemCount;
    }
    watch(
      () => filterState.search,
      () => {
        filterItems();
      }
    );
    provideCommandContext({
      allItems,
      allGroups,
      filterState
    });
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ListboxRoot_default), mergeProps({ "data-slot": "command" }, unref(forwarded), {
        class: unref(cn)(
          "bg-popover text-popover-foreground flex h-full w-full flex-col overflow-hidden rounded-md",
          props.class
        )
      }), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16, ["class"]);
    };
  }
};
const _sfc_main$u = {
  __name: "CommandEmpty",
  props: {
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false }
  },
  setup(__props) {
    const props = __props;
    const delegatedProps = reactiveOmit(props, "class");
    const { filterState } = useCommand();
    const isRender = computed(
      () => !!filterState.search && filterState.filtered.count === 0
    );
    return (_ctx, _cache) => {
      return isRender.value ? (openBlock(), createBlock(unref(Primitive), mergeProps({
        key: 0,
        "data-slot": "command-empty"
      }, unref(delegatedProps), {
        class: unref(cn)("py-6 text-center text-sm", props.class)
      }), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16, ["class"])) : createCommentVNode("", true);
    };
  }
};
const _sfc_main$t = {
  __name: "CommandGroup",
  props: {
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false },
    heading: { type: String, required: false }
  },
  setup(__props) {
    const props = __props;
    const delegatedProps = reactiveOmit(props, "class");
    const { allGroups, filterState } = useCommand();
    const id = useId();
    const isRender = computed(
      () => !filterState.search ? true : filterState.filtered.groups.has(id)
    );
    provideCommandGroupContext({ id });
    onMounted(() => {
      if (!allGroups.value.has(id)) allGroups.value.set(id, /* @__PURE__ */ new Set());
    });
    onUnmounted(() => {
      allGroups.value.delete(id);
    });
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ListboxGroup_default), mergeProps(unref(delegatedProps), {
        id: unref(id),
        "data-slot": "command-group",
        class: unref(cn)("text-foreground overflow-hidden", props.class),
        hidden: isRender.value ? void 0 : true
      }), {
        default: withCtx(() => [
          __props.heading ? (openBlock(), createBlock(unref(ListboxGroupLabel_default), {
            key: 0,
            class: "px-2 py-1.5 text-xs font-medium text-muted-foreground"
          }, {
            default: withCtx(() => [
              createTextVNode(toDisplayString(__props.heading), 1)
            ]),
            _: 1
          })) : createCommentVNode("", true),
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16, ["id", "class", "hidden"]);
    };
  }
};
const _hoisted_1$k = {
  "data-slot": "command-input-wrapper",
  class: "flex h-12 items-center gap-2 px-3 command-input-wrapper"
};
const _sfc_main$s = /* @__PURE__ */ Object.assign({
  inheritAttrs: false
}, {
  __name: "CommandInput",
  props: {
    modelValue: { type: String, required: false },
    autoFocus: { type: Boolean, required: false },
    disabled: { type: Boolean, required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false }
  },
  setup(__props) {
    const props = __props;
    const delegatedProps = reactiveOmit(props, "class");
    const forwardedProps = useForwardProps(delegatedProps);
    const { filterState } = useCommand();
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$k, [
        createVNode(unref(Search), { class: "size-4 shrink-0 opacity-50" }),
        createVNode(unref(ListboxFilter_default), mergeProps({ ...unref(forwardedProps), ..._ctx.$attrs }, {
          modelValue: unref(filterState).search,
          "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => unref(filterState).search = $event),
          "data-slot": "command-input",
          "auto-focus": "",
          class: unref(cn)(
            "placeholder:text-muted-foreground flex h-12 w-full rounded-md bg-transparent py-3 text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50",
            props.class
          )
        }), null, 16, ["modelValue", "class"])
      ]);
    };
  }
});
const _sfc_main$r = {
  __name: "CommandItem",
  props: {
    value: { type: null, required: true },
    disabled: { type: Boolean, required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false }
  },
  emits: ["select"],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emits = __emit;
    const delegatedProps = reactiveOmit(props, "class");
    const forwarded = useForwardPropsEmits(delegatedProps, emits);
    const id = useId();
    const { filterState, allItems, allGroups } = useCommand();
    const groupContext = useCommandGroup();
    const isRender = computed(() => {
      if (!filterState.search) {
        return true;
      } else {
        const filteredCurrentItem = filterState.filtered.items.get(id);
        if (filteredCurrentItem === void 0) {
          return true;
        }
        return filteredCurrentItem > 0;
      }
    });
    const itemRef = ref();
    const currentElement = useCurrentElement(itemRef);
    onMounted(() => {
      var _a, _b;
      if (!(currentElement.value instanceof HTMLElement)) return;
      allItems.value.set(
        id,
        currentElement.value.textContent ?? ((_a = props.value) == null ? void 0 : _a.toString()) ?? ""
      );
      const groupId = groupContext == null ? void 0 : groupContext.id;
      if (groupId) {
        if (!allGroups.value.has(groupId)) {
          allGroups.value.set(groupId, /* @__PURE__ */ new Set([id]));
        } else {
          (_b = allGroups.value.get(groupId)) == null ? void 0 : _b.add(id);
        }
      }
    });
    onUnmounted(() => {
      allItems.value.delete(id);
    });
    return (_ctx, _cache) => {
      return isRender.value ? (openBlock(), createBlock(unref(ListboxItem_default), mergeProps({ key: 0 }, unref(forwarded), {
        id: unref(id),
        ref_key: "itemRef",
        ref: itemRef,
        "data-slot": "command-item",
        class: unref(cn)(
          `data-[highlighted]:text-accent-foreground [&_svg:not([class*='text-'])]:text-muted-foreground relative flex cursor-default items-center gap-2 rounded-sm text-sm outline-hidden select-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4`,
          props.class
        ),
        onSelect: _cache[0] || (_cache[0] = () => {
          unref(filterState).search = "";
        })
      }), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16, ["id", "class"])) : createCommentVNode("", true);
    };
  }
};
const _hoisted_1$j = { role: "presentation" };
const _sfc_main$q = {
  __name: "CommandList",
  props: {
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false }
  },
  setup(__props) {
    const props = __props;
    const delegatedProps = reactiveOmit(props, "class");
    const forwarded = useForwardProps(delegatedProps);
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ListboxContent_default), mergeProps({ "data-slot": "command-list" }, unref(forwarded), {
        class: unref(cn)(
          "max-h-[300px] scroll-py-1 overflow-x-hidden overflow-y-auto",
          props.class
        )
      }), {
        default: withCtx(() => [
          createBaseVNode("div", _hoisted_1$j, [
            renderSlot(_ctx.$slots, "default")
          ])
        ]),
        _: 3
      }, 16, ["class"]);
    };
  }
};
const _sfc_main$p = {
  __name: "CommandSeparator",
  props: {
    orientation: { type: String, required: false },
    decorative: { type: Boolean, required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false }
  },
  setup(__props) {
    const props = __props;
    const delegatedProps = reactiveOmit(props, "class");
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(Separator_default), mergeProps({ "data-slot": "command-separator" }, unref(delegatedProps), {
        class: unref(cn)("bg-border -mx-1 h-px", props.class)
      }), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16, ["class"]);
    };
  }
};
const [useCommand, provideCommandContext] = createContext("Command");
const [useCommandGroup, provideCommandGroupContext] = createContext("CommandGroup");
const _sfc_main$o = {
  __name: "ContextMenu",
  props: {
    dir: { type: String, required: false },
    modal: { type: Boolean, required: false }
  },
  emits: ["update:open"],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emits = __emit;
    const forwarded = useForwardPropsEmits(props, emits);
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ContextMenuRoot_default), mergeProps({ "data-slot": "context-menu" }, unref(forwarded)), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16);
    };
  }
};
const _sfc_main$n = {
  __name: "ContextMenuContent",
  props: {
    forceMount: { type: Boolean, required: false },
    loop: { type: Boolean, required: false },
    sideFlip: { type: Boolean, required: false },
    alignOffset: { type: Number, required: false },
    alignFlip: { type: Boolean, required: false },
    avoidCollisions: { type: Boolean, required: false },
    collisionBoundary: { type: null, required: false },
    collisionPadding: { type: [Number, Object], required: false },
    sticky: { type: String, required: false },
    hideWhenDetached: { type: Boolean, required: false },
    positionStrategy: { type: String, required: false },
    disableUpdateOnLayoutShift: { type: Boolean, required: false },
    prioritizePosition: { type: Boolean, required: false },
    reference: { type: null, required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false }
  },
  emits: [
    "escapeKeyDown",
    "pointerDownOutside",
    "focusOutside",
    "interactOutside",
    "closeAutoFocus"
  ],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emits = __emit;
    const delegatedProps = reactiveOmit(props, "class");
    const forwarded = useForwardPropsEmits(delegatedProps, emits);
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ContextMenuPortal_default), null, {
        default: withCtx(() => [
          createVNode(unref(ContextMenuContent_default), mergeProps({ "data-slot": "context-menu-content" }, unref(forwarded), {
            class: unref(cn)(
              "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 max-h-(--reka-context-menu-content-available-height) overflow-x-hidden overflow-y-auto rounded-md p-1 shadow-md",
              props.class
            )
          }), {
            default: withCtx(() => [
              renderSlot(_ctx.$slots, "default")
            ]),
            _: 3
          }, 16, ["class"])
        ]),
        _: 3
      });
    };
  }
};
const _sfc_main$m = {
  __name: "ContextMenuItem",
  props: {
    disabled: { type: Boolean, required: false },
    textValue: { type: String, required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false },
    inset: { type: Boolean, required: false },
    variant: { type: String, required: false, default: "default" }
  },
  emits: ["select"],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emits = __emit;
    const delegatedProps = reactiveOmit(props, "class");
    const forwarded = useForwardPropsEmits(delegatedProps, emits);
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ContextMenuItem_default), mergeProps({
        "data-slot": "context-menu-item",
        "data-inset": __props.inset ? "" : void 0,
        "data-variant": __props.variant
      }, unref(forwarded), {
        class: unref(cn)(
          `focus:text-accent-foreground data-[variant=destructive]:text-destructive-foreground data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/40 data-[variant=destructive]:focus:text-destructive-foreground data-[variant=destructive]:*:[svg]:!text-destructive-foreground [&_svg:not([class*='text-'])]:text-muted-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[inset]:pl-8 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4`,
          props.class
        )
      }), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16, ["data-inset", "data-variant", "class"]);
    };
  }
};
const _sfc_main$l = {
  __name: "ContextMenuSeparator",
  props: {
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false },
    class: { type: null, required: false }
  },
  setup(__props) {
    const props = __props;
    const delegatedProps = reactiveOmit(props, "class");
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ContextMenuSeparator_default), mergeProps({ "data-slot": "context-menu-separator" }, unref(delegatedProps), {
        class: unref(cn)("bg-border -mx-1 my-1 h-px", props.class)
      }), null, 16, ["class"]);
    };
  }
};
const _sfc_main$k = {
  __name: "ContextMenuTrigger",
  props: {
    disabled: { type: Boolean, required: false },
    asChild: { type: Boolean, required: false },
    as: { type: null, required: false }
  },
  setup(__props) {
    const props = __props;
    const forwardedProps = useForwardProps(props);
    return (_ctx, _cache) => {
      return openBlock(), createBlock(unref(ContextMenuTrigger_default), mergeProps({ "data-slot": "context-menu-trigger" }, unref(forwardedProps)), {
        default: withCtx(() => [
          renderSlot(_ctx.$slots, "default")
        ]),
        _: 3
      }, 16);
    };
  }
};
const _hoisted_1$i = { key: 0 };
const _hoisted_2$f = {
  key: 0,
  class: "count"
};
const _sfc_main$j = {
  __name: "CommandVue",
  setup(__props) {
    const { proxy } = getCurrentInstance();
    const $translate = proxy == null ? void 0 : proxy.$translate;
    const main = inject("main");
    const settingsState = inject("settingsState", null);
    const setEditMode = inject("setEditMode", null);
    const closeDialog = inject("closeDialog", null);
    const isCreatePreset = inject("isCreatePreset", null);
    const presetList = ref(main.localStorageSetting.preset || []);
    watch(() => main.localStorageSetting.preset, (newPresets) => {
      presetList.value = newPresets || [];
    }, { deep: true });
    const openStates = ref({});
    const safeGetLocalStorage = () => {
      try {
        return JSON.parse(localStorage.getItem(main.localStorageKey) || "{}");
      } catch (error) {
        console.error("Failed to parse localStorage:", error);
        return { preset: [] };
      }
    };
    const safeSetLocalStorage = (settings) => {
      try {
        localStorage.setItem(main.localStorageKey, JSON.stringify(settings));
        main.localStorageSetting = settings;
        presetList.value = settings.preset || [];
        return true;
      } catch (error) {
        console.error("Failed to save to localStorage:", error);
        alert($translate("main.SettingSidebar.preset.error.saveFailed"));
        return false;
      }
    };
    const handleApplyPreset = (preset) => {
      settingsState.format = preset.format;
      settingsState.quality = preset.quality;
      settingsState.sizeType = preset.sizeType;
      settingsState.sizeValue = preset.sizeValue;
      settingsState.animatedFps = preset.animatedFps;
      settingsState.codec = preset.codec;
      if (closeDialog) closeDialog();
    };
    const handleEdit = (preset, index) => {
      if (index < 0 || index >= presetList.value.length) {
        console.error("Invalid preset index:", index);
        return;
      }
      const editingPreset = { ...preset };
      const editingIndex = index;
      if (setEditMode) {
        setEditMode(editingPreset, editingIndex);
      } else {
        console.error("setEditMode not found in injection");
      }
    };
    const handleClone = (preset, index) => {
      const existingSettings = safeGetLocalStorage();
      if (!existingSettings.preset) {
        existingSettings.preset = [];
      }
      const clonedPreset = {
        ...preset,
        id: `${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        name: `${preset.name}`,
        createdAt: (/* @__PURE__ */ new Date()).toISOString()
      };
      const targetIndex = Math.min(index + 1, existingSettings.preset.length);
      existingSettings.preset.splice(targetIndex, 0, clonedPreset);
      safeSetLocalStorage(existingSettings);
    };
    const handleRemove = (preset, index) => {
      const existingSettings = safeGetLocalStorage();
      if (!existingSettings.preset || existingSettings.preset.length === 0) {
        return;
      }
      existingSettings.preset.splice(index, 1);
      safeSetLocalStorage(existingSettings);
    };
    const openContextMenu = (presetId, event) => {
      event.stopPropagation();
      event.preventDefault();
      const target = event.currentTarget.closest(".command-item");
      if (target) {
        const contextMenuEvent = new MouseEvent("contextmenu", {
          bubbles: true,
          cancelable: true,
          view: window,
          clientX: event.clientX,
          clientY: event.clientY
        });
        target.dispatchEvent(contextMenuEvent);
      }
    };
    return (_ctx, _cache) => {
      const _component_el_icon = ElIcon;
      return openBlock(), createBlock(unref(_sfc_main$v), { class: "rounded-lg shadow-md max-w-[450px] command-wrapper" }, {
        default: withCtx(() => [
          createVNode(unref(_sfc_main$s), {
            placeholder: unref($translate)("main.SettingSidebar.combobox.searchPlaceholder"),
            class: "command-input"
          }, null, 8, ["placeholder"]),
          createVNode(unref(_sfc_main$q), { class: "overflow-y-hidden" }, {
            default: withCtx(() => [
              createVNode(unref(_sfc_main$u), null, {
                default: withCtx(() => [
                  createTextVNode(toDisplayString(unref($translate)("main.SettingSidebar.preset.command.noResults")), 1)
                ]),
                _: 1
              }),
              createVNode(unref(_sfc_main$t), { class: "presets-group" }, {
                default: withCtx(() => [
                  (openBlock(true), createElementBlock(Fragment, null, renderList(presetList.value, (preset, index) => {
                    return openBlock(), createBlock(unref(_sfc_main$o), {
                      key: preset.id,
                      open: openStates.value[preset.id],
                      "onUpdate:open": ($event) => openStates.value[preset.id] = $event
                    }, {
                      default: withCtx(() => [
                        createVNode(unref(_sfc_main$k), { "as-child": "" }, {
                          default: withCtx(() => [
                            createVNode(unref(_sfc_main$r), {
                              onClick: ($event) => handleApplyPreset(preset),
                              class: "command-item"
                            }, {
                              default: withCtx(() => [
                                createBaseVNode("span", null, toDisplayString(preset.name), 1),
                                createVNode(_component_el_icon, {
                                  class: "icon icon-more",
                                  onClick: ($event) => openContextMenu(preset.id, $event)
                                }, null, 8, ["onClick"])
                              ]),
                              _: 2
                            }, 1032, ["onClick"])
                          ]),
                          _: 2
                        }, 1024),
                        createVNode(unref(_sfc_main$n), { class: "context-menu-content" }, {
                          default: withCtx(() => [
                            createVNode(unref(_sfc_main$m), {
                              onClick: ($event) => handleEdit(preset, index),
                              class: "context-menu-item"
                            }, {
                              default: withCtx(() => [
                                createVNode(_component_el_icon, { class: "icon-edit" }),
                                createTextVNode(" " + toDisplayString(unref($translate)("main.SettingSidebar.preset.command.edit")), 1)
                              ]),
                              _: 2
                            }, 1032, ["onClick"]),
                            createVNode(unref(_sfc_main$l), { class: "context-menu-separator" }),
                            createVNode(unref(_sfc_main$m), {
                              onClick: ($event) => handleClone(preset, index),
                              class: "context-menu-item"
                            }, {
                              default: withCtx(() => [
                                createVNode(_component_el_icon, { class: "icon-clone" }),
                                createTextVNode(" " + toDisplayString(unref($translate)("main.SettingSidebar.preset.command.clone")), 1)
                              ]),
                              _: 2
                            }, 1032, ["onClick"]),
                            createVNode(unref(_sfc_main$m), {
                              onClick: ($event) => handleRemove(preset, index),
                              class: "context-menu-item"
                            }, {
                              default: withCtx(() => [
                                createVNode(_component_el_icon, { class: "icon-remove" }),
                                createTextVNode(" " + toDisplayString(unref($translate)("main.SettingSidebar.preset.command.remove")), 1)
                              ]),
                              _: 2
                            }, 1032, ["onClick"])
                          ]),
                          _: 2
                        }, 1024)
                      ]),
                      _: 2
                    }, 1032, ["open", "onUpdate:open"]);
                  }), 128))
                ]),
                _: 1
              }),
              createVNode(unref(_sfc_main$p), { class: "command-separator" }),
              createVNode(unref(_sfc_main$t), { class: "shrink-0" }, {
                default: withCtx(() => [
                  createVNode(unref(_sfc_main$r), {
                    class: "new-preset-section command-item",
                    onClick: _cache[0] || (_cache[0] = ($event) => isCreatePreset.value = true)
                  }, {
                    default: withCtx(() => {
                      var _a;
                      return [
                        createVNode(_component_el_icon, { class: "icon-plus" }),
                        ((_a = unref(main)) == null ? void 0 : _a.status) !== "processing" ? (openBlock(), createElementBlock("span", _hoisted_1$i, [
                          createTextVNode(toDisplayString(unref($translate)("main.SettingSidebar.preset.command.createPreset")) + " ", 1),
                          _ctx.exportableTaskCount > 0 ? (openBlock(), createElementBlock("span", _hoisted_2$f, "(" + toDisplayString(_ctx.exportableTaskCount) + ")", 1)) : createCommentVNode("", true)
                        ])) : createCommentVNode("", true)
                      ];
                    }),
                    _: 1
                  })
                ]),
                _: 1
              })
            ]),
            _: 1
          })
        ]),
        _: 1
      });
    };
  }
};
const _hoisted_1$h = { class: "image-vue" };
const _hoisted_2$e = ["src", "alt"];
const _sfc_main$i = {
  __name: "ImageVue",
  props: {
    width: {
      type: Number,
      required: true
    },
    height: {
      type: Number,
      required: true
    },
    src: {
      type: String,
      required: true
    },
    darkSrc: {
      type: String,
      required: false
    }
  },
  setup(__props) {
    const props = __props;
    const base_path = __dirname + "/images/";
    const THEME_SUPPORT2 = {
      Auto: !eagle.app.isDarkColors(),
      LIGHT: true,
      LIGHTGRAY: true,
      GRAY: false,
      DARK: false,
      BLUE: false,
      PURPLE: false
    };
    const uri = ref("");
    watchEffect(() => {
      uri.value = THEME_SUPPORT2[eagle.app.theme] ? props.src : props.darkSrc ?? props.src;
    });
    eagle.onThemeChanged((theme) => {
      uri.value = THEME_SUPPORT2[theme] ? props.src : props.darkSrc ?? props.src;
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$h, [
        createBaseVNode("img", {
          style: normalizeStyle({
            width: props.width + "px",
            height: props.height + "px"
          }),
          src: base_path + unref(uri),
          alt: unref(uri),
          loading: "lazy"
        }, null, 12, _hoisted_2$e),
        renderSlot(_ctx.$slots, "default")
      ]);
    };
  }
};
const _hoisted_1$g = { class: "dialog-body" };
const _hoisted_2$d = { class: "empty-state-text" };
const _hoisted_3$9 = { class: "title" };
const _hoisted_4$6 = { class: "subtitle" };
const _sfc_main$h = {
  __name: "presetDialogVue",
  setup(__props) {
    const isCreatePreset = ref(false);
    const isDialogOpen = ref(false);
    const containerRef = ref(null);
    const main = inject("main");
    onMounted(() => {
      const sidebar = document.querySelector(".settings-sidebar");
      if (sidebar) {
        containerRef.value = sidebar;
      }
    });
    const editMode = ref(false);
    const editingPreset = ref(null);
    const editingIndex = ref(-1);
    const setEditMode = (preset, index) => {
      editingPreset.value = preset;
      editingIndex.value = index;
      isCreatePreset.value = true;
      editMode.value = true;
    };
    const closeCreatePanel = () => {
      isCreatePreset.value = false;
      editMode.value = false;
      editingPreset.value = null;
      editingIndex.value = -1;
    };
    const closeDialog = () => {
      if (!isCreatePreset.value) {
        isDialogOpen.value = false;
        closeCreatePanel();
      }
    };
    provide("closeCreatePanel", closeCreatePanel);
    provide("setEditMode", setEditMode);
    provide("editMode", editMode);
    provide("editingPreset", editingPreset);
    provide("editingIndex", editingIndex);
    provide("closeDialog", closeDialog);
    provide("isCreatePreset", isCreatePreset);
    return (_ctx, _cache) => {
      const _component_el_icon = ElIcon;
      const _component_el_button = ElButton;
      const _directive_tippy = resolveDirective("tippy");
      return openBlock(), createElementBlock(Fragment, null, [
        withDirectives((openBlock(), createElementBlock("button", {
          class: normalizeClass(["header-button", { "is-active": isDialogOpen.value }]),
          onClick: _cache[0] || (_cache[0] = ($event) => isDialogOpen.value = true)
        }, [
          createVNode(_component_el_icon, { class: "icon icon-export-preset" })
        ], 2)), [
          [_directive_tippy, {
            content: _ctx.$translate("main.SettingSidebar.preset.dialog.tooltipTitle"),
            placement: "bottom",
            delay: [200, 0],
            duration: [150, 0]
          }]
        ]),
        createVNode(CustomDialogContent, {
          modelValue: isDialogOpen.value,
          "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => isDialogOpen.value = $event),
          container: containerRef.value,
          class: "preset-dialog"
        }, {
          default: withCtx(() => {
            var _a, _b, _c, _d, _e, _f;
            return [
              createBaseVNode("div", _hoisted_1$g, [
                isCreatePreset.value ? (openBlock(), createBlock(_sfc_main$w, { key: 0 })) : createCommentVNode("", true),
                !((_c = (_b = (_a = unref(main)) == null ? void 0 : _a.localStorageSetting) == null ? void 0 : _b.preset) == null ? void 0 : _c.length) && !isCreatePreset.value ? (openBlock(), createElementBlock(Fragment, { key: 1 }, [
                  createVNode(_sfc_main$i, {
                    darkSrc: `dark/preset-empty.png`,
                    src: `light/preset-empty.png`
                  }),
                  createBaseVNode("div", _hoisted_2$d, [
                    createBaseVNode("span", _hoisted_3$9, toDisplayString(_ctx.$translate("main.SettingSidebar.preset.dialog.emptyTitle")), 1),
                    createBaseVNode("span", _hoisted_4$6, toDisplayString(_ctx.$translate("main.SettingSidebar.preset.dialog.emptyDescription")), 1)
                  ]),
                  createVNode(_component_el_button, {
                    type: "primary",
                    onClick: _cache[1] || (_cache[1] = ($event) => isCreatePreset.value = true),
                    tabindex: 9,
                    class: "create-preset-button",
                    "aria-label": _ctx.$translate(`main.SettingSidebar.operationMode.${_ctx.operationMode}`)
                  }, {
                    default: withCtx(() => [
                      createVNode(_component_el_icon, { class: "icon-btn-plus" }),
                      createBaseVNode("span", null, toDisplayString(_ctx.$translate("main.SettingSidebar.preset.dialog.createButton")), 1)
                    ]),
                    _: 1
                  }, 8, ["aria-label"])
                ], 64)) : createCommentVNode("", true),
                ((_f = (_e = (_d = unref(main)) == null ? void 0 : _d.localStorageSetting) == null ? void 0 : _e.preset) == null ? void 0 : _f.length) && !isCreatePreset.value ? (openBlock(), createBlock(_sfc_main$j, { key: 2 })) : createCommentVNode("", true)
              ])
            ];
          }),
          _: 1
        }, 8, ["modelValue", "container"])
      ], 64);
    };
  }
};
const PresetDialogVue = /* @__PURE__ */ _export_sfc(_sfc_main$h, [["__scopeId", "data-v-f7513c3e"]]);
const _hoisted_1$f = { class: "settings-header" };
const _hoisted_2$c = { class: "header-drag" };
const _hoisted_3$8 = ["alt"];
const _hoisted_4$5 = { class: "header-title" };
const _hoisted_5$3 = { class: "button-group" };
const _hoisted_6$2 = ["aria-label"];
const _sfc_main$g = {
  __name: "SettingsHeader",
  setup(__props) {
    const handleClose = () => {
      window.close();
    };
    return (_ctx, _cache) => {
      const _component_el_icon = ElIcon;
      return openBlock(), createElementBlock("div", _hoisted_1$f, [
        createBaseVNode("div", _hoisted_2$c, [
          createBaseVNode("img", {
            class: "header-icon",
            src: _imports_0,
            alt: _ctx.$translate("main.SettingSidebar.header.iconAlt")
          }, null, 8, _hoisted_3$8),
          createBaseVNode("span", _hoisted_4$5, toDisplayString(_ctx.$translate("manifest.app.name")), 1)
        ]),
        createBaseVNode("div", _hoisted_5$3, [
          createVNode(PresetDialogVue),
          createBaseVNode("button", {
            class: "header-button",
            onClick: handleClose,
            "aria-label": _ctx.$translate("main.SettingSidebar.header.close"),
            tabindex: "99"
          }, [
            createVNode(_component_el_icon, { class: "icon icon-close" })
          ], 8, _hoisted_6$2)
        ])
      ]);
    };
  }
};
const SettingsHeader = /* @__PURE__ */ _export_sfc(_sfc_main$g, [["__scopeId", "data-v-ba70b9e7"]]);
const _hoisted_1$e = { class: "format-settings-group" };
const _hoisted_2$b = { class: "settings-option" };
const _sfc_main$f = {
  __name: "FormatSettingsGroup",
  props: {
    settingsState: {
      type: Object,
      required: true
    },
    options: {
      type: Object,
      required: true
    },
    updateSetting: {
      type: Function,
      required: true
    },
    updateDropdownState: {
      type: Function,
      required: true
    }
  },
  setup(__props) {
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$e, [
        createBaseVNode("div", _hoisted_2$b, [
          createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.format.label")), 1),
          createVNode(ComboBoxVue, {
            "model-value": __props.settingsState.format,
            "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => __props.updateSetting("format", $event)),
            options: __props.options.formatOptions,
            placeholder: _ctx.$translate("main.SettingSidebar.combobox.searchPlaceholder"),
            "translate-prefix": "main.SettingSidebar.format.format-options.",
            class: "settings-combobox",
            onVisibleChange: _cache[1] || (_cache[1] = (visible) => __props.updateDropdownState("format", visible)),
            tabindex: 1,
            "aria-label": _ctx.$translate("main.SettingSidebar.format.label")
          }, null, 8, ["model-value", "options", "placeholder", "aria-label"])
        ])
      ]);
    };
  }
};
const FormatSettingsGroup = /* @__PURE__ */ _export_sfc(_sfc_main$f, [["__scopeId", "data-v-6425d3e7"]]);
const _hoisted_1$d = { class: "quality-settings-group" };
const _hoisted_2$a = {
  key: 0,
  class: "settings-option"
};
const _hoisted_3$7 = {
  key: 1,
  class: "settings-option"
};
const _hoisted_4$4 = {
  key: 2,
  class: "settings-option"
};
const _sfc_main$e = {
  __name: "QualitySettingsGroup",
  props: {
    settingsState: {
      type: Object,
      required: true
    },
    options: {
      type: Object,
      required: true
    },
    dropdownStates: {
      type: Object,
      required: true
    },
    shouldShowQuality: {
      type: Boolean,
      required: true
    },
    shouldShowFps: {
      type: Boolean,
      required: true
    },
    shouldShowCodec: {
      type: Boolean,
      required: true
    },
    updateSetting: {
      type: Function,
      required: true
    },
    updateDropdownState: {
      type: Function,
      required: true
    }
  },
  setup(__props) {
    const settingsState = inject("settingsState");
    const props = __props;
    const getCodecOptions = (settingsState2, options) => {
      var _a;
      return ((_a = options.codecOptions.find((option) => option.label === settingsState2.format)) == null ? void 0 : _a.options) || [];
    };
    const getDefaultCodec = (settingsState2, options) => {
      const codecOptionsList2 = getCodecOptions(settingsState2, options);
      if (codecOptionsList2.includes(settingsState2.codec)) {
        return settingsState2.codec;
      }
      settingsState2.codec = codecOptionsList2[0];
      return codecOptionsList2[0];
    };
    const codecOptionsList = computed(() => {
      return getCodecOptions(settingsState, props.options);
    });
    const defaultCodec = computed(() => {
      return getDefaultCodec(settingsState, props.options);
    });
    watch(() => settingsState.format, () => {
      codecOptionsList.value = getCodecOptions(settingsState, props.options);
      defaultCodec.value = getDefaultCodec(settingsState, props.options);
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$d, [
        __props.shouldShowQuality ? (openBlock(), createElementBlock("div", _hoisted_2$a, [
          createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.quality.label")), 1),
          createVNode(ComboBoxVue, {
            "model-value": unref(settingsState).quality,
            "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => __props.updateSetting("quality", $event)),
            options: __props.options.qualityOptions,
            class: normalizeClass(["settings-combobox", __props.dropdownStates.quality ? "is-active" : ""]),
            placeholder: _ctx.$translate("main.SettingSidebar.combobox.searchPlaceholder"),
            onVisibleChange: _cache[1] || (_cache[1] = (visible) => __props.updateDropdownState("quality", visible)),
            tabindex: 2,
            "aria-label": _ctx.$translate("main.SettingSidebar.quality.label")
          }, null, 8, ["model-value", "options", "placeholder", "class", "aria-label"])
        ])) : createCommentVNode("", true),
        __props.shouldShowFps ? (openBlock(), createElementBlock("div", _hoisted_3$7, [
          createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.fps.label")), 1),
          createVNode(ComboBoxVue, {
            "model-value": unref(settingsState).animatedFps,
            "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => __props.updateSetting("animatedFps", $event)),
            options: __props.options.fpsOptions,
            class: normalizeClass(["settings-combobox", __props.dropdownStates.fps ? "is-active" : ""]),
            placeholder: _ctx.$translate("main.SettingSidebar.combobox.searchPlaceholder"),
            "translate-prefix": "main.SettingSidebar.fps.fps-options.",
            onVisibleChange: _cache[3] || (_cache[3] = (visible) => __props.updateDropdownState("fps", visible)),
            tabindex: 3
          }, null, 8, ["model-value", "options", "placeholder", "class"])
        ])) : createCommentVNode("", true),
        __props.shouldShowCodec ? (openBlock(), createElementBlock("div", _hoisted_4$4, [
          createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.codec.label")), 1),
          createVNode(ComboBoxVue, {
            "model-value": unref(defaultCodec),
            "onUpdate:modelValue": _cache[4] || (_cache[4] = ($event) => __props.updateSetting("codec", $event)),
            options: unref(codecOptionsList),
            class: normalizeClass(["settings-combobox", __props.dropdownStates.codec ? "is-active" : ""]),
            placeholder: _ctx.$translate("main.SettingSidebar.combobox.searchPlaceholder"),
            "translate-prefix": "main.SettingSidebar.codec.codec-options.",
            onVisibleChange: _cache[5] || (_cache[5] = (visible) => __props.updateDropdownState("codec", visible)),
            tabindex: 4
          }, null, 8, ["model-value", "options", "placeholder", "class"])
        ])) : createCommentVNode("", true)
      ]);
    };
  }
};
const QualitySettingsGroup = /* @__PURE__ */ _export_sfc(_sfc_main$e, [["__scopeId", "data-v-9e1f5ab1"]]);
const _hoisted_1$c = { class: "size-settings-group" };
const _hoisted_2$9 = { class: "settings-option" };
const _hoisted_3$6 = {
  key: 0,
  class: "settings-option"
};
const _hoisted_4$3 = { class: "size-input-wrapper" };
const _hoisted_5$2 = ["value", "aria-label"];
const _sfc_main$d = {
  __name: "SizeSettingsGroup",
  props: {
    settingsState: {
      type: Object,
      required: true
    },
    options: {
      type: Object,
      required: true
    },
    dropdownStates: {
      type: Object,
      required: true
    },
    shouldShowSizeInput: {
      type: Boolean,
      required: true
    },
    updateSetting: {
      type: Function,
      required: true
    },
    updateDropdownState: {
      type: Function,
      required: true
    },
    validateNumberValue: {
      type: Function,
      required: true
    }
  },
  setup(__props) {
    const props = __props;
    const handleKeydown = (e) => {
      if (["e", "E", "+", "-"].includes(e.key)) {
        e.preventDefault();
      }
    };
    watch(() => props.settingsState.sizeType, (newValue) => {
      if (newValue !== "original") {
        nextTick(() => {
          var _a;
          (_a = document.querySelector(".settings-size-input")) == null ? void 0 : _a.focus();
        });
      }
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$c, [
        createBaseVNode("div", _hoisted_2$9, [
          createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.size.label")), 1),
          createVNode(ComboBoxVue, {
            "model-value": __props.settingsState.sizeType,
            "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => __props.updateSetting("sizeType", $event)),
            options: __props.options.sizeOptions,
            "translate-prefix": "main.SettingSidebar.size.size-options.",
            class: normalizeClass(["settings-combobox", __props.dropdownStates.size ? "is-active" : ""]),
            placeholder: _ctx.$translate("main.SettingSidebar.combobox.searchPlaceholder"),
            onVisibleChange: _cache[1] || (_cache[1] = (visible) => __props.updateDropdownState("size", visible)),
            tabindex: 4,
            "aria-label": _ctx.$translate("main.SettingSidebar.size.label")
          }, null, 8, ["model-value", "options", "placeholder", "class", "aria-label"])
        ]),
        __props.shouldShowSizeInput ? (openBlock(), createElementBlock("div", _hoisted_3$6, [
          createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.size.size-options." + __props.settingsState.sizeType)), 1),
          createBaseVNode("div", _hoisted_4$3, [
            createBaseVNode("input", {
              type: "number",
              value: __props.settingsState.sizeValue,
              class: "settings-input settings-size-input",
              onKeydown: handleKeydown,
              onBlur: _cache[2] || (_cache[2] = (e) => __props.validateNumberValue(e, "sizeValue")),
              tabindex: 5,
              "aria-label": _ctx.$translate("main.SettingSidebar.size.size-options." + __props.settingsState.sizeType)
            }, null, 40, _hoisted_5$2),
            _cache[3] || (_cache[3] = createBaseVNode("span", { class: "unit" }, "px", -1))
          ])
        ])) : createCommentVNode("", true)
      ]);
    };
  }
};
const SizeSettingsGroup = /* @__PURE__ */ _export_sfc(_sfc_main$d, [["__scopeId", "data-v-75a78be0"]]);
const _hoisted_1$b = { class: "naming-settings-group" };
const _hoisted_2$8 = { class: "settings-option" };
const _hoisted_3$5 = {
  key: 0,
  class: "settings-option"
};
const _hoisted_4$2 = { for: "newNameInput" };
const _hoisted_5$1 = ["value"];
const _hoisted_6$1 = {
  key: 1,
  class: "settings-option"
};
const _hoisted_7$1 = { class: "settings-counter" };
const _hoisted_8$1 = ["aria-label"];
const _hoisted_9 = ["value"];
const _hoisted_10 = ["aria-label"];
const _sfc_main$c = {
  __name: "NamingSettingsGroup",
  props: {
    settingsState: {
      type: Object,
      required: true
    },
    dropdownStates: {
      type: Object,
      required: true
    },
    updateSetting: {
      type: Function,
      required: true
    },
    updateDropdownState: {
      type: Function,
      required: true
    },
    validateNumberValue: {
      type: Function,
      required: true
    }
  },
  setup(__props) {
    const props = __props;
    const increaseNumber = () => {
      props.updateSetting("startNumber", props.settingsState.startNumber + 1);
    };
    const decreaseNumber = () => {
      if (props.settingsState.startNumber > 1) {
        props.updateSetting("startNumber", props.settingsState.startNumber - 1);
      }
    };
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$b, [
        createBaseVNode("div", _hoisted_2$8, [
          createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.namingSettings.nameType.label")), 1),
          createVNode(ComboBoxVue, {
            "model-value": __props.settingsState.nameType,
            "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => __props.updateSetting("nameType", $event)),
            options: ["original", "custom"],
            class: normalizeClass(["settings-combobox", __props.dropdownStates.nameType ? "is-active" : ""]),
            "translate-prefix": "main.SettingSidebar.namingSettings.nameType.",
            placeholder: _ctx.$translate("main.SettingSidebar.namingSettings.nameType.searchPlaceholder"),
            onVisibleChange: _cache[1] || (_cache[1] = (visible) => __props.updateDropdownState("nameType", visible)),
            tabindex: 6,
            "aria-label": _ctx.$translate("main.SettingSidebar.namingSettings.nameType.label")
          }, null, 8, ["model-value", "placeholder", "class", "aria-label"])
        ]),
        __props.settingsState.nameType === "custom" ? (openBlock(), createElementBlock("div", _hoisted_3$5, [
          createBaseVNode("label", _hoisted_4$2, toDisplayString(_ctx.$translate("main.SettingSidebar.namingSettings.customName.label")), 1),
          createBaseVNode("input", {
            type: "text",
            value: __props.settingsState.newFileName,
            onInput: _cache[2] || (_cache[2] = ($event) => __props.updateSetting("newFileName", $event.target.value)),
            id: "newNameInput",
            class: "settings-input",
            tabindex: 7
          }, null, 40, _hoisted_5$1)
        ])) : createCommentVNode("", true),
        __props.settingsState.nameType === "custom" ? (openBlock(), createElementBlock("div", _hoisted_6$1, [
          createBaseVNode("label", null, toDisplayString(_ctx.$translate("main.SettingSidebar.namingSettings.startNumber.label")), 1),
          createBaseVNode("div", _hoisted_7$1, [
            createBaseVNode("button", {
              class: normalizeClass(["settings-counter-button settings-counter-button-minus", { "settings-counter-button-disabled": __props.settingsState.startNumber <= 1 }]),
              onClick: decreaseNumber,
              "aria-label": _ctx.$translate("main.SettingSidebar.namingSettings.decreaseNumber")
            }, [
              createVNode(_sfc_main$i, {
                name: "minus",
                src: "light/base/ic-slide-bar-minus.svg",
                darkSrc: "dark/base/ic-slide-bar-minus.svg"
              })
            ], 10, _hoisted_8$1),
            createBaseVNode("input", {
              type: "number",
              class: "settings-counter-display",
              value: __props.settingsState.startNumber,
              onBlur: _cache[3] || (_cache[3] = (e) => __props.validateNumberValue(e, "startNumber")),
              tabindex: 8
            }, null, 40, _hoisted_9),
            createBaseVNode("button", {
              class: "settings-counter-button settings-counter-button-plus",
              onClick: increaseNumber,
              "aria-label": _ctx.$translate("main.SettingSidebar.namingSettings.increaseNumber")
            }, [
              createVNode(_sfc_main$i, {
                name: "plus",
                src: "light/base/ic-slide-bar-plus.svg",
                darkSrc: "dark/base/ic-slide-bar-plus.svg"
              })
            ], 8, _hoisted_10)
          ])
        ])) : createCommentVNode("", true)
      ]);
    };
  }
};
const NamingSettingsGroup = /* @__PURE__ */ _export_sfc(_sfc_main$c, [["__scopeId", "data-v-32210aaa"]]);
const _hoisted_1$a = { class: "tip" };
const _hoisted_2$7 = { class: "content" };
const _hoisted_3$4 = { class: "text" };
const _sfc_main$b = {
  __name: "NotifyVue",
  setup(__props, { expose: __expose }) {
    const main = inject("main");
    const notifyRef = ref(null);
    const positionOffset = ref(0);
    const exportNotification = ref({
      show: false,
      outputPath: null,
      successCount: 0,
      totalCount: 0
    });
    const adjustPosition = async () => {
      await nextTick();
      if (notifyRef.value && exportNotification.value.show) {
        const element = notifyRef.value;
        const elementWidth = element.offsetWidth;
        const layoutMainElement = document.querySelector(".layout-main");
        if (layoutMainElement) {
          const layoutMainRect = layoutMainElement.getBoundingClientRect();
          const layoutMainCenterX = layoutMainRect.left + layoutMainRect.width / 2;
          const elementHalfWidth = elementWidth / 2;
          const margin = 24;
          const leftBoundary = layoutMainRect.left + margin;
          const rightBoundary = layoutMainRect.right - margin;
          let targetLeft = layoutMainCenterX;
          if (layoutMainCenterX - elementHalfWidth < leftBoundary) {
            targetLeft = leftBoundary + elementHalfWidth;
          } else if (layoutMainCenterX + elementHalfWidth > rightBoundary) {
            targetLeft = rightBoundary - elementHalfWidth;
          }
          const viewportCenterX = window.innerWidth / 2;
          positionOffset.value = targetLeft - viewportCenterX;
        } else {
          const viewportWidth = window.innerWidth;
          const idealCenterX = viewportWidth / 2;
          const elementHalfWidth = elementWidth / 2;
          const margin = 24;
          const leftBoundary = margin;
          const rightBoundary = viewportWidth - margin;
          let targetLeft = idealCenterX;
          if (idealCenterX - elementHalfWidth < leftBoundary) {
            targetLeft = leftBoundary + elementHalfWidth;
          } else if (idealCenterX + elementHalfWidth > rightBoundary) {
            targetLeft = rightBoundary - elementHalfWidth;
          }
          const centerPosition = viewportWidth / 2;
          positionOffset.value = targetLeft - centerPosition;
        }
      }
    };
    const showExportSuccess = (outputPath, successCount, totalCount = null) => {
      exportNotification.value = {
        show: true,
        outputPath,
        successCount,
        totalCount: totalCount || successCount
      };
      adjustPosition();
    };
    const hideNotification = () => {
      exportNotification.value.show = false;
    };
    const openOutputFolder = async () => {
      if (exportNotification.value.outputPath) {
        try {
          await eagle.shell.showItemInFolder(exportNotification.value.outputPath);
        } catch (error) {
          console.error("Failed to open output folder:", error);
        }
      }
    };
    watch(() => main.status, (newStatus) => {
      if (newStatus === "processing") {
        hideNotification();
      }
    });
    watch(() => exportNotification.value, () => {
      if (exportNotification.value.show) {
        adjustPosition();
      }
    }, { deep: true });
    const handleResize = () => {
      if (exportNotification.value.show) {
        adjustPosition();
      }
    };
    onMounted(() => {
      window.addEventListener("resize", handleResize);
    });
    onUnmounted(() => {
      window.removeEventListener("resize", handleResize);
    });
    __expose({
      showExportSuccess,
      hideNotification
    });
    return (_ctx, _cache) => {
      const _component_el_icon = ElIcon;
      return withDirectives((openBlock(), createElementBlock("div", {
        ref_key: "notifyRef",
        ref: notifyRef,
        class: "notify-vue",
        style: normalizeStyle({ transform: `translateX(calc(-50% + ${positionOffset.value}px))` })
      }, [
        createBaseVNode("div", _hoisted_1$a, [
          createBaseVNode("div", _hoisted_2$7, [
            createVNode(_component_el_icon, { class: "icon icon-success" }),
            createBaseVNode("span", _hoisted_3$4, [
              createTextVNode(toDisplayString(_ctx.$translate("main.notification.exportPartialSuccess", {
                successCount: exportNotification.value.successCount,
                totalCount: exportNotification.value.totalCount
              })) + " ", 1),
              createBaseVNode("span", {
                class: "open-folder-link",
                onClick: openOutputFolder
              }, toDisplayString(_ctx.$translate("main.notification.openFolder")), 1)
            ])
          ]),
          _cache[0] || (_cache[0] = createBaseVNode("div", { class: "divider" }, null, -1)),
          createBaseVNode("button", {
            class: "close-button",
            onClick: hideNotification
          }, [
            createVNode(_component_el_icon, { class: "icon icon-close" })
          ])
        ])
      ], 4)), [
        [vShow, exportNotification.value.show]
      ]);
    };
  }
};
const NotifyVue = /* @__PURE__ */ _export_sfc(_sfc_main$b, [["__scopeId", "data-v-79a859fd"]]);
const ExportStatus = {
  IDLE: "idle",
  EXPORTING: "exporting",
  COMPLETED: "completed",
  CANCELLED: "cancelled",
  ERROR: "error"
};
const ErrorType = {
  UNKNOWN: "unknownError",
  USER_CANCELLED: "userCanceled"
};
const ConflictAction = {
  ASK: "ask",
  REPLACE: "replace",
  KEEP_BOTH: "keepBoth"
};
const DialogType = {
  ERROR: "error",
  CONFIRM: "warning"
};
const createExportState = () => ({
  status: ExportStatus.IDLE,
  isExporting: false,
  currentFrame: 0,
  totalFrames: 0,
  progressPercentage: 0,
  exportStartTime: null,
  exportableTaskCount: 0
});
const createErrorState = () => ({
  visible: false,
  type: DialogType.ERROR,
  errorType: ErrorType.UNKNOWN,
  errorDescription: "",
  ok: () => {
  }
});
const createConflictState = () => ({
  visible: false,
  type: DialogType.CONFIRM,
  conflictFiles: [],
  pendingOperation: null,
  cancel: () => {
  },
  replace: () => {
  },
  keepBoth: () => {
  }
});
const createExportSettings = (settings = {}) => ({
  format: settings.format || "png",
  quality: settings.quality || 80,
  animatedFps: settings.animatedFps || 30,
  codec: settings.codec || null,
  exportType: settings.exportType || "current",
  sizeType: settings.sizeType || "original",
  sizeValue: settings.sizeValue || 100,
  nameType: settings.nameType || "original",
  newFileName: settings.newFileName || "",
  startNumber: settings.startNumber || 1,
  exportCount: settings.exportCount || 0,
  isReplaceMode: settings.isReplaceMode || false,
  runtimeConflictAction: settings.runtimeConflictAction || null,
  keepBothMode: settings.keepBothMode || false
});
const ExportEvents = {
  // 按鈕事件
  EXPORT_CLICKED: "export-clicked",
  CLOSE_CLICKED: "close-clicked",
  // 進度事件
  PROGRESS_CANCEL: "progress-cancel",
  // 錯誤事件
  ERROR_OCCURRED: "error-occurred",
  ERROR_DISMISSED: "error-dismissed",
  // 衝突事件
  CONFLICT_DETECTED: "conflict-detected",
  CONFLICT_RESOLVED: "conflict-resolved",
  // 狀態變更事件
  STATUS_CHANGED: "status-changed",
  PROGRESS_UPDATED: "progress-updated"
};
const ExportButtonProps = {
  isExporting: { type: Boolean, default: false },
  operationMode: { type: String, default: "export" },
  exportableTaskCount: { type: Number, default: 0 },
  processingStatus: { type: String, default: null }
};
const ProgressDialogProps = {
  visible: { type: Boolean, default: false },
  currentFrame: { type: Number, default: 0 },
  totalFrames: { type: Number, default: 0 },
  progressPercentage: { type: Number, default: 0 }
};
const ErrorDialogsProps = {
  errorDialog: { type: Object, required: true },
  conflictDialog: { type: Object, required: true }
};
const ExportOrchestratorProps = {
  exportState: { type: Object, required: true },
  exportSettings: { type: Object, required: true },
  operationMode: { type: String, default: "export" }
};
const _hoisted_1$9 = { class: "export-buttons" };
const _hoisted_2$6 = {
  key: 0,
  class: "button-text"
};
const _hoisted_3$3 = {
  key: 0,
  class: "count"
};
const _sfc_main$a = {
  __name: "ExportButton",
  props: {
    ...ExportButtonProps
  },
  emits: [
    ExportEvents.EXPORT_CLICKED,
    ExportEvents.CLOSE_CLICKED
  ],
  setup(__props, { emit: __emit }) {
    const { proxy } = getCurrentInstance();
    const $translate = proxy.$translate;
    const props = __props;
    const emit = __emit;
    const isProcessing = computed(() => {
      return props.processingStatus === "processing";
    });
    const handleExportClick = () => {
      if (!props.isExporting) {
        emit(ExportEvents.EXPORT_CLICKED);
      }
    };
    const handleCloseClick = () => {
      emit(ExportEvents.CLOSE_CLICKED);
    };
    return (_ctx, _cache) => {
      const _component_el_icon = ElIcon;
      const _component_el_button = ElButton;
      return openBlock(), createElementBlock("div", _hoisted_1$9, [
        createVNode(_component_el_button, {
          type: "primary",
          class: "primary-button",
          onClick: handleExportClick,
          tabindex: 9,
          disabled: _ctx.isExporting,
          "aria-label": unref($translate)(`main.SettingSidebar.operationMode.${_ctx.operationMode}`)
        }, {
          default: withCtx(() => [
            !isProcessing.value ? (openBlock(), createElementBlock("span", _hoisted_2$6, [
              createTextVNode(toDisplayString(unref($translate)(`main.SettingSidebar.operationMode.${_ctx.operationMode}`)) + " ", 1),
              _ctx.exportableTaskCount > 0 ? (openBlock(), createElementBlock("span", _hoisted_3$3, "(" + toDisplayString(_ctx.exportableTaskCount) + ")", 1)) : createCommentVNode("", true)
            ])) : (openBlock(), createBlock(_component_el_icon, {
              key: 1,
              class: "icon icon-processing"
            }))
          ]),
          _: 1
        }, 8, ["disabled", "aria-label"]),
        createVNode(_component_el_button, {
          onClick: handleCloseClick,
          class: "secondary-button",
          tabindex: 10,
          "aria-label": unref($translate)("main.SettingSidebar.header.close")
        }, {
          default: withCtx(() => [
            createTextVNode(toDisplayString(unref($translate)("main.SettingSidebar.header.close")), 1)
          ]),
          _: 1
        }, 8, ["aria-label"])
      ]);
    };
  }
};
const ExportButton = /* @__PURE__ */ _export_sfc(_sfc_main$a, [["__scopeId", "data-v-7bf53f6f"]]);
const _hoisted_1$8 = { class: "eta-component" };
const _sfc_main$9 = {
  __name: "EtaComponent",
  props: {
    curr: {
      type: Number,
      required: true,
      default: 0
    },
    total: {
      type: Number,
      required: true,
      default: 0
    },
    interval: {
      type: Number,
      default: 500
    },
    prefix: {
      type: String,
      default: ""
    }
  },
  setup(__props) {
    const props = __props;
    const smoothedSpeed = ref(null);
    const lastUpdate = ref({ time: 0, value: 0 });
    const hasStartedMoving = ref(false);
    let intervalId = null;
    const addSample = () => {
      const now = Date.now();
      const currentValue = props.curr;
      if (!hasStartedMoving.value && lastUpdate.value.value > 0) {
        const valueDelta = currentValue - lastUpdate.value.value;
        if (valueDelta > 0) {
          hasStartedMoving.value = true;
          lastUpdate.value = { time: now, value: currentValue };
          return;
        }
      }
      if (hasStartedMoving.value && lastUpdate.value.time > 0) {
        const timeDelta = now - lastUpdate.value.time;
        const valueDelta = currentValue - lastUpdate.value.value;
        if (timeDelta > 0 && valueDelta >= 0) {
          const currentSpeed = valueDelta / timeDelta;
          if (smoothedSpeed.value === null) {
            smoothedSpeed.value = currentSpeed;
          } else {
            const alpha = 0.1;
            smoothedSpeed.value = smoothedSpeed.value * (1 - alpha) + currentSpeed * alpha;
          }
        }
      }
      lastUpdate.value = { time: now, value: currentValue };
    };
    const clearSamples = () => {
      smoothedSpeed.value = null;
      lastUpdate.value = { time: 0, value: 0 };
      hasStartedMoving.value = false;
    };
    const estimatedSeconds = computed(() => {
      if (props.curr >= props.total && props.total > 0) {
        return 0;
      }
      if (props.total <= 0 || props.curr < 0) {
        return null;
      }
      if (smoothedSpeed.value === null || smoothedSpeed.value <= 0) {
        return null;
      }
      const remaining = props.total - props.curr;
      const estimatedMs = remaining / smoothedSpeed.value;
      const seconds = Math.floor(estimatedMs / 1e3);
      return Math.min(seconds, 24 * 3600);
    });
    const formatTime = (seconds) => {
      const h = Math.floor(seconds / 3600);
      const m = Math.floor(seconds % 3600 / 60);
      const s = seconds % 60;
      if (h > 0) {
        return `${h}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
      } else {
        return `${m}:${String(s).padStart(2, "0")}`;
      }
    };
    const formattedEta = computed(() => {
      const seconds = estimatedSeconds.value;
      if (props.curr >= props.total && props.total > 0) {
        const timeStr2 = "0:00";
        return props.prefix ? `${props.prefix} (${timeStr2})` : `(${timeStr2})`;
      }
      if (seconds === null || seconds <= 0) {
        return props.prefix || "";
      }
      const timeStr = formatTime(seconds);
      return props.prefix ? `${props.prefix} (${timeStr})` : `(${timeStr})`;
    });
    watch(() => props.curr, (newVal, oldVal) => {
      if (oldVal && newVal < oldVal * 0.5) {
        clearSamples();
      }
    });
    watch(() => props.total, () => {
      clearSamples();
    });
    onMounted(() => {
      addSample();
      intervalId = setInterval(addSample, props.interval);
    });
    onUnmounted(() => {
      if (intervalId) {
        clearInterval(intervalId);
        intervalId = null;
      }
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("span", _hoisted_1$8, toDisplayString(formattedEta.value), 1);
    };
  }
};
const EtaComponent = /* @__PURE__ */ _export_sfc(_sfc_main$9, [["__scopeId", "data-v-81385d6b"]]);
const _hoisted_1$7 = {
  key: 0,
  class: "progress-dialog-wrapper"
};
const _hoisted_2$5 = { class: "progress-container" };
const _hoisted_3$2 = { class: "progress-box" };
const _hoisted_4$1 = { class: "progress-text" };
const _hoisted_5 = { class: "progress-info" };
const _hoisted_6 = { class: "progress-bar-wrapper" };
const _hoisted_7 = { class: "progress-bar" };
const _hoisted_8 = ["aria-label"];
const _sfc_main$8 = {
  __name: "ProgressDialog",
  props: {
    ...ProgressDialogProps
  },
  emits: [
    ExportEvents.PROGRESS_CANCEL
  ],
  setup(__props, { emit: __emit }) {
    const { proxy } = getCurrentInstance();
    const $translate = proxy.$translate;
    const props = __props;
    const emit = __emit;
    const progressInfo = computed(() => {
      const current = props.currentFrame || 0;
      const total = props.totalFrames || 0;
      if (total === 0) {
        return "0/0";
      }
      return `${current}/${total}`;
    });
    const progressBarWidth = computed(() => {
      if (props.progressPercentage < 0) return "0%";
      if (props.progressPercentage > 100) return "100%";
      return `${props.progressPercentage}%`;
    });
    const handleCancelClick = () => {
      emit(ExportEvents.PROGRESS_CANCEL);
    };
    const handleOverlayClick = () => {
    };
    return (_ctx, _cache) => {
      return _ctx.visible ? (openBlock(), createElementBlock("div", _hoisted_1$7, [
        createBaseVNode("div", {
          class: "overlay",
          onClick: handleOverlayClick
        }),
        createBaseVNode("div", _hoisted_2$5, [
          createBaseVNode("div", _hoisted_3$2, [
            createBaseVNode("div", _hoisted_4$1, [
              createTextVNode(toDisplayString(unref($translate)("main.progressDialog.title")) + " ", 1),
              createBaseVNode("span", _hoisted_5, toDisplayString(progressInfo.value), 1),
              createVNode(EtaComponent, {
                curr: _ctx.currentFrame,
                total: _ctx.totalFrames,
                interval: 500
              }, null, 8, ["curr", "total"])
            ]),
            createBaseVNode("div", _hoisted_6, [
              createBaseVNode("div", _hoisted_7, [
                createBaseVNode("div", {
                  class: "progress-bar-fill",
                  style: normalizeStyle({ width: progressBarWidth.value })
                }, null, 4)
              ])
            ]),
            createBaseVNode("button", {
              class: "progress-cancel-button",
              onClick: handleCancelClick,
              "aria-label": unref($translate)("main.progressDialog.cancel")
            }, toDisplayString(unref($translate)("main.progressDialog.cancel")), 9, _hoisted_8)
          ])
        ])
      ])) : createCommentVNode("", true);
    };
  }
};
const ProgressDialog = /* @__PURE__ */ _export_sfc(_sfc_main$8, [["__scopeId", "data-v-6aadd319"]]);
const _hoisted_1$6 = { class: "dialog-container" };
const _hoisted_2$4 = { class: "main" };
const _hoisted_3$1 = { class: "title" };
const _hoisted_4 = { class: "description" };
const _sfc_main$7 = {
  __name: "DialogVue",
  props: {
    modelValue: {
      type: Boolean,
      default: false,
      required: true
    },
    type: {
      type: String,
      default: "warning",
      required: true
    },
    closeOnClickModal: {
      type: Boolean,
      default: true
    },
    showCancelBtn: {
      type: Boolean,
      default: true
    },
    showOkBtn: {
      type: Boolean,
      default: true
    },
    // 新增：第三個按鈕支援
    showThirdBtn: {
      type: Boolean,
      default: false
    },
    thirdBtnType: {
      type: String,
      default: "default"
    }
  },
  emits: ["ok", "cancel", "third", "update:modelValue"],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emit = __emit;
    const ok = () => {
      emit("ok");
      visible.value = false;
    };
    const cancel = () => {
      emit("cancel");
      visible.value = false;
    };
    const third = () => {
      emit("third");
      visible.value = false;
    };
    const visible = computed({
      get: () => props.modelValue,
      set: (value) => {
        emit("update:modelValue", value);
      }
    });
    return (_ctx, _cache) => {
      const _component_el_button = ElButton;
      const _component_el_dialog = ElDialog;
      return openBlock(), createBlock(_component_el_dialog, {
        modelValue: visible.value,
        "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => visible.value = $event),
        class: "dialog-vue",
        "append-to-body": "",
        "align-center": "",
        onClose: cancel,
        "close-on-click-modal": props.closeOnClickModal
      }, {
        default: withCtx(() => [
          createBaseVNode("div", _hoisted_1$6, [
            createVNode(_sfc_main$i, {
              class: "dialog-icon",
              width: "36",
              height: "36",
              src: `light/base/dialog-${props.type}.png`,
              darkSrc: `dark/base/dialog-${props.type}.png`
            }, null, 8, ["src", "darkSrc"]),
            createBaseVNode("div", _hoisted_2$4, [
              createBaseVNode("div", _hoisted_3$1, [
                renderSlot(_ctx.$slots, "title", {}, () => [
                  _cache[1] || (_cache[1] = createTextVNode("title", -1))
                ])
              ]),
              createBaseVNode("div", _hoisted_4, [
                renderSlot(_ctx.$slots, "description", {}, () => [
                  _cache[2] || (_cache[2] = createTextVNode("description", -1))
                ])
              ]),
              createBaseVNode("div", {
                class: normalizeClass(["action", { "action--three-buttons": props.showThirdBtn }])
              }, [
                props.showCancelBtn ? (openBlock(), createBlock(_component_el_button, {
                  key: 0,
                  class: "cancel",
                  type: "",
                  onClick: cancel
                }, {
                  default: withCtx(() => [
                    renderSlot(_ctx.$slots, "cancel", {}, () => [
                      _cache[3] || (_cache[3] = createTextVNode("cancel", -1))
                    ])
                  ]),
                  _: 3
                })) : createCommentVNode("", true),
                props.showThirdBtn ? (openBlock(), createBlock(_component_el_button, {
                  key: 1,
                  class: "third",
                  type: props.thirdBtnType,
                  onClick: third
                }, {
                  default: withCtx(() => [
                    renderSlot(_ctx.$slots, "third", {}, () => [
                      _cache[4] || (_cache[4] = createTextVNode("third", -1))
                    ])
                  ]),
                  _: 3
                }, 8, ["type"])) : createCommentVNode("", true),
                props.showOkBtn ? (openBlock(), createBlock(_component_el_button, {
                  key: 2,
                  class: "ok",
                  type: "primary",
                  onClick: ok
                }, {
                  default: withCtx(() => [
                    renderSlot(_ctx.$slots, "ok", {}, () => [
                      _cache[5] || (_cache[5] = createTextVNode("ok", -1))
                    ])
                  ]),
                  _: 3
                })) : createCommentVNode("", true)
              ], 2)
            ])
          ])
        ]),
        _: 3
      }, 8, ["modelValue", "close-on-click-modal"]);
    };
  }
};
const _hoisted_1$5 = { class: "error-dialogs" };
const _hoisted_2$3 = ["innerHTML"];
const _sfc_main$6 = {
  __name: "ErrorDialogs",
  props: {
    ...ErrorDialogsProps
  },
  emits: [
    ExportEvents.ERROR_DISMISSED,
    ExportEvents.CONFLICT_RESOLVED
  ],
  setup(__props, { emit: __emit }) {
    const { proxy } = getCurrentInstance();
    const $translate = proxy.$translate;
    const props = __props;
    const emit = __emit;
    const localErrorDialog = reactive({
      visible: false,
      type: "error",
      errorType: ErrorType.UNKNOWN,
      errorDescription: ""
    });
    const localConflictDialog = reactive({
      visible: false,
      type: "warning",
      conflictFiles: []
    });
    watch(() => props.errorDialog, (newVal) => {
      if (newVal) {
        Object.assign(localErrorDialog, newVal);
      }
    }, { deep: true, immediate: true });
    watch(() => props.conflictDialog, (newVal) => {
      if (newVal) {
        Object.assign(localConflictDialog, newVal);
      }
    }, { deep: true, immediate: true });
    computed(() => localErrorDialog.errorType === ErrorType.UNKNOWN);
    const getConflictDescription = () => {
      var _a;
      return $translate("main.SettingSidebar.dialogs.fileDuplication.description", { count: ((_a = localConflictDialog.conflictFiles) == null ? void 0 : _a.length) || 0 });
    };
    const handleConflictCancel = () => {
      localConflictDialog.visible = false;
      emit(ExportEvents.CONFLICT_RESOLVED, {
        action: ConflictAction.ASK,
        cancelled: true
      });
    };
    const handleConflictReplace = () => {
      localConflictDialog.visible = false;
      emit(ExportEvents.CONFLICT_RESOLVED, {
        action: ConflictAction.REPLACE,
        cancelled: false,
        conflictFiles: localConflictDialog.conflictFiles
      });
    };
    const handleConflictKeepBoth = () => {
      localConflictDialog.visible = false;
      emit(ExportEvents.CONFLICT_RESOLVED, {
        action: ConflictAction.KEEP_BOTH,
        cancelled: false,
        conflictFiles: localConflictDialog.conflictFiles
      });
    };
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$5, [
        createVNode(_sfc_main$7, {
          modelValue: localConflictDialog.visible,
          "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => localConflictDialog.visible = $event),
          type: localConflictDialog.type,
          showThirdBtn: true,
          thirdBtnType: "default",
          onCancel: handleConflictCancel,
          onOk: handleConflictReplace,
          onThird: handleConflictKeepBoth
        }, {
          title: withCtx(() => [
            createTextVNode(toDisplayString(unref($translate)("main.SettingSidebar.dialogs.fileDuplication.title")), 1)
          ]),
          description: withCtx(() => [
            createBaseVNode("div", {
              innerHTML: getConflictDescription()
            }, null, 8, _hoisted_2$3)
          ]),
          cancel: withCtx(() => [
            createTextVNode(toDisplayString(unref($translate)("main.SettingSidebar.dialogs.fileDuplication.cancel")), 1)
          ]),
          third: withCtx(() => [
            createTextVNode(toDisplayString(unref($translate)("main.SettingSidebar.dialogs.fileDuplication.keepBoth")), 1)
          ]),
          ok: withCtx(() => [
            createTextVNode(toDisplayString(unref($translate)("main.SettingSidebar.dialogs.fileDuplication.replace")), 1)
          ]),
          _: 1
        }, 8, ["modelValue", "type"])
      ]);
    };
  }
};
const scriptRel = "modulepreload";
const assetsURL = function(dep, importerUrl) {
  return new URL(dep, importerUrl).href;
};
const seen = {};
const __vitePreload = function preload(baseModule, deps, importerUrl) {
  let promise = Promise.resolve();
  if (deps && deps.length > 0) {
    const links = document.getElementsByTagName("link");
    const cspNonceMeta = document.querySelector(
      "meta[property=csp-nonce]"
    );
    const cspNonce = (cspNonceMeta == null ? void 0 : cspNonceMeta.nonce) || (cspNonceMeta == null ? void 0 : cspNonceMeta.getAttribute("nonce"));
    promise = Promise.allSettled(
      deps.map((dep) => {
        dep = assetsURL(dep, importerUrl);
        if (dep in seen) return;
        seen[dep] = true;
        const isCss = dep.endsWith(".css");
        const cssSelector = isCss ? '[rel="stylesheet"]' : "";
        const isBaseRelative = !!importerUrl;
        if (isBaseRelative) {
          for (let i = links.length - 1; i >= 0; i--) {
            const link2 = links[i];
            if (link2.href === dep && (!isCss || link2.rel === "stylesheet")) {
              return;
            }
          }
        } else if (document.querySelector(`link[href="${dep}"]${cssSelector}`)) {
          return;
        }
        const link = document.createElement("link");
        link.rel = isCss ? "stylesheet" : scriptRel;
        if (!isCss) {
          link.as = "script";
        }
        link.crossOrigin = "";
        link.href = dep;
        if (cspNonce) {
          link.setAttribute("nonce", cspNonce);
        }
        document.head.appendChild(link);
        if (isCss) {
          return new Promise((res, rej) => {
            link.addEventListener("load", res);
            link.addEventListener(
              "error",
              () => rej(new Error(`Unable to preload CSS for ${dep}`))
            );
          });
        }
      })
    );
  }
  function handlePreloadError(err) {
    const e = new Event("vite:preloadError", {
      cancelable: true
    });
    e.payload = err;
    window.dispatchEvent(e);
    if (!e.defaultPrevented) {
      throw err;
    }
  }
  return promise.then((res) => {
    for (const item of res || []) {
      if (item.status !== "rejected") continue;
      handlePreloadError(item.reason);
    }
    return baseModule().catch(handlePreloadError);
  });
};
const convertSettings = (settings) => {
  return createExportSettings({
    format: settings.format,
    quality: settings.quality,
    animatedFps: settings.animatedFps,
    codec: settings.codec,
    exportType: settings.exportType,
    sizeType: settings.sizeType,
    sizeValue: settings.sizeValue,
    nameType: settings.nameType,
    newFileName: settings.newFileName,
    startNumber: settings.startNumber,
    exportCount: settings.exportCount
  });
};
const getOutputPath = async (items, operationMode) => {
  if (operationMode === "replace") {
    const firstItemPath = items[0].filePath;
    if (!firstItemPath) {
      throw new Error(i18next.t("error.cannotDetermineSourcePath"));
    }
    return path.dirname(firstItemPath);
  }
  const outputDialog = await eagle.dialog.showOpenDialog({
    properties: ["openDirectory", "createDirectory"]
  });
  if (outputDialog.canceled) {
    return null;
  }
  return outputDialog.filePaths[0];
};
const checkFileConflicts = async (items, outputPath, settings) => {
  const fs2 = require("fs");
  const path2 = require("path");
  const { FileNameAllocator: FileNameAllocator2 } = await __vitePreload(async () => {
    const { FileNameAllocator: FileNameAllocator3 } = await Promise.resolve().then(() => taskBatcher);
    return { FileNameAllocator: FileNameAllocator3 };
  }, true ? void 0 : void 0, import.meta.url);
  const conflictFiles = [];
  const fileNameAllocator = new FileNameAllocator2();
  const tasksForAllocation = items.map((item) => ({
    id: item.id,
    fileName: item.name || item.filePath.split("/").pop().split("\\").pop().split(".").shift(),
    name: item.name,
    format: settings.format === "original" ? item.ext : settings.format,
    ext: item.ext
  }));
  const allocatedNames = fileNameAllocator.allocateFileNames(tasksForAllocation, {
    nameType: settings.nameType,
    customName: settings.newFileName,
    startNumber: parseInt(settings.startNumber) || 1
  });
  const nameMap = /* @__PURE__ */ new Map();
  allocatedNames.forEach((allocation) => {
    nameMap.set(allocation.id, allocation.exportName);
  });
  for (const item of items) {
    const fileName = nameMap.get(item.id);
    const outputFormat = settings.format === "original" ? item.ext : settings.format;
    const outputFileName = `${fileName}.${outputFormat}`;
    const outputFilePath = path2.join(outputPath, outputFileName);
    try {
      await fs2.promises.access(outputFilePath);
      conflictFiles.push({
        originalFile: item.filePath,
        outputFile: outputFilePath,
        fileName: outputFileName,
        itemName: item.name
      });
    } catch (error) {
    }
  }
  return conflictFiles;
};
const createTaskStateUpdater = (tasks, updateTaskStatus) => {
  return (summary) => {
    var _a, _b;
    (_a = summary.successfulTasks) == null ? void 0 : _a.forEach((result) => {
      if (result.success) {
        const task = tasks.value.find((t) => {
          var _a2;
          return ((_a2 = t.item) == null ? void 0 : _a2.filePath) === result.src;
        });
        if (task && task.status !== "success") {
          updateTaskStatus(task.id, "success", 100);
        }
      }
    });
    (_b = summary.failedTasks) == null ? void 0 : _b.forEach((error) => {
      const task = tasks.value.find((t) => t.filePath === error.src);
      if (task && task.status !== "failed") {
        updateTaskStatus(task.id, "failed", 0, error.error);
      }
    });
  };
};
const createSettingsSaver = (exportSettings, main, convertSettings2) => {
  return () => {
    const settingsToSave = {
      ...convertSettings2(exportSettings.value),
      preset: main.localStorageSetting.preset
    };
    localStorage.setItem(main.localStorageKey, JSON.stringify(settingsToSave));
  };
};
const createExportCallbacks = (emit, cleanup, updateTaskStates, saveSettings) => ({
  onTaskComplete: (result) => {
    emit(ExportEvents.PROGRESS_UPDATED, {
      currentFrame: result.currentFrame || 0,
      totalFrames: result.totalFrames || 0,
      progressPercentage: result.totalFrames > 0 ? Math.round((result.currentFrame || 0) / result.totalFrames * 100) : 0
    });
  },
  onProgress: (progress) => {
    emit(ExportEvents.PROGRESS_UPDATED, { progressPercentage: progress });
  },
  onError: (error) => {
    const errorMessage = Object.entries(error).map(([key, value]) => `${key}: ${value}`).join(", ");
    console.error("Task failed:", errorMessage);
  },
  onCancelled: (summary) => {
    emit(ExportEvents.STATUS_CHANGED, ExportStatus.CANCELLED);
    cleanup();
  },
  onComplete: (summary) => {
    updateTaskStates(summary);
    saveSettings();
    emit(ExportEvents.STATUS_CHANGED, ExportStatus.COMPLETED);
    emit(ExportEvents.PROGRESS_UPDATED, { completed: true, summary });
    cleanup();
  }
});
const _hoisted_1$4 = { class: "export-orchestrator" };
const _sfc_main$5 = {
  __name: "ExportOrchestrator",
  props: {
    ...ExportOrchestratorProps
  },
  emits: [
    ExportEvents.STATUS_CHANGED,
    ExportEvents.PROGRESS_UPDATED,
    ExportEvents.ERROR_OCCURRED,
    ExportEvents.CONFLICT_DETECTED
  ],
  setup(__props, { expose: __expose, emit: __emit }) {
    const emit = __emit;
    const main = inject("main");
    const exportSettings = inject("exportSettings");
    const { operationMode } = inject("modeManager");
    const { updateTaskStatus, updateTaskNewFormat, tasks, pauseAllTasks } = inject("taskManager");
    const batchConflictAction = ref(null);
    const handleFileConflicts = async (items, outputPath, settings, conflictAction = ConflictAction.ASK) => {
      if (operationMode.value === "replace") {
        emit(ExportEvents.STATUS_CHANGED, ExportStatus.EXPORTING);
        return await main.convertBatch(items, outputPath, settings);
      }
      const conflicts = await checkFileConflicts(items, outputPath, settings);
      const action = batchConflictAction.value || conflictAction;
      const createSettings = (additionalProps = {}) => ({
        ...settings,
        runtimeConflictAction: action === ConflictAction.ASK ? null : action,
        keepBothMode: action === ConflictAction.KEEP_BOTH,
        ...additionalProps
      });
      if (conflicts.length === 0 || action !== ConflictAction.ASK) {
        emit(ExportEvents.STATUS_CHANGED, ExportStatus.EXPORTING);
        return await main.convertBatch(items, outputPath, createSettings());
      }
      return new Promise((resolve, reject) => {
        emit(ExportEvents.CONFLICT_DETECTED, {
          conflictFiles: conflicts,
          resolve: async (userAction) => {
            try {
              if (userAction && userAction.cancelled) {
                cleanup();
                resolve();
                return;
              }
              batchConflictAction.value = userAction;
              const result = await handleFileConflicts(items, outputPath, settings, userAction);
              resolve(result);
            } catch (error) {
              reject(error);
            }
          },
          reject
        });
      });
    };
    const updateTaskStates = createTaskStateUpdater(tasks, updateTaskStatus);
    const saveSettings = createSettingsSaver(exportSettings, main, convertSettings);
    const createBatchCallbacks = () => createExportCallbacks(emit, cleanup, updateTaskStates, saveSettings);
    const cleanup = () => {
      batchConflictAction.value = null;
    };
    const executeExport = async () => {
      try {
        const validTasks = tasks.value.filter(
          (task) => task.item && task.status !== "failed" && task.isFormatSupported !== false
        );
        if (validTasks.length === 0) {
          throw new Error(i18next.t("error.noItemsSelected"));
        }
        const startTime = Date.now();
        emit(ExportEvents.PROGRESS_UPDATED, {
          currentFrame: 0,
          totalFrames: validTasks.length,
          exportStartTime: startTime
        });
        const eagleItems = validTasks.map((task) => task.item);
        const outputPath = await getOutputPath(eagleItems, operationMode.value);
        if (outputPath === null) {
          cleanup();
          return;
        }
        const settings = {
          ...convertSettings(exportSettings.value),
          isReplaceMode: operationMode.value === "replace",
          callbacks: createBatchCallbacks()
        };
        const items = validTasks.map((task) => {
          updateTaskNewFormat(task.id, settings.format);
          return task.item;
        });
        await handleFileConflicts(items, outputPath, settings);
      } catch (error) {
        cleanup();
        if (error.message.includes("userCanceled") || error.message.includes("User cancelled")) {
          return;
        }
        emit(ExportEvents.ERROR_OCCURRED, {
          type: ErrorType.UNKNOWN,
          description: error.message
        });
      }
    };
    const cancelExport = async () => {
      if (main.cancelConversion) main.cancelConversion();
      if (pauseAllTasks) {
        await pauseAllTasks();
      }
      cleanup();
      emit(ExportEvents.STATUS_CHANGED, ExportStatus.CANCELLED);
    };
    __expose({
      executeExport,
      cancelExport,
      setBatchConflictAction: (action) => {
        batchConflictAction.value = action;
      },
      clearBatchConflictAction: () => {
        batchConflictAction.value = null;
      }
    });
    onUnmounted(cleanup);
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$4);
    };
  }
};
const ExportOrchestrator = /* @__PURE__ */ _export_sfc(_sfc_main$5, [["__scopeId", "data-v-f6cd75ab"]]);
const _hoisted_1$3 = { class: "export-actions" };
const _sfc_main$4 = {
  __name: "ExportActions",
  setup(__props) {
    const main = inject("main");
    const exportSettings = inject("exportSettings");
    const { operationMode } = inject("modeManager");
    const { tasks, updateTaskStatus } = inject("taskManager");
    const triggerExport = inject("triggerExport");
    const notifyVueRef = ref(null);
    const exportOrchestratorRef = ref(null);
    const exportState = reactive(createExportState());
    const errorDialogState = reactive(createErrorState());
    const conflictDialogState = reactive(createConflictState());
    const resetExportState = () => {
      exportState.currentFrame = 0;
      exportState.totalFrames = 0;
      exportState.progressPercentage = 0;
      exportState.exportStartTime = null;
    };
    const handleExportClick = () => {
      if (exportOrchestratorRef.value && !exportState.isExporting) {
        resetExportState();
        exportOrchestratorRef.value.executeExport();
      }
    };
    const handleCloseClick = () => {
      window.close();
    };
    const handleProgressCancel = async () => {
      if (exportOrchestratorRef.value) {
        await exportOrchestratorRef.value.cancelExport();
        resetExportState();
      }
    };
    const handleErrorDismissed = () => {
      errorDialogState.visible = false;
    };
    const handleConflictResolved = (resolution) => {
      conflictDialogState.visible = false;
      if (conflictDialogState.pendingOperation) {
        if (resolution.cancelled) {
          conflictDialogState.pendingOperation.resolve({ cancelled: true });
        } else {
          conflictDialogState.pendingOperation.resolve(resolution.action);
        }
        conflictDialogState.pendingOperation = null;
      }
    };
    const handleStatusChanged = (status) => {
      exportState.isExporting = status === ExportStatus.EXPORTING;
      if (status === ExportStatus.COMPLETED || status === ExportStatus.CANCELLED || status === ExportStatus.ERROR) {
        setTimeout(() => {
          if (!exportState.isExporting) {
            resetExportState();
          }
        }, 1e3);
      }
    };
    const handleProgressUpdated = (progress) => {
      var _a;
      if (progress.currentFrame !== void 0) {
        exportState.currentFrame = progress.currentFrame;
      }
      if (progress.totalFrames !== void 0) {
        exportState.totalFrames = progress.totalFrames;
      }
      if (progress.progressPercentage !== void 0) {
        exportState.progressPercentage = progress.progressPercentage;
      }
      if (progress.exportStartTime !== void 0) {
        exportState.exportStartTime = progress.exportStartTime;
      }
      if (progress.completed && progress.summary) {
        const summary = progress.summary;
        const successCount = ((_a = summary.successfulTasks) == null ? void 0 : _a.length) || 0;
        if (successCount > 0 && notifyVueRef.value) {
          const lastSuccessfulTask = summary.successfulTasks[summary.successfulTasks.length - 1];
          if (lastSuccessfulTask && lastSuccessfulTask.outputPath) {
            notifyVueRef.value.showExportSuccess(
              lastSuccessfulTask.outputPath,
              successCount,
              main.items.length
            );
          }
        }
      }
    };
    const handleErrorOccurred = (error) => {
      if (error.type === ErrorType.USER_CANCELLED) {
        resetExportState();
        return;
      }
      resetExportState();
      errorDialogState.visible = true;
      errorDialogState.errorType = error.type || ErrorType.UNKNOWN;
      errorDialogState.errorDescription = error.description || "";
    };
    const handleConflictDetected = (conflictData) => {
      conflictDialogState.visible = true;
      conflictDialogState.conflictFiles = conflictData.conflictFiles || [];
      conflictDialogState.pendingOperation = {
        resolve: conflictData.resolve,
        reject: conflictData.reject
      };
    };
    const updateExportableTaskCount = () => {
      var _a, _b;
      exportState.exportableTaskCount = ((_a = tasks.value) == null ? void 0 : _a.length) > 0 ? tasks.value.filter((task) => task.status === "success" && task.item).length : ((_b = main.items) == null ? void 0 : _b.length) || 0;
    };
    watch(triggerExport, () => !exportState.isExporting && handleExportClick());
    watch(tasks, updateExportableTaskCount, { deep: true });
    watch(() => main.items, updateExportableTaskCount, { deep: true });
    onMounted(() => {
      updateExportableTaskCount();
    });
    return (_ctx, _cache) => {
      var _a;
      return openBlock(), createElementBlock("div", _hoisted_1$3, [
        createVNode(ExportButton, {
          "is-exporting": exportState.isExporting,
          "operation-mode": unref(operationMode),
          "exportable-task-count": exportState.exportableTaskCount,
          "processing-status": (_a = unref(main)) == null ? void 0 : _a.status,
          onExportClicked: handleExportClick,
          onCloseClicked: handleCloseClick
        }, null, 8, ["is-exporting", "operation-mode", "exportable-task-count", "processing-status"]),
        createVNode(_sfc_main$6, {
          "error-dialog": errorDialogState,
          "conflict-dialog": conflictDialogState,
          onErrorDismissed: handleErrorDismissed,
          onConflictResolved: handleConflictResolved
        }, null, 8, ["error-dialog", "conflict-dialog"]),
        createVNode(ProgressDialog, {
          visible: exportState.isExporting,
          "current-frame": exportState.currentFrame,
          "total-frames": exportState.totalFrames,
          "progress-percentage": exportState.progressPercentage,
          onProgressCancel: handleProgressCancel
        }, null, 8, ["visible", "current-frame", "total-frames", "progress-percentage"]),
        createVNode(NotifyVue, {
          ref_key: "notifyVueRef",
          ref: notifyVueRef
        }, null, 512),
        createVNode(ExportOrchestrator, {
          ref_key: "exportOrchestratorRef",
          ref: exportOrchestratorRef,
          "export-state": exportState,
          "export-settings": unref(exportSettings),
          "operation-mode": unref(operationMode),
          onStatusChanged: handleStatusChanged,
          onProgressUpdated: handleProgressUpdated,
          onErrorOccurred: handleErrorOccurred,
          onConflictDetected: handleConflictDetected
        }, null, 8, ["export-state", "export-settings", "operation-mode"])
      ]);
    };
  }
};
const ExportActions = /* @__PURE__ */ _export_sfc(_sfc_main$4, [["__scopeId", "data-v-681f12af"]]);
const FORMAT_CONFIG = {
  original: { hasQuality: false, hasAnimation: false, extensions: [] },
  jpg: { hasQuality: true, hasAnimation: false, extensions: ["jpg", "jpeg"] },
  png: { hasQuality: false, hasAnimation: true, extensions: ["png"] },
  webp: { hasQuality: true, hasAnimation: true, extensions: ["webp"] },
  bmp: { hasQuality: false, hasAnimation: false, extensions: ["bmp"] },
  gif: { hasQuality: false, hasAnimation: true, extensions: ["gif"] },
  tif: { hasQuality: false, hasAnimation: false, extensions: ["tif", "tiff"] },
  avif: { hasQuality: true, hasAnimation: false, extensions: ["avif"] },
  jxl: { hasQuality: true, hasAnimation: false, extensions: ["jxl"] },
  ico: { hasQuality: false, hasAnimation: false, extensions: ["ico"] },
  exr: { hasQuality: false, hasAnimation: false, extensions: ["exr"] },
  hdr: { hasQuality: false, hasAnimation: false, extensions: ["hdr"] },
  tga: { hasQuality: false, hasAnimation: false, extensions: ["tga"] },
  mp4: { hasQuality: true, hasAnimation: true, hasCodec: true, extensions: ["mp4"] },
  webm: { hasQuality: true, hasAnimation: true, hasCodec: true, extensions: ["webm"] }
};
const VALIDATION_RULES = {
  sizeValue: (v, defaultValue = 900) => Math.max(1, Math.floor(Number(v) || defaultValue)),
  startNumber: (v, defaultValue = 1) => Math.max(1, Math.floor(Number(v) || defaultValue)),
  quality: (v, defaultValue = 100) => Math.max(5, Math.min(100, Math.floor(Number(v) || defaultValue)))
};
const generateQualityOptions = () => {
  const options = [];
  for (let i = 100; i > 0; i -= 5) {
    options.push(i);
  }
  return options;
};
function useSettingsState() {
  const main = inject("main");
  const updateExportSettings = inject("updateExportSettings");
  const { updateAllTasksNewFormat } = inject("taskManager");
  const localStorageSetting = (main == null ? void 0 : main.localStorageSetting) || {};
  const settingsState = reactive({
    // 格式設置
    format: (localStorageSetting == null ? void 0 : localStorageSetting.format) || "jpg",
    quality: (localStorageSetting == null ? void 0 : localStorageSetting.quality) || 100,
    animatedFps: (localStorageSetting == null ? void 0 : localStorageSetting.animatedFps) || 30,
    codec: (localStorageSetting == null ? void 0 : localStorageSetting.codec) || null,
    // 尺寸設置
    sizeType: (localStorageSetting == null ? void 0 : localStorageSetting.sizeType) || "original",
    sizeValue: (localStorageSetting == null ? void 0 : localStorageSetting.sizeValue) || 900,
    // 命名設置
    nameType: (localStorageSetting == null ? void 0 : localStorageSetting.nameType) || "original",
    newFileName: (localStorageSetting == null ? void 0 : localStorageSetting.newFileName) || "newFileName",
    startNumber: (localStorageSetting == null ? void 0 : localStorageSetting.startNumber) || 1
  });
  const options = {
    formatOptions: [
      {
        label: "original",
        options: ["original"]
      },
      {
        label: "common",
        options: ["jpg", "png", "bmp", "gif", "tif", "ico"]
      },
      {
        label: "nextGen",
        options: ["webp", "avif"]
      },
      {
        label: "other",
        options: ["hdr", "exr", "tga"]
      },
      {
        label: "video",
        options: ["mp4", "webm"]
      }
    ],
    qualityOptions: generateQualityOptions(),
    codecOptions: [
      {
        label: "mp4",
        options: ["h264", "h265"]
      },
      {
        label: "webm",
        options: ["vp8", "vp9"]
      }
    ],
    sizeOptions: [
      "original",
      "maxWidth",
      "maxHeight",
      "minWidth",
      "minHeight",
      "maxSide",
      "minSide"
    ],
    fpsOptions: ["sameAsSource", 5, 10, 12, 15, 20, 23.976, 24, 25, 29.97, 30, 48, 50, 59.94, 60]
  };
  const dropdownStates = ref({
    format: false,
    quality: false,
    size: false,
    fps: false,
    codec: false,
    nameType: false
  });
  const updateSetting = (key, value) => {
    const validator = VALIDATION_RULES[key];
    const defaultValue = localStorageSetting == null ? void 0 : localStorageSetting[key];
    const validatedValue = validator ? validator(value, defaultValue) : value;
    settingsState[key] = validatedValue;
    syncToParent();
    if (key === "format") {
      updateAllTasksNewFormat(validatedValue);
    }
  };
  const syncToParent = () => {
    if (updateExportSettings) {
      updateExportSettings({
        format: settingsState.format,
        quality: settingsState.quality,
        animatedFps: settingsState.animatedFps,
        codec: settingsState.codec,
        sizeType: settingsState.sizeType,
        sizeValue: settingsState.sizeValue,
        nameType: settingsState.nameType,
        newFileName: settingsState.newFileName,
        startNumber: settingsState.startNumber
      });
    }
  };
  const updateDropdownState = (key, visible) => {
    dropdownStates.value[key] = visible;
  };
  const validateNumberValue = (event, fieldType) => {
    const currentValue = event.target.value;
    const numValue = Number(currentValue);
    if (!isNaN(numValue) && currentValue !== "") {
      updateSetting(fieldType, numValue);
      if (settingsState[fieldType] !== numValue) {
        event.target.value = settingsState[fieldType];
      }
    } else {
      event.target.value = settingsState[fieldType];
    }
  };
  watch(
    settingsState,
    () => {
      syncToParent();
    },
    { deep: true }
  );
  syncToParent();
  return {
    settingsState,
    options,
    dropdownStates,
    updateSetting,
    updateDropdownState,
    validateNumberValue,
    syncToParent
  };
}
function useSettingsValidation(settingsState) {
  const { tasks } = inject("taskManager");
  const shouldShowQuality = computed(() => {
    const config = FORMAT_CONFIG[settingsState.format];
    return (config == null ? void 0 : config.hasQuality) ?? false;
  });
  const shouldShowFps = computed(() => {
    const config = FORMAT_CONFIG[settingsState.format];
    if (!(config == null ? void 0 : config.hasAnimation)) return false;
    const isAnimatedFormat2 = ["png", "webp", "gif", "mp4", "webm", "mov", "m4v", "mkv"];
    return tasks.value.some(
      (task) => {
        var _a;
        return isAnimatedFormat2.includes((_a = task.item) == null ? void 0 : _a.ext);
      }
    );
  });
  const shouldShowSizeInput = computed(() => {
    return settingsState.sizeType !== "original";
  });
  const shouldShowCodec = computed(() => {
    const config = FORMAT_CONFIG[settingsState.format];
    return (config == null ? void 0 : config.hasCodec) ?? false;
  });
  const validatePresetForm = (presetData) => {
    const { name, format, quality, animatedFps, sizeType } = presetData;
    if (!name || !format || !sizeType) {
      return false;
    }
    const config = FORMAT_CONFIG[format];
    if ((config == null ? void 0 : config.hasQuality) && !quality) {
      return false;
    }
    if ((config == null ? void 0 : config.hasAnimation) && shouldShowFps.value && !animatedFps) {
      return false;
    }
    return true;
  };
  const getFormatConfig = (format) => {
    return FORMAT_CONFIG[format] || FORMAT_CONFIG.original;
  };
  const isSupportedFormat = (format) => {
    return Object.keys(FORMAT_CONFIG).includes(format);
  };
  const isAnimatedFormat = (format) => {
    var _a;
    return ((_a = FORMAT_CONFIG[format]) == null ? void 0 : _a.hasAnimation) ?? false;
  };
  const isLossyFormat = (format) => {
    var _a;
    return ((_a = FORMAT_CONFIG[format]) == null ? void 0 : _a.hasQuality) ?? false;
  };
  return {
    shouldShowQuality,
    shouldShowFps,
    shouldShowCodec,
    shouldShowSizeInput,
    validatePresetForm,
    getFormatConfig,
    isSupportedFormat,
    isAnimatedFormat,
    isLossyFormat
  };
}
const _hoisted_1$2 = {
  class: "settings-content",
  role: "main"
};
const _hoisted_2$2 = { class: "divider" };
const _sfc_main$3 = {
  __name: "SettingsSidebarVue",
  setup(__props) {
    const settingsContainerRef = ref(null);
    const main = inject("main");
    const { showNamingSettings } = inject("modeManager");
    const {
      settingsState,
      options,
      dropdownStates,
      updateSetting,
      updateDropdownState,
      validateNumberValue
    } = useSettingsState();
    provide("settingsState", settingsState);
    const {
      shouldShowQuality,
      shouldShowFps,
      shouldShowCodec,
      shouldShowSizeInput
    } = useSettingsValidation(settingsState);
    const { handleTabKey } = useTabNavigation(settingsContainerRef);
    const handleKeyDown = (event) => {
      if (event.key === "Tab") {
        handleTabKey(event);
      }
    };
    watch(
      () => main == null ? void 0 : main.item,
      (newItem) => {
        if (newItem) {
          updateSetting("newFileName", newItem.name);
        }
      },
      { immediate: true }
    );
    onMounted(() => {
      if (main == null ? void 0 : main.item) {
        updateSetting("newFileName", main.item.name);
      }
      window.addEventListener("keydown", handleKeyDown);
    });
    onUnmounted(() => {
      window.removeEventListener("keydown", handleKeyDown);
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", {
        class: "settings-sidebar",
        ref_key: "settingsContainerRef",
        ref: settingsContainerRef
      }, [
        createVNode(SettingsHeader),
        createBaseVNode("div", _hoisted_1$2, [
          createVNode(FormatSettingsGroup, {
            "settings-state": unref(settingsState),
            options: unref(options),
            "update-setting": unref(updateSetting),
            "update-dropdown-state": unref(updateDropdownState)
          }, null, 8, ["settings-state", "options", "update-setting", "update-dropdown-state"]),
          unref(shouldShowQuality) || unref(shouldShowFps) || unref(shouldShowCodec) ? (openBlock(), createBlock(QualitySettingsGroup, {
            key: 0,
            "settings-state": unref(settingsState),
            options: unref(options),
            "dropdown-states": unref(dropdownStates),
            "should-show-quality": unref(shouldShowQuality),
            "should-show-fps": unref(shouldShowFps),
            "should-show-codec": unref(shouldShowCodec),
            "update-setting": unref(updateSetting),
            "update-dropdown-state": unref(updateDropdownState)
          }, null, 8, ["settings-state", "options", "dropdown-states", "should-show-quality", "should-show-fps", "should-show-codec", "update-setting", "update-dropdown-state"])) : createCommentVNode("", true),
          createVNode(SizeSettingsGroup, {
            "settings-state": unref(settingsState),
            options: unref(options),
            "dropdown-states": unref(dropdownStates),
            "should-show-size-input": unref(shouldShowSizeInput),
            "update-setting": unref(updateSetting),
            "update-dropdown-state": unref(updateDropdownState),
            "validate-number-value": unref(validateNumberValue)
          }, null, 8, ["settings-state", "options", "dropdown-states", "should-show-size-input", "update-setting", "update-dropdown-state", "validate-number-value"]),
          withDirectives(createBaseVNode("div", _hoisted_2$2, null, 512), [
            [vShow, unref(showNamingSettings)]
          ]),
          withDirectives(createVNode(NamingSettingsGroup, {
            "settings-state": unref(settingsState),
            "dropdown-states": unref(dropdownStates),
            "update-setting": unref(updateSetting),
            "update-dropdown-state": unref(updateDropdownState),
            "validate-number-value": unref(validateNumberValue)
          }, null, 8, ["settings-state", "dropdown-states", "update-setting", "update-dropdown-state", "validate-number-value"]), [
            [vShow, unref(showNamingSettings)]
          ])
        ]),
        createVNode(ExportActions)
      ], 512);
    };
  }
};
const SettingsSidebarVue = /* @__PURE__ */ _export_sfc(_sfc_main$3, [["__scopeId", "data-v-d92e6f43"]]);
const _hoisted_1$1 = { class: "layout-container" };
const _hoisted_2$1 = { class: "layout-sidebar" };
const _hoisted_3 = { class: "layout-main" };
const _sfc_main$2 = {
  __name: "LayoutVue",
  setup(__props) {
    const main = inject("main");
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$1, [
        createBaseVNode("div", _hoisted_2$1, [
          createVNode(SettingsSidebarVue)
        ]),
        createBaseVNode("div", _hoisted_3, [
          _cache[0] || (_cache[0] = createBaseVNode("div", { class: "drag-helper" }, null, -1)),
          unref(main).items ? (openBlock(), createBlock(TaskListVue, { key: 0 })) : createCommentVNode("", true)
        ])
      ]);
    };
  }
};
const __unplugin_components_0 = /* @__PURE__ */ _export_sfc(_sfc_main$2, [["__scopeId", "data-v-690448a2"]]);
function useModeManager() {
  const operationMode = ref("export");
  const isExportMode = computed(() => operationMode.value === "export");
  const isReplaceMode = computed(() => operationMode.value === "replace");
  const showNamingSettings = computed(() => isExportMode.value);
  const setOperationMode = (mode) => {
    if (["export", "replace"].includes(mode)) {
      operationMode.value = mode;
      localStorage.setItem("eagle.plugin.operationMode", mode);
    }
  };
  const loadSavedMode = () => {
    const saved = localStorage.getItem("eagle.plugin.operationMode");
    if (saved && ["export", "replace"].includes(saved)) {
      operationMode.value = saved;
    }
  };
  watch(operationMode, (newMode) => {
    console.log(`Operation mode switched to: ${newMode}`);
  });
  const getOutputConfig = (originalItem, settings) => {
    if (isReplaceMode.value) {
      return {
        useOriginalPath: true,
        fileName: originalItem.name,
        shouldReplace: true
      };
    } else {
      return {
        useOriginalPath: false,
        fileName: settings.nameType === "original" ? originalItem.name : settings.newFileName,
        shouldReplace: false
      };
    }
  };
  loadSavedMode();
  return {
    // 響應式狀態
    operationMode,
    // 計算屬性
    isExportMode,
    isReplaceMode,
    showNamingSettings,
    // 方法
    setOperationMode,
    loadSavedMode,
    getOutputConfig
  };
}
function useProgressiveLoader() {
  const isLoading = ref(false);
  const isComplete = ref(false);
  const isCancelled = ref(false);
  const progress = ref({
    total: 0,
    loaded: 0,
    currentBatch: 0,
    totalBatches: 0,
    percentage: 0,
    estimatedTime: 0
  });
  let startTime = 0;
  let cancelToken = false;
  let taskIdSet = /* @__PURE__ */ new Set();
  const loadTasksProgressively = async (items, options = {}) => {
    const {
      createTask,
      onProgress = () => {
      },
      onBatchComplete = () => {
      },
      onComplete = () => {
      },
      batchSize = 200,
      batchDelay = 16
      // 1 frame at 60fps
    } = options;
    isLoading.value = true;
    isComplete.value = false;
    isCancelled.value = false;
    cancelToken = false;
    startTime = Date.now();
    progress.value = {
      total: items.length,
      loaded: 0,
      currentBatch: 0,
      totalBatches: Math.ceil(items.length / batchSize),
      percentage: 0,
      estimatedTime: 0
    };
    console.log(
      `Progressive loader: Starting to load ${items.length} items in ${progress.value.totalBatches} batches`
    );
    const batches = chunkArray(items, batchSize);
    const loadedTasks = [];
    try {
      for (let i = 0; i < batches.length; i++) {
        if (cancelToken) {
          isCancelled.value = true;
          console.log("Progressive loader: Cancelled by user");
          break;
        }
        const batch = batches[i];
        const batchStartTime = Date.now();
        await new Promise((resolve) => setTimeout(resolve, batchDelay));
        const batchTasks = [];
        for (const item of batch) {
          if (!taskIdSet.has(item.id)) {
            taskIdSet.add(item.id);
            if (createTask) {
              const task = await createTask(item);
              if (task) {
                batchTasks.push(markRaw(task));
              }
            } else {
              batchTasks.push(markRaw(item));
            }
          }
        }
        loadedTasks.push(...batchTasks);
        progress.value.loaded += batch.length;
        progress.value.currentBatch = i + 1;
        progress.value.percentage = Math.round(
          progress.value.loaded / progress.value.total * 100
        );
        const elapsed = Date.now() - startTime;
        const avgTimePerItem = elapsed / progress.value.loaded;
        const remainingItems = progress.value.total - progress.value.loaded;
        progress.value.estimatedTime = Math.round(avgTimePerItem * remainingItems / 1e3);
        onProgress({
          ...progress.value,
          batchTasks,
          batchTime: Date.now() - batchStartTime
        });
        await nextTick();
        onBatchComplete(i, batchTasks, {
          batchIndex: i,
          batchSize: batch.length,
          totalLoaded: progress.value.loaded
        });
        console.log(
          `Batch ${i + 1}/${progress.value.totalBatches} completed: ${batchTasks.length} tasks loaded`
        );
      }
      isComplete.value = true;
      const totalTime = Date.now() - startTime;
      console.log(
        `Progressive loader: Completed in ${totalTime}ms. Loaded ${loadedTasks.length} tasks`
      );
      onComplete({
        success: true,
        totalTasks: loadedTasks.length,
        totalTime,
        cancelled: isCancelled.value
      });
    } catch (error) {
      console.error("Progressive loader error:", error);
      onComplete({
        success: false,
        error: error.message,
        totalTasks: loadedTasks.length,
        cancelled: isCancelled.value
      });
    } finally {
      isLoading.value = false;
    }
    return loadedTasks;
  };
  const cancel = () => {
    cancelToken = true;
    isCancelled.value = true;
    console.log("Progressive loader: Cancel requested");
  };
  const reset = () => {
    isLoading.value = false;
    isComplete.value = false;
    isCancelled.value = false;
    cancelToken = false;
    taskIdSet.clear();
    progress.value = {
      total: 0,
      loaded: 0,
      currentBatch: 0,
      totalBatches: 0,
      percentage: 0,
      estimatedTime: 0
    };
  };
  const getOptimalBatchSize = (totalItems) => {
    if (totalItems <= 500) return 100;
    if (totalItems <= 1e3) return 150;
    if (totalItems <= 2e3) return 200;
    if (totalItems <= 5e3) return 250;
    return 300;
  };
  return {
    // 狀態
    isLoading,
    isComplete,
    isCancelled,
    progress,
    // 方法
    loadTasksProgressively,
    cancel,
    reset,
    getOptimalBatchSize
  };
}
function chunkArray(array, chunkSize) {
  const chunks = [];
  for (let i = 0; i < array.length; i += chunkSize) {
    chunks.push(array.slice(i, i + chunkSize));
  }
  return chunks;
}
const fs = require("fs");
const { fileURLToPath } = require("url");
const ffprobe = require(`${__dirname}/modules/fileConverter/toolkits/ffprobe`);
async function checkFileExists(filePath) {
  try {
    let normalPath = fileURLToPath(filePath);
    await fs.promises.access(normalPath);
    return filePath;
  } catch (error) {
    return null;
  }
}
function useTaskManager(exportSettings, mainInstance) {
  const tasks = shallowRef([]);
  const progressiveLoader = useProgressiveLoader();
  const taskBatcher2 = new TaskBatcher({
    batchSize: 200,
    maxMemoryMB: 50,
    // 限制記憶體使用
    autoTune: true
  });
  const taskObjectPool = new TaskObjectPool(200);
  const isLoadingTasks = ref(false);
  const loadProgress = computed(() => progressiveLoader.progress.value);
  const pendingTasks = computed(() => tasks.value.filter((task) => task.status === "waiting"));
  const processingTasks = computed(
    () => tasks.value.filter((task) => task.status === "processing")
  );
  const completedTasks = computed(() => tasks.value.filter((task) => task.status === "success"));
  const failedTasks = computed(() => tasks.value.filter((task) => task.status === "failed"));
  const taskStats = computed(() => ({
    total: tasks.value.length,
    pending: pendingTasks.value.length,
    processing: processingTasks.value.length,
    completed: completedTasks.value.length,
    failed: failedTasks.value.length
  }));
  const formatFileSize = (bytes) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = [i18next.t("units.fileSize.bytes"), i18next.t("units.fileSize.kilobytes"), i18next.t("units.fileSize.megabytes"), i18next.t("units.fileSize.gigabytes")];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  };
  const addTask = async (item) => {
    var _a;
    const existingTask = tasks.value.find((task2) => task2.id === item.id);
    if (existingTask) {
      console.warn(i18next.t("console.warnings.taskAlreadyExists", { id: item.id }));
      return existingTask;
    }
    const validation = taskValidator.validateTask(item, exportSettings == null ? void 0 : exportSettings.value);
    let task = {
      // 核心屬性 - 保留 id 方便快速存取
      id: item.id,
      item,
      // 保存完整的 Eagle item 物件
      // 轉換相關屬性
      newFormat: ((_a = exportSettings == null ? void 0 : exportSettings.value) == null ? void 0 : _a.format) || "",
      convertedSize: 0,
      status: validation.isValid ? "waiting" : "warn",
      thumbnailUrl: await checkFileExists(item.thumbnailURL),
      // 錯誤相關屬性 - 統一處理
      error: validation.errorMessage,
      isFormatSupported: validation.isValid,
      errorType: validation.errorType,
      errorMessage: validation.errorMessage,
      // 時間戳記
      createdAt: /* @__PURE__ */ new Date(),
      updatedAt: /* @__PURE__ */ new Date()
    };
    try {
      if (item.filePath && ffprobe.isSupported(item.filePath)) {
        const [dimensions, isAnimated] = await Promise.all([
          ffprobe.getDimensions(item.filePath),
          ffprobe.isAnimatedImage(item.filePath)
        ]);
        if (dimensions && dimensions.width && dimensions.height) {
          task.item.width = dimensions.width;
          task.item.height = dimensions.height;
          task.item.isAnimated = isAnimated;
          console.log(`Updated media info for ${item.name}: ${dimensions.width}x${dimensions.height}, animated: ${isAnimated}`);
        } else {
          console.log(`No valid dimensions from ffprobe for ${item.name}, keeping original values`);
        }
      }
    } catch (error) {
      console.warn(`Failed to get media info for ${item.name}:`, error.message);
    }
    tasks.value.push(task);
    if (!validation.isValid) {
      console.warn(`Task validation failed for ${item.name}: ${validation.errorMessage}`);
    }
    return task;
  };
  const addMultipleTasks = async (items) => {
    console.warn(
      "useTaskManager: addMultipleTasks is deprecated, use addTasksProgressively instead"
    );
    return await addTasksProgressively(items);
  };
  const addTasksProgressively = async (items, options = {}) => {
    if (!items || items.length === 0) {
      return [];
    }
    console.log(`TaskManager: Starting progressive loading of ${items.length} items`);
    isLoadingTasks.value = true;
    const tasksNeedingDimensionUpdate = [];
    try {
      const newTasks = await taskBatcher2.processBatch(
        items,
        async (item) => {
          var _a;
          const existingTask = tasks.value.find((task2) => task2.id === item.id);
          if (existingTask) {
            return null;
          }
          const validation = taskValidator.validateTask(item, exportSettings == null ? void 0 : exportSettings.value);
          const task = taskObjectPool.acquire(item);
          const thumbnailUrl = await checkFileExists(item.thumbnailURL);
          task.thumbnailUrl = thumbnailUrl;
          task.newFormat = ((_a = exportSettings == null ? void 0 : exportSettings.value) == null ? void 0 : _a.format) || "";
          task.status = validation.isValid ? "waiting" : "warn";
          task.error = validation.errorMessage;
          task.isFormatSupported = validation.isValid;
          task.errorType = validation.errorType;
          task.errorMessage = validation.errorMessage;
          if (item.filePath && ffprobe.isSupported(item.filePath)) {
            tasksNeedingDimensionUpdate.push(task);
          }
          if (!validation.isValid) {
            console.warn(`Task validation failed: ${item.ext} for file ${item.name} - ${validation.errorMessage}`);
          }
          return markRaw(task);
        },
        {
          onProgress: (progressInfo) => {
            console.log(
              `Progress: Batch ${progressInfo.batchIndex}/${progressInfo.totalBatches}, Memory: ${progressInfo.memoryStats.memoryPressureMB}MB`
            );
            if (options.onProgress) {
              options.onProgress(progressInfo);
            }
          },
          onBatchComplete: (batchIndex, batchTasks) => {
            if (batchTasks.length > 0) {
              tasks.value = [...tasks.value, ...batchTasks];
              console.log(
                `Batch ${batchIndex + 1} completed: Added ${batchTasks.length} tasks, Total: ${tasks.value.length}`
              );
            }
            if (options.onBatchComplete) {
              options.onBatchComplete(batchIndex, batchTasks);
            }
          },
          batchDelay: options.batchDelay || 16
          // 60fps
        }
      );
      console.log(
        `TaskManager: Progressive loading completed. Total tasks: ${tasks.value.length}`
      );
      if (options.onComplete) {
        options.onComplete({
          success: true,
          totalLoaded: newTasks.length,
          totalTasks: tasks.value.length,
          pendingDimensionUpdates: tasksNeedingDimensionUpdate.length,
          performanceStats: taskBatcher2.getPerformanceStats()
        });
      }
      return newTasks;
    } catch (error) {
      console.error(i18next.t("console.errors.progressiveLoading", { error: error.message }));
      if (options.onComplete) {
        options.onComplete({
          success: false,
          error: error.message,
          totalLoaded: 0,
          totalTasks: tasks.value.length
        });
      }
      throw error;
    } finally {
      isLoadingTasks.value = false;
    }
  };
  const cancelTaskLoading = () => {
    if (isLoadingTasks.value) {
      progressiveLoader.cancel();
      console.log(i18next.t("console.info.taskLoadingCancelled"));
    }
  };
  const removeTask = (taskId) => {
    const index = tasks.value.findIndex((task) => task.id === taskId);
    if (index > -1) {
      const removedTask = tasks.value[index];
      taskBatcher2.deduplicator.removeTask(taskId);
      taskObjectPool.release(removedTask);
      const newTasks = [...tasks.value];
      newTasks.splice(index, 1);
      tasks.value = newTasks;
      return true;
    }
    return false;
  };
  const updateTaskStatus = (taskId, status, progress = null, error = null) => {
    const taskIndex = tasks.value.findIndex((task) => task.id === taskId);
    if (taskIndex !== -1) {
      const task = tasks.value[taskIndex];
      const updatedTask = {
        ...task,
        status,
        updatedAt: /* @__PURE__ */ new Date(),
        errorMessage: error || null,
        ...progress !== null && { progress },
        ...error !== null && { error }
      };
      const newTasks = [...tasks.value];
      newTasks[taskIndex] = updatedTask;
      tasks.value = newTasks;
      return updatedTask;
    }
    console.warn(i18next.t("console.errors.taskNotFound", { taskId }));
    return null;
  };
  const updateMultipleTaskStatus = (updates) => {
    const updatedTasks = [];
    const newTasksArray = [...tasks.value];
    updates.forEach(({ taskId, status, progress = null, error = null }) => {
      const taskIndex = newTasksArray.findIndex((task) => task.id === taskId);
      if (taskIndex !== -1) {
        const task = newTasksArray[taskIndex];
        const updatedTask = {
          ...task,
          status,
          updatedAt: /* @__PURE__ */ new Date(),
          ...progress !== null && { progress },
          ...error !== null && { error }
        };
        newTasksArray[taskIndex] = updatedTask;
        updatedTasks.push(updatedTask);
      } else {
        console.warn(`Task ${taskId} not found for batch status update`);
      }
    });
    if (updatedTasks.length > 0) {
      tasks.value = newTasksArray;
      console.log(`Batch updated ${updatedTasks.length} tasks`);
    }
    return updatedTasks;
  };
  const updateTaskNewFormat = (taskId, newFormat) => {
    const taskIndex = tasks.value.findIndex((task) => task.id === taskId);
    if (taskIndex !== -1) {
      const task = tasks.value[taskIndex];
      const updatedTask = {
        ...task,
        newFormat,
        updatedAt: /* @__PURE__ */ new Date()
      };
      const newTasks = [...tasks.value];
      newTasks[taskIndex] = updatedTask;
      tasks.value = newTasks;
      return updatedTask;
    }
    return null;
  };
  const updateAllTasksNewFormat = (newFormat) => {
    const newTasks = tasks.value.map((task) => {
      return {
        ...task,
        newFormat,
        updatedAt: /* @__PURE__ */ new Date()
      };
    });
    tasks.value = newTasks;
  };
  const revalidateAllTasks = () => {
    const settings = exportSettings == null ? void 0 : exportSettings.value;
    if (!settings) return;
    console.log(`Revalidating all tasks for settings change: format=${settings.format}, sizeType=${settings.sizeType}`);
    const newTasks = tasks.value.map((task) => {
      if (task.status === "success" || task.status === "failed") {
        return task;
      }
      const validation = taskValidator.validateTask(task.item, settings);
      const shouldUpdate = validation.isValid && task.status === "warn" || // 從警告恢復為正常
      !validation.isValid && task.status === "waiting" || // 從正常變為警告
      task.errorType !== validation.errorType;
      if (shouldUpdate) {
        const newStatus = validation.isValid ? "waiting" : "warn";
        console.log(
          `Task ${task.item.name} validation changed: ${task.status}→${newStatus} (${validation.errorType || "none"})`
        );
        return {
          ...task,
          status: newStatus,
          error: validation.errorMessage,
          errorMessage: validation.errorMessage,
          errorType: validation.errorType,
          isFormatSupported: validation.isValid,
          // FIXED: 正確的邏輯
          updatedAt: /* @__PURE__ */ new Date()
        };
      }
      return task;
    });
    tasks.value = newTasks;
  };
  const handleSettingsChange = (oldSettings, newSettings) => {
    if (taskValidator.needsRevalidation(oldSettings, newSettings)) {
      console.log("Settings change requires task revalidation");
      revalidateAllTasks();
    }
    if ((oldSettings == null ? void 0 : oldSettings.format) !== (newSettings == null ? void 0 : newSettings.format) && (newSettings == null ? void 0 : newSettings.format)) {
      updateAllTasksNewFormat(newSettings.format);
    }
  };
  const updateTaskConvertedSize = (taskId, convertedSize) => {
    const taskIndex = tasks.value.findIndex((task) => task.id === taskId);
    if (taskIndex !== -1) {
      const task = tasks.value[taskIndex];
      const updatedTask = {
        ...task,
        convertedSize,
        updatedAt: /* @__PURE__ */ new Date()
      };
      const newTasks = [...tasks.value];
      newTasks[taskIndex] = updatedTask;
      tasks.value = newTasks;
      return updatedTask;
    }
    return null;
  };
  const updateTaskExportInfo = (taskId, exportInfo) => {
    const taskIndex = tasks.value.findIndex((task) => task.id === taskId);
    if (taskIndex !== -1) {
      const task = tasks.value[taskIndex];
      const updatedTask = {
        ...task,
        exportPath: exportInfo.outputPath || null,
        convertedSize: exportInfo.convertedSize || 0,
        originalDimensions: exportInfo.originalDimensions || null,
        exportDimensions: exportInfo.exportDimensions || null,
        newFileName: exportInfo.newFileName || null,
        updatedAt: /* @__PURE__ */ new Date()
      };
      const newTasks = [...tasks.value];
      newTasks[taskIndex] = updatedTask;
      tasks.value = newTasks;
      return updatedTask;
    }
    return null;
  };
  const updateTaskItemInfo = (taskId, updates) => {
    const taskIndex = tasks.value.findIndex((task) => task.id === taskId);
    if (taskIndex !== -1) {
      const task = tasks.value[taskIndex];
      const updatedTask = {
        ...task,
        item: {
          ...task.item,
          ...updates
        },
        updatedAt: /* @__PURE__ */ new Date()
      };
      const newTasks = [...tasks.value];
      newTasks[taskIndex] = updatedTask;
      tasks.value = newTasks;
      return updatedTask;
    }
    return null;
  };
  const getTask = (taskId) => {
    return tasks.value.find((task) => task.id === taskId);
  };
  const getFormattedTasks = computed(() => {
    return tasks.value.map((task) => ({
      ...task,
      originalSizeFormatted: formatFileSize(task.originalSize),
      convertedSizeFormatted: task.convertedSize > 0 ? formatFileSize(task.convertedSize) : "-",
      statusText: getStatusText(task.status)
    }));
  });
  const getStatusText = (status) => {
    const statusMap = {
      waiting: "等待中",
      processing: "處理中",
      success: "已完成",
      failed: "失敗"
    };
    return statusMap[status] || status;
  };
  const resetTaskStatus = (taskId) => {
    const taskIndex = tasks.value.findIndex((task) => task.id === taskId);
    if (taskIndex !== -1) {
      const task = tasks.value[taskIndex];
      const updatedTask = {
        ...task,
        status: "waiting",
        progress: 0,
        error: null,
        convertedSize: 0,
        updatedAt: /* @__PURE__ */ new Date()
      };
      const newTasks = [...tasks.value];
      newTasks[taskIndex] = updatedTask;
      tasks.value = newTasks;
      return updatedTask;
    }
    return null;
  };
  const resetAllFailedTasks = () => {
    const newTasks = tasks.value.map((task) => {
      if (task.status === "failed") {
        return {
          ...task,
          status: "waiting",
          progress: 0,
          error: null,
          convertedSize: 0,
          updatedAt: /* @__PURE__ */ new Date()
        };
      }
      return task;
    });
    tasks.value = newTasks;
  };
  const getPerformanceStats = () => {
    return {
      taskBatcher: taskBatcher2.getPerformanceStats(),
      objectPool: taskObjectPool.getStats(),
      ffprobe: ffprobe.getStats(),
      // 添加 ffprobe 統計
      loadProgress: loadProgress.value,
      totalTasks: tasks.value.length,
      memoryOptimization: {
        tasksInPool: taskObjectPool.getStats().poolSize,
        tasksInUse: taskObjectPool.getStats().inUse,
        deduplicationRate: taskBatcher2.deduplicator.getStats()
      }
    };
  };
  const forceMemoryCleanup = async () => {
    const completedTasks2 = tasks.value.filter((task) => task.status === "success");
    completedTasks2.forEach((task) => {
      taskObjectPool.release(task);
    });
    tasks.value = tasks.value.filter((task) => task.status !== "success");
    await taskBatcher2.performMemoryCleanup();
    ffprobe.clearCache();
    console.log("TaskManager: Forced memory cleanup completed");
  };
  const pauseAllTasks = async () => {
    if (mainInstance && typeof mainInstance.cancelConversion === "function") {
      const cancelled = mainInstance.cancelConversion();
      if (cancelled) {
        console.log("TaskManager: Cancelled ongoing conversion");
      }
    }
    const processingTaskIds = tasks.value.filter((task) => task.status === "processing").map((task) => task.id);
    if (processingTaskIds.length > 0) {
      const updates = processingTaskIds.map((taskId) => ({
        taskId,
        status: "waiting",
        progress: null,
        error: null
      }));
      updateMultipleTaskStatus(updates);
      console.log(`TaskManager: Paused ${processingTaskIds.length} processing tasks`);
    }
  };
  const clearAllTasks = () => {
    tasks.value.forEach((task) => taskObjectPool.release(task));
    tasks.value = [];
    taskBatcher2.deduplicator.clear();
    console.log("TaskManager: All tasks cleared");
  };
  const clearCompletedTasks = () => {
    const completedTasks2 = tasks.value.filter((task) => task.status === "success");
    completedTasks2.forEach((task) => taskObjectPool.release(task));
    tasks.value = tasks.value.filter((task) => task.status !== "success");
    console.log(`TaskManager: Cleared ${completedTasks2.length} completed tasks`);
  };
  return {
    // 響應式數據
    tasks,
    // 計算屬性
    pendingTasks,
    processingTasks,
    completedTasks,
    failedTasks,
    taskStats,
    getFormattedTasks,
    // 載入狀態
    isLoadingTasks,
    loadProgress,
    // 核心方法
    addTask,
    addMultipleTasks,
    // 保持向後相容性
    addTasksProgressively,
    // 新的主要方法
    cancelTaskLoading,
    removeTask,
    // 任務操作方法
    updateTaskStatus,
    updateMultipleTaskStatus,
    updateTaskNewFormat,
    updateAllTasksNewFormat,
    revalidateAllTasks,
    handleSettingsChange,
    updateTaskConvertedSize,
    updateTaskExportInfo,
    updateTaskItemInfo,
    getTask,
    resetTaskStatus,
    resetAllFailedTasks,
    // 工具方法
    formatFileSize,
    getStatusText,
    getPerformanceStats,
    forceMemoryCleanup,
    pauseAllTasks,
    clearAllTasks,
    clearCompletedTasks,
    // 相容性別名 - 保持舊 API 不破壞
    checkAndUpdateSizeLimitations: revalidateAllTasks,
    // 直接存取優化工具（進階用法）
    progressiveLoader,
    taskBatcher: taskBatcher2
  };
}
const _hoisted_1 = ["src", "alt"];
const _hoisted_2 = {
  key: 1,
  class: "preview-placeholder"
};
const MAX_SIZE = 300;
const OFFSET = 10;
const _sfc_main$1 = {
  __name: "HoverPreview",
  props: {
    visible: {
      type: Boolean,
      default: false
    },
    imageUrl: {
      type: String,
      default: ""
    },
    alt: {
      type: String,
      default: ""
    },
    mouseX: {
      type: Number,
      default: 0
    },
    mouseY: {
      type: Number,
      default: 0
    }
  },
  setup(__props) {
    const props = __props;
    const previewRef = ref(null);
    const imageLoaded = ref(false);
    const imageError = ref(false);
    const previewStyle = computed(() => {
      if (!props.visible) {
        return {
          display: "none"
        };
      }
      const windowWidth = window.innerWidth;
      const windowHeight = window.innerHeight;
      let left = props.mouseX + OFFSET;
      let top = props.mouseY + OFFSET;
      if (left + MAX_SIZE > windowWidth) {
        left = props.mouseX - MAX_SIZE - OFFSET;
      }
      if (top + MAX_SIZE > windowHeight) {
        top = props.mouseY - MAX_SIZE - OFFSET;
      }
      left = Math.max(OFFSET, left);
      top = Math.max(OFFSET, top);
      return {
        position: "fixed",
        left: `${left}px`,
        top: `${top}px`,
        zIndex: 9999,
        pointerEvents: "none",
        opacity: imageLoaded.value ? 1 : 0,
        transition: "opacity 0.2s ease"
      };
    });
    const handleImageLoad = () => {
      imageLoaded.value = true;
      imageError.value = false;
    };
    const handleImageError = () => {
      imageLoaded.value = false;
      imageError.value = true;
    };
    watch(() => props.visible, (newVal) => {
      if (!newVal) {
        imageLoaded.value = false;
        imageError.value = false;
      } else if (props.imageUrl) {
        imageLoaded.value = false;
        imageError.value = false;
      }
    });
    watch(() => props.imageUrl, () => {
      imageLoaded.value = false;
      imageError.value = false;
    });
    return (_ctx, _cache) => {
      const _component_el_icon = ElIcon;
      return __props.visible ? (openBlock(), createElementBlock("div", {
        key: 0,
        class: "hover-preview",
        style: normalizeStyle(previewStyle.value),
        ref_key: "previewRef",
        ref: previewRef
      }, [
        __props.imageUrl ? (openBlock(), createElementBlock("img", {
          key: 0,
          src: __props.imageUrl,
          alt: __props.alt,
          class: "preview-image",
          onLoad: handleImageLoad,
          onError: handleImageError
        }, null, 40, _hoisted_1)) : (openBlock(), createElementBlock("div", _hoisted_2, [
          createVNode(_component_el_icon, { class: "icon-image" })
        ]))
      ], 4)) : createCommentVNode("", true);
    };
  }
};
const HoverPreview = /* @__PURE__ */ _export_sfc(_sfc_main$1, [["__scopeId", "data-v-9ce497ba"]]);
const _sfc_main = {
  __name: "App",
  setup(__props) {
    const mousetrap = inject("mousetrap");
    const main = reactive(new Main());
    main.localStorageKey = `eagle.plugin.${eagle.plugin.manifest.id}.setting`;
    main.localStorageSetting = JSON.parse(localStorage.getItem(main.localStorageKey) || "{}");
    const modeManager = useModeManager();
    provide("modeManager", modeManager);
    const exportSettings = ref({
      exportType: "",
      frameValue: "",
      secondValue: "",
      format: "",
      quality: "",
      sizeType: "",
      sizeValue: "",
      nameType: "",
      newFileName: "",
      startNumber: "",
      exportCount: ""
    });
    provide(
      "exportSettings",
      computed(() => exportSettings.value)
    );
    const updateExportSettings = (newSettings) => {
      const oldSettings = { ...exportSettings.value };
      exportSettings.value = { ...exportSettings.value, ...newSettings };
      taskManager.handleSettingsChange(oldSettings, exportSettings.value);
    };
    provide("updateExportSettings", updateExportSettings);
    const taskManager = useTaskManager(computed(() => exportSettings.value), markRaw(main));
    provide("taskManager", taskManager);
    const { globalHoverPreview } = useHoverPreview();
    const triggerExport = ref(0);
    provide("triggerExport", triggerExport);
    main.setTaskManager(taskManager);
    const eagleTheme = ref("light");
    const initializeTheme = () => {
      const theme = eagle.app.theme;
      const THEME_SUPPORT2 = {
        Auto: eagle.app.isDarkColors() ? "dark" : "light",
        LIGHT: "light",
        LIGHTGRAY: "light",
        GRAY: "dark",
        DARK: "dark",
        BLUE: "dark",
        PURPLE: "dark"
      };
      eagleTheme.value = THEME_SUPPORT2[theme] ?? "light";
    };
    let isIinit = false;
    provide("eagleTheme", eagleTheme);
    eagle.onThemeChanged(() => {
      initializeTheme();
    });
    onMounted(async () => {
      initializeTheme();
      mousetrap.bind(["mod+enter"], () => {
        triggerExport.value++;
        return false;
      });
      mousetrap.bind(["esc"], () => {
        window.close();
        return false;
      });
    });
    eagle.onPluginRun(async (plugin2) => {
      if (!isIinit) {
        await new Promise((resolve) => setTimeout(resolve, 50));
        await eagle.window.setOpacity(1);
      }
      isIinit = true;
      let items = await eagle.item.getSelected();
      taskManager.tasks.value = [];
      if (!items.length) window.close();
      main.items = items.map((item) => markRaw(item));
    });
    eagle.onLibraryChanged((libraryPath) => {
      console.log(`偵測到資源庫切換，新的資源庫路徑: ${libraryPath}`);
      window.close();
    });
    provide("main", main);
    return (_ctx, _cache) => {
      const _component_LayoutVue = __unplugin_components_0;
      const _component_BodyVue = __unplugin_components_1;
      return openBlock(), createBlock(_component_BodyVue, null, {
        default: withCtx(() => [
          createVNode(_component_LayoutVue),
          createVNode(HoverPreview, {
            visible: unref(globalHoverPreview).visible,
            imageUrl: unref(globalHoverPreview).imageUrl,
            alt: unref(globalHoverPreview).alt,
            mouseX: unref(globalHoverPreview).mouseX,
            mouseY: unref(globalHoverPreview).mouseY
          }, null, 8, ["visible", "imageUrl", "alt", "mouseX", "mouseY"])
        ]),
        _: 1
      });
    };
  }
};
const i18nPlugin = {
  install: (app2, options) => {
    app2.config.globalProperties.$translate = (s, options2) => {
      return i18next.t(
        s == null ? void 0 : s.trim().split(" ").map((s2, i) => i == 0 ? s2 : s2.charAt(0).toUpperCase() + s2.slice(1)).join(""),
        options2
      );
    };
  }
};
const keyboardPlugin = {
  install: (app2, options) => {
    app2.config.globalProperties.$keyboard = (s) => {
      s = s.toUpperCase();
      const data = [
        ["CTRL", "⌘"],
        ["ALT", "⌥"],
        ["SHIFT", "⇧"]
      ];
      if (eagle.app.isMac) {
        for (let i of data) {
          s = s.replace(i[0], i[1]);
        }
      } else {
        for (let i of data) {
          s = s.replace(i[1], i[0]);
        }
      }
      return s;
    };
  }
};
const utils = require(`${__dirname}/modules/utils`);
const app = createApp(_sfc_main);
app.use(i18nPlugin);
app.use(keyboardPlugin);
app.use(plugin);
app.use(VueMousetrapPlugin).provide("mousetrap", app.config.globalProperties.$mousetrap);
eagle.onPluginCreate(async (plugin2) => {
  require(`${__dirname}/modules/utils`);
  app.mount("#app");
  document.querySelector("html").setAttribute("lang", eagle.app.locale);
  toggleTheme();
  process.on("uncaughtException", (error) => {
    eagle.log.error("uncaughtException:" + error);
  });
  const isFFemptInstalled = await eagle.extraModule.ffmpeg.isInstalled();
  if (!isFFemptInstalled) {
    await eagle.extraModule.ffmpeg.install();
    return;
  }
});
eagle.onThemeChanged((theme) => {
  toggleTheme();
});
window.addEventListener("load", async () => {
  await utils.file.deleteFolder(`${__dirname}/temp`);
  await utils.file.createFolder(`${__dirname}/temp`);
});
window.addEventListener("unload", async () => {
  await utils.file.deleteFolder(`${__dirname}/temp`);
});
const THEME_SUPPORT = {
  Auto: eagle.app.isDarkColors() ? "gray" : "light",
  LIGHT: "light",
  LIGHTGRAY: "lightgray",
  GRAY: "gray",
  DARK: "dark",
  BLUE: "blue",
  PURPLE: "purple"
};
async function toggleTheme() {
  const theme = eagle.app.theme;
  const themeName = THEME_SUPPORT[theme] ?? "dark";
  const htmlEl = document.querySelector("html");
  htmlEl.classList.add("no-transition");
  htmlEl.setAttribute("theme", themeName);
  htmlEl.setAttribute("platform", eagle.app.platform);
  if (["dark", "gray", "blue", "purple"].includes(themeName)) {
    htmlEl.classList.add("dark");
  } else {
    htmlEl.classList.remove("dark");
  }
  await nextTick();
  htmlEl.classList.remove("no-transition");
}
