import { r as ref, h as onMounted, n as nextTick, a4 as onUnmounted, o as openBlock, c as createElementBlock, q as watchEffect, a as createBaseVNode, I as normalizeStyle, u as unref, B as renderSlot, t as inject, z as onBeforeMount, a8 as shallowReactive, E as createBlock, F as withCtx, K as createVNode, H as normalizeClass, L as Fragment, Y as toDisplayString, a9 as createTextVNode, Q as createCommentVNode, a3 as withModifiers, G as withDirectives, J as vShow, v as computed, aa as vModelText, x as isRef, ab as mergeModels, ac as useModel, a0 as resolveComponent, ad as resolveDirective, ae as renderList, M as reactive, s as onBeforeUnmount, af as markRaw, A as provide, a7 as createApp } from "./@vue-DNuWswdR.js";
import { E as ElButton, a as ElDialog, b as ElDropdownItem, c as ElDropdownMenu, d as ElDropdown, e as ElEmpty } from "./element-plus-CxpFFEjL.js";
import { i as interact } from "./interactjs-BwhnLrJL.js";
import "./async-BF5sgVhb.js";
import { p as plugin } from "./vue-tippy-As68hqvG.js";
import { V as VueMousetrapPlugin } from "./vue-mousetrap-B6S1_FIG.js";
/* empty css                  */
import "./@vueuse-CQqYka_z.js";
import "./lodash-es-BeCtuktY.js";
import "./@element-plus-46jeqYt_.js";
import "./@ctrl-CUqN8X7N.js";
import "./@popperjs-BEcqXvUX.js";
import "./mousetrap-joIOW03i.js";
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
const _sfc_main$e = {
  __name: "ButtonComet",
  setup(__props) {
    const cometEl = ref(null);
    onMounted(() => {
      function createRoundedRectangleSinglePathSVG(width, height) {
        const border = 1.5;
        const radius = 6;
        const innerRadius = radius - border;
        const pathData = `
				M0,${radius} a${radius},${radius} 0 0 1 ${radius},-${radius} h${width - 2 * radius} a${radius},${radius} 0 0 1 ${radius},${radius} 
				v${height - 2 * radius} a${radius},${radius} 0 0 1 -${radius},${radius} h-${width - 2 * radius} a${radius},${radius} 0 0 1 -${radius},-${radius} Z 
				M${border},${border + innerRadius} a${innerRadius},${innerRadius} 0 0 1 ${innerRadius},-${innerRadius} h${width - 2 * (innerRadius + border)} a${innerRadius},${innerRadius} 0 0 1 ${innerRadius},${innerRadius} 
				v${height - 2 * (innerRadius + border)} a${innerRadius},${innerRadius} 0 0 1 -${innerRadius},${innerRadius} h-${width - 2 * (innerRadius + border)} a${innerRadius},${innerRadius} 0 0 1 -${innerRadius},-${innerRadius} Z
			`;
        return `<svg width="${width}" height="${height}" viewBox="0 0 ${width} ${height}" fill="none" xmlns="http://www.w3.org/2000/svg">
				<path d="${pathData}" fill="#D9D9D9" fill-rule="evenodd"/>
			</svg>`;
      }
      nextTick(() => {
        const comet = cometEl.value;
        const button = comet.closest("button");
        const width = button.clientWidth;
        const height = button.clientHeight;
        const svg = createRoundedRectangleSinglePathSVG(width, height);
        const svgURL = `url('data:image/svg+xml,${encodeURIComponent(svg)}')`;
        comet.style.webkitMaskImage = svgURL;
      });
    });
    onUnmounted(() => {
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", {
        class: "comet mask",
        ref_key: "cometEl",
        ref: cometEl
      }, null, 512);
    };
  }
};
const __unplugin_components_1 = /* @__PURE__ */ _export_sfc(_sfc_main$e, [["__scopeId", "data-v-80c25e79"]]);
const _hoisted_1$b = { class: "image-vue" };
const _hoisted_2$6 = ["src", "alt"];
const _sfc_main$d = {
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
      return openBlock(), createElementBlock("div", _hoisted_1$b, [
        createBaseVNode("img", {
          style: normalizeStyle({
            width: props.width + "px",
            height: props.height + "px"
          }),
          src: base_path + unref(uri),
          alt: unref(uri),
          loading: "lazy"
        }, null, 12, _hoisted_2$6),
        renderSlot(_ctx.$slots, "default")
      ]);
    };
  }
};
const _hoisted_1$a = { class: "title" };
const _hoisted_2$5 = { class: "description" };
const _hoisted_3$5 = { class: "size" };
const _hoisted_4$4 = { class: "title" };
const _hoisted_5$4 = { class: "description" };
const _hoisted_6$3 = { class: "title" };
const _hoisted_7$2 = { class: "description" };
const _hoisted_8$2 = { class: "title" };
const _hoisted_9$2 = ["innerHTML"];
const _sfc_main$c = {
  __name: "ModelDialogVue",
  emits: ["close"],
  setup(__props, { emit: __emit }) {
    const ai_eraser2 = inject("ai-eraser");
    const utils2 = require(`${__dirname}/modules/utils`);
    const emit = __emit;
    onBeforeMount(async () => {
      const missingDependencies = await ai_eraser2.getMissingDependencies();
      if (missingDependencies.length > 0) {
        model.status = "prepare";
        for (const dependencyInfo of missingDependencies) {
          model.totalSize += dependencyInfo.size;
        }
        model.visible = true;
      } else {
        close();
      }
    });
    const close = () => {
      model.visible = false;
      emit("close");
    };
    const model = shallowReactive({
      visible: false,
      status: "prepare",
      totalSize: 0,
      process: 0,
      prepare: {
        ok: async () => {
          model.status = "downloading";
          model.process = 0;
          try {
            await ai_eraser2.downloadMissingDependencies((process2) => {
              model.process = process2;
            });
            model.status = "success";
          } catch (e) {
            model.status = "failed";
          }
        },
        cancel: () => {
          window.close();
        }
      },
      downloading: {
        cancel: () => {
          window.close();
        }
      },
      success: {
        ok: async () => {
          try {
            close();
          } catch (e) {
            model.status = "failed";
          }
        }
      },
      failed: {
        ok: () => {
          model.status = "prepare";
        }
      }
    });
    return (_ctx, _cache) => {
      const _component_ImageVue = _sfc_main$d;
      const _component_ButtonComet = __unplugin_components_1;
      const _component_el_button = ElButton;
      const _component_el_dialog = ElDialog;
      return openBlock(), createBlock(_component_el_dialog, {
        modelValue: unref(model).visible,
        "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => unref(model).visible = $event),
        class: "model-dialog-vue",
        "close-on-click-modal": false,
        "append-to-body": "",
        "align-center": ""
      }, {
        default: withCtx(() => [
          createVNode(_component_ImageVue, {
            class: "status-img",
            width: "96",
            height: "96",
            src: "light/model/model-status.png",
            darkSrc: "dark/model/model-status.png"
          }, {
            default: withCtx(() => [
              createBaseVNode("div", {
                class: normalizeClass(["status", ["status-" + unref(model).status]]),
                style: normalizeStyle(`--percent: ${unref(model).process}%;`)
              }, null, 6)
            ]),
            _: 1
          }),
          unref(model).status === "prepare" ? (openBlock(), createElementBlock(Fragment, { key: 0 }, [
            createBaseVNode("div", _hoisted_1$a, toDisplayString(_ctx.$translate("modelDialog.prepare.title")), 1),
            createBaseVNode("div", _hoisted_2$5, toDisplayString(_ctx.$translate("modelDialog.prepare.description")), 1),
            createVNode(_component_el_button, {
              type: "primary",
              class: "ok ai-style",
              onClick: unref(model).prepare.ok
            }, {
              default: withCtx(() => [
                createVNode(_component_ButtonComet),
                createTextVNode(" " + toDisplayString(_ctx.$translate("modelDialog.prepare.ok")) + " ", 1),
                createBaseVNode("span", _hoisted_3$5, "(" + toDisplayString(unref(utils2).number.format(unref(model).totalSize)) + ")", 1)
              ]),
              _: 1
            }, 8, ["onClick"]),
            createVNode(_component_el_button, {
              type: "",
              class: "cancel",
              onClick: unref(model).prepare.cancel
            }, {
              default: withCtx(() => [
                createTextVNode(toDisplayString(_ctx.$translate("modelDialog.prepare.cancel")), 1)
              ]),
              _: 1
            }, 8, ["onClick"])
          ], 64)) : createCommentVNode("", true),
          unref(model).status === "downloading" ? (openBlock(), createElementBlock(Fragment, { key: 1 }, [
            createBaseVNode("div", _hoisted_4$4, toDisplayString(_ctx.$translate("modelDialog.downloading.title")), 1),
            createBaseVNode("div", _hoisted_5$4, toDisplayString(_ctx.$translate("modelDialog.downloading.description")), 1),
            createVNode(_component_el_button, {
              type: "",
              class: "cancel",
              onClick: unref(model).downloading.cancel
            }, {
              default: withCtx(() => [
                createTextVNode(toDisplayString(_ctx.$translate("modelDialog.downloading.cancel")), 1)
              ]),
              _: 1
            }, 8, ["onClick"])
          ], 64)) : createCommentVNode("", true),
          unref(model).status === "success" ? (openBlock(), createElementBlock(Fragment, { key: 2 }, [
            createBaseVNode("div", _hoisted_6$3, toDisplayString(_ctx.$translate("modelDialog.success.title")), 1),
            createBaseVNode("div", _hoisted_7$2, toDisplayString(_ctx.$translate("modelDialog.success.description")), 1),
            createVNode(_component_el_button, {
              type: "primary",
              class: "ok ai-style",
              onClick: unref(model).success.ok
            }, {
              default: withCtx(() => [
                createVNode(_component_ButtonComet),
                createTextVNode(" " + toDisplayString(_ctx.$translate("modelDialog.success.ok")), 1)
              ]),
              _: 1
            }, 8, ["onClick"])
          ], 64)) : createCommentVNode("", true),
          unref(model).status === "failed" ? (openBlock(), createElementBlock(Fragment, { key: 3 }, [
            createBaseVNode("div", _hoisted_8$2, toDisplayString(_ctx.$translate("modelDialog.failed.title")), 1),
            createBaseVNode("div", {
              class: "description",
              innerHTML: _ctx.$translate("modelDialog.failed.description")
            }, null, 8, _hoisted_9$2),
            createVNode(_component_el_button, {
              type: "primary",
              class: "ok ai-style",
              onClick: unref(model).failed.ok
            }, {
              default: withCtx(() => [
                createTextVNode(toDisplayString(_ctx.$translate("modelDialog.failed.ok")), 1)
              ]),
              _: 1
            }, 8, ["onClick"])
          ], 64)) : createCommentVNode("", true)
        ]),
        _: 1
      }, 8, ["modelValue"]);
    };
  }
};
const _sfc_main$b = {};
const _hoisted_1$9 = { class: "body-vue" };
function _sfc_render(_ctx, _cache) {
  return openBlock(), createElementBlock("div", _hoisted_1$9, [
    renderSlot(_ctx.$slots, "default")
  ]);
}
const __unplugin_components_8 = /* @__PURE__ */ _export_sfc(_sfc_main$b, [["render", _sfc_render]]);
const _hoisted_1$8 = { class: "tip" };
const _sfc_main$a = {
  __name: "DropZoneVue",
  props: {
    style: {
      type: Boolean,
      default: true
    }
  },
  emits: ["drop"],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emit = __emit;
    const active = ref(false);
    const onDrop = (e) => {
      setInactive();
      const files = [...e.dataTransfer.files];
      emit("drop", files);
    };
    function setActive() {
      active.value = true;
    }
    function setInactive() {
      active.value = false;
    }
    const events = ["dragenter", "dragover", "dragleave", "drop"];
    function preventDefaults(e) {
      e.preventDefault();
    }
    onMounted(() => {
      events.forEach((eventName) => {
        document.body.addEventListener(eventName, preventDefaults);
      });
    });
    onUnmounted(() => {
      events.forEach((eventName) => {
        document.body.removeEventListener(eventName, preventDefaults);
      });
    });
    return (_ctx, _cache) => {
      const _component_ImageVue = _sfc_main$d;
      return openBlock(), createElementBlock("div", {
        class: normalizeClass(["drop-zone-vue", {
          dropping: unref(active),
          "no-style": !props.style
        }]),
        onDragenter: withModifiers(setActive, ["prevent", "stop"])
      }, [
        renderSlot(_ctx.$slots, "default"),
        createBaseVNode("div", {
          class: "overlay",
          onDragleave: withModifiers(setInactive, ["prevent", "stop"]),
          onDrop: withModifiers(onDrop, ["prevent", "stop"])
        }, [
          createBaseVNode("div", _hoisted_1$8, [
            createVNode(_component_ImageVue, {
              width: "16",
              height: "16",
              src: "base/ic-drop-zone-download.svg"
            }),
            createTextVNode(" " + toDisplayString(_ctx.$translate("component.dropZone.tip")), 1)
          ])
        ], 32)
      ], 34);
    };
  }
};
const _hoisted_1$7 = { class: "notify-vue" };
const _hoisted_2$4 = { class: "tip" };
const _hoisted_3$4 = /* @__PURE__ */ createBaseVNode("span", { class: "status-processing" }, null, -1);
const _hoisted_4$3 = { class: "text" };
const _hoisted_5$3 = /* @__PURE__ */ createBaseVNode("span", { class: "status-success" }, null, -1);
const _hoisted_6$2 = { class: "text" };
const _hoisted_7$1 = /* @__PURE__ */ createBaseVNode("span", { class: "status-fail" }, null, -1);
const _hoisted_8$1 = { class: "text" };
const _hoisted_9$1 = { style: { "margin-right": "12px" } };
const _hoisted_10$1 = /* @__PURE__ */ createBaseVNode("span", { class: "dash" }, null, -1);
const _sfc_main$9 = {
  __name: "NotifyVue",
  setup(__props) {
    const main = inject("main");
    const onClick = () => {
      if (main.status !== "processing") {
        main.status = "";
      }
    };
    return (_ctx, _cache) => {
      return withDirectives((openBlock(), createElementBlock("div", _hoisted_1$7, [
        createBaseVNode("div", {
          class: "overlay",
          onClick
        }),
        createBaseVNode("div", _hoisted_2$4, [
          unref(main).status === "processing" ? (openBlock(), createElementBlock(Fragment, { key: 0 }, [
            _hoisted_3$4,
            createBaseVNode("span", _hoisted_4$3, toDisplayString(_ctx.$translate("main.status.processing")), 1)
          ], 64)) : createCommentVNode("", true),
          unref(main).status === "success" ? (openBlock(), createElementBlock(Fragment, { key: 1 }, [
            _hoisted_5$3,
            createBaseVNode("span", _hoisted_6$2, toDisplayString(_ctx.$translate("main.status.success")), 1)
          ], 64)) : createCommentVNode("", true),
          unref(main).status === "fail" ? (openBlock(), createElementBlock(Fragment, { key: 2 }, [
            _hoisted_7$1,
            createBaseVNode("span", _hoisted_8$1, [
              createBaseVNode("span", _hoisted_9$1, toDisplayString(_ctx.$translate("main.status.fail")), 1),
              createBaseVNode("span", {
                style: { "text-decoration": "underline" },
                onClick: _cache[0] || (_cache[0] = () => unref(main).exportFile())
              }, toDisplayString(_ctx.$translate("main.status.retry")), 1),
              _hoisted_10$1,
              createBaseVNode("span", {
                style: { "text-decoration": "underline" },
                onClick
              }, toDisplayString(_ctx.$translate("main.status.cancel")), 1)
            ])
          ], 64)) : createCommentVNode("", true)
        ])
      ], 512)), [
        [vShow, unref(main).status === "processing" || unref(main).status === "success" || unref(main).status === "fail"]
      ]);
    };
  }
};
const _hoisted_1$6 = { class: "slide-bar-vue" };
const _hoisted_2$3 = { class: "range-wrap" };
const _hoisted_3$3 = ["min", "max", "step"];
const _sfc_main$8 = {
  __name: "SlideBarVue",
  props: {
    modelValue: {
      type: Number,
      default: 0
    },
    data: {
      type: Array,
      default: Array.from({ length: 101 }, (_, index) => index),
      required: true
    },
    step: {
      type: Number
    },
    width: {
      Number,
      default: 100
    },
    showMinus: {
      type: Boolean,
      default: true
    },
    showPlus: {
      type: Boolean,
      default: true
    }
  },
  emits: ["update:modelValue", "changed"],
  setup(__props, { expose: __expose, emit: __emit }) {
    require(`${__dirname}/modules/utils/time`);
    const props = __props;
    const emit = __emit;
    const min = computed(() => Number(props.data[0] ?? 0));
    const max = computed(() => Number(props.data[props.data.length - 1] ?? 100));
    const step = computed(() => props.step ?? (max.value - min.value) / 100 ?? 1);
    const slide_bar_value = computed({
      get: () => props.modelValue,
      set: (value) => {
        emit("update:modelValue", value);
        emit("changed", value);
      }
    });
    function findClosestIndex(target) {
      let closestIndex = 0;
      let closestDifference = Math.abs(target - props.data[0]);
      for (let i = 1; i < props.data.length; i++) {
        const difference = Math.abs(target - props.data[i]);
        if (difference < closestDifference) {
          closestIndex = i;
          closestDifference = difference;
        }
      }
      return closestIndex;
    }
    const minus = () => {
      const index = findClosestIndex(props.modelValue);
      const value = props.data[index - 1 < 0 ? 0 : index - 1];
      slide_bar_value.value = value;
    };
    const plus = () => {
      const index = findClosestIndex(props.modelValue);
      const value = props.data[index + 1 > props.data.length - 1 ? props.data.length - 1 : index + 1];
      slide_bar_value.value = value;
    };
    __expose({
      minus,
      plus
    });
    return (_ctx, _cache) => {
      const _component_ImageVue = _sfc_main$d;
      return openBlock(), createElementBlock("div", _hoisted_1$6, [
        props.showMinus ? (openBlock(), createBlock(_component_ImageVue, {
          key: 0,
          onClick: minus,
          class: "icon",
          width: "23",
          height: "23",
          src: "light/base/ic-slide-bar-minus.svg",
          darkSrc: "dark/base/ic-slide-bar-minus.svg"
        })) : createCommentVNode("", true),
        createBaseVNode("div", _hoisted_2$3, [
          createBaseVNode("div", {
            class: "range-progressbar",
            style: normalizeStyle({ width: props.width + "px" })
          }, [
            createBaseVNode("div", {
              class: "current",
              style: normalizeStyle({
                width: (unref(slide_bar_value) - unref(min)) / (unref(max) - unref(min)) * 100 + "%"
              })
            }, null, 4),
            withDirectives(createBaseVNode("input", {
              "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => isRef(slide_bar_value) ? slide_bar_value.value = $event : null),
              type: "range",
              tabindex: "-1",
              min: unref(min),
              max: unref(max),
              step: unref(step)
            }, null, 8, _hoisted_3$3), [
              [vModelText, unref(slide_bar_value)]
            ])
          ], 4)
        ]),
        props.showPlus ? (openBlock(), createBlock(_component_ImageVue, {
          key: 1,
          onClick: plus,
          class: "icon",
          width: "23",
          height: "23",
          src: "light/base/ic-slide-bar-plus.svg",
          darkSrc: "dark/base/ic-slide-bar-plus.svg"
        })) : createCommentVNode("", true)
      ]);
    };
  }
};
const _hoisted_1$5 = { class: "toolbar" };
const _hoisted_2$2 = { class: "toolbar-group" };
const _hoisted_3$2 = { class: "name" };
const _hoisted_4$2 = { class: "name" };
const _hoisted_5$2 = { class: "toolbar-group" };
const _hoisted_6$1 = { class: "toolbar-group-item no-hover" };
const _hoisted_7 = { style: { "margin-right": "2px" } };
const _hoisted_8 = { class: "toolbar-group" };
const _hoisted_9 = { style: { "margin-right": "2px" } };
const _hoisted_10 = { style: { "margin-right": "4px" } };
const _hoisted_11 = { class: "toolbar-group" };
const _hoisted_12 = { class: "toolbar-group" };
const _hoisted_13 = {
  class: "toolbar-group-item",
  style: { "padding": "0 8px" }
};
const _hoisted_14 = { class: "export-file" };
const _sfc_main$7 = {
  __name: "ToolbarVue",
  props: /* @__PURE__ */ mergeModels({
    brushSizeRange: {
      type: Object,
      default: () => ({
        min: 10,
        max: 100,
        step: 10
      })
    },
    pinchZoomEl: {
      type: Object,
      required: true
    }
  }, {
    "brushSize": {},
    "brushSizeModifiers": {}
  }),
  emits: ["update:brushSize"],
  setup(__props, { expose: __expose }) {
    const main = inject("main");
    const brushSize = useModel(__props, "brushSize");
    const props = __props;
    const ratio = computed({
      get: () => {
        var _a;
        return (_a = props.pinchZoomEl) == null ? void 0 : _a.offset.ratio;
      },
      set: (value) => {
        var _a;
        (_a = props.pinchZoomEl) == null ? void 0 : _a.scale(value);
      }
    });
    __expose({
      brushSizeRange: props.brushSizeRange
    });
    return (_ctx, _cache) => {
      var _a;
      const _component_SlideBarVue = _sfc_main$8;
      const _component_el_dropdown_item = ElDropdownItem;
      const _component_key = resolveComponent("key");
      const _component_keys = resolveComponent("keys");
      const _component_el_dropdown_menu = ElDropdownMenu;
      const _component_el_dropdown = ElDropdown;
      const _component_ImageVue = _sfc_main$d;
      const _component_tippy = resolveComponent("tippy");
      const _directive_tippy = resolveDirective("tippy");
      return openBlock(), createElementBlock("div", _hoisted_1$5, [
        createBaseVNode("div", _hoisted_2$2, [
          createVNode(_component_SlideBarVue, {
            modelValue: unref(ratio),
            "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => isRef(ratio) ? ratio.value = $event : null),
            class: "toolbar-group-item no-hover",
            data: (_a = props.pinchZoomEl) == null ? void 0 : _a.scaleStep,
            width: "50",
            onChanged: _cache[1] || (_cache[1] = (value) => {
              var _a2;
              (_a2 = props.pinchZoomEl) == null ? void 0 : _a2.scale(value);
            })
          }, null, 8, ["modelValue", "data"]),
          createVNode(_component_el_dropdown, {
            class: "toolbar-group-item",
            style: { "width": "76px" },
            trigger: "click",
            placement: "top"
          }, {
            dropdown: withCtx(() => [
              createVNode(_component_el_dropdown_menu, null, {
                default: withCtx(() => {
                  var _a2;
                  return [
                    (openBlock(true), createElementBlock(Fragment, null, renderList(((_a2 = props.pinchZoomEl) == null ? void 0 : _a2.scaleStep) ?? [], (i) => {
                      var _a3;
                      return openBlock(), createBlock(_component_el_dropdown_item, {
                        class: normalizeClass(["keyboard", {
                          active: ((_a3 = props.pinchZoomEl) == null ? void 0 : _a3.offset.ratio) === i
                        }]),
                        key: i,
                        onClick: () => {
                          var _a4;
                          (_a4 = props.pinchZoomEl) == null ? void 0 : _a4.scale(i);
                        }
                      }, {
                        default: withCtx(() => [
                          createTextVNode(toDisplayString(i * 100) + "% ", 1)
                        ]),
                        _: 2
                      }, 1032, ["class", "onClick"]);
                    }), 128)),
                    createVNode(_component_el_dropdown_item, {
                      class: "keyboard no-tick",
                      onClick: _cache[2] || (_cache[2] = () => {
                        var _a3;
                        (_a3 = props.pinchZoomEl) == null ? void 0 : _a3.scale(1);
                      }),
                      divided: ""
                    }, {
                      default: withCtx(() => [
                        createBaseVNode("div", _hoisted_3$2, toDisplayString(_ctx.$translate("component.compare.toolbar.scale.origin")), 1),
                        createVNode(_component_keys, null, {
                          default: withCtx(() => [
                            (openBlock(true), createElementBlock(Fragment, null, renderList([_ctx.$keyboard("⌘"), "0"], (key) => {
                              return openBlock(), createBlock(_component_key, { key }, {
                                default: withCtx(() => [
                                  createTextVNode(toDisplayString(key), 1)
                                ]),
                                _: 2
                              }, 1024);
                            }), 128))
                          ]),
                          _: 1
                        })
                      ]),
                      _: 1
                    }),
                    createVNode(_component_el_dropdown_item, {
                      class: "keyboard no-tick",
                      onClick: _cache[3] || (_cache[3] = () => {
                        var _a3;
                        (_a3 = props.pinchZoomEl) == null ? void 0 : _a3.fit();
                      })
                    }, {
                      default: withCtx(() => [
                        createBaseVNode("div", _hoisted_4$2, toDisplayString(_ctx.$translate("component.compare.toolbar.scale.fit")), 1),
                        createVNode(_component_keys, null, {
                          default: withCtx(() => [
                            (openBlock(true), createElementBlock(Fragment, null, renderList([_ctx.$keyboard("⌘"), "9"], (key) => {
                              return openBlock(), createBlock(_component_key, { key }, {
                                default: withCtx(() => [
                                  createTextVNode(toDisplayString(key), 1)
                                ]),
                                _: 2
                              }, 1024);
                            }), 128))
                          ]),
                          _: 1
                        })
                      ]),
                      _: 1
                    })
                  ];
                }),
                _: 1
              })
            ]),
            default: withCtx(() => {
              var _a2;
              return [
                createTextVNode(toDisplayString(Math.round(((_a2 = props.pinchZoomEl) == null ? void 0 : _a2.offset.ratio) * 100)) + "% ", 1)
              ];
            }),
            _: 1
          })
        ]),
        createBaseVNode("div", _hoisted_5$2, [
          createVNode(_component_tippy, {
            allowHTML: "",
            placement: "bottom",
            duration: "[200,0]",
            delay: "[0,0]"
          }, {
            default: withCtx(() => [
              createBaseVNode("div", _hoisted_6$1, [
                createVNode(_component_ImageVue, {
                  style: { "width": "100%", "height": "100%" },
                  width: "24",
                  height: "24",
                  src: "light/ic-brush.svg",
                  darkSrc: "dark/ic-brush.svg"
                }),
                createVNode(_component_SlideBarVue, {
                  modelValue: brushSize.value,
                  "onUpdate:modelValue": _cache[4] || (_cache[4] = ($event) => brushSize.value = $event),
                  style: { "margin": "0 4px" },
                  data: Array.from(
                    { length: props.brushSizeRange.max },
                    (_, i) => i + props.brushSizeRange.min
                  ),
                  step: props.brushSizeRange.step,
                  width: "50",
                  showMinus: false,
                  showPlus: false
                }, null, 8, ["modelValue", "data", "step"])
              ])
            ]),
            content: withCtx(() => [
              createBaseVNode("span", _hoisted_7, toDisplayString(_ctx.$translate("main.toolbar.brushSize")), 1),
              createVNode(_component_key, null, {
                default: withCtx(() => [
                  createTextVNode(toDisplayString(_ctx.$keyboard("[")), 1)
                ]),
                _: 1
              }),
              createVNode(_component_key, null, {
                default: withCtx(() => [
                  createTextVNode(toDisplayString(_ctx.$keyboard("]")), 1)
                ]),
                _: 1
              })
            ]),
            _: 1
          })
        ]),
        createBaseVNode("div", _hoisted_8, [
          createBaseVNode("div", {
            class: normalizeClass(["toolbar-group-item", {
              disabled: !unref(main).imageHistoryManager.undoStack.length
            }]),
            onClick: _cache[5] || (_cache[5] = ($event) => unref(main).imageHistoryManager.undo())
          }, [
            createVNode(_component_tippy, {
              allowHTML: "",
              placement: "bottom",
              duration: "[200,0]",
              delay: "[0,0]"
            }, {
              default: withCtx(() => [
                createVNode(_component_ImageVue, {
                  style: { "width": "100%", "height": "100%" },
                  width: "16",
                  height: "16",
                  src: "light/ic-undo.svg",
                  darkSrc: "dark/ic-undo.svg"
                })
              ]),
              content: withCtx(() => [
                createBaseVNode("span", _hoisted_9, toDisplayString(_ctx.$translate("main.toolbar.undo")), 1),
                createVNode(_component_key, null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(_ctx.$keyboard("ctrl")), 1)
                  ]),
                  _: 1
                }),
                createVNode(_component_key, null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(_ctx.$keyboard("z")), 1)
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ], 2),
          createBaseVNode("div", {
            class: normalizeClass(["toolbar-group-item", {
              disabled: !unref(main).imageHistoryManager.redoStack.length
            }]),
            onClick: _cache[6] || (_cache[6] = ($event) => unref(main).imageHistoryManager.redo())
          }, [
            createVNode(_component_tippy, {
              allowHTML: "",
              placement: "bottom",
              duration: "[200,0]",
              delay: "[0,0]"
            }, {
              default: withCtx(() => [
                createVNode(_component_ImageVue, {
                  style: { "width": "100%", "height": "100%" },
                  width: "16",
                  height: "16",
                  src: "light/ic-redo.svg",
                  darkSrc: "dark/ic-redo.svg"
                })
              ]),
              content: withCtx(() => [
                createBaseVNode("span", _hoisted_10, toDisplayString(_ctx.$translate("main.toolbar.redo")), 1),
                createVNode(_component_key, null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(_ctx.$keyboard("shift")), 1)
                  ]),
                  _: 1
                }),
                createVNode(_component_key, null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(_ctx.$keyboard("ctrl")), 1)
                  ]),
                  _: 1
                }),
                createVNode(_component_key, null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(_ctx.$keyboard("z")), 1)
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ], 2)
        ]),
        createBaseVNode("div", _hoisted_11, [
          withDirectives((openBlock(), createElementBlock("div", {
            class: "toolbar-group-item",
            onMousedown: _cache[7] || (_cache[7] = withModifiers(
              () => {
                unref(main).imageHistoryManager.tmp = unref(main).imageHistoryManager.current;
                unref(main).imageHistoryManager.current = unref(main).imageHistoryManager.original;
              },
              ["prevent"]
            )),
            onMouseup: _cache[8] || (_cache[8] = withModifiers(
              () => {
                unref(main).imageHistoryManager.current = unref(main).imageHistoryManager.tmp;
              },
              ["prevent"]
            ))
          }, [
            createVNode(_component_ImageVue, {
              style: { "width": "100%", "height": "100%" },
              width: "16",
              height: "12",
              src: "light/ic-eye.svg",
              darkSrc: "dark/ic-eye.svg"
            })
          ], 32)), [
            [_directive_tippy, {
              content: _ctx.$translate(`main.toolbar.eye`),
              delay: [0, 0],
              duration: [200, 0]
            }]
          ])
        ]),
        createBaseVNode("div", _hoisted_12, [
          createVNode(_component_el_dropdown, {
            class: "no-tick",
            trigger: "click"
          }, {
            dropdown: withCtx(() => [
              createVNode(_component_el_dropdown_menu, { class: "no-tick" }, {
                default: withCtx(() => [
                  createVNode(_component_el_dropdown_item, {
                    onClick: _cache[9] || (_cache[9] = ($event) => unref(main).replace())
                  }, {
                    default: withCtx(() => [
                      createTextVNode(toDisplayString(_ctx.$translate("main.download.replace")), 1)
                    ]),
                    _: 1
                  }),
                  createVNode(_component_el_dropdown_item, {
                    onClick: _cache[10] || (_cache[10] = ($event) => unref(main).saveAs())
                  }, {
                    default: withCtx(() => [
                      createTextVNode(toDisplayString(_ctx.$translate("main.download.saveAs")), 1)
                    ]),
                    _: 1
                  })
                ]),
                _: 1
              })
            ]),
            default: withCtx(() => [
              createBaseVNode("div", _hoisted_13, [
                createVNode(_component_ImageVue, {
                  style: { "width": "100%", "height": "100%" },
                  width: "16",
                  height: "16",
                  src: "light/ic-save.svg",
                  darkSrc: "dark/ic-save.svg"
                }),
                createBaseVNode("div", _hoisted_14, toDisplayString(_ctx.$translate("main.toolbar.exportFile")), 1)
              ])
            ]),
            _: 1
          })
        ])
      ]);
    };
  }
};
const _hoisted_1$4 = { class: "pinch-zoom-container" };
const boundary = 100;
const _sfc_main$6 = {
  __name: "PinchZoomVue",
  props: {
    container: {
      type: String,
      default: ".pinch-zoom-container"
    },
    fit: {
      type: Boolean,
      default: false
    },
    dragMove: {
      type: Boolean,
      default: false
    }
  },
  setup(__props, { expose: __expose }) {
    const utils2 = require(`${__dirname}/modules/utils`);
    const props = __props;
    const pinchZoomEl = ref(null);
    const scaleStep = [0.05, 0.1, 0.25, 0.5, 1, 1.25, 1.5, 2, 3, 4, 8];
    const isReady = ref(false);
    const isTrust = ref(false);
    const offset = reactive({
      x: 0,
      y: 0,
      ratio: 1
    });
    const container = reactive({
      width: 0,
      height: 0
    });
    const child = reactive({
      width: 0,
      height: 0
    });
    const fit = () => {
      offset.ratio = Math.min(container.width / child.width, container.height / child.height);
      center();
      updateTransform();
    };
    const scale = async (ratio) => {
      offset.ratio = ratio;
      center();
      updateTransform();
    };
    const updateTransform = () => {
      pinchZoomEl.value.setTransform({
        x: offset.x,
        y: offset.y,
        scale: offset.ratio
      });
    };
    const getOffset = () => {
      offset.x = pinchZoomEl.value.x;
      offset.y = pinchZoomEl.value.y;
      offset.ratio = pinchZoomEl.value.scale;
    };
    const setOffset = (x, y, ratio) => {
      offset.x = x;
      offset.y = y;
      offset.ratio = ratio;
      updateTransform();
    };
    const resize = async () => {
      if (!pinchZoomEl.value) return;
      await utils2.time.imgLoad(pinchZoomEl.value.children);
      container.width = pinchZoomEl.value.clientWidth;
      container.height = pinchZoomEl.value.clientHeight;
      child.width = pinchZoomEl.value.firstElementChild.clientWidth;
      child.height = pinchZoomEl.value.firstElementChild.clientHeight;
      if (props.fit) {
        fit();
      } else {
        center();
        updateTransform();
      }
    };
    const center = () => {
      offset.x = (container.width - child.width * offset.ratio) / 2;
      offset.y = (container.height - child.height * offset.ratio) / 2;
    };
    const wheelEventHandle = async (event2) => {
      if (event2.metaKey || event2.ctrlKey || props.dragMove) getOffset();
      if (props.dragMove) {
        move(0, 0);
      } else {
        const divisor = 2;
        const deltaX = -1 * event2.deltaX / divisor;
        const deltaY = -1 * event2.deltaY / divisor;
        move(deltaX, deltaY);
      }
      await utils2.time.sleep(1);
      updateTransform();
    };
    const mousedownEventHandle = (event2) => {
      if (props.dragMove) document.querySelector(props.container).style.cursor = "grabbing";
      isTrust.value = true;
    };
    const mousemoveEventHandle = (event2) => {
      if (isTrust.value) {
        const deltaX = event2.movementX;
        const deltaY = event2.movementY;
        move(deltaX, deltaY);
      }
    };
    const mouseupEventHandle = (event2) => {
      if (props.dragMove) document.querySelector(props.container).style.cursor = "grab";
      isTrust.value = false;
    };
    const move = (x, y) => {
      offset.x += x;
      offset.y += y;
      if (container.width >= child.width * offset.ratio) {
        offset.x = Math.max(offset.x, 0);
        offset.x = Math.min(offset.x, container.width - child.width * offset.ratio);
      } else {
        offset.x = Math.min(offset.x, boundary);
        offset.x = Math.max(offset.x, container.width - child.width * offset.ratio - boundary);
      }
      if (container.height >= child.height * offset.ratio) {
        offset.y = Math.max(offset.y, 0);
        offset.y = Math.min(offset.y, container.height - child.height * offset.ratio);
      } else {
        offset.y = Math.min(offset.y, boundary);
        offset.y = Math.max(offset.y, container.height - child.height * offset.ratio - boundary);
      }
      updateTransform();
    };
    const scaleIn = () => {
      const ratio = scaleStep.find((e) => e > offset.ratio);
      if (!ratio) return;
      offset.ratio = ratio;
      center();
      updateTransform();
    };
    const scaleOut = () => {
      const ratio = scaleStep.findLast((e) => e < offset.ratio);
      if (!ratio) return;
      offset.ratio = ratio;
      center();
      updateTransform();
    };
    const view = async (x, y, width, height) => {
      const left = -1 * offset.x <= x * offset.ratio;
      const right = (x + width) * offset.ratio <= -1 * offset.x + container.width;
      const top = -1 * offset.y <= y * offset.ratio;
      const bottom = (y + height) * offset.ratio <= -1 * offset.y + container.height;
      if (top && right && bottom && left) return;
      offset.x = -1 * (x + width / 2) * offset.ratio + container.width / 2;
      offset.y = -1 * (y + height / 2) * offset.ratio + container.height / 2;
      offset.ratio = Math.min(
        container.width / width,
        container.height / height,
        scaleStep[scaleStep.length - 1]
      ) / 1.5;
      updateTransform();
    };
    const resizeObserver = new ResizeObserver(resize);
    const mutationObserver = new MutationObserver(resize);
    onMounted(async () => {
      interact("#scrollbar-vertical-thumb").styleCursor(false).draggable({
        lockAxis: "y",
        listeners: {
          move: (event2) => {
            const ratio = container.height / (child.height * offset.ratio);
            offset.y -= event2.dy / ratio;
            offset.y = Math.min(offset.y, boundary);
            offset.y = Math.max(
              offset.y,
              container.height - child.height * offset.ratio - boundary
            );
            updateTransform();
          }
        }
      });
      interact("#scrollbar-horizontal-thumb").styleCursor(false).draggable({
        lockAxis: "x",
        listeners: {
          move: (event2) => {
            const ratio = container.width / (child.width * offset.ratio);
            offset.x -= event2.dx / ratio;
            offset.x = Math.min(offset.x, boundary);
            offset.x = Math.max(
              offset.x,
              container.width - child.width * offset.ratio - boundary
            );
            updateTransform();
          }
        }
      });
      const containerEl = document.querySelector(props.container);
      containerEl.style.overflow = "hidden";
      if (props.dragMove) containerEl.style.cursor = "grab";
      containerEl.addEventListener("wheel", wheelEventHandle, { passive: true });
      if (props.dragMove) {
        containerEl.addEventListener("mousedown", mousedownEventHandle);
        window.addEventListener("mousemove", mousemoveEventHandle);
        window.addEventListener("mouseup", mouseupEventHandle);
      }
      const containerChildEl = containerEl.firstElementChild;
      const className = `pinchZoom-trigger-${utils2.string.generateRandomString(5)}`;
      containerChildEl.classList.add(className);
      pinchZoomEl.value.setAttribute("container", `.${className}`);
      pinchZoomEl.value.setAttribute("min-scale", scaleStep[0]);
      pinchZoomEl.value.setAttribute("max-scale", scaleStep[scaleStep.length - 1]);
      pinchZoomEl.value.setAttribute("drag-move", props.dragMove);
      await utils2.time.imgLoad(pinchZoomEl.value.children);
      resizeObserver.observe(containerEl);
      if (pinchZoomEl.value) {
        mutationObserver.observe(pinchZoomEl.value.firstElementChild, {
          attributes: true
        });
      }
      isReady.value = true;
    });
    onBeforeUnmount(() => {
      const containerEl = document.querySelector(props.container);
      containerEl.style.overflow = "";
      containerEl.removeEventListener("wheel", wheelEventHandle);
      if (props.dragMove) {
        containerEl.removeEventListener("mousedown", mousedownEventHandle);
        window.removeEventListener("mousemove", mousemoveEventHandle);
        window.removeEventListener("mouseup", mouseupEventHandle);
      }
      resizeObserver.disconnect();
      mutationObserver.disconnect();
    });
    __expose({
      offset,
      scaleStep,
      fit,
      scale,
      scaleIn,
      scaleOut,
      width: () => child.width,
      height: () => child.height,
      view,
      setOffset,
      element: () => pinchZoomEl.value
    });
    return (_ctx, _cache) => {
      const _component_pinch_zoom = resolveComponent("pinch-zoom");
      return withDirectives((openBlock(), createElementBlock("div", _hoisted_1$4, [
        createVNode(_component_pinch_zoom, {
          class: "pinch-zoom",
          ref_key: "pinchZoomEl",
          ref: pinchZoomEl
        }, {
          default: withCtx(() => [
            renderSlot(_ctx.$slots, "default")
          ]),
          _: 3
        }, 512),
        withDirectives(createBaseVNode("div", {
          class: "scrollbar scrollbar-vertical",
          onMousedown: _cache[1] || (_cache[1] = withModifiers(
            ($event) => {
              _ctx.percent = $event.offsetY / $event.target.clientHeight;
              unref(offset).y = -1 * unref(child).height * unref(offset).ratio * _ctx.percent;
              unref(offset).y += $event.target.clientHeight / 2;
              updateTransform();
            },
            ["self", "prevent", "stop"]
          ))
        }, [
          createBaseVNode("div", {
            id: "scrollbar-vertical-thumb",
            class: "scrollbar-thumb",
            style: normalizeStyle({
              top: `${-1 * unref(offset).y / (unref(child).height * unref(offset).ratio) * 100}%`,
              height: `${unref(container).height / (unref(child).height * unref(offset).ratio) * 100}%`
            }),
            onMousedown: _cache[0] || (_cache[0] = withModifiers(() => {
            }, ["stop"]))
          }, null, 36)
        ], 544), [
          [vShow, Math.floor(unref(child).height * unref(offset).ratio) > unref(container).height]
        ]),
        withDirectives(createBaseVNode("div", {
          class: "scrollbar scrollbar-horizontal",
          onMousedown: _cache[3] || (_cache[3] = withModifiers(
            ($event) => {
              _ctx.percent = $event.offsetX / $event.target.clientWidth;
              unref(offset).x = -1 * unref(child).width * unref(offset).ratio * _ctx.percent;
              unref(offset).x += $event.target.clientWidth / 2;
              updateTransform();
            },
            ["prevent", "stop"]
          ))
        }, [
          createBaseVNode("div", {
            id: "scrollbar-horizontal-thumb",
            class: "scrollbar-thumb",
            style: normalizeStyle({
              left: `${-1 * unref(offset).x / (unref(child).width * unref(offset).ratio) * 100}%`,
              width: `${unref(container).width / (unref(child).width * unref(offset).ratio) * 100}%`
            }),
            onMousedown: _cache[2] || (_cache[2] = withModifiers(() => {
            }, ["stop"]))
          }, null, 36)
        ], 544), [
          [vShow, Math.floor(unref(child).width * unref(offset).ratio) > unref(container).width]
        ])
      ], 512)), [
        [vShow, unref(isReady)]
      ]);
    };
  }
};
const _sfc_main$5 = {
  __name: "CursorVue",
  props: {
    container: {
      type: String,
      default: ".cursor-vue"
    },
    size: {
      type: Number,
      default: 50
    },
    showTip: {
      type: Boolean,
      default: true
    }
  },
  setup(__props) {
    const props = __props;
    const x = ref(0);
    const y = ref(0);
    function update(event2) {
      x.value = event2.x;
      y.value = event2.y - 48;
    }
    onMounted(() => {
      window.addEventListener("mousemove", update);
      document.querySelector(props.container).style.cursor = "none";
    });
    onUnmounted(() => window.removeEventListener("mousemove", update));
    return (_ctx, _cache) => {
      const _component_tippy = resolveComponent("tippy");
      return openBlock(), createElementBlock("div", {
        class: "cursor-vue",
        style: normalizeStyle({
          transform: `translate(${unref(x)}px, ${unref(y)}px)`
        })
      }, [
        createVNode(_component_tippy, {
          allowHTML: "",
          placement: "right",
          duration: "[200,0]",
          delay: "[0,0]"
        }, {
          default: withCtx(() => [
            createBaseVNode("div", {
              class: "cursor",
              style: normalizeStyle({
                "--size": __props.size + "px"
              })
            }, null, 4)
          ]),
          content: withCtx(() => [
            createTextVNode(" 測試123 ")
          ]),
          _: 1
        })
      ], 4);
    };
  }
};
const _hoisted_1$3 = ["src", "alt"];
const _sfc_main$4 = {
  __name: "PaintBoardVue",
  props: /* @__PURE__ */ mergeModels({
    trigger: {
      type: String,
      default: ""
    },
    src: {
      type: String,
      default: ""
    },
    offset: {
      type: Object,
      default: () => ({
        x: 0,
        y: 0,
        ratio: 1
      })
    }
  }, {
    "brushSize": {},
    "brushSizeModifiers": {}
  }),
  emits: /* @__PURE__ */ mergeModels(["startDrawing", "drawing", "endDrawing"], ["update:brushSize"]),
  setup(__props, { expose: __expose, emit: __emit }) {
    const utils2 = require(`${__dirname}/modules/utils`);
    const brushSize = useModel(__props, "brushSize");
    const props = __props;
    const boundary2 = {
      width: 0,
      height: 48
    };
    const width = ref(0);
    const height = ref(0);
    const emit = __emit;
    const canvasEl = ref(null);
    const ctx = ref(null);
    const showLayers = ref(true);
    let painting = false;
    function startDrawing(e) {
      painting = true;
      draw(e);
      emit("startDrawing");
    }
    function endDrawing() {
      if (!painting) return;
      painting = false;
      ctx.value.beginPath();
      emit("endDrawing");
    }
    function draw(e) {
      if (!painting) return;
      ctx.value.lineWidth = props.brushSize / props.offset.ratio;
      ctx.value.lineCap = "round";
      ctx.value.strokeStyle = "white";
      const x = (e.clientX - canvasEl.value.offsetLeft - props.offset.x - boundary2.width) / props.offset.ratio;
      const y = (e.clientY - canvasEl.value.offsetTop - props.offset.y - boundary2.height) / props.offset.ratio;
      ctx.value.lineTo(x, y);
      ctx.value.stroke();
      ctx.value.beginPath();
      ctx.value.moveTo(x, y);
    }
    const eye = async () => {
      showLayers.value = !showLayers.value;
    };
    const exportBase64 = async () => {
      const srcToBase64 = async (src) => {
        const img = await utils2.image.create(src);
        const canvas = document.createElement("canvas");
        const ctx2 = canvas.getContext("2d", { willReadFrequently: true });
        canvas.width = img.width;
        canvas.height = img.height;
        ctx2.drawImage(img, 0, 0);
        return canvas.toDataURL("image/png");
      };
      const imageBase64 = await srcToBase64(props.src);
      const maskBase64 = canvasEl.value.toDataURL("image/png");
      return {
        imageBase64,
        maskBase64
      };
    };
    let triggerEl = null;
    onMounted(async () => {
      const img = await utils2.image.create(props.src);
      width.value = img.width;
      height.value = img.height;
      canvasEl.value.width = img.width;
      canvasEl.value.height = img.height;
      ctx.value = canvasEl.value.getContext("2d");
      triggerEl = document.querySelector(canvasEl.value.className);
      if (props.trigger && document.querySelector(props.trigger)) {
        triggerEl = document.querySelector(props.trigger);
      }
      triggerEl.addEventListener("mousedown", startDrawing);
      triggerEl.addEventListener("mousemove", draw);
      triggerEl.addEventListener("mouseup", endDrawing);
      triggerEl.addEventListener("mouseleave", endDrawing);
    });
    onUnmounted(() => {
      triggerEl.removeEventListener("mousedown", startDrawing);
      triggerEl.removeEventListener("mousemove", draw);
      triggerEl.removeEventListener("mouseup", endDrawing);
      triggerEl.removeEventListener("mouseleave", endDrawing);
    });
    function clear() {
      ctx.value.clearRect(0, 0, canvasEl.value.width, canvasEl.value.height);
    }
    __expose({
      eye,
      exportBase64,
      clear
    });
    return (_ctx, _cache) => {
      const _component_CursorVue = _sfc_main$5;
      return openBlock(), createElementBlock(Fragment, null, [
        createBaseVNode("div", {
          class: "paint-board-vue",
          style: normalizeStyle({
            width: unref(width) + "px",
            height: unref(height) + "px"
          })
        }, [
          createBaseVNode("img", {
            class: "img",
            src: props.src,
            alt: props.src
          }, null, 8, _hoisted_1$3),
          createBaseVNode("canvas", {
            ref_key: "canvasEl",
            ref: canvasEl,
            class: "canvas"
          }, null, 512)
        ], 4),
        createVNode(_component_CursorVue, {
          container: ".pinch-zoom-container",
          size: brushSize.value
        }, null, 8, ["size"])
      ], 64);
    };
  }
};
const _hoisted_1$2 = { class: "dialog-container" };
const _hoisted_2$1 = { class: "main" };
const _hoisted_3$1 = { class: "title" };
const _hoisted_4$1 = { class: "description" };
const _hoisted_5$1 = { class: "action" };
const _sfc_main$3 = {
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
    }
  },
  emits: ["ok", "cancel", "update:modelValue"],
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
    const visible = computed({
      get: () => props.modelValue,
      set: (value) => {
        emit("update:modelValue", value);
      }
    });
    return (_ctx, _cache) => {
      const _component_ImageVue = _sfc_main$d;
      const _component_el_button = ElButton;
      const _component_el_dialog = ElDialog;
      return openBlock(), createBlock(_component_el_dialog, {
        modelValue: unref(visible),
        "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => isRef(visible) ? visible.value = $event : null),
        class: "dialog-vue",
        "append-to-body": "",
        "align-center": "",
        onClose: cancel,
        "close-on-click-modal": props.closeOnClickModal
      }, {
        default: withCtx(() => [
          createBaseVNode("div", _hoisted_1$2, [
            createVNode(_component_ImageVue, {
              class: "dialog-icon",
              width: "36",
              height: "36",
              src: `light/base/dialog-${props.type}.png`,
              darkSrc: `dark/base/dialog-${props.type}.png`
            }, null, 8, ["src", "darkSrc"]),
            createBaseVNode("div", _hoisted_2$1, [
              createBaseVNode("div", _hoisted_3$1, [
                renderSlot(_ctx.$slots, "title", {}, () => [
                  createTextVNode("title")
                ])
              ]),
              createBaseVNode("div", _hoisted_4$1, [
                renderSlot(_ctx.$slots, "description", {}, () => [
                  createTextVNode("description")
                ])
              ]),
              createBaseVNode("div", _hoisted_5$1, [
                props.showCancelBtn ? (openBlock(), createBlock(_component_el_button, {
                  key: 0,
                  class: "cancel",
                  type: "",
                  onClick: cancel
                }, {
                  default: withCtx(() => [
                    renderSlot(_ctx.$slots, "cancel", {}, () => [
                      createTextVNode("cancel")
                    ])
                  ]),
                  _: 3
                })) : createCommentVNode("", true),
                props.showOkBtn ? (openBlock(), createBlock(_component_el_button, {
                  key: 1,
                  class: "ok ai-style",
                  type: "primary",
                  onClick: ok
                }, {
                  default: withCtx(() => [
                    renderSlot(_ctx.$slots, "ok", {}, () => [
                      createTextVNode("ok")
                    ])
                  ]),
                  _: 3
                })) : createCommentVNode("", true)
              ])
            ])
          ])
        ]),
        _: 3
      }, 8, ["modelValue", "close-on-click-modal"]);
    };
  }
};
const _hoisted_1$1 = { style: { "margin-right": "2px" } };
const _sfc_main$2 = {
  __name: "ThumbtackVue",
  setup(__props) {
    const mousetrap = inject("mousetrap");
    const isAlwaysOnTop = ref(false);
    const toggleAlwaysOnTop = async () => {
      isAlwaysOnTop.value = !isAlwaysOnTop.value;
      await eagle.window.setAlwaysOnTop(isAlwaysOnTop.value);
    };
    onMounted(async () => {
      mousetrap.bind(["shift+t"], toggleAlwaysOnTop);
      isAlwaysOnTop.value = await eagle.window.isAlwaysOnTop();
    });
    return (_ctx, _cache) => {
      const _component_ImageVue = _sfc_main$d;
      const _component_key = resolveComponent("key");
      const _component_tippy = resolveComponent("tippy");
      return openBlock(), createBlock(_component_tippy, {
        allowHTML: "",
        placement: "bottom",
        duration: "[200,0]",
        delay: "[0,0]"
      }, {
        default: withCtx(() => [
          createVNode(_component_ImageVue, {
            class: normalizeClass(["icon", {
              "icon-active": unref(isAlwaysOnTop)
            }]),
            width: "24",
            height: "24",
            src: unref(isAlwaysOnTop) ? "light/base/ic-thumbtack-pinned.svg" : "light/base/ic-thumbtack.svg",
            darkSrc: unref(isAlwaysOnTop) ? "dark/base/ic-thumbtack-pinned.svg" : "dark/base/ic-thumbtack.svg",
            onClick: toggleAlwaysOnTop
          }, null, 8, ["class", "src", "darkSrc"])
        ]),
        content: withCtx(() => [
          createBaseVNode("span", _hoisted_1$1, toDisplayString(unref(isAlwaysOnTop) ? _ctx.$translate("header.thumbtack.isNotAlwaysOnTop") : _ctx.$translate("header.thumbtack.isAlwaysOnTop")), 1),
          createVNode(_component_key, null, {
            default: withCtx(() => [
              createTextVNode(toDisplayString(_ctx.$keyboard("shift")), 1)
            ]),
            _: 1
          }),
          createVNode(_component_key, null, {
            default: withCtx(() => [
              createTextVNode(toDisplayString(_ctx.$keyboard("T")), 1)
            ]),
            _: 1
          })
        ]),
        _: 1
      });
    };
  }
};
const _imports_0 = "" + new URL("../../logo.png", import.meta.url).href;
const _hoisted_1 = { class: "header-vue" };
const _hoisted_2 = { class: "drag" };
const _hoisted_3 = /* @__PURE__ */ createBaseVNode("img", {
  class: "logo",
  src: _imports_0,
  alt: "logo"
}, null, -1);
const _hoisted_4 = { class: "title" };
const _hoisted_5 = { class: "action" };
const _hoisted_6 = {
  key: 0,
  class: "dash"
};
const _sfc_main$1 = {
  __name: "HeaderVue",
  props: {
    dash: {
      type: Boolean,
      default: false
    }
  },
  setup(__props) {
    const props = __props;
    const title = eagle.plugin.manifest.name;
    const closeDialog = reactive({
      visible: false,
      type: "warning",
      ok: () => {
        window.close();
      }
    });
    return (_ctx, _cache) => {
      const _component_ThumbtackVue = _sfc_main$2;
      const _component_ImageVue = _sfc_main$d;
      const _component_DialogVue = _sfc_main$3;
      return openBlock(), createElementBlock(Fragment, null, [
        createBaseVNode("div", _hoisted_1, [
          createBaseVNode("div", _hoisted_2, [
            _hoisted_3,
            createBaseVNode("span", _hoisted_4, toDisplayString(unref(title)), 1)
          ]),
          createBaseVNode("div", _hoisted_5, [
            renderSlot(_ctx.$slots, "default"),
            props.dash ? (openBlock(), createElementBlock("div", _hoisted_6)) : createCommentVNode("", true),
            createVNode(_component_ThumbtackVue),
            createVNode(_component_ImageVue, {
              class: "icon close",
              width: "24",
              height: "24",
              src: "light/base/ic-header-close.svg",
              darkSrc: "dark/base/ic-header-close.svg",
              onClick: _cache[0] || (_cache[0] = ($event) => unref(closeDialog).visible = true)
            })
          ])
        ]),
        createVNode(_component_DialogVue, {
          modelValue: unref(closeDialog).visible,
          "onUpdate:modelValue": _cache[1] || (_cache[1] = ($event) => unref(closeDialog).visible = $event),
          type: unref(closeDialog).type,
          onOk: unref(closeDialog).ok
        }, {
          title: withCtx(() => [
            createTextVNode(toDisplayString(_ctx.$translate("header.dialog.exit.title")), 1)
          ]),
          description: withCtx(() => [
            createTextVNode(toDisplayString(_ctx.$translate("header.dialog.exit.description")), 1)
          ]),
          cancel: withCtx(() => [
            createTextVNode(toDisplayString(_ctx.$translate("header.dialog.exit.cancel")), 1)
          ]),
          ok: withCtx(() => [
            createTextVNode(toDisplayString(_ctx.$translate("header.dialog.exit.ok")), 1)
          ]),
          _: 1
        }, 8, ["modelValue", "type", "onOk"])
      ], 64);
    };
  }
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
const utils$1 = require(`${__dirname}/modules/utils`);
const AIEraser = require(`${__dirname}/modules/ai-eraser`);
class Main {
  constructor() {
    this.isLoading = true;
    this.suppertedFileTypes = ["jpg", "jpeg", "png", "gif", "webp", "bmp", "svg"];
    this.isInputError = false;
    this.errorType = "";
    this.item_id = null;
    this.brushSize = 50;
    this.status = "";
    this.imageHistoryManager = new DataHistoryManager();
  }
  async convert(image, mask) {
    try {
      this.status = "processing";
      console.time("process");
      eagle.log.info(`start converting`);
      const result = await AIEraser.convert(image, mask);
      eagle.log.info(`end converting`);
      console.timeEnd("process");
      return result;
    } catch (error) {
      eagle.log.error(error);
    } finally {
      this.status = "";
    }
  }
  async saveAs() {
    try {
      eagle.log.info("start saveAs");
      const dialog = await eagle.dialog.showOpenDialog({
        properties: ["openDirectory"]
      });
      if (dialog.canceled) return;
      const folder = dialog.filePaths[0];
      const url = this.imageHistoryManager.current;
      const file_path = `${folder}/${crypto.randomUUID()}.png`;
      await utils$1.file.save(file_path, url);
      await eagle.shell.showItemInFolder(file_path);
      this.status = "success";
    } catch (error) {
      eagle.log.error(error);
      this.status = "fail";
    } finally {
      eagle.log.info("end saveAs");
    }
  }
  async replace() {
    try {
      const url = this.imageHistoryManager.current;
      const tempFilePath = `${__dirname}/temp/${crypto.randomUUID()}.png`;
      await utils$1.file.save(tempFilePath, url);
      await (await eagle.item.getById(this.item_id)).replaceFile(tempFilePath);
      await utils$1.file.destroy(tempFilePath);
      this.status = "success";
    } catch (error) {
      eagle.log.error(error);
      this.status = "fail";
    } finally {
      eagle.log.info("end replace");
    }
  }
}
const _sfc_main = {
  __name: "App",
  setup(__props) {
    const mousetrap = inject("mousetrap");
    const utils2 = require(`${__dirname}/modules/utils`);
    const main = reactive(new Main());
    const pinchZoomEl = ref(null);
    const paintBoardEl = ref(null);
    const toolbarEl = ref(null);
    onMounted(async () => {
      const items = await eagle.item.getSelected();
      const item = items == null ? void 0 : items[0];
      if (item) {
        await loadData(item);
      }
      mousetrap.bind(["command+0", "ctrl+0"], () => {
        if (main.status === "processing") return false;
        pinchZoomEl.value.scale(1);
        return false;
      });
      mousetrap.bind(["command+9", "ctrl+9"], () => {
        if (main.status === "processing") return false;
        pinchZoomEl.value.fit();
        return false;
      });
      mousetrap.bind(["+", "=", "command++", "command+=", "ctrl++", "ctrl+="], () => {
        if (main.status === "processing") return false;
        pinchZoomEl.value.scaleIn();
        return false;
      });
      mousetrap.bind(["-", "command+-", "ctrl+-"], () => {
        if (main.status === "processing") return false;
        pinchZoomEl.value.scaleOut();
        return false;
      });
      mousetrap.bind(["command+z", "ctrl+z"], () => {
        if (main.status === "processing") return false;
        main.imageHistoryManager.undo();
        return false;
      });
      mousetrap.bind(["command+shift+z", "ctrl+shift+z"], () => {
        if (main.status === "processing") return false;
        main.imageHistoryManager.redo();
        return false;
      });
      mousetrap.bind(["["], () => {
        var _a, _b;
        if (main.status === "processing") return false;
        main.brushSize = Math.max(
          main.brushSize - ((_a = toolbarEl.value) == null ? void 0 : _a.brushSizeRange.step),
          (_b = toolbarEl.value) == null ? void 0 : _b.brushSizeRange.min
        );
        return false;
      });
      mousetrap.bind(["]"], () => {
        var _a, _b;
        if (main.status === "processing") return false;
        main.brushSize = Math.min(
          main.brushSize + ((_a = toolbarEl.value) == null ? void 0 : _a.brushSizeRange.step),
          (_b = toolbarEl.value) == null ? void 0 : _b.brushSizeRange.max
        );
        return false;
      });
    });
    const onFileDrop = async (files) => {
      if (files.length) {
        const file = files[0];
        const id = new RegExp("(?<=\\images[\\\\\\/])(.*?)(?=\\.info)", "g").exec(file.path)[0] ?? null;
        if (!id) throw "file not come from Eagle";
        const item = await eagle.item.getById(id);
        await loadData(item);
      }
    };
    eagle.onPluginRun(async () => {
      const items = await eagle.item.getSelected();
      if (items.length) {
        const item = markRaw(items[0]);
        await loadData(item);
      }
    });
    const loadData = async (item) => {
      main.imageHistoryManager.reset();
      main.isInputError = false;
      if (!main.suppertedFileTypes.includes(item.ext)) {
        main.isInputError = true;
        main.errorType = "fileExtensionNotSupported";
        throw new Error(main.errorType);
      }
      if (item.width > 1e4 || item.height > 1e4) {
        main.isInputError = true;
        main.errorType = "resolutionNotSupported";
        throw new Error(main.errorType);
      }
      main.item_id = item.id;
      const img = await utils2.image.create(item.fileURL);
      const canvas = document.createElement("canvas");
      const ctx = canvas.getContext("2d");
      canvas.width = img.width;
      canvas.height = img.height;
      ctx.drawImage(img, 0, 0);
      main.imageHistoryManager.add(canvas.toDataURL("image/png"));
    };
    provide("main", main);
    return (_ctx, _cache) => {
      const _component_HeaderVue = _sfc_main$1;
      const _component_PaintBoardVue = _sfc_main$4;
      const _component_PinchZoomVue = _sfc_main$6;
      const _component_ToolbarVue = _sfc_main$7;
      const _component_NotifyVue = _sfc_main$9;
      const _component_ImageVue = _sfc_main$d;
      const _component_el_empty = ElEmpty;
      const _component_DropZoneVue = _sfc_main$a;
      const _component_BodyVue = __unplugin_components_8;
      const _component_ModelDialogVue = _sfc_main$c;
      return openBlock(), createElementBlock(Fragment, null, [
        createVNode(_component_HeaderVue, { dash: false }),
        !unref(main).isLoading ? (openBlock(), createBlock(_component_BodyVue, { key: 0 }, {
          default: withCtx(() => [
            createVNode(_component_DropZoneVue, { onDrop: onFileDrop }, {
              default: withCtx(() => [
                unref(main).item_id && unref(main).imageHistoryManager.original ? (openBlock(), createElementBlock(Fragment, { key: 0 }, [
                  createVNode(_component_PinchZoomVue, {
                    style: { "bottom": "60px" },
                    ref_key: "pinchZoomEl",
                    ref: pinchZoomEl,
                    dragMove: false,
                    fit: ""
                  }, {
                    default: withCtx(() => {
                      var _a;
                      return [
                        createVNode(_component_PaintBoardVue, {
                          trigger: ".pinch-zoom-container",
                          ref_key: "paintBoardEl",
                          ref: paintBoardEl,
                          src: unref(main).imageHistoryManager.view,
                          offset: (_a = unref(pinchZoomEl)) == null ? void 0 : _a.offset,
                          brushSize: unref(main).brushSize,
                          onEndDrawing: _cache[0] || (_cache[0] = async () => {
                            var _a2, _b;
                            const { imageBase64, maskBase64 } = await ((_a2 = unref(paintBoardEl)) == null ? void 0 : _a2.exportBase64()) ?? {};
                            const resultLink = await unref(main).convert(imageBase64, maskBase64);
                            unref(main).imageHistoryManager.add(resultLink);
                            (_b = unref(paintBoardEl)) == null ? void 0 : _b.clear();
                          })
                        }, null, 8, ["src", "offset", "brushSize"])
                      ];
                    }),
                    _: 1
                  }, 512),
                  createVNode(_component_ToolbarVue, {
                    ref_key: "toolbarEl",
                    ref: toolbarEl,
                    brushSize: unref(main).brushSize,
                    "onUpdate:brushSize": _cache[1] || (_cache[1] = ($event) => unref(main).brushSize = $event),
                    brushSizeRange: _ctx.brushSizeRange,
                    pinchZoomEl: unref(pinchZoomEl),
                    class: normalizeClass({ disabled: unref(main).status !== "" })
                  }, null, 8, ["brushSize", "brushSizeRange", "pinchZoomEl", "class"]),
                  createVNode(_component_NotifyVue)
                ], 64)) : !unref(main).item_id ? (openBlock(), createElementBlock(Fragment, { key: 1 }, [
                  unref(main).isInputError ? (openBlock(), createBlock(_component_el_empty, {
                    key: 0,
                    description: _ctx.$translate(`main.error`),
                    "image-size": 256
                  }, {
                    image: withCtx(() => [
                      createVNode(_component_ImageVue, {
                        width: "256",
                        height: "144",
                        src: "light/fail.png",
                        darkSrc: "dark/fail.png"
                      })
                    ]),
                    default: withCtx(() => [
                      createTextVNode(" " + toDisplayString(_ctx.$translate(`error.${unref(main).errorType}`)), 1)
                    ]),
                    _: 1
                  }, 8, ["description"])) : (openBlock(), createBlock(_component_el_empty, {
                    key: 1,
                    description: _ctx.$translate("main.empty.title"),
                    "image-size": 256
                  }, {
                    image: withCtx(() => [
                      createVNode(_component_ImageVue, {
                        width: "256",
                        height: "144",
                        src: "light/empty.png",
                        darkSrc: "dark/empty.png"
                      })
                    ]),
                    default: withCtx(() => [
                      createTextVNode(" " + toDisplayString(_ctx.$translate("main.empty.content")), 1)
                    ]),
                    _: 1
                  }, 8, ["description"]))
                ], 64)) : createCommentVNode("", true)
              ]),
              _: 1
            })
          ]),
          _: 1
        })) : createCommentVNode("", true),
        createVNode(_component_ModelDialogVue, {
          onClose: _cache[2] || (_cache[2] = async () => {
            unref(main).isLoading = false;
          })
        })
      ], 64);
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
(() => {
  class Pointer {
    constructor(nativePointer) {
      this.id = -1;
      this.nativePointer = nativePointer;
      this.pageX = nativePointer.pageX;
      this.pageY = nativePointer.pageY;
      this.clientX = nativePointer.clientX;
      this.clientY = nativePointer.clientY;
      if (self.Touch && nativePointer instanceof Touch) {
        this.id = nativePointer.identifier;
      } else if (isPointerEvent(nativePointer)) {
        this.id = nativePointer.pointerId;
      }
    }
    /**
     * Returns an expanded set of Pointers for high-resolution inputs.
     */
    getCoalesced() {
      if ("getCoalescedEvents" in this.nativePointer) {
        return this.nativePointer.getCoalescedEvents().map((p) => new Pointer(p));
      }
      return [this];
    }
  }
  const isPointerEvent = (event2) => self.PointerEvent && event2 instanceof PointerEvent;
  const noop = () => {
  };
  class PointerTracker {
    /**
     * Track pointers across a particular element
     *
     * @param element Element to monitor.
     * @param callbacks
     */
    constructor(_element, callbacks) {
      this._element = _element;
      this.startPointers = [];
      this.currentPointers = [];
      const { start = () => true, move = noop, end = noop } = callbacks;
      this._startCallback = start;
      this._moveCallback = move;
      this._endCallback = end;
      this._pointerStart = this._pointerStart.bind(this);
      this._touchStart = this._touchStart.bind(this);
      this._move = this._move.bind(this);
      this._triggerPointerEnd = this._triggerPointerEnd.bind(this);
      this._pointerEnd = this._pointerEnd.bind(this);
      this._touchEnd = this._touchEnd.bind(this);
      if (self.PointerEvent) {
        this._element.addEventListener("pointerdown", this._pointerStart);
      } else {
        this._element.addEventListener("mousedown", this._pointerStart);
        this._element.addEventListener("touchstart", this._touchStart);
        this._element.addEventListener("touchmove", this._move);
        this._element.addEventListener("touchend", this._touchEnd);
      }
    }
    /**
     * Call the start callback for this pointer, and track it if the user wants.
     *
     * @param pointer Pointer
     * @param event Related event
     * @returns Whether the pointer is being tracked.
     */
    _triggerPointerStart(pointer, event2) {
      if (!this._startCallback(pointer, event2)) return false;
      this.currentPointers.push(pointer);
      this.startPointers.push(pointer);
      return true;
    }
    /**
     * Listener for mouse/pointer starts. Bound to the class in the constructor.
     *
     * @param event This will only be a MouseEvent if the browser doesn't support
     * pointer events.
     */
    _pointerStart(event2) {
      if (event2.button !== 0) return;
      if (!this._triggerPointerStart(new Pointer(event2), event2)) return;
      if (isPointerEvent(event2)) {
        this._element.setPointerCapture(event2.pointerId);
        this._element.addEventListener("pointermove", this._move);
        this._element.addEventListener("pointerup", this._pointerEnd);
      } else {
        window.addEventListener("mousemove", this._move);
        window.addEventListener("mouseup", this._pointerEnd);
      }
    }
    /**
     * Listener for touchstart. Bound to the class in the constructor.
     * Only used if the browser doesn't support pointer events.
     */
    _touchStart(event2) {
      for (const touch of Array.from(event2.changedTouches)) {
        this._triggerPointerStart(new Pointer(touch), event2);
      }
    }
    /**
     * Listener for pointer/mouse/touch move events.
     * Bound to the class in the constructor.
     */
    _move(event2) {
      const previousPointers = this.currentPointers.slice();
      const changedPointers = "changedTouches" in event2 ? Array.from(event2.changedTouches).map((t) => new Pointer(t)) : [new Pointer(event2)];
      const trackedChangedPointers = [];
      for (const pointer of changedPointers) {
        const index = this.currentPointers.findIndex((p) => p.id === pointer.id);
        if (index === -1) continue;
        trackedChangedPointers.push(pointer);
        this.currentPointers[index] = pointer;
      }
      if (trackedChangedPointers.length === 0) return;
      this._moveCallback(previousPointers, trackedChangedPointers, event2);
    }
    /**
     * Call the end callback for this pointer.
     *
     * @param pointer Pointer
     * @param event Related event
     */
    _triggerPointerEnd(pointer, event2) {
      const index = this.currentPointers.findIndex((p) => p.id === pointer.id);
      if (index === -1) return false;
      this.currentPointers.splice(index, 1);
      this.startPointers.splice(index, 1);
      this._endCallback(pointer, event2);
      return true;
    }
    /**
     * Listener for mouse/pointer ends. Bound to the class in the constructor.
     * @param event This will only be a MouseEvent if the browser doesn't support
     * pointer events.
     */
    _pointerEnd(event2) {
      if (!this._triggerPointerEnd(new Pointer(event2), event2)) return;
      if (isPointerEvent(event2)) {
        if (this.currentPointers.length) return;
        this._element.removeEventListener("pointermove", this._move);
        this._element.removeEventListener("pointerup", this._pointerEnd);
      } else {
        window.removeEventListener("mousemove", this._move);
        window.removeEventListener("mouseup", this._pointerEnd);
      }
    }
    /**
     * Listener for touchend. Bound to the class in the constructor.
     * Only used if the browser doesn't support pointer events.
     */
    _touchEnd(event2) {
      for (const touch of Array.from(event2.changedTouches)) {
        this._triggerPointerEnd(new Pointer(touch), event2);
      }
    }
  }
  function styleInject(css2, ref2) {
    if (ref2 === void 0) ref2 = {};
    var insertAt = ref2.insertAt;
    if (typeof document === "undefined") {
      return;
    }
    var head = document.head || document.getElementsByTagName("head")[0];
    var style = document.createElement("style");
    style.type = "text/css";
    if (insertAt === "top") {
      if (head.firstChild) {
        head.insertBefore(style, head.firstChild);
      } else {
        head.appendChild(style);
      }
    } else {
      head.appendChild(style);
    }
    if (style.styleSheet) {
      style.styleSheet.cssText = css2;
    } else {
      style.appendChild(document.createTextNode(css2));
    }
  }
  var scrubber = "styles_scrubber__39cN6";
  var twoUpHandle = "styles_two-up-handle__2kVsP";
  var css = 'two-up{display:grid;position:relative;--split-point:0;--accent-color:#777;--track-color:var(--accent-color);--thumb-background:#fff;--thumb-color:var(--accent-color);--thumb-size:62px;--bar-size:6px;--bar-touch-size:30px}two-up>*{grid-area:1/1}two-up[legacy-clip-compat]>:not(.styles_two-up-handle__2kVsP){position:absolute}.styles_two-up-handle__2kVsP{touch-action:none;position:relative;width:var(--bar-touch-size);transform:translateX(var(--split-point)) translateX(-50%);will-change:transform;cursor:ew-resize}.styles_two-up-handle__2kVsP:before{content:"";display:block;height:100%;width:var(--bar-size);margin:0 auto;box-shadow:inset calc(var(--bar-size) / 2) 0 0 rgba(0,0,0,.1),0 1px 4px rgba(0,0,0,.4);background:var(--track-color)}.styles_scrubber__39cN6{display:flex;position:absolute;top:50%;left:50%;transform-origin:50% 50%;transform:translate(-50%,-50%);width:var(--thumb-size);height:calc(var(--thumb-size) * .9);background:var(--thumb-background);border:1px solid rgba(0,0,0,.2);border-radius:var(--thumb-size);box-shadow:0 1px 4px rgba(0,0,0,.1);color:var(--thumb-color);box-sizing:border-box;padding:0 calc(var(--thumb-size) * .24)}.styles_scrubber__39cN6 svg{flex:1}two-up[orientation=vertical] .styles_two-up-handle__2kVsP{width:auto;height:var(--bar-touch-size);transform:translateY(var(--split-point)) translateY(-50%);cursor:ns-resize}two-up[orientation=vertical] .styles_two-up-handle__2kVsP:before{width:auto;height:var(--bar-size);box-shadow:inset 0 calc(var(--bar-size) / 2) 0 rgba(0,0,0,.1),0 1px 4px rgba(0,0,0,.4);margin:calc((var(--bar-touch-size) - var(--bar-size)) / 2) 0 0}two-up[orientation=vertical] .styles_scrubber__39cN6{box-shadow:1px 0 4px rgba(0,0,0,.1);transform:translate(-50%,-50%) rotate(-90deg)}two-up>:first-child:not(.styles_two-up-handle__2kVsP){-webkit-clip-path:inset(0 calc(100% - var(--split-point)) 0 0);clip-path:inset(0 calc(100% - var(--split-point)) 0 0)}two-up>:nth-child(2):not(.styles_two-up-handle__2kVsP){-webkit-clip-path:inset(0 0 0 var(--split-point));clip-path:inset(0 0 0 var(--split-point))}two-up[orientation=vertical]>:first-child:not(.styles_two-up-handle__2kVsP){-webkit-clip-path:inset(0 0 calc(100% - var(--split-point)) 0);clip-path:inset(0 0 calc(100% - var(--split-point)) 0)}two-up[orientation=vertical]>:nth-child(2):not(.styles_two-up-handle__2kVsP){-webkit-clip-path:inset(var(--split-point) 0 0 0);clip-path:inset(var(--split-point) 0 0 0)}@supports not ((clip-path:inset(0 0 0 0)) or (-webkit-clip-path:inset(0 0 0 0))){two-up[legacy-clip-compat]>:first-child:not(.styles_two-up-handle__2kVsP){clip:rect(auto var(--split-point) auto auto)}two-up[legacy-clip-compat]>:nth-child(2):not(.styles_two-up-handle__2kVsP){clip:rect(auto auto auto var(--split-point))}two-up[orientation=vertical][legacy-clip-compat]>:first-child:not(.styles_two-up-handle__2kVsP){clip:rect(auto auto var(--split-point) auto)}two-up[orientation=vertical][legacy-clip-compat]>:nth-child(2):not(.styles_two-up-handle__2kVsP){clip:rect(var(--split-point) auto auto auto)}}';
  styleInject(css);
  const legacyClipCompatAttr = "legacy-clip-compat";
  const orientationAttr = "orientation";
  class TwoUp extends HTMLElement {
    constructor() {
      super();
      this._handle = document.createElement("div");
      this._position = 0;
      this._relativePosition = 0.5;
      this._positionOnPointerStart = 0;
      this._everConnected = false;
      this._handle.className = twoUpHandle;
      new MutationObserver(() => this._childrenChange()).observe(this, { childList: true });
      if ("ResizeObserver" in window) {
        new ResizeObserver(() => this._resetPosition()).observe(this);
      } else {
        window.addEventListener("resize", () => this._resetPosition());
      }
      const pointerTracker = new PointerTracker(this._handle, {
        start: (_, event2) => {
          if (pointerTracker.currentPointers.length === 1) return false;
          event2.preventDefault();
          this._positionOnPointerStart = this._position;
          return true;
        },
        move: () => {
          this._pointerChange(
            pointerTracker.startPointers[0],
            pointerTracker.currentPointers[0]
          );
        }
      });
    }
    static get observedAttributes() {
      return [orientationAttr];
    }
    connectedCallback() {
      this._childrenChange();
      this._handle.innerHTML = `<div class="${scrubber}">${`<svg viewBox="0 0 27 20" fill="currentColor">${'<path d="M17 19.2l9.5-9.6L16.9 0zM9.6 0L0 9.6l9.6 9.6z"/>'}</svg>`}</div>`;
      if (!this._everConnected) {
        this._resetPosition();
        this._everConnected = true;
      }
    }
    attributeChangedCallback(name) {
      if (name === orientationAttr) {
        this._resetPosition();
      }
    }
    _resetPosition() {
      requestAnimationFrame(() => {
        const bounds = this.getBoundingClientRect();
        const dimensionAxis = this.orientation === "vertical" ? "height" : "width";
        this._position = bounds[dimensionAxis] * this._relativePosition;
        this._setPosition();
      });
    }
    /**
     * If true, this element works in browsers that don't support clip-path (Edge).
     * However, this means you'll have to set the height of this element manually.
     */
    get legacyClipCompat() {
      return this.hasAttribute(legacyClipCompatAttr);
    }
    set legacyClipCompat(val) {
      if (val) {
        this.setAttribute(legacyClipCompatAttr, "");
      } else {
        this.removeAttribute(legacyClipCompatAttr);
      }
    }
    /**
     * Split vertically rather than horizontally.
     */
    get orientation() {
      const value = this.getAttribute(orientationAttr);
      if (value && value.toLowerCase() === "vertical") return "vertical";
      return "horizontal";
    }
    set orientation(val) {
      this.setAttribute(orientationAttr, val);
    }
    /**
     * Called when element's child list changes
     */
    _childrenChange() {
      if (this.lastElementChild !== this._handle) {
        this.appendChild(this._handle);
      }
    }
    /**
     * Called when a pointer moves.
     */
    _pointerChange(startPoint, currentPoint) {
      const pointAxis = this.orientation === "vertical" ? "clientY" : "clientX";
      const dimensionAxis = this.orientation === "vertical" ? "height" : "width";
      const bounds = this.getBoundingClientRect();
      this._position = this._positionOnPointerStart + (currentPoint[pointAxis] - startPoint[pointAxis]);
      this._position = Math.max(0, Math.min(this._position, bounds[dimensionAxis]));
      this._relativePosition = this._position / bounds[dimensionAxis];
      this._setPosition();
    }
    _setPosition() {
      this.style.setProperty("--split-point", `${this._position}px`);
    }
  }
  customElements.define("two-up", TwoUp);
})();
(() => {
  function styleInject(css2, ref2) {
    if (ref2 === void 0) ref2 = {};
    var insertAt = ref2.insertAt;
    if (typeof document === "undefined") return;
    var head = document.head || document.getElementsByTagName("head")[0];
    var style = document.createElement("style");
    style.type = "text/css";
    if (insertAt === "top") {
      if (head.firstChild) {
        head.insertBefore(style, head.firstChild);
      } else {
        head.appendChild(style);
      }
    } else {
      head.appendChild(style);
    }
    if (style.styleSheet) {
      style.styleSheet.cssText = css2;
    } else {
      style.appendChild(document.createTextNode(css2));
    }
  }
  var css = "pinch-zoom {\ndisplay: block;overflow: hidden;\ntouch-action: none;\n--scale: 1;\n  --x: 0;\n  --y: 0;\n}\npinch-zoom > * {\n  transform: translate(var(--x), var(--y)) scale(var(--scale));\n  transform-origin: 0 0;\n  will-change: transform;\n}\n";
  styleInject(css);
  function getAbsoluteValue(value, max) {
    if (typeof value === "number") return value;
    if (value.trimRight().endsWith("%")) return max * parseFloat(value) / 100;
    return parseFloat(value);
  }
  let cachedSvg;
  function getSVG() {
    return cachedSvg || (cachedSvg = document.createElementNS("http://www.w3.org/2000/svg", "svg"));
  }
  function createMatrix() {
    return getSVG().createSVGMatrix();
  }
  function createPoint() {
    return getSVG().createSVGPoint();
  }
  const containerAttr = "container";
  const minScaleAttr = "min-scale";
  const maxScaleAttr = "max-scale";
  const dragMoveAttr = "drag-move";
  const MIN_SCALE = 0.05;
  const MAX_SCALE = 8;
  class PinchZoom extends HTMLElement {
    constructor() {
      super();
      this._transform = createMatrix();
      new MutationObserver(() => {
        this._stageElChange();
      }).observe(this, {
        childList: true
      });
    }
    static get observedAttributes() {
      return [containerAttr, minScaleAttr, maxScaleAttr, dragMoveAttr];
    }
    attributeChangedCallback(name, oldValue, newValue) {
      if (name === containerAttr) {
        this.container.addEventListener("wheel", this._onWheel.bind(this), {
          passive: false
        });
      }
      if (name === minScaleAttr && this.scale < this.minScale) {
        this.setTransform({ scale: this.minScale });
      }
      if (name === maxScaleAttr && this.scale > this.maxScale) {
        this.setTransform({ scale: this.maxScale });
      }
    }
    set container(value) {
      this.setAttribute(containerAttr, value);
    }
    get container() {
      return document.querySelector(this.getAttribute(containerAttr)) || this;
    }
    set minScale(value) {
      this.setAttribute(minScaleAttr, String(value));
    }
    get minScale() {
      const attrValue = this.getAttribute(minScaleAttr);
      if (!attrValue) return MIN_SCALE;
      const value = parseFloat(attrValue);
      if (Number.isFinite(value)) return Math.max(MIN_SCALE, value);
      return MIN_SCALE;
    }
    set maxScale(value) {
      this.setAttribute(maxScaleAttr, String(value));
    }
    get maxScale() {
      const attrValue = this.getAttribute(maxScaleAttr);
      if (!attrValue) return MAX_SCALE;
      const value = parseFloat(attrValue);
      if (Number.isFinite(value)) return Math.min(MAX_SCALE, value);
      return MIN_SCALE;
    }
    set dragMove(value) {
      this.setAttribute(dragMoveAttr, String(value));
    }
    get dragMove() {
      const attrValue = this.getAttribute(dragMoveAttr);
      if (!attrValue) return false;
      return JSON.parse(attrValue);
    }
    connectedCallback() {
      this._stageElChange();
    }
    get x() {
      return this._transform.e;
    }
    get y() {
      return this._transform.f;
    }
    get scale() {
      return this._transform.a;
    }
    scaleTo(scale, opts = {}) {
      let { originX = 0, originY = 0 } = opts;
      const { relativeTo = "content", allowChangeEvent = false } = opts;
      const relativeToEl = relativeTo === "content" ? this._positioningEl : this;
      if (!relativeToEl || !this._positioningEl) {
        this.setTransform({ scale, allowChangeEvent });
        return;
      }
      const rect = relativeToEl.getBoundingClientRect();
      originX = getAbsoluteValue(originX, rect.width);
      originY = getAbsoluteValue(originY, rect.height);
      if (relativeTo === "content") {
        originX += this.x;
        originY += this.y;
      } else {
        const currentRect = this._positioningEl.getBoundingClientRect();
        originX -= currentRect.left;
        originY -= currentRect.top;
      }
      this._applyChange({
        allowChangeEvent,
        originX,
        originY,
        scaleDiff: scale / this.scale
      });
    }
    setTransform(opts = {}) {
      const { scale = this.scale, allowChangeEvent = false } = opts;
      let { x = this.x, y = this.y } = opts;
      if (!this._positioningEl) {
        this._updateTransform(scale, x, y, allowChangeEvent);
        return;
      }
      const thisBounds = this.getBoundingClientRect();
      const positioningElBounds = this._positioningEl.getBoundingClientRect();
      if (!thisBounds.width || !thisBounds.height) {
        this._updateTransform(scale, x, y, allowChangeEvent);
        return;
      }
      let topLeft = createPoint();
      topLeft.x = positioningElBounds.left - thisBounds.left;
      topLeft.y = positioningElBounds.top - thisBounds.top;
      let bottomRight = createPoint();
      bottomRight.x = positioningElBounds.width + topLeft.x;
      bottomRight.y = positioningElBounds.height + topLeft.y;
      const matrix = createMatrix().translate(x, y).scale(scale).multiply(this._transform.inverse());
      topLeft = topLeft.matrixTransform(matrix);
      bottomRight = bottomRight.matrixTransform(matrix);
      if (topLeft.x > thisBounds.width) {
        x += thisBounds.width - topLeft.x;
      } else if (bottomRight.x < 0) {
        x += -bottomRight.x;
      }
      if (topLeft.y > thisBounds.height) {
        y += thisBounds.height - topLeft.y;
      } else if (bottomRight.y < 0) {
        y += -bottomRight.y;
      }
      this._updateTransform(scale, x, y, allowChangeEvent);
    }
    _updateTransform(scale, x, y, allowChangeEvent) {
      if (scale > this.maxScale) return;
      if (scale < this.minScale) return;
      if (scale === this.scale && (event == null ? void 0 : event.metaKey)) return;
      this._transform.e = x;
      this._transform.f = y;
      this._transform.d = this._transform.a = scale;
      this.style.setProperty("--x", this.x + "px");
      this.style.setProperty("--y", this.y + "px");
      this.style.setProperty("--scale", this.scale + "");
      if (allowChangeEvent) {
        const event2 = new Event("change", { bubbles: true });
        this.dispatchEvent(event2);
      }
    }
    _stageElChange() {
      this._positioningEl = void 0;
      if (this.children.length === 0) return;
      this._positioningEl = this.children[0];
      if (this.children.length > 1) {
        console.warn("<pinch-zoom> must not have more than one child.");
      }
      this.setTransform({ allowChangeEvent: true });
    }
    _onWheel(event2) {
      if (event2.metaKey || event2.ctrlKey || this.dragMove) {
        if (!this._positioningEl) return;
        const currentRect = this._positioningEl.getBoundingClientRect();
        const { deltaY } = event2;
        const divisor = 100;
        const scaleDiff = 1 - (deltaY > 0 ? 1 : -1) * Math.min(divisor / 2, Math.abs(deltaY)) / divisor;
        this._applyChange({
          scaleDiff,
          originX: event2.clientX - currentRect.left,
          originY: event2.clientY - currentRect.top,
          allowChangeEvent: true
        });
      }
    }
    _applyChange(opts = {}) {
      const {
        panX = 0,
        panY = 0,
        originX = 0,
        originY = 0,
        scaleDiff = 1,
        allowChangeEvent = false
      } = opts;
      const matrix = createMatrix().translate(panX, panY).translate(originX, originY).translate(this.x, this.y).scale(scaleDiff).translate(-originX, -originY).scale(this.scale);
      this.setTransform({
        allowChangeEvent,
        scale: Math.min(this.maxScale, Math.max(matrix.a, this.minScale)),
        x: matrix.e,
        y: matrix.f
      });
    }
  }
  customElements.define("pinch-zoom", PinchZoom);
})();
const utils = require(`${__dirname}/modules/utils`);
const app = createApp(_sfc_main);
const ai_eraser = require(`${__dirname}/modules/ai-eraser`);
app.use(i18nPlugin);
app.use(keyboardPlugin);
app.use(plugin);
app.use(VueMousetrapPlugin).provide("mousetrap", app.config.globalProperties.$mousetrap);
app.provide("ai-eraser", ai_eraser);
eagle.onPluginCreate(async (plugin2) => {
  const utils2 = require(`${__dirname}/modules/utils`);
  if (eagle.app.platform === "darwin") await utils2.time.sleep(600);
  await eagle.window.setOpacity(1);
  app.mount("#app");
  toggleTheme();
  process.on("uncaughtException", (error) => {
    eagle.log.error("uncaughtException:" + error);
  });
});
window.onbeforeunload = (event2) => {
  ai_eraser.stopServer();
};
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
  await nextTick();
  htmlEl.classList.remove("no-transition");
}
