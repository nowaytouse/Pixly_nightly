import { m as mousetrap } from "./mousetrap-joIOW03i.js";
const VueMousetrapPlugin = {
  install(app) {
    app.config.globalProperties.$mousetrap = mousetrap;
  }
};
export {
  VueMousetrapPlugin as V
};
