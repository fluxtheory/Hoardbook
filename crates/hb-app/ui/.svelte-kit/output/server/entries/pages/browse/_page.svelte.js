import { c as create_ssr_component, a as subscribe, b as add_attribute, d as escape } from "../../../chunks/ssr.js";
import "@tauri-apps/api/core";
import { a as contacts } from "../../../chunks/stores2.js";
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $contacts, $$unsubscribe_contacts;
  $$unsubscribe_contacts = subscribe(contacts, (value) => $contacts = value);
  let input = "";
  let loading = false;
  let result = null;
  $contacts.some((c) => c.hb_id === result?.hb_id);
  $$unsubscribe_contacts();
  return `<div class="p-6 space-y-6 max-w-2xl"><div><h2 class="text-lg font-semibold mb-3" data-svelte-h="svelte-bn8j04">Browse</h2> <div class="flex gap-2"><input class="input flex-1 font-mono text-sm" type="text" placeholder="Paste a Hoardbook ID (hb1_…)"${add_attribute("value", input, 0)}> <button class="btn variant-filled-primary" ${!input.trim() || loading ? "disabled" : ""}>${escape("Lookup")}</button></div></div> ${``}</div>`;
});
export {
  Page as default
};
