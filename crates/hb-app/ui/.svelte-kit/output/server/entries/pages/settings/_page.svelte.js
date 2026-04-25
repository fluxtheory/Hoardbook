import { c as create_ssr_component, a as subscribe, d as escape, e as each, b as add_attribute } from "../../../chunks/ssr.js";
import "@tauri-apps/api/core";
import { i as identity } from "../../../chunks/stores2.js";
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $identity, $$unsubscribe_identity;
  $$unsubscribe_identity = subscribe(identity, (value) => $identity = value);
  let relayUrls = [];
  let newRelay = "";
  $$unsubscribe_identity();
  return `<div class="p-6 space-y-8 max-w-lg"><h2 class="text-lg font-semibold" data-svelte-h="svelte-xfs78y">Settings</h2>  <section class="space-y-3"><h3 class="text-sm font-semibold text-surface-400 uppercase tracking-wide" data-svelte-h="svelte-e6hbxl">Identity</h3> ${$identity ? `<div class="card bg-surface-800 p-4 space-y-3"><div><div class="text-xs text-surface-400 mb-1" data-svelte-h="svelte-1qbtvpg">Your Hoardbook ID</div> <div class="font-mono text-sm break-all bg-surface-900 rounded px-3 py-2">${escape($identity.hb_id)}</div></div> <button class="btn variant-ghost btn-sm">${escape("Copy ID")}</button> <p class="text-xs text-surface-500" data-svelte-h="svelte-ccwxp7">Share this ID with others so they can look you up.</p></div>` : `<div class="card bg-surface-800 p-4 space-y-3"><p class="text-sm text-surface-300" data-svelte-h="svelte-3it00g">No identity yet. Generate a keypair to get started.</p> <button class="btn variant-filled-primary" ${""}>${escape("Generate Keypair")}</button></div>`}</section>  <section class="space-y-3"><h3 class="text-sm font-semibold text-surface-400 uppercase tracking-wide" data-svelte-h="svelte-11crzo1">Relays</h3> <div class="card bg-surface-800 p-4 space-y-3">${relayUrls.length === 0 ? `<p class="text-sm text-surface-500" data-svelte-h="svelte-5rl6n1">No relays configured. Add one to publish and browse.</p>` : `<ul class="space-y-1">${each(relayUrls, (url) => {
    return `<li class="flex items-center gap-2"><span class="font-mono text-sm text-surface-300 flex-1 truncate">${escape(url)}</span> <button class="btn variant-ghost-error btn-sm" data-svelte-h="svelte-10re2hi">✕</button> </li>`;
  })}</ul>`} <div class="flex gap-2"><input class="input flex-1 font-mono text-sm" type="text" placeholder="http://localhost:3000"${add_attribute("value", newRelay, 0)}> <button class="btn variant-ghost" ${!newRelay.trim() ? "disabled" : ""}>Add</button></div> <button class="btn variant-filled-primary w-full" ${""}>${escape("Save Relays")}</button></div></section></div>`;
});
export {
  Page as default
};
