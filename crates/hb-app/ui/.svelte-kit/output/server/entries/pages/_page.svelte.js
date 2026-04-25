import { c as create_ssr_component, f as createEventDispatcher, b as add_attribute, d as escape, a as subscribe, e as each, v as validate_component } from "../../chunks/ssr.js";
import "@tauri-apps/api/core";
import { p as profile, i as identity, c as collections } from "../../chunks/stores2.js";
import { C as CollectionPanel } from "../../chunks/CollectionPanel.js";
import "@tauri-apps/plugin-dialog";
const ScanDialog = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  createEventDispatcher();
  let { open = false } = $$props;
  let path = "";
  let pathAlias = "";
  let depth = 3;
  let excludeRaw = ".git, node_modules, __pycache__";
  if ($$props.open === void 0 && $$bindings.open && open !== void 0) $$bindings.open(open);
  return `${open ? `  <div class="fixed inset-0 bg-black/60 z-40 flex items-center justify-center" role="presentation"><div class="card bg-zinc-800 border border-zinc-600 shadow-2xl p-6 w-full max-w-md space-y-4 z-50"><h3 class="text-lg font-semibold" data-svelte-h="svelte-opf5d8">Add Collection</h3> <label class="label"><span data-svelte-h="svelte-1fwxyg0">Directory path</span> <div class="flex gap-2"><input class="input flex-1" type="text" placeholder="C:\\Movies or /mnt/data/books"${add_attribute("value", path, 0)}> <button class="btn variant-ghost-surface shrink-0" type="button" data-svelte-h="svelte-4cjjto">Browse…</button></div></label> <label class="label"><span data-svelte-h="svelte-1krde78">Display name (alias)</span> <input class="input" type="text" placeholder="My Movie Collection"${add_attribute("value", pathAlias, 0)}></label> <label class="label"><span>Scan depth: ${escape(depth)}</span> <input class="input" type="range" min="1" max="8"${add_attribute("value", depth, 0)}></label> <label class="label"><span data-svelte-h="svelte-6qqkhg">Exclude (comma-separated)</span> <input class="input" type="text"${add_attribute("value", excludeRaw, 0)}></label> <div class="flex justify-end gap-2 pt-2"><button class="btn variant-ghost" data-svelte-h="svelte-1jlmx8k">Cancel</button> <button class="btn variant-filled-primary" ${"disabled"}>${escape("Scan")}</button></div></div></div>` : ``}`;
});
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $profile, $$unsubscribe_profile;
  let $identity, $$unsubscribe_identity;
  let $collections, $$unsubscribe_collections;
  $$unsubscribe_profile = subscribe(profile, (value) => $profile = value);
  $$unsubscribe_identity = subscribe(identity, (value) => $identity = value);
  $$unsubscribe_collections = subscribe(collections, (value) => $collections = value);
  let scanOpen = false;
  let saving = false;
  let form = {
    display_name: "",
    bio: void 0,
    since: void 0,
    est_size: void 0,
    languages: [],
    contact_hint: void 0,
    email: void 0
  };
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    {
      if ($profile && !saving) {
        form = { ...$profile };
      }
    }
    $$rendered = `${!$identity ? `<div class="flex items-center justify-center h-full text-surface-400" data-svelte-h="svelte-14bzui9"><div class="text-center"><p class="text-lg">No identity yet.</p> <a href="/settings" class="btn variant-filled-primary mt-4">Go to Settings →</a></div></div>` : `<div class="flex h-full"> <div class="w-96 flex-shrink-0 border-r border-surface-700 p-6 overflow-y-auto space-y-4"><h2 class="text-lg font-semibold" data-svelte-h="svelte-157dcg8">My Profile</h2> <label class="label"><span data-svelte-h="svelte-1mmy0jq">Display name <span class="text-error-400">*</span></span> <input class="input" type="text" placeholder="e.g. DataHoarder_42"${add_attribute("value", form.display_name, 0)}></label> <label class="label"><span data-svelte-h="svelte-1r9rrnk">Bio</span> <textarea class="textarea" rows="3" placeholder="What do you collect?">${escape(form.bio || "")}</textarea></label> <div class="grid grid-cols-2 gap-3"><label class="label"><span data-svelte-h="svelte-1v0nf7k">Since (year)</span> <input class="input" type="number" min="1990" max="2099" placeholder="2018"${add_attribute("value", form.since, 0)}></label> <label class="label"><span data-svelte-h="svelte-1bc89m1">Est. total size</span> <input class="input" type="text" placeholder="12 TB"${add_attribute("value", form.est_size, 0)}></label></div> <label class="label"><span data-svelte-h="svelte-1yiq2av">Languages (comma-separated)</span> <input class="input" type="text" placeholder="en, jp, fr"${add_attribute("value", form.languages.join(", "), 0)}></label> <label class="label"><span data-svelte-h="svelte-14ri0sv">Contact hint</span> <input class="input" type="text" placeholder="Matrix: @you:matrix.org"${add_attribute("value", form.contact_hint, 0)}></label> <label class="label"><span data-svelte-h="svelte-zmcb0m">Email <span class="text-warning-400 text-xs">(public)</span></span> <input class="input" type="email" placeholder="you@example.com"${add_attribute("value", form.email, 0)}> <p class="text-surface-400 text-xs mt-0.5" data-svelte-h="svelte-11wq7w4">Visible to anyone who views your profile.</p></label> <div class="flex gap-2 pt-2"><button class="btn variant-ghost flex-1" ${!form.display_name || saving ? "disabled" : ""}>${escape("Save draft")}</button> <button class="btn variant-filled-primary flex-1" ${""}>${escape("Publish")}</button></div></div>  <div class="flex-1 p-6 overflow-y-auto space-y-4"><div class="flex items-center justify-between"><h2 class="text-lg font-semibold" data-svelte-h="svelte-q41t18">My Collections</h2> <button class="btn variant-filled-primary btn-sm" data-svelte-h="svelte-naltvp">+ Add Collection</button></div> ${$collections.length === 0 ? `<div class="text-surface-400 text-sm py-8 text-center" data-svelte-h="svelte-x0r7mt"><p>No collections yet.</p> <p class="mt-1">Click &quot;Add Collection&quot; to scan a directory.</p></div>` : `${each($collections, (col) => {
      return `${validate_component(CollectionPanel, "CollectionPanel").$$render($$result, { collection: col }, {}, {
        default: () => {
          return `<div class="px-4 py-2 border-t border-surface-700 flex justify-end"><button class="btn variant-ghost-primary btn-sm" data-svelte-h="svelte-1tzxr5e">Publish
							</button></div> `;
        }
      })}`;
    })}`}</div></div> ${validate_component(ScanDialog, "ScanDialog").$$render(
      $$result,
      { open: scanOpen },
      {
        open: ($$value) => {
          scanOpen = $$value;
          $$settled = false;
        }
      },
      {}
    )}`}`;
  } while (!$$settled);
  $$unsubscribe_profile();
  $$unsubscribe_identity();
  $$unsubscribe_collections();
  return $$rendered;
});
export {
  Page as default
};
