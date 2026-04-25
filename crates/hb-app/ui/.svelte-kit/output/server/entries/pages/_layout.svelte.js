import { c as create_ssr_component, a as subscribe, e as each, b as add_attribute, d as escape } from "../../chunks/ssr.js";
import { p as page } from "../../chunks/stores.js";
import "@tauri-apps/api/core";
import { t as toastMessage } from "../../chunks/stores2.js";
const Layout = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let currentPath;
  let $page, $$unsubscribe_page;
  let $toastMessage, $$unsubscribe_toastMessage;
  $$unsubscribe_page = subscribe(page, (value) => $page = value);
  $$unsubscribe_toastMessage = subscribe(toastMessage, (value) => $toastMessage = value);
  const navItems = [
    { href: "/", label: "Home" },
    { href: "/contacts", label: "Contacts" },
    { href: "/browse", label: "Browse" },
    { href: "/chat", label: "Chat" },
    { href: "/settings", label: "Settings" }
  ];
  currentPath = $page.url.pathname;
  $$unsubscribe_page();
  $$unsubscribe_toastMessage();
  return `<div class="flex h-screen bg-surface-900 text-surface-50 overflow-hidden"> <nav class="w-48 flex-shrink-0 bg-surface-800 border-r border-surface-700 flex flex-col py-4"><div class="px-4 mb-6" data-svelte-h="svelte-je4imf"><span class="text-lg font-bold text-primary-400">Hoardbook</span></div> ${each(navItems, (item) => {
    return `<a${add_attribute("href", item.href, 0)} class="${[
      "px-4 py-2.5 text-sm transition-colors hover:bg-surface-700",
      (currentPath === item.href ? "bg-surface-700" : "") + " " + (currentPath === item.href ? "text-primary-400" : "") + " " + (currentPath !== item.href ? "text-surface-300" : "")
    ].join(" ").trim()}">${escape(item.label)} </a>`;
  })}</nav>  <main class="flex-1 overflow-y-auto">${slots.default ? slots.default({}) : ``}</main></div>  ${$toastMessage ? `<div class="${[
    "fixed bottom-4 right-4 z-50 px-4 py-2 rounded shadow-lg text-sm font-medium",
    ($toastMessage.kind === "success" ? "bg-success-500" : "") + " " + ($toastMessage.kind === "error" ? "bg-error-500" : "")
  ].join(" ").trim()}">${escape($toastMessage.text)}</div>` : ``}`;
});
export {
  Layout as default
};
