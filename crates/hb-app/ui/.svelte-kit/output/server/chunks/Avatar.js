import { c as create_ssr_component, d as escape } from "./ssr.js";
import { a as avatarHue } from "./icons.js";
const Avatar = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let h;
  let grad;
  let br;
  let fs;
  let { letter = "?" } = $$props;
  let { size = 36 } = $$props;
  let { hue = void 0 } = $$props;
  if ($$props.letter === void 0 && $$bindings.letter && letter !== void 0) $$bindings.letter(letter);
  if ($$props.size === void 0 && $$bindings.size && size !== void 0) $$bindings.size(size);
  if ($$props.hue === void 0 && $$bindings.hue && hue !== void 0) $$bindings.hue(hue);
  h = hue ?? avatarHue(letter);
  grad = `linear-gradient(135deg, oklch(0.55 0.10 ${h}) 0%, oklch(0.40 0.08 ${h + 40}) 100%)`;
  br = size > 28 ? "8px" : "6px";
  fs = `${(size * 0.42).toFixed(1)}px`;
  return `<div style="${"width:" + escape(size, true) + "px; height:" + escape(size, true) + "px; border-radius:" + escape(br, true) + "; background:" + escape(grad, true) + "; color:oklch(0.98 0 0); display:flex; align-items:center; justify-content:center; font-weight:700; font-family:var(--font-ui); font-size:" + escape(fs, true) + "; flex-shrink:0; letter-spacing:-0.5px; box-shadow:inset 0 0 0 1px oklch(1 0 0 / 0.08);"}">${escape(letter.toUpperCase())}</div>`;
});
export {
  Avatar as A
};
