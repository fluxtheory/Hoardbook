import { c as create_ssr_component, d as escape, a as subscribe, e as each, v as validate_component } from "../../../chunks/ssr.js";
import "@tauri-apps/api/core";
import { a as contacts } from "../../../chunks/stores2.js";
import { C as CollectionPanel } from "../../../chunks/CollectionPanel.js";
const ProfileCard = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let initial;
  let shortId;
  let { peer } = $$props;
  if ($$props.peer === void 0 && $$bindings.peer && peer !== void 0) $$bindings.peer(peer);
  initial = peer.profile?.display_name?.[0]?.toUpperCase() ?? "?";
  shortId = peer.hb_id.length > 14 ? peer.hb_id.slice(0, 8) + "…" + peer.hb_id.slice(-4) : peer.hb_id;
  return `<div class="card p-4 flex gap-4 items-start"> <div class="avatar"><div class="w-12 h-12 rounded-full bg-primary-500 flex items-center justify-center text-xl font-bold text-white">${escape(initial)}</div></div>  <div class="flex-1 min-w-0"><div class="flex items-center gap-2 flex-wrap"><span class="font-semibold truncate">${escape(peer.profile?.display_name ?? "Unknown")}</span> ${peer.online ? `<span class="badge variant-filled-success text-xs" data-svelte-h="svelte-1b18y8">Online</span>` : `<span class="badge variant-ghost-surface text-xs" data-svelte-h="svelte-q9kqwh">Offline</span>`}</div> <div class="text-xs text-surface-400 font-mono mt-0.5">${escape(shortId)}</div> ${peer.profile?.bio ? `<p class="text-sm text-surface-300 mt-1 line-clamp-2">${escape(peer.profile.bio)}</p>` : ``} <div class="flex gap-3 mt-1 text-xs text-surface-400 flex-wrap">${peer.profile?.est_size ? `<span>~${escape(peer.profile.est_size)}</span>` : ``} ${peer.collections.length > 0 ? `<span>${escape(peer.collections.length)} collection${escape(peer.collections.length !== 1 ? "s" : "")}</span>` : ``}</div></div> ${slots.default ? slots.default({}) : ``}</div>`;
});
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $contacts, $$unsubscribe_contacts;
  $$unsubscribe_contacts = subscribe(contacts, (value) => $contacts = value);
  let expanded = null;
  let refreshing = null;
  $$unsubscribe_contacts();
  return `<div class="p-6 space-y-4"><h2 class="text-lg font-semibold" data-svelte-h="svelte-477gt6">Contacts</h2> ${$contacts.length === 0 ? `<p class="text-surface-400 text-sm" data-svelte-h="svelte-1rhb7a1">No contacts yet. Go to <a href="/browse" class="text-primary-400 underline">Browse</a> to follow someone.</p>` : `${each($contacts, (peer) => {
    return `<div class="space-y-2">${validate_component(ProfileCard, "ProfileCard").$$render($$result, { peer }, {}, {
      default: () => {
        return `<div class="flex flex-col gap-1 flex-shrink-0"><button class="btn variant-ghost btn-sm" ${refreshing === peer.hb_id ? "disabled" : ""}>${escape(refreshing === peer.hb_id ? "…" : "Refresh")}</button> <button class="btn variant-ghost btn-sm">${escape(expanded === peer.hb_id ? "Collapse" : "Collections")} </button></div> `;
      }
    })} ${expanded === peer.hb_id ? `<div class="pl-4 space-y-2">${peer.collections.length === 0 ? `<p class="text-surface-500 text-sm" data-svelte-h="svelte-1tg8nwi">No collections published.</p>` : `${each(peer.collections, (col) => {
      return `${validate_component(CollectionPanel, "CollectionPanel").$$render($$result, { collection: col }, {}, {})}`;
    })}`} </div>` : ``} </div>`;
  })}`}</div>`;
});
export {
  Page as default
};
