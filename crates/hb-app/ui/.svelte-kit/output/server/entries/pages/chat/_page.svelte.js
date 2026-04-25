import { c as create_ssr_component, a as subscribe, d as escape, e as each } from "../../../chunks/ssr.js";
import { b as inboxMessages, a as contacts, i as identity, s as sentMessages } from "../../../chunks/stores2.js";
import "@tauri-apps/api/core";
function shortId(hb_id) {
  return hb_id.length > 16 ? hb_id.slice(0, 8) + "…" + hb_id.slice(-4) : hb_id;
}
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let unreadCounts;
  let $inboxMessages, $$unsubscribe_inboxMessages;
  let $contacts, $$unsubscribe_contacts;
  let $identity, $$unsubscribe_identity;
  let $$unsubscribe_sentMessages;
  $$unsubscribe_inboxMessages = subscribe(inboxMessages, (value) => $inboxMessages = value);
  $$unsubscribe_contacts = subscribe(contacts, (value) => $contacts = value);
  $$unsubscribe_identity = subscribe(identity, (value) => $identity = value);
  $$unsubscribe_sentMessages = subscribe(sentMessages, (value) => value);
  let selectedPeer = null;
  $identity?.hb_id ?? "";
  unreadCounts = Object.fromEntries($contacts.map((c) => [c.hb_id, $inboxMessages.filter((m) => m.from === c.hb_id).length]));
  $$unsubscribe_inboxMessages();
  $$unsubscribe_contacts();
  $$unsubscribe_identity();
  $$unsubscribe_sentMessages();
  return `${!$identity ? `<div class="flex items-center justify-center h-full text-surface-400" data-svelte-h="svelte-14bzui9"><div class="text-center"><p class="text-lg">No identity yet.</p> <a href="/settings" class="btn variant-filled-primary mt-4">Go to Settings →</a></div></div>` : `<div class="flex h-full"> <div class="w-56 flex-shrink-0 border-r border-surface-700 flex flex-col"><div class="flex items-center justify-between px-4 py-3 border-b border-surface-700"><span class="text-sm font-semibold" data-svelte-h="svelte-kiuuly">Conversations</span> <button class="btn-icon btn-icon-sm variant-ghost" title="Refresh inbox" ${""}>${escape("↺")}</button></div> ${$contacts.length === 0 ? `<div class="p-4 text-surface-400 text-xs" data-svelte-h="svelte-1sf40zx">Add contacts via Browse to start chatting.</div>` : `<ul class="flex-1 overflow-y-auto">${each($contacts, (peer) => {
    return `<li><button class="${[
      "w-full text-left px-4 py-3 text-sm transition-colors hover:bg-surface-700 flex items-center gap-2",
      (selectedPeer?.hb_id === peer.hb_id ? "bg-surface-700" : "") + " " + (selectedPeer?.hb_id === peer.hb_id ? "text-primary-400" : "")
    ].join(" ").trim()}"> <div class="w-7 h-7 rounded-full bg-primary-700 flex items-center justify-center text-xs font-bold flex-shrink-0">${escape((peer.profile?.display_name ?? peer.hb_id)[0].toUpperCase())}</div> <div class="flex-1 min-w-0"><p class="truncate font-medium">${escape(peer.profile?.display_name ?? shortId(peer.hb_id))} </p></div> ${unreadCounts[peer.hb_id] > 0 ? `<span class="flex-shrink-0 bg-primary-500 text-white text-xs rounded-full px-1.5 py-0.5">${escape(unreadCounts[peer.hb_id])} </span>` : ``}</button> </li>`;
  })}</ul>`}</div>  <div class="flex-1 flex flex-col overflow-hidden">${`<div class="flex-1 flex items-center justify-center text-surface-400 flex-col gap-3 p-8" data-svelte-h="svelte-15m8sim"><p class="text-base">Select a contact to view the conversation.</p> <p class="text-xs text-center max-w-sm text-warning-400">⚠ Messages are stored unencrypted on relay servers and are publicly readable
						by anyone who knows your Hoardbook ID.</p></div>`}</div></div>`}`;
});
export {
  Page as default
};
