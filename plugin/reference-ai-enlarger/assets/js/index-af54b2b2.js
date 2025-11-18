import { r as ref, p as watchEffect, o as openBlock, c as createElementBlock, a as createBaseVNode, J as normalizeStyle, u as unref, C as renderSlot, t as computed, F as createBlock, G as withCtx, L as createVNode, a5 as createTextVNode, H as withDirectives, R as createCommentVNode, K as vShow, v as isRef, f as onMounted, n as nextTick, aZ as onUnmounted, z as onBeforeMount, bb as shallowReactive, I as normalizeClass, M as Fragment, Z as toDisplayString, a4 as withModifiers, bx as vModelText, O as reactive, q as onBeforeUnmount, a1 as resolveComponent, s as inject, b5 as resolveDirective, Q as resolveDynamicComponent, b4 as renderList, bv as vModelRadio, A as provide, ak as createApp } from "./@vue-08890544.js";
import { E as ElButton, a as ElDialog, b as ElDropdownItem, c as ElDropdownMenu, d as ElDropdown, e as ElEmpty, f as ElPopover, g as ElProgress } from "./element-plus-76f9d4e1.js";
import "./@imengyu-fef40a69.js";
import { i as interact } from "./interactjs-6343f899.js";
import { q as queue } from "./async-324d8929.js";
import { s as script$1, a as script } from "./vue-virtual-scroller-65a319ab.js";
import { V as VueTippy } from "./vue-tippy-f254cbb0.js";
import { V as VueMousetrapPlugin } from "./vue-mousetrap-b8952c29.js";
import "./@vueuse-c1991bbf.js";
import "./@element-plus-631de72b.js";
import "./@ctrl-ab5a38b7.js";
import "./lodash-es-96e680b8.js";
import "./@popperjs-8eb851c6.js";
import "./vue-d2e68e9a.js";
import "./vue-resize-9f64b672.js";
import "./vue-observe-visibility-87695bca.js";
import "./mitt-a1032778.js";
import "./mousetrap-ce6b1514.js";
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
    if (link.integrity)
      fetchOpts.integrity = link.integrity;
    if (link.referrerPolicy)
      fetchOpts.referrerPolicy = link.referrerPolicy;
    if (link.crossOrigin === "use-credentials")
      fetchOpts.credentials = "include";
    else if (link.crossOrigin === "anonymous")
      fetchOpts.credentials = "omit";
    else
      fetchOpts.credentials = "same-origin";
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
const ImageVue_vue_vue_type_style_index_0_lang = "";
const _hoisted_1$d = { class: "image-vue" };
const _hoisted_2$9 = ["src", "alt"];
const base_path = "./images/";
const _sfc_main$h = {
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
      return openBlock(), createElementBlock("div", _hoisted_1$d, [
        createBaseVNode("img", {
          style: normalizeStyle({
            width: props.width + "px",
            height: props.height + "px"
          }),
          src: base_path + unref(uri),
          alt: unref(uri),
          loading: "lazy"
        }, null, 12, _hoisted_2$9),
        renderSlot(_ctx.$slots, "default")
      ]);
    };
  }
};
const DialogVue_vue_vue_type_style_index_0_lang = "";
const _hoisted_1$c = { class: "dialog-container" };
const _hoisted_2$8 = { class: "main" };
const _hoisted_3$7 = { class: "title" };
const _hoisted_4$5 = { class: "description" };
const _hoisted_5$4 = { class: "action" };
const _sfc_main$g = {
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
  setup(__props, { emit }) {
    const props = __props;
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
      const _component_ImageVue = _sfc_main$h;
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
          createBaseVNode("div", _hoisted_1$c, [
            createVNode(_component_ImageVue, {
              class: "dialog-icon",
              width: "36",
              height: "36",
              src: `light/base/dialog-${props.type}.png`,
              darkSrc: `dark/base/dialog-${props.type}.png`
            }, null, 8, ["src", "darkSrc"]),
            createBaseVNode("div", _hoisted_2$8, [
              createBaseVNode("div", _hoisted_3$7, [
                renderSlot(_ctx.$slots, "title", {}, () => [
                  createTextVNode("title")
                ])
              ]),
              createBaseVNode("div", _hoisted_4$5, [
                renderSlot(_ctx.$slots, "description", {}, () => [
                  createTextVNode("description")
                ])
              ]),
              withDirectives(createBaseVNode("div", _hoisted_5$4, [
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
              ], 512), [
                [vShow, props.showCancelBtn || props.showOkBtn]
              ])
            ])
          ])
        ]),
        _: 3
      }, 8, ["modelValue", "close-on-click-modal"]);
    };
  }
};
const ButtonComet_vue_vue_type_style_index_0_scoped_80c25e79_lang = "";
const _export_sfc = (sfc, props) => {
  const target = sfc.__vccOpts || sfc;
  for (const [key, val] of props) {
    target[key] = val;
  }
  return target;
};
const _sfc_main$f = {
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
const __unplugin_components_13 = /* @__PURE__ */ _export_sfc(_sfc_main$f, [["__scopeId", "data-v-80c25e79"]]);
const t = (s, options) => i18next.t(
  s == null ? void 0 : s.trim().split(" ").map((s2, i) => i == 0 ? s2 : s2.charAt(0).toUpperCase() + s2.slice(1)).join(""),
  options
);
const keyboard = (s) => {
  s = s.toLowerCase();
  const data = [
    ["ctrl", "⌘"],
    ["alt", "⌥"],
    ["shift", "⇧"]
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
const ModelDialogVue_vue_vue_type_style_index_0_lang = "";
const _hoisted_1$b = { class: "title" };
const _hoisted_2$7 = { class: "description" };
const _hoisted_3$6 = { class: "size" };
const _hoisted_4$4 = { class: "title" };
const _hoisted_5$3 = { class: "description" };
const _hoisted_6$3 = { class: "title" };
const _hoisted_7$2 = { class: "description" };
const _hoisted_8$1 = { class: "title" };
const _hoisted_9$1 = ["innerHTML"];
const _hoisted_10$1 = { class: "title" };
const _hoisted_11$1 = ["innerHTML"];
const _sfc_main$e = {
  __name: "ModelDialogVue",
  emits: ["close"],
  setup(__props, { emit }) {
    const imageEnlarger2 = require(`${__dirname}/modules/image-enlarger`);
    const utils2 = require(`${__dirname}/modules/utils`);
    onBeforeMount(async () => {
      const missingDependencies = await imageEnlarger2.getMissingDependencies();
      if (missingDependencies.length > 0) {
        model.totalSize = missingDependencies.reduce((acc, cur) => acc + cur.size, 0);
        model.visible = true;
      } else {
        emit("close");
      }
    });
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
            await downloadAndSetup();
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
        ok: () => {
          emit("close");
          model.visible = false;
        }
      },
      initialize: {
        cancel: () => {
          window.close();
        }
      },
      failed: {
        ok: () => {
          model.status = "prepare";
        }
      }
    });
    const downloadAndSetup = async () => {
      const missingDependencies = await imageEnlarger2.getMissingDependencies();
      return new Promise(async (resolve, reject) => {
        if (missingDependencies.length > 0) {
          try {
            await imageEnlarger2.download(
              function onProgress(downloaded, totalSize) {
                const percent = downloaded / totalSize * 100;
                model.process = percent;
              },
              function onComplete() {
                console.log("download completed");
                resolve();
              }
            );
          } catch (error) {
            reject(error);
          }
        }
        resolve();
      });
    };
    return (_ctx, _cache) => {
      const _component_ImageVue = _sfc_main$h;
      const _component_ButtonComet = __unplugin_components_13;
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
            createBaseVNode("div", _hoisted_1$b, toDisplayString(unref(t)("modelDialog.prepare.title")), 1),
            createBaseVNode("div", _hoisted_2$7, toDisplayString(unref(t)("modelDialog.prepare.description")), 1),
            createVNode(_component_el_button, {
              type: "primary",
              class: "ok ai-style",
              onClick: unref(model).prepare.ok
            }, {
              default: withCtx(() => [
                createVNode(_component_ButtonComet),
                createTextVNode(" " + toDisplayString(unref(t)("modelDialog.prepare.ok")) + " ", 1),
                createBaseVNode("span", _hoisted_3$6, "(" + toDisplayString(unref(utils2).number.format(unref(model).totalSize)) + ")", 1)
              ]),
              _: 1
            }, 8, ["onClick"]),
            createVNode(_component_el_button, {
              type: "",
              class: "cancel",
              onClick: unref(model).prepare.cancel
            }, {
              default: withCtx(() => [
                createTextVNode(toDisplayString(unref(t)("modelDialog.prepare.cancel")), 1)
              ]),
              _: 1
            }, 8, ["onClick"])
          ], 64)) : createCommentVNode("", true),
          unref(model).status === "downloading" ? (openBlock(), createElementBlock(Fragment, { key: 1 }, [
            createBaseVNode("div", _hoisted_4$4, toDisplayString(unref(t)("modelDialog.downloading.title")), 1),
            createBaseVNode("div", _hoisted_5$3, toDisplayString(unref(t)("modelDialog.downloading.description")), 1),
            createVNode(_component_el_button, {
              type: "",
              class: "cancel",
              onClick: unref(model).downloading.cancel
            }, {
              default: withCtx(() => [
                createTextVNode(toDisplayString(unref(t)("modelDialog.downloading.cancel")), 1)
              ]),
              _: 1
            }, 8, ["onClick"])
          ], 64)) : createCommentVNode("", true),
          unref(model).status === "success" ? (openBlock(), createElementBlock(Fragment, { key: 2 }, [
            createBaseVNode("div", _hoisted_6$3, toDisplayString(unref(t)("modelDialog.success.title")), 1),
            createBaseVNode("div", _hoisted_7$2, toDisplayString(unref(t)("modelDialog.success.description")), 1),
            createVNode(_component_el_button, {
              type: "primary",
              class: "ok ai-style",
              onClick: unref(model).success.ok
            }, {
              default: withCtx(() => [
                createVNode(_component_ButtonComet),
                createTextVNode(" " + toDisplayString(unref(t)("modelDialog.success.ok")), 1)
              ]),
              _: 1
            }, 8, ["onClick"])
          ], 64)) : createCommentVNode("", true),
          unref(model).status === "initialize" ? (openBlock(), createElementBlock(Fragment, { key: 3 }, [
            createBaseVNode("div", _hoisted_8$1, toDisplayString(unref(t)("modelDialog.initialize.title")), 1),
            createBaseVNode("div", {
              class: "description",
              innerHTML: unref(t)("modelDialog.initialize.description")
            }, null, 8, _hoisted_9$1),
            createVNode(_component_el_button, {
              type: "",
              class: "cancel",
              onClick: unref(model).initialize.cancel
            }, {
              default: withCtx(() => [
                createTextVNode(toDisplayString(unref(t)("modelDialog.initialize.cancel")), 1)
              ]),
              _: 1
            }, 8, ["onClick"])
          ], 64)) : createCommentVNode("", true),
          unref(model).status === "failed" ? (openBlock(), createElementBlock(Fragment, { key: 4 }, [
            createBaseVNode("div", _hoisted_10$1, toDisplayString(unref(t)("modelDialog.failed.title")), 1),
            createBaseVNode("div", {
              class: "description",
              innerHTML: unref(t)("modelDialog.failed.description")
            }, null, 8, _hoisted_11$1),
            createVNode(_component_el_button, {
              type: "primary",
              class: "ok ai-style",
              onClick: unref(model).failed.ok
            }, {
              default: withCtx(() => [
                createTextVNode(toDisplayString(unref(t)("modelDialog.failed.ok")), 1)
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
const BodyVue_vue_vue_type_style_index_0_lang = "";
const _sfc_main$d = {};
const _hoisted_1$a = { class: "body-vue" };
function _sfc_render$3(_ctx, _cache) {
  return openBlock(), createElementBlock("div", _hoisted_1$a, [
    renderSlot(_ctx.$slots, "default")
  ]);
}
const __unplugin_components_14 = /* @__PURE__ */ _export_sfc(_sfc_main$d, [["render", _sfc_render$3]]);
const DropZoneVue_vue_vue_type_style_index_0_lang = "";
const _hoisted_1$9 = ["onDragenter"];
const _hoisted_2$6 = ["onDragleave", "onDrop"];
const _hoisted_3$5 = { class: "tip" };
const _sfc_main$c = {
  __name: "DropZoneVue",
  props: {
    style: {
      type: Boolean,
      default: true
    }
  },
  emits: ["drop"],
  setup(__props, { emit }) {
    const props = __props;
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
      const _component_ImageVue = _sfc_main$h;
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
          createBaseVNode("div", _hoisted_3$5, [
            createVNode(_component_ImageVue, {
              width: "16",
              height: "16",
              src: "base/ic-drop-zone-download.svg"
            }),
            createTextVNode(" " + toDisplayString(unref(t)("component.dropZone.tip")), 1)
          ])
        ], 40, _hoisted_2$6)
      ], 42, _hoisted_1$9);
    };
  }
};
const ViewerVue_vue_vue_type_style_index_0_lang = "";
const _sfc_main$b = {
  __name: "ViewerVue",
  props: {
    // 圖片來源
    src: {
      type: String,
      required: true
    },
    width: {
      type: Number,
      required: true
    },
    height: {
      type: Number,
      required: true
    },
    // 裁切區域
    selection: {
      type: Object,
      required: true,
      default: () => {
        return {
          x: 0,
          y: 0,
          width: 0,
          height: 0
        };
      }
    }
  },
  setup(__props) {
    const props = __props;
    const viewerEl = ref(null);
    const size = computed(() => {
      var _a, _b;
      return {
        width: ((_a = viewerEl.value) == null ? void 0 : _a.clientWidth) || 0,
        height: ((_b = viewerEl.value) == null ? void 0 : _b.clientHeight) || 0
      };
    });
    const ratio = computed(() => {
      const width_ratio = size.value.width / props.selection.width;
      const height_ratio = size.value.height / props.selection.height;
      return Math.min(width_ratio, height_ratio);
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", {
        class: "viewer",
        ref_key: "viewerEl",
        ref: viewerEl
      }, [
        createBaseVNode("div", {
          class: "img",
          style: normalizeStyle({
            backgroundImage: `url('${props.src.replace(/\\/g, "/")}')`,
            backgroundPosition: `-${props.selection.x * unref(ratio)}px -${props.selection.y * unref(ratio)}px`,
            backgroundSize: `${props.width * unref(ratio)}px ${props.height * unref(ratio)}px`,
            width: `${props.selection.width * unref(ratio)}px`,
            height: `${props.selection.height * unref(ratio)}px`,
            backgroundRepeat: "no-repeat"
          })
        }, null, 4)
      ], 512);
    };
  }
};
const SlideBarVue_vue_vue_type_style_index_0_lang = "";
const _hoisted_1$8 = { class: "slide-bar-vue" };
const _hoisted_2$5 = { class: "range-wrap" };
const _hoisted_3$4 = { class: "range-progressbar" };
const _hoisted_4$3 = ["min", "max", "step"];
const _sfc_main$a = {
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
    }
  },
  emits: ["update:modelValue", "changed"],
  setup(__props, { expose: __expose, emit }) {
    const props = __props;
    require(`${__dirname}/modules/utils/time`);
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
      const _component_ImageVue = _sfc_main$h;
      return openBlock(), createElementBlock("div", _hoisted_1$8, [
        createVNode(_component_ImageVue, {
          onClick: minus,
          class: "icon",
          width: "23",
          height: "23",
          src: "light/base/ic-slide-bar-minus.svg",
          darkSrc: "dark/base/ic-slide-bar-minus.svg"
        }),
        createBaseVNode("div", _hoisted_2$5, [
          createBaseVNode("div", _hoisted_3$4, [
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
            }, null, 8, _hoisted_4$3), [
              [vModelText, unref(slide_bar_value)]
            ])
          ])
        ]),
        createVNode(_component_ImageVue, {
          onClick: plus,
          class: "icon",
          width: "23",
          height: "23",
          src: "light/base/ic-slide-bar-plus.svg",
          darkSrc: "dark/base/ic-slide-bar-plus.svg"
        })
      ]);
    };
  }
};
const PinchZoomVue_vue_vue_type_style_index_0_lang = "";
const _hoisted_1$7 = { class: "pinch-zoom-container" };
const boundary = 100;
const _sfc_main$9 = {
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
    const props = __props;
    const utils2 = require(`${__dirname}/modules/utils`);
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
      if (!pinchZoomEl.value)
        return;
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
      if (event2.metaKey || event2.ctrlKey || props.dragMove)
        getOffset();
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
      if (props.dragMove)
        document.querySelector(props.container).style.cursor = "grabbing";
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
      if (props.dragMove)
        document.querySelector(props.container).style.cursor = "grab";
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
      if (!ratio)
        return;
      offset.ratio = ratio;
      center();
      updateTransform();
    };
    const scaleOut = () => {
      const ratio = scaleStep.findLast((e) => e < offset.ratio);
      if (!ratio)
        return;
      offset.ratio = ratio;
      center();
      updateTransform();
    };
    const view = async (x, y, width, height) => {
      const left = -1 * offset.x <= x * offset.ratio;
      const right = (x + width) * offset.ratio <= -1 * offset.x + container.width;
      const top = -1 * offset.y <= y * offset.ratio;
      const bottom = (y + height) * offset.ratio <= -1 * offset.y + container.height;
      if (top && right && bottom && left)
        return;
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
      if (props.dragMove)
        containerEl.style.cursor = "grab";
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
      return withDirectives((openBlock(), createElementBlock("div", _hoisted_1$7, [
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
const ImageSplitVue_vue_vue_type_style_index_0_lang = "";
const _sfc_main$8 = {};
function _sfc_render$2(_ctx, _cache) {
  const _component_two_up = resolveComponent("two-up");
  return openBlock(), createBlock(_component_two_up, {
    class: "image-split-vue",
    "legacy-clip-compat": ""
  }, {
    default: withCtx(() => [
      renderSlot(_ctx.$slots, "one"),
      renderSlot(_ctx.$slots, "two")
    ]),
    _: 3
  });
}
const ImageSplitVue = /* @__PURE__ */ _export_sfc(_sfc_main$8, [["render", _sfc_render$2]]);
const ImageSyncVue_vue_vue_type_style_index_0_lang = "";
const _sfc_main$7 = {};
const _hoisted_1$6 = { class: "image-sync-vue" };
const _hoisted_2$4 = { class: "line" };
function _sfc_render$1(_ctx, _cache) {
  return openBlock(), createElementBlock("div", _hoisted_1$6, [
    createBaseVNode("div", null, [
      renderSlot(_ctx.$slots, "one")
    ]),
    createBaseVNode("div", _hoisted_2$4, [
      renderSlot(_ctx.$slots, "two")
    ])
  ]);
}
const ImageSyncVue = /* @__PURE__ */ _export_sfc(_sfc_main$7, [["render", _sfc_render$1]]);
const ImageOverlayVue_vue_vue_type_style_index_0_lang = "";
const _sfc_main$6 = {};
const _hoisted_1$5 = { class: "overlap-vue" };
function _sfc_render(_ctx, _cache) {
  return openBlock(), createElementBlock("div", _hoisted_1$5, [
    renderSlot(_ctx.$slots, "one"),
    renderSlot(_ctx.$slots, "two")
  ]);
}
const ImageOverlayVue = /* @__PURE__ */ _export_sfc(_sfc_main$6, [["render", _sfc_render]]);
const ImageCompareVue_vue_vue_type_style_index_0_lang = "";
const _hoisted_1$4 = { class: "image-compare-vue" };
const _hoisted_2$3 = { class: "toolbar" };
const _hoisted_3$3 = { class: "toolbar-group" };
const _hoisted_4$2 = { class: "name" };
const _hoisted_5$2 = { class: "name" };
const _hoisted_6$2 = { class: "toolbar-group" };
const _hoisted_7$1 = ["onClick"];
const _sfc_main$5 = {
  __name: "ImageCompareVue",
  props: {
    sync: {
      type: Boolean,
      default: true
    },
    split: {
      type: Boolean,
      default: true
    },
    overlap: {
      type: Boolean,
      default: true
    }
  },
  setup(__props) {
    const props = __props;
    const utils2 = require(`${__dirname}/modules/utils`);
    const mousetrap = inject("mousetrap");
    const typeIndex = ref(0);
    const ratio = ref(0);
    const pinchZoomOneEl = ref(null);
    const pinchZoomTwoEl = ref(null);
    onMounted(async () => {
      mousetrap.bind(["command+0", "ctrl+0"], () => {
        pinchZoomOneEl.value.scale(1);
        pinchZoomTwoEl.value.scale(1);
        return false;
      });
      mousetrap.bind(["command+9", "ctrl+9"], () => {
        pinchZoomOneEl.value.fit();
        pinchZoomTwoEl.value.fit();
        return false;
      });
      mousetrap.bind(["+", "=", "command++", "command+=", "ctrl++", "ctrl+="], () => {
        pinchZoomOneEl.value.scaleIn();
        pinchZoomTwoEl.value.scaleIn();
        return false;
      });
      mousetrap.bind(["-", "command+-", "ctrl+-"], () => {
        pinchZoomOneEl.value.scaleOut();
        pinchZoomTwoEl.value.scaleOut();
        return false;
      });
    });
    watchEffect(() => {
      var _a;
      ratio.value = (_a = pinchZoomOneEl.value) == null ? void 0 : _a.offset.ratio;
    });
    const syncPinchZoomOne = async () => {
      await utils2.time.sleep(1);
      const offset = pinchZoomTwoEl.value.offset;
      pinchZoomOneEl.value.setOffset(offset.x, offset.y, offset.ratio);
    };
    const syncPinchZoomTwo = async () => {
      await utils2.time.sleep(1);
      const offset = pinchZoomOneEl.value.offset;
      pinchZoomTwoEl.value.setOffset(offset.x, offset.y, offset.ratio);
    };
    return (_ctx, _cache) => {
      var _a;
      const _component_PinchZoomVue = _sfc_main$9;
      const _component_SlideBarVue = _sfc_main$a;
      const _component_el_dropdown_item = ElDropdownItem;
      const _component_key = resolveComponent("key");
      const _component_keys = resolveComponent("keys");
      const _component_el_dropdown_menu = ElDropdownMenu;
      const _component_el_dropdown = ElDropdown;
      const _component_ImageVue = _sfc_main$h;
      const _directive_tippy = resolveDirective("tippy");
      return openBlock(), createElementBlock(Fragment, null, [
        createBaseVNode("div", _hoisted_1$4, [
          (openBlock(), createBlock(resolveDynamicComponent([ImageSyncVue, ImageSplitVue, ImageOverlayVue][unref(typeIndex)]), null, {
            one: withCtx(() => [
              createVNode(_component_PinchZoomVue, {
                fit: "",
                class: "pinch-zoom-one",
                container: ".image-compare-vue",
                ref_key: "pinchZoomOneEl",
                ref: pinchZoomOneEl,
                dragMove: true,
                onMousemove: withModifiers(syncPinchZoomTwo, ["prevent"]),
                onWheel: withModifiers(syncPinchZoomTwo, ["prevent"])
              }, {
                default: withCtx(() => [
                  renderSlot(_ctx.$slots, "one")
                ]),
                _: 3
              }, 8, ["onMousemove", "onWheel"])
            ]),
            two: withCtx(() => [
              createVNode(_component_PinchZoomVue, {
                fit: "",
                class: "pinch-zoom-two",
                container: ".image-compare-vue",
                ref_key: "pinchZoomTwoEl",
                ref: pinchZoomTwoEl,
                dragMove: true,
                onMousemove: withModifiers(syncPinchZoomOne, ["prevent"]),
                onWheel: withModifiers(syncPinchZoomOne, ["prevent"])
              }, {
                default: withCtx(() => [
                  renderSlot(_ctx.$slots, "two")
                ]),
                _: 3
              }, 8, ["onMousemove", "onWheel"])
            ]),
            _: 3
          }))
        ]),
        createBaseVNode("div", _hoisted_2$3, [
          renderSlot(_ctx.$slots, "prefix"),
          createBaseVNode("div", _hoisted_3$3, [
            createVNode(_component_SlideBarVue, {
              modelValue: unref(ratio),
              "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => isRef(ratio) ? ratio.value = $event : null),
              class: "toolbar-group-item no-hover",
              data: ((_a = unref(pinchZoomOneEl)) == null ? void 0 : _a.scaleStep) ?? [],
              onChanged: _cache[1] || (_cache[1] = (value) => {
                var _a2, _b;
                (_a2 = unref(pinchZoomOneEl)) == null ? void 0 : _a2.scale(value);
                (_b = unref(pinchZoomTwoEl)) == null ? void 0 : _b.scale(value);
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
                      (openBlock(true), createElementBlock(Fragment, null, renderList(((_a2 = unref(pinchZoomOneEl)) == null ? void 0 : _a2.scaleStep) ?? [], (i) => {
                        var _a3;
                        return openBlock(), createBlock(_component_el_dropdown_item, {
                          class: normalizeClass(["keyboard", {
                            active: ((_a3 = unref(pinchZoomOneEl)) == null ? void 0 : _a3.offset.ratio) === i
                          }]),
                          key: i,
                          onClick: () => {
                            var _a4, _b;
                            (_a4 = unref(pinchZoomOneEl)) == null ? void 0 : _a4.scale(i);
                            (_b = unref(pinchZoomTwoEl)) == null ? void 0 : _b.scale(i);
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
                          var _a3, _b;
                          (_a3 = unref(pinchZoomOneEl)) == null ? void 0 : _a3.scale(1);
                          (_b = unref(pinchZoomTwoEl)) == null ? void 0 : _b.scale(1);
                        }),
                        divided: ""
                      }, {
                        default: withCtx(() => [
                          createBaseVNode("div", _hoisted_4$2, toDisplayString(unref(t)("component.compare.toolbar.scale.origin")), 1),
                          createVNode(_component_keys, null, {
                            default: withCtx(() => [
                              (openBlock(true), createElementBlock(Fragment, null, renderList([unref(keyboard)("⌘"), "0"], (key) => {
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
                          var _a3, _b;
                          (_a3 = unref(pinchZoomOneEl)) == null ? void 0 : _a3.fit();
                          (_b = unref(pinchZoomTwoEl)) == null ? void 0 : _b.fit();
                        })
                      }, {
                        default: withCtx(() => [
                          createBaseVNode("div", _hoisted_5$2, toDisplayString(unref(t)("component.compare.toolbar.scale.fit")), 1),
                          createVNode(_component_keys, null, {
                            default: withCtx(() => [
                              (openBlock(true), createElementBlock(Fragment, null, renderList([unref(keyboard)("⌘"), "9"], (key) => {
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
                  createTextVNode(toDisplayString(Math.round(((_a2 = unref(pinchZoomOneEl)) == null ? void 0 : _a2.offset.ratio) * 100)) + "% ", 1)
                ];
              }),
              _: 1
            })
          ]),
          renderSlot(_ctx.$slots, "default"),
          createBaseVNode("div", _hoisted_6$2, [
            (openBlock(), createElementBlock(Fragment, null, renderList(["sync", "split", "overlap"], (type, index) => {
              return openBlock(), createElementBlock(Fragment, { key: type }, [
                props[type] ? withDirectives((openBlock(), createElementBlock("div", {
                  key: 0,
                  class: normalizeClass(["toolbar-group-item", { active: unref(typeIndex) === index }]),
                  onClick: ($event) => typeIndex.value = index
                }, [
                  createVNode(_component_ImageVue, {
                    width: "15",
                    height: "16",
                    src: `light/base/ic-image-compare-${type}.svg`,
                    darkSrc: `dark/base/ic-image-compare-${type}.svg`
                  }, null, 8, ["src", "darkSrc"])
                ], 10, _hoisted_7$1)), [
                  [_directive_tippy, {
                    content: unref(t)(`component.compare.${type}.tip`),
                    delay: [0, 0],
                    duration: [200, 0]
                  }]
                ]) : createCommentVNode("", true)
              ], 64);
            }), 64))
          ]),
          renderSlot(_ctx.$slots, "postfix")
        ])
      ], 64);
    };
  }
};
const StatusVue_vue_vue_type_style_index_0_lang = "";
const _sfc_main$4 = {
  __name: "StatusVue",
  props: {
    title: {
      type: String,
      default: " "
    },
    description: {
      type: String,
      default: " "
    },
    type: {
      type: String,
      default: "empty"
    }
  },
  setup(__props) {
    const props = __props;
    return (_ctx, _cache) => {
      const _component_ImageVue = _sfc_main$h;
      const _component_el_empty = ElEmpty;
      return openBlock(), createBlock(_component_el_empty, {
        description: props.title,
        "image-size": 256
      }, {
        image: withCtx(() => [
          createVNode(_component_ImageVue, {
            class: "status-img",
            width: "256",
            height: "144",
            src: "light/status.png",
            darkSrc: "dark/status.png"
          }, {
            default: withCtx(() => [
              createBaseVNode("div", {
                class: normalizeClass(["status", ["status-" + props.type]])
              }, null, 2)
            ]),
            _: 1
          })
        ]),
        default: withCtx(() => [
          createTextVNode(" " + toDisplayString(props.description), 1)
        ]),
        _: 1
      }, 8, ["description"]);
    };
  }
};
const _hoisted_1$3 = { style: { "margin-right": "2px" } };
const _sfc_main$3 = {
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
      const _component_ImageVue = _sfc_main$h;
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
          createBaseVNode("span", _hoisted_1$3, toDisplayString(unref(isAlwaysOnTop) ? unref(t)("header.thumbtack.isNotAlwaysOnTop") : unref(t)("header.thumbtack.isAlwaysOnTop")), 1),
          createVNode(_component_key, null, {
            default: withCtx(() => [
              createTextVNode(toDisplayString(unref(keyboard)("shift")), 1)
            ]),
            _: 1
          }),
          createVNode(_component_key, null, {
            default: withCtx(() => [
              createTextVNode("T")
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
const HeaderVue_vue_vue_type_style_index_0_lang = "";
const _hoisted_1$2 = { class: "header-vue" };
const _hoisted_2$2 = { class: "drag" };
const _hoisted_3$2 = /* @__PURE__ */ createBaseVNode("img", {
  class: "logo",
  src: _imports_0,
  alt: "logo"
}, null, -1);
const _hoisted_4$1 = { class: "title" };
const _hoisted_5$1 = { class: "action" };
const _hoisted_6$1 = /* @__PURE__ */ createBaseVNode("div", { class: "dash" }, null, -1);
const _sfc_main$2 = {
  __name: "HeaderVue",
  setup(__props) {
    const title = eagle.plugin.manifest.name;
    const closeDialog = reactive({
      visible: false,
      type: "warning",
      ok: () => {
        window.close();
      }
    });
    return (_ctx, _cache) => {
      const _component_ThumbtackVue = _sfc_main$3;
      const _component_ImageVue = _sfc_main$h;
      const _component_DialogVue = _sfc_main$g;
      return openBlock(), createElementBlock(Fragment, null, [
        createBaseVNode("div", _hoisted_1$2, [
          createBaseVNode("div", _hoisted_2$2, [
            _hoisted_3$2,
            createBaseVNode("span", _hoisted_4$1, toDisplayString(unref(title)), 1)
          ]),
          createBaseVNode("div", _hoisted_5$1, [
            renderSlot(_ctx.$slots, "default"),
            _hoisted_6$1,
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
            createTextVNode(toDisplayString(unref(t)("header.dialog.exit.title")), 1)
          ]),
          description: withCtx(() => [
            createTextVNode(toDisplayString(unref(t)("header.dialog.exit.description")), 1)
          ]),
          cancel: withCtx(() => [
            createTextVNode(toDisplayString(unref(t)("header.dialog.exit.cancel")), 1)
          ]),
          ok: withCtx(() => [
            createTextVNode(toDisplayString(unref(t)("header.dialog.exit.ok")), 1)
          ]),
          _: 1
        }, 8, ["modelValue", "type", "onOk"])
      ], 64);
    };
  }
};
const ToggleSwitchSliderVue_vue_vue_type_style_index_0_lang = "";
const _hoisted_1$1 = { class: "toggle-switch-slider-vue" };
const _hoisted_2$1 = ["for"];
const _hoisted_3$1 = ["id", "name", "value"];
const _sfc_main$1 = {
  __name: "ToggleSwitchSliderVue",
  props: {
    modelValue: {
      type: Number,
      required: true
    },
    data: {
      type: Array,
      required: true
    }
  },
  emits: ["update:modelValue"],
  setup(__props, { emit }) {
    const props = __props;
    const value = computed({
      get() {
        return props.modelValue;
      },
      set(value2) {
        emit("update:modelValue", value2);
      }
    });
    const name = crypto.randomUUID();
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1$1, [
        (openBlock(true), createElementBlock(Fragment, null, renderList(props.data, (data) => {
          return openBlock(), createElementBlock("label", {
            key: data,
            for: `toggle-switch-slider-${data}`
          }, [
            withDirectives(createBaseVNode("input", {
              "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => isRef(value) ? value.value = $event : null),
              type: "radio",
              id: `toggle-switch-slider-${data}`,
              name: unref(name),
              value: data,
              hidden: ""
            }, null, 8, _hoisted_3$1), [
              [vModelRadio, unref(value)]
            ]),
            renderSlot(_ctx.$slots, "default", { data }, () => [
              createTextVNode(toDisplayString(data), 1)
            ])
          ], 8, _hoisted_2$1);
        }), 128))
      ]);
    };
  }
};
class Task {
  constructor(args) {
    this.id = args.id ?? crypto.randomUUID();
    this.name = args.name;
    this.ext = args.ext;
    this.url = args.url;
    this.path = args.path;
    this.thumbnailPath = args.thumbnailPath;
    this.thumbnailURL = args.thumbnailURL;
    this.width = args.width;
    this.height = args.height;
    this.result = args.result ?? {
      state: "waiting",
      message: "",
      data: null
    };
  }
  isWaiting() {
    return this.result.state === "waiting";
  }
  isProcessing() {
    return this.result.state === "processing";
  }
  isSuccessful() {
    return this.result.state === "success" || this.result.state === "replaced";
  }
  isFailed() {
    return this.result.state === "failed";
  }
  isReplaced() {
    return this.result.state === "replaced";
  }
  waiting() {
    this.result.state = "waiting";
    this.result.message = "";
    this.result.data = null;
  }
  processing(process2 = 0) {
    this.result.state = "processing";
    this.result.message = "";
    this.result.data = process2;
  }
  success(data = null) {
    this.result.state = "success";
    this.result.message = "";
    this.result.data = data;
  }
  replaced() {
    this.result.state = "replaced";
    this.result.message = "";
  }
  failed(message = "") {
    this.result.state = "failed";
    this.result.message = message;
    this.result.data = null;
  }
  async execute(func = async () => {
  }) {
    this.processing();
    try {
      this.success(await func());
    } catch (err) {
      this.failed(err);
    }
  }
}
class PluginError extends Error {
  constructor(message) {
    super(message);
    this.name = "PluginError";
    const hasI18n = i18next.exists(`error.${message}`);
    if (hasI18n) {
      this.message = i18next.t(`error.${message}`);
    } else {
      this.message = message;
    }
  }
}
class Queue {
  constructor(params) {
    this.data = reactive([]);
    this.dataMap = {};
    this.queue = null;
    this.concurrency = 3;
    this.state = reactive({ isWorking: false });
    this.initQueue(params);
  }
  get length() {
    return this.data.length;
  }
  get waiting() {
    return this.data.filter((task) => task.result.state === "waiting");
  }
  get completed() {
    return [...this.success, ...this.failed];
  }
  get success() {
    return this.data.filter((task) => task.result.state === "success");
  }
  get failed() {
    return this.data.filter((task) => task.result.state === "failed");
  }
  enqueue(data, index = Infinity) {
    const ar = [data].flat(Infinity);
    for (let i = 0; i < ar.length; i++) {
      const task = reactive(new Task(ar[i]));
      if (this.dataMap[task.id])
        continue;
      this.dataMap[task.id] = task;
      this.data.splice(index + i, 0, task);
      if (this.queue) {
        this.queue.push(task);
        this.state.isWorking = true;
      }
    }
  }
  dequeue(index = 0) {
    const task = this.data.splice(index, 1)[0];
    delete this.dataMap[task.id];
    if (this.queue) {
      this.queue.remove(({ data }) => {
        return data.id === task.id;
      });
    }
    return task;
  }
  dequeueById(id) {
    const task = this.dataMap[id];
    if (!task)
      return;
    const index = this.data.indexOf(task);
    return this.dequeue(index);
  }
  initQueue({ onProcess }) {
    this.queue = queue(async (task, callback) => {
      this.state.isWorking = true;
      try {
        task.processing();
        const result = await onProcess(task);
        await task.success(result);
      } catch (err) {
        console.error(err, task);
        const pluginError = new PluginError((err == null ? void 0 : err.message) ?? err);
        await task.failed(pluginError.message);
      } finally {
        callback();
      }
    }, this.concurrency);
    this.queue.drain(() => {
      console.log("All tasks have been processed.");
      this.state.isWorking = false;
    });
  }
  clear(type = "data") {
    console.time("remove data : " + type);
    const set = new Set(this[type]);
    this.data = this.data.filter((task) => !set.has(task));
    this[type] = [];
    console.timeEnd("remove data : " + type);
  }
  async reset() {
  }
  resetCompletedTasksToWaiting() {
    for (let task of this.completed) {
      task.waiting();
      this.queue.push(task);
    }
  }
}
const utils = require(`${__dirname}/modules/utils`);
const imageEnlarger = require(`${__dirname}/modules/image-enlarger`);
class Main {
  constructor() {
    this.ALGORITHM_SETTING_KEY = `eagle.plugin.${eagle.plugin.manifest.id}.algorithm`;
    this.replaceState = reactive({
      current: 0,
      total: 0
    });
    this.isLoading = true;
    this.currentId = null;
    this.taskQueue = reactive(
      new Queue({
        onProcess: this.convert.bind(this)
      })
    );
  }
  async convert(task, algorithm = "") {
    try {
      eagle.log.info(`start enlarging #${task.id}: ${task.name}.${task.ext}`);
      if (!["jpg", "jpeg", "png", "webp", "avif"].includes(task.ext.toLowerCase()))
        throw "fileExtensionNotSupported";
      await utils.image.preVerify(task.path, {
        checkSupportFileSize: true,
        checkSupportResolution: true
      });
      if (algorithm === "") {
        algorithm = localStorage.getItem(this.ALGORITHM_SETTING_KEY) ?? "realesrgan-x4plus-anime";
      }
      task.algorithm = algorithm;
      const tempOutputPath = `${__dirname}/temp/${crypto.randomUUID()}.${task.ext}`;
      await imageEnlarger.ensureDirectoryExistence(tempOutputPath);
      const outputPath = await imageEnlarger.imageEnlarge(
        algorithm,
        task.path,
        tempOutputPath,
        (progress) => {
          task.result.data = progress;
        }
      );
      await utils.image.postVerify(task.path, outputPath, {
        checkEqualResolution: false,
        checkHistograms: false,
        checkFileSize: true
      });
      task.convertedUrl = outputPath;
      return outputPath;
    } catch (error) {
      eagle.log.error(`#${task.id} enlarging error : ${error}`);
      if (error.includes("invalidGpuDevice"))
        throw "invalidGpuDevice";
      throw error;
    } finally {
      eagle.log.info(`end enlarging #${task.id}`);
    }
  }
  async saveAs(id = null) {
    try {
      eagle.log.info("start saveAs");
      const dialog = await eagle.dialog.showOpenDialog({
        properties: ["openDirectory"]
      });
      if (dialog.canceled)
        return;
      const folder = dialog.filePaths[0];
      if (id) {
        const task = this.taskQueue.dataMap[id];
        if (!task.isSuccessful())
          return;
        const file_path = `${folder}/${task.name}.${task.ext}`;
        await utils.file.saveAs(task.convertedUrl, file_path);
        await eagle.shell.showItemInFolder(file_path);
        return 1;
      } else {
        const tasks = this.taskQueue.success;
        for (let task2 of tasks) {
          const file_path = `${folder}/${task2.name}.${task2.ext}`;
          await utils.file.saveAs(task2.convertedUrl, file_path);
        }
        const task = this.taskQueue.data[0];
        await eagle.shell.showItemInFolder(`${folder}/${task.name}.${task.ext}`);
        return tasks.length;
      }
    } catch (error) {
      eagle.log.error(`saveAs error : ${error}`);
    } finally {
      eagle.log.info("end saveAs");
    }
  }
  async replace(id = null) {
    try {
      eagle.log.info("start replace");
      if (id) {
        const task = this.taskQueue.dataMap[id];
        if (!task.isSuccessful())
          return;
        await (await eagle.item.getById(task.id)).replaceFile(task.convertedUrl);
        task.replaced();
        return 1;
      }
    } catch (error) {
      eagle.log.error(`replace error : ${error}`);
    } finally {
      eagle.log.info("end replace");
    }
  }
  async replaceAll() {
    try {
      eagle.log.info("start replace");
      const tasks = this.taskQueue.success;
      const noReplacedTasks = tasks.filter((task) => !task.isReplaced());
      if (noReplacedTasks.length === 0)
        return 0;
      let success = 0;
      this.replaceState.total = tasks.length;
      for (let task of noReplacedTasks) {
        this.replaceState.current++;
        try {
          await (await eagle.item.getById(task.id)).replaceFile(task.convertedUrl);
          task.replaced();
          success++;
        } catch (err) {
          task.failed(err);
          console.error(err);
        }
      }
      this.replaceState.current = 0;
      this.replaceState.total = 0;
      return success;
    } catch (error) {
      eagle.log.error(`replace error : ${error}`);
    } finally {
      eagle.log.info("end replace");
    }
  }
}
const App_vue_vue_type_style_index_0_lang = "";
const App_vue_vue_type_style_index_1_lang = "";
const _hoisted_1 = { class: "title" };
const _hoisted_2 = { class: "content" };
const _hoisted_3 = /* @__PURE__ */ createBaseVNode("div", { class: "dash" }, null, -1);
const _hoisted_4 = { class: "action" };
const _hoisted_5 = {
  key: 0,
  class: "layout"
};
const _hoisted_6 = { class: "main" };
const _hoisted_7 = ["src", "alt"];
const _hoisted_8 = ["src", "alt"];
const _hoisted_9 = { class: "toolbar-group" };
const _hoisted_10 = { style: { "font-size": "14px" } };
const _hoisted_11 = { class: "sidebar" };
const _hoisted_12 = ["onClick"];
const _hoisted_13 = { class: "info" };
const _hoisted_14 = { class: "name" };
const _hoisted_15 = { class: "size" };
const _hoisted_16 = {
  key: 0,
  class: "status"
};
const _hoisted_17 = /* @__PURE__ */ createBaseVNode("svg", {
  width: "0",
  height: "0"
}, [
  /* @__PURE__ */ createBaseVNode("defs", null, [
    /* @__PURE__ */ createBaseVNode("linearGradient", {
      id: "mmmm",
      x1: "0%",
      y1: "0%",
      x2: "100%",
      y2: "0%"
    }, [
      /* @__PURE__ */ createBaseVNode("stop", {
        offset: "0%",
        style: { "stop-color": "var(--color-ai-gradient-1)" }
      }),
      /* @__PURE__ */ createBaseVNode("stop", {
        offset: "100%",
        style: { "stop-color": "var(--color-ai-gradient-2)" }
      })
    ])
  ])
], -1);
const _hoisted_18 = ["onClick"];
const _hoisted_19 = { class: "feature" };
const _hoisted_20 = { class: "progress ai-style" };
const _hoisted_21 = { class: "progress ai-style" };
const _hoisted_22 = ["innerHTML"];
const _hoisted_23 = ["innerHTML"];
const _hoisted_24 = ["innerHTML"];
const _hoisted_25 = ["innerHTML"];
const _sfc_main = {
  __name: "App",
  setup(__props) {
    const mousetrap = inject("mousetrap");
    const utils2 = require(`${__dirname}/modules/utils`);
    const main2 = reactive(new Main());
    const settingDropdownEl = ref(null);
    const downloadTipVisible = ref(true);
    const taskDialogVisible = ref(false);
    const taskDialogItemCount = ref(0);
    const iGPUWarningVisible = ref(false);
    const restartTasksVisible = ref(false);
    let currentAlgorithm = localStorage.getItem(main2.ALGORITHM_SETTING_KEY) ?? "realesrgan-x4plus-anime";
    const setting = shallowReactive({
      algorithm: currentAlgorithm,
      data: {
        "realesrgan-x4plus": t("header.setting.photo"),
        "realesrgan-x4plus-anime": t("header.setting.illustration")
      },
      ok: async () => {
        settingDropdownEl.value.handleClose();
        const isAlgorithmChanged = setting.algorithm !== currentAlgorithm;
        localStorage.setItem(main2.ALGORITHM_SETTING_KEY, setting.algorithm);
        currentAlgorithm = setting.algorithm;
        if (isAlgorithmChanged) {
          if (main2.taskQueue.completed.length > 0) {
            restartTasksVisible.value = true;
          }
        }
      },
      cancel: () => {
        settingDropdownEl.value.handleClose();
      }
    });
    const loadData = async () => {
      var _a;
      const items = await eagle.item.getSelected();
      const tasks = items.map(
        (item) => reactive({
          id: item.id,
          name: item.name,
          ext: item.ext,
          url: item.fileURL,
          path: item.filePath,
          thumbnailPath: item.thumbnailPath,
          thumbnailURL: item.thumbnailURL,
          width: item.width,
          height: item.height
        })
      );
      main2.taskQueue.enqueue(tasks);
      if (!current.value)
        current.value = (_a = main2.taskQueue.data[0]) == null ? void 0 : _a.id;
    };
    onMounted(() => {
      mousetrap.bind("up", () => {
        const index = main2.taskQueue.data.indexOf(current.value);
        if (index > 0)
          current.value = main2.taskQueue.data[index - 1].id;
      });
      mousetrap.bind("down", () => {
        const index = main2.taskQueue.data.indexOf(current.value);
        if (index < main2.taskQueue.length - 1)
          current.value = main2.taskQueue.data[index + 1].id;
      });
    });
    eagle.onPluginRun(async () => {
      await loadData();
    });
    const onFileDrop = async (files) => {
      var _a;
      const tasks = [];
      for (const file of files) {
        try {
          const id = new RegExp("(?<=\\images[\\\\\\/])(.*?)(?=\\.info)", "g").exec(file.path)[0] ?? null;
          if (!id)
            throw "file not come from Eagle";
          const item = await eagle.item.getById(id);
          tasks.push({
            id,
            name: item.name,
            ext: item.ext,
            url: item.fileURL,
            path: item.filePath,
            thumbnailPath: item.thumbnailPath,
            thumbnailURL: item.thumbnailURL,
            width: item.width,
            height: item.height
          });
        } catch (error) {
          eagle.log.error(error);
        }
      }
      main2.taskQueue.enqueue(tasks);
      if (!current.value)
        current.value = (_a = main2.taskQueue.data[0]) == null ? void 0 : _a.id;
    };
    const onModelDialogClose = async () => {
      var _a;
      main2.isLoading = true;
      await loadData();
      main2.isLoading = false;
      iGPUWarningVisible.value = ((_a = utils2 == null ? void 0 : utils2.os) == null ? void 0 : _a.isIntegratedGPU()) ?? false;
    };
    const current = computed({
      get: () => {
        return main2.taskQueue.dataMap[main2.currentId];
      },
      set: async (id) => {
        main2.currentId = id;
        await nextTick();
        const currentEl = document.querySelector(".sidebar .list .item.active");
        currentEl == null ? void 0 : currentEl.focus();
      }
    });
    const removeTask = (task) => {
      var _a;
      if (current.value.id === task.id) {
        let index = main2.taskQueue.data.indexOf(task);
        if (index - 1 > -1) {
          index--;
        } else if (index + 1 < main2.taskQueue.length) {
          index++;
        } else {
          index = -1;
        }
        main2.currentId = (_a = main2.taskQueue.data[index]) == null ? void 0 : _a.id;
      }
      main2.taskQueue.dequeueById(task.id);
    };
    const restartTasks = async () => {
      main2.taskQueue.resetCompletedTasksToWaiting();
    };
    provide("main", main2);
    return (_ctx, _cache) => {
      const _component_ImageVue = _sfc_main$h;
      const _component_ToggleSwitchSliderVue = _sfc_main$1;
      const _component_el_button = ElButton;
      const _component_el_dropdown = ElDropdown;
      const _component_HeaderVue = _sfc_main$2;
      const _component_StatusVue = _sfc_main$4;
      const _component_el_dropdown_item = ElDropdownItem;
      const _component_el_dropdown_menu = ElDropdownMenu;
      const _component_el_popover = ElPopover;
      const _component_ImageCompareVue = _sfc_main$5;
      const _component_ViewerVue = _sfc_main$b;
      const _component_el_progress = ElProgress;
      const _component_DropZoneVue = _sfc_main$c;
      const _component_ButtonComet = __unplugin_components_13;
      const _component_BodyVue = __unplugin_components_14;
      const _component_ModelDialogVue = _sfc_main$e;
      const _component_DialogVue = _sfc_main$g;
      const _directive_tippy = resolveDirective("tippy");
      return openBlock(), createElementBlock(Fragment, null, [
        createVNode(_component_HeaderVue, null, {
          default: withCtx(() => [
            withDirectives((openBlock(), createBlock(_component_el_dropdown, {
              ref_key: "settingDropdownEl",
              ref: settingDropdownEl,
              trigger: "click",
              "popper-class": "setting-menu",
              class: "no-tick",
              placement: "bottom-end"
            }, {
              dropdown: withCtx(() => [
                createBaseVNode("div", _hoisted_1, toDisplayString(unref(t)("header.setting.algorithm")), 1),
                createBaseVNode("div", _hoisted_2, [
                  createVNode(_component_ToggleSwitchSliderVue, {
                    modelValue: unref(setting).algorithm,
                    "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => unref(setting).algorithm = $event),
                    data: Object.keys(unref(setting).data)
                  }, {
                    default: withCtx(({ data }) => [
                      createTextVNode(toDisplayString(unref(setting).data[data]), 1)
                    ]),
                    _: 1
                  }, 8, ["modelValue", "data"])
                ]),
                _hoisted_3,
                createBaseVNode("div", _hoisted_4, [
                  createVNode(_component_el_button, {
                    type: "primary",
                    class: "ok ai-style",
                    onClick: unref(setting).ok
                  }, {
                    default: withCtx(() => [
                      createTextVNode(toDisplayString(unref(t)("header.setting.apply")), 1)
                    ]),
                    _: 1
                  }, 8, ["onClick"]),
                  createVNode(_component_el_button, {
                    type: "",
                    class: "cancel",
                    onClick: unref(setting).cancel
                  }, {
                    default: withCtx(() => [
                      createTextVNode(toDisplayString(unref(t)("header.setting.cancel")), 1)
                    ]),
                    _: 1
                  }, 8, ["onClick"])
                ])
              ]),
              default: withCtx(() => [
                createVNode(_component_ImageVue, {
                  class: "icon",
                  width: "24",
                  height: "24",
                  src: "light/ic-setting.svg",
                  darkSrc: "dark/ic-setting.svg"
                })
              ]),
              _: 1
            })), [
              [_directive_tippy, {
                content: unref(t)("header.setting.tip"),
                placement: "bottom",
                delay: [0, 0],
                duration: [200, 0]
              }]
            ])
          ]),
          _: 1
        }),
        !unref(main2).isLoading ? (openBlock(), createBlock(_component_BodyVue, { key: 0 }, {
          default: withCtx(() => [
            unref(main2).taskQueue.data.length ? (openBlock(), createElementBlock("div", _hoisted_5, [
              createBaseVNode("div", _hoisted_6, [
                unref(current) ? (openBlock(), createElementBlock(Fragment, { key: 0 }, [
                  unref(current).isWaiting() ? (openBlock(), createBlock(_component_StatusVue, {
                    key: 0,
                    type: "processing",
                    description: unref(t)("main.status.waiting")
                  }, null, 8, ["description"])) : createCommentVNode("", true),
                  unref(current).isProcessing() ? (openBlock(), createBlock(_component_StatusVue, {
                    key: 1,
                    type: "processing",
                    description: `${unref(t)("main.status.processing")} (${unref(current).result.data}%)`
                  }, null, 8, ["description"])) : createCommentVNode("", true),
                  unref(current).isSuccessful() ? (openBlock(), createBlock(_component_ImageCompareVue, {
                    key: 2,
                    overlap: false
                  }, {
                    one: withCtx(() => [
                      createBaseVNode("img", {
                        src: unref(current).url,
                        alt: unref(current).url,
                        style: { "width": "100%" }
                      }, null, 8, _hoisted_7)
                    ]),
                    two: withCtx(() => [
                      createBaseVNode("img", {
                        src: unref(current).convertedUrl,
                        alt: unref(current).convertedUrl,
                        style: { "width": "100%" }
                      }, null, 8, _hoisted_8)
                    ]),
                    postfix: withCtx(() => [
                      createVNode(_component_el_popover, {
                        placement: "top",
                        visible: unref(downloadTipVisible)
                      }, {
                        reference: withCtx(() => [
                          createVNode(_component_el_dropdown, {
                            class: "no-tick",
                            trigger: "click"
                          }, {
                            dropdown: withCtx(() => [
                              createVNode(_component_el_dropdown_menu, { class: "no-tick" }, {
                                default: withCtx(() => [
                                  createVNode(_component_el_dropdown_item, {
                                    onClick: _cache[2] || (_cache[2] = async () => {
                                      const count = await unref(main2).replace(
                                        unref(current).id
                                      );
                                      if (count > 0) {
                                        taskDialogVisible.value = true;
                                        taskDialogItemCount.value = count;
                                      }
                                    })
                                  }, {
                                    default: withCtx(() => [
                                      createTextVNode(toDisplayString(unref(t)("main.download.replace")), 1)
                                    ]),
                                    _: 1
                                  }),
                                  createVNode(_component_el_dropdown_item, {
                                    onClick: _cache[3] || (_cache[3] = async () => {
                                      const count = await unref(main2).saveAs(
                                        unref(current).id
                                      );
                                      if (count > 0) {
                                        taskDialogVisible.value = true;
                                        taskDialogItemCount.value = count;
                                      }
                                    })
                                  }, {
                                    default: withCtx(() => [
                                      createTextVNode(toDisplayString(unref(t)("main.download.saveAs")), 1)
                                    ]),
                                    _: 1
                                  })
                                ]),
                                _: 1
                              })
                            ]),
                            default: withCtx(() => [
                              createBaseVNode("div", {
                                class: "toolbar-group-item",
                                onMouseenter: _cache[1] || (_cache[1] = ($event) => downloadTipVisible.value = false)
                              }, [
                                createVNode(_component_ImageVue, {
                                  style: { "width": "100%", "height": "100%" },
                                  width: "17",
                                  height: "16",
                                  src: "light/ic-download.svg",
                                  darkSrc: "dark/ic-download.svg"
                                })
                              ], 32)
                            ]),
                            _: 1
                          })
                        ]),
                        default: withCtx(() => [
                          createBaseVNode("span", _hoisted_10, toDisplayString(unref(t)("main.download.tip")), 1)
                        ]),
                        _: 1
                      }, 8, ["visible"])
                    ]),
                    default: withCtx(() => [
                      createBaseVNode("div", _hoisted_9, [
                        createVNode(_component_el_dropdown, {
                          class: "toolbar-group-item",
                          trigger: "click"
                        }, {
                          dropdown: withCtx(() => [
                            createVNode(_component_el_dropdown_menu, null, {
                              default: withCtx(() => [
                                (openBlock(true), createElementBlock(Fragment, null, renderList(unref(setting).data, (value, key) => {
                                  return openBlock(), createBlock(_component_el_dropdown_item, {
                                    key,
                                    class: normalizeClass({
                                      active: unref(current).algorithm === key
                                    }),
                                    onClick: () => {
                                      unref(current).execute(
                                        async () => await unref(main2).convert(unref(current), key)
                                      );
                                    }
                                  }, {
                                    default: withCtx(() => [
                                      createTextVNode(toDisplayString(value), 1)
                                    ]),
                                    _: 2
                                  }, 1032, ["class", "onClick"]);
                                }), 128))
                              ]),
                              _: 1
                            })
                          ]),
                          default: withCtx(() => [
                            createTextVNode(toDisplayString(unref(setting).data[unref(current).algorithm]) + " ", 1)
                          ]),
                          _: 1
                        })
                      ])
                    ]),
                    _: 1
                  })) : createCommentVNode("", true),
                  unref(current).isFailed() ? (openBlock(), createBlock(_component_StatusVue, {
                    key: 3,
                    type: "failed",
                    title: unref(t)("main.status.failed"),
                    description: unref(current).result.message
                  }, null, 8, ["title", "description"])) : createCommentVNode("", true)
                ], 64)) : createCommentVNode("", true)
              ]),
              createBaseVNode("div", _hoisted_11, [
                createVNode(_component_DropZoneVue, {
                  class: "list",
                  onDrop: onFileDrop,
                  style: false
                }, {
                  default: withCtx(() => [
                    createVNode(unref(script$1), {
                      items: unref(main2).taskQueue.data,
                      minItemSize: 60,
                      itemSize: 58,
                      class: "scroller"
                    }, {
                      default: withCtx(({ item, index, active }) => [
                        createVNode(unref(script), {
                          item,
                          active,
                          "data-index": index,
                          class: "ai-style"
                        }, {
                          default: withCtx(() => [
                            createBaseVNode("div", {
                              class: normalizeClass(["item", {
                                active: item.id === unref(current).id
                              }]),
                              onClick: ($event) => current.value = item.id,
                              tabindex: "-1"
                            }, [
                              createVNode(_component_ViewerVue, {
                                class: "preview",
                                src: item.thumbnailURL,
                                width: item.width,
                                height: item.height,
                                selection: {
                                  x: 0,
                                  y: 0,
                                  width: item.width,
                                  height: item.height
                                }
                              }, null, 8, ["src", "width", "height", "selection"]),
                              createBaseVNode("div", _hoisted_13, [
                                createBaseVNode("div", _hoisted_14, toDisplayString(item.name), 1),
                                createBaseVNode("div", _hoisted_15, [
                                  (item.isSuccessful() || item.isProcessing() || item.isWaiting()) && item.width && item.height ? (openBlock(), createElementBlock(Fragment, { key: 0 }, [
                                    createTextVNode(" 4x / " + toDisplayString(item.width) + " x " + toDisplayString(item.height), 1)
                                  ], 64)) : createCommentVNode("", true),
                                  item.isFailed() ? (openBlock(), createElementBlock(Fragment, { key: 1 }, [
                                    createTextVNode(toDisplayString(unref(t)("main.status.failed")), 1)
                                  ], 64)) : createCommentVNode("", true)
                                ])
                              ]),
                              item.isProcessing() ? (openBlock(), createElementBlock("div", _hoisted_16, [
                                _hoisted_17,
                                createVNode(_component_el_progress, {
                                  percentage: item.result.data,
                                  type: "circle",
                                  width: "14",
                                  "stroke-width": "2",
                                  class: "ai-progress"
                                }, null, 8, ["percentage"])
                              ])) : withDirectives((openBlock(), createElementBlock("div", {
                                key: 1,
                                class: normalizeClass(["status hover-cancel", ["status-" + item.result.state]]),
                                onClick: withModifiers(($event) => removeTask(item), ["stop"])
                              }, null, 10, _hoisted_18)), [
                                [_directive_tippy, {
                                  content: unref(t)("main.status.cancel"),
                                  placement: "bottom",
                                  delay: [0, 0],
                                  duration: [200, 0]
                                }]
                              ])
                            ], 10, _hoisted_12)
                          ]),
                          _: 2
                        }, 1032, ["item", "active", "data-index"])
                      ]),
                      _: 1
                    }, 8, ["items"])
                  ]),
                  _: 1
                }),
                createBaseVNode("div", _hoisted_19, [
                  withDirectives(createBaseVNode("div", _hoisted_20, [
                    createBaseVNode("div", {
                      class: "current",
                      style: normalizeStyle({
                        width: `${unref(main2).taskQueue.completed.length / unref(main2).taskQueue.length * 100}%`
                      })
                    }, null, 4),
                    createBaseVNode("div", {
                      class: "comet",
                      style: normalizeStyle({
                        left: `calc(${unref(main2).taskQueue.completed.length / unref(main2).taskQueue.length * 100}% - 24px`
                      })
                    }, null, 4)
                  ], 512), [
                    [vShow, unref(main2).taskQueue.state.isWorking]
                  ]),
                  withDirectives(createBaseVNode("div", { class: "progress-status" }, toDisplayString(unref(t)("main.feature.processing")) + " " + toDisplayString(unref(main2).taskQueue.completed.length) + "/" + toDisplayString(unref(main2).taskQueue.length), 513), [
                    [vShow, unref(main2).taskQueue.state.isWorking]
                  ]),
                  withDirectives(createBaseVNode("div", _hoisted_21, [
                    createBaseVNode("div", {
                      class: "current",
                      style: normalizeStyle({
                        width: `${unref(main2).replaceState.current / unref(main2).replaceState.total * 100}%`
                      })
                    }, null, 4),
                    createBaseVNode("div", {
                      class: "comet",
                      style: normalizeStyle({
                        left: `calc(${unref(main2).replaceState.current / unref(main2).replaceState.total * 100}% - 24px`
                      })
                    }, null, 4)
                  ], 512), [
                    [vShow, unref(main2).replaceState.total]
                  ]),
                  withDirectives(createBaseVNode("div", { class: "progress-status" }, toDisplayString(unref(t)("main.feature.processing")) + " " + toDisplayString(unref(main2).replaceState.current) + "/" + toDisplayString(unref(main2).replaceState.total), 513), [
                    [vShow, unref(main2).replaceState.total > 0]
                  ]),
                  createVNode(_component_el_button, {
                    type: "",
                    onClick: _cache[4] || (_cache[4] = async () => {
                      const count = await unref(main2).saveAs();
                      taskDialogVisible.value = true;
                      taskDialogItemCount.value = count;
                    }),
                    disabled: unref(main2).taskQueue.state.isWorking
                  }, {
                    default: withCtx(() => [
                      createVNode(_component_ImageVue, {
                        width: "16",
                        height: "16",
                        src: "light/ic-saveAs.svg",
                        darkSrc: "dark/ic-saveAs.svg"
                      }),
                      createTextVNode(" " + toDisplayString(unref(t)("main.feature.saveAs")), 1)
                    ]),
                    _: 1
                  }, 8, ["disabled"]),
                  createVNode(_component_el_button, {
                    class: "ai-style",
                    type: "primary",
                    onClick: _cache[5] || (_cache[5] = async () => {
                      const count = await unref(main2).replaceAll();
                      if (count > 0) {
                        taskDialogVisible.value = true;
                        taskDialogItemCount.value = count;
                      }
                    }),
                    disabled: unref(main2).taskQueue.state.isWorking
                  }, {
                    default: withCtx(() => [
                      withDirectives(createVNode(_component_ButtonComet, null, null, 512), [
                        [vShow, !unref(main2).taskQueue.state.isWorking]
                      ]),
                      createVNode(_component_ImageVue, {
                        width: "16",
                        height: "16",
                        src: "light/ic-replace.svg",
                        darkSrc: "dark/ic-replace.svg"
                      }),
                      createTextVNode(" " + toDisplayString(unref(t)("main.feature.replace")), 1)
                    ]),
                    _: 1
                  }, 8, ["disabled"])
                ])
              ])
            ])) : (openBlock(), createBlock(_component_DropZoneVue, {
              key: 1,
              onDrop: onFileDrop
            }, {
              default: withCtx(() => [
                createVNode(_component_StatusVue, {
                  type: "empty",
                  title: unref(t)("main.empty.title"),
                  description: unref(t)("main.empty.content")
                }, null, 8, ["title", "description"])
              ]),
              _: 1
            }))
          ]),
          _: 1
        })) : createCommentVNode("", true),
        createVNode(_component_ModelDialogVue, { onClose: onModelDialogClose }),
        createVNode(_component_DialogVue, {
          type: "success",
          modelValue: unref(taskDialogVisible),
          "onUpdate:modelValue": _cache[6] || (_cache[6] = ($event) => isRef(taskDialogVisible) ? taskDialogVisible.value = $event : null),
          showCancelBtn: false
        }, {
          title: withCtx(() => [
            createTextVNode(toDisplayString(unref(t)("main.dialog.success.title")), 1)
          ]),
          description: withCtx(() => [
            createBaseVNode("span", {
              innerHTML: unref(t)("main.dialog.success.description", { count: unref(taskDialogItemCount) })
            }, null, 8, _hoisted_22)
          ]),
          ok: withCtx(() => [
            createTextVNode(toDisplayString(unref(t)("main.dialog.success.ok")), 1)
          ]),
          _: 1
        }, 8, ["modelValue"]),
        createVNode(_component_DialogVue, {
          type: "success",
          modelValue: unref(iGPUWarningVisible),
          "onUpdate:modelValue": _cache[7] || (_cache[7] = ($event) => isRef(iGPUWarningVisible) ? iGPUWarningVisible.value = $event : null),
          showOkBtn: true,
          showCancelBtn: false
        }, {
          title: withCtx(() => [
            createBaseVNode("span", {
              innerHTML: unref(t)("main.iGPUWarning.title")
            }, null, 8, _hoisted_23)
          ]),
          description: withCtx(() => [
            createBaseVNode("span", {
              innerHTML: unref(t)("main.iGPUWarning.description")
            }, null, 8, _hoisted_24)
          ]),
          ok: withCtx(() => [
            createTextVNode(toDisplayString(unref(t)("main.dialog.success.ok")), 1)
          ]),
          _: 1
        }, 8, ["modelValue"]),
        createVNode(_component_DialogVue, {
          type: "success",
          modelValue: unref(restartTasksVisible),
          "onUpdate:modelValue": _cache[8] || (_cache[8] = ($event) => isRef(restartTasksVisible) ? restartTasksVisible.value = $event : null),
          onOk: restartTasks,
          style: { "max-width": "480px" }
        }, {
          title: withCtx(() => [
            createTextVNode(toDisplayString(unref(t)("main.dialog.restart-tasks.title")), 1)
          ]),
          description: withCtx(() => [
            createBaseVNode("span", {
              innerHTML: unref(t)("main.dialog.restart-tasks.description")
            }, null, 8, _hoisted_25)
          ]),
          ok: withCtx(() => [
            createTextVNode(toDisplayString(unref(t)("main.dialog.restart-tasks.ok")), 1)
          ]),
          cancel: withCtx(() => [
            createTextVNode(toDisplayString(unref(t)("main.dialog.restart-tasks.cancel")), 1)
          ]),
          _: 1
        }, 8, ["modelValue"])
      ], 64);
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
      if (!this._startCallback(pointer, event2))
        return false;
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
      if (event2.button !== 0)
        return;
      if (!this._triggerPointerStart(new Pointer(event2), event2))
        return;
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
      const changedPointers = "changedTouches" in event2 ? Array.from(event2.changedTouches).map((t2) => new Pointer(t2)) : [new Pointer(event2)];
      const trackedChangedPointers = [];
      for (const pointer of changedPointers) {
        const index = this.currentPointers.findIndex((p) => p.id === pointer.id);
        if (index === -1)
          continue;
        trackedChangedPointers.push(pointer);
        this.currentPointers[index] = pointer;
      }
      if (trackedChangedPointers.length === 0)
        return;
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
      if (index === -1)
        return false;
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
      if (!this._triggerPointerEnd(new Pointer(event2), event2))
        return;
      if (isPointerEvent(event2)) {
        if (this.currentPointers.length)
          return;
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
    if (ref2 === void 0)
      ref2 = {};
    var insertAt = ref2.insertAt;
    if (!css2 || typeof document === "undefined") {
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
          if (pointerTracker.currentPointers.length === 1)
            return false;
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
      if (value && value.toLowerCase() === "vertical")
        return "vertical";
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
    if (ref2 === void 0)
      ref2 = {};
    var insertAt = ref2.insertAt;
    if (!css2 || typeof document === "undefined")
      return;
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
    if (typeof value === "number")
      return value;
    if (value.trimRight().endsWith("%"))
      return max * parseFloat(value) / 100;
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
      if (!attrValue)
        return MIN_SCALE;
      const value = parseFloat(attrValue);
      if (Number.isFinite(value))
        return Math.max(MIN_SCALE, value);
      return MIN_SCALE;
    }
    set maxScale(value) {
      this.setAttribute(maxScaleAttr, String(value));
    }
    get maxScale() {
      const attrValue = this.getAttribute(maxScaleAttr);
      if (!attrValue)
        return MAX_SCALE;
      const value = parseFloat(attrValue);
      if (Number.isFinite(value))
        return Math.min(MAX_SCALE, value);
      return MIN_SCALE;
    }
    set dragMove(value) {
      this.setAttribute(dragMoveAttr, String(value));
    }
    get dragMove() {
      const attrValue = this.getAttribute(dragMoveAttr);
      if (!attrValue)
        return false;
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
      if (scale > this.maxScale)
        return;
      if (scale < this.minScale)
        return;
      if (scale === this.scale && (event == null ? void 0 : event.metaKey))
        return;
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
      if (this.children.length === 0)
        return;
      this._positioningEl = this.children[0];
      if (this.children.length > 1) {
        console.warn("<pinch-zoom> must not have more than one child.");
      }
      this.setTransform({ allowChangeEvent: true });
    }
    _onWheel(event2) {
      if (event2.metaKey || event2.ctrlKey || this.dragMove) {
        if (!this._positioningEl)
          return;
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
const main = "";
const app = createApp(_sfc_main);
app.use(VueTippy);
app.use(VueMousetrapPlugin).provide("mousetrap", app.config.globalProperties.$mousetrap);
eagle.onPluginCreate(async (plugin) => {
  const utils2 = require(`${plugin.path}/modules/utils`);
  window.addEventListener("load", async () => {
    await utils2.file.deleteFolder(`${plugin.path}/temp`);
    await utils2.file.createFolder(`${plugin.path}/temp`);
  });
  window.addEventListener("unload", async () => {
    await utils2.file.deleteFolder(`${plugin.path}/temp`);
  });
  process.on("uncaughtException", (error) => {
    eagle.log.error(`uncaughtException : ${error}`);
  });
  if (eagle.app.platform === "darwin")
    await utils2.time.sleep(600);
  await eagle.window.setOpacity(1);
  app.mount("#app");
  toggleTheme();
});
eagle.onThemeChanged(() => {
  toggleTheme();
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
