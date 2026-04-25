import { c as create_ssr_component, d as escape } from "./ssr.js";
const CollectionPanel = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { collection } = $$props;
  if ($$props.collection === void 0 && $$bindings.collection && collection !== void 0) $$bindings.collection(collection);
  return `<div class="border border-surface-700 rounded-lg overflow-hidden"><button class="w-full flex items-center justify-between px-4 py-3 bg-surface-800 hover:bg-surface-700 transition-colors text-left"><div><div class="font-medium">${escape(collection.path_alias)}</div> <div class="text-xs text-surface-400 mt-0.5">${escape(collection.item_count)} items
				${collection.est_size ? `· ~${escape(collection.est_size)}` : ``} ${collection.content_type.length > 0 ? `· ${escape(collection.content_type.join(", "))}` : ``}</div></div> <span class="text-surface-400 text-sm">${escape("▼")}</span></button> ${``} ${slots.default ? slots.default({}) : ``}</div>`;
});
export {
  CollectionPanel as C
};
