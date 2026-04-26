import { w as writable } from "./index.js";
const identity = writable(null);
const profile = writable(null);
const collections = writable([]);
const contacts = writable([]);
const inboxMessages = writable([]);
const sentMessages = writable([]);
const toastMessage = writable(null);
const navIcons = {
  Home: `<svg width="16" height="16" viewBox="0 0 16 16" fill="none"><path d="M2.5 7L8 2.5L13.5 7v6a1 1 0 01-1 1h-9a1 1 0 01-1-1V7z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/></svg>`,
  Contacts: `<svg width="16" height="16" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="6" r="2.5" stroke="currentColor" stroke-width="1.4"/><path d="M3 13.5c.5-2.5 2.5-3.5 5-3.5s4.5 1 5 3.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>`,
  Browse: `<svg width="16" height="16" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="5.5" stroke="currentColor" stroke-width="1.4"/><path d="M2.5 8h11M8 2.5c1.8 2 1.8 9 0 11M8 2.5c-1.8 2-1.8 9 0 11" stroke="currentColor" stroke-width="1.4"/></svg>`,
  Chat: `<svg width="16" height="16" viewBox="0 0 16 16" fill="none"><path d="M2.5 7c0-2.5 2-4 5.5-4s5.5 1.5 5.5 4-2 4-5.5 4c-.6 0-1.2 0-1.7-.1L4 12.5V10c-1-.7-1.5-1.7-1.5-3z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/></svg>`,
  Settings: `<svg width="16" height="16" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.4"/><path d="M8 1.5v2M8 12.5v2M14.5 8h-2M3.5 8h-2M12.6 3.4l-1.4 1.4M4.8 11.2l-1.4 1.4M12.6 12.6l-1.4-1.4M4.8 4.8L3.4 3.4" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/></svg>`
};
const icons = {
  search: `<svg width="14" height="14" viewBox="0 0 16 16" fill="none"><circle cx="7" cy="7" r="4.5" stroke="currentColor" stroke-width="1.4"/><path d="M10.5 10.5L13 13" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>`,
  plus: `<svg width="12" height="12" viewBox="0 0 12 12" fill="none"><path d="M6 2v8M2 6h8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>`,
  refresh: `<svg width="13" height="13" viewBox="0 0 14 14" fill="none"><path d="M2 7a5 5 0 018.5-3.5L12 5M12 2v3h-3M12 7a5 5 0 01-8.5 3.5L2 9M2 12V9h3" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  copy: `<svg width="13" height="13" viewBox="0 0 14 14" fill="none"><rect x="3.5" y="3.5" width="7" height="8" rx="1.2" stroke="currentColor" stroke-width="1.3"/><path d="M5.5 3.5V2.5a1 1 0 011-1h4a1 1 0 011 1v6a1 1 0 01-1 1h-1" stroke="currentColor" stroke-width="1.3"/></svg>`,
  close: `<svg width="11" height="11" viewBox="0 0 12 12" fill="none"><path d="M3 3l6 6M9 3l-6 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>`,
  folder: `<svg width="13" height="13" viewBox="0 0 14 14" fill="none"><path d="M2 4a1 1 0 011-1h2.5l1 1.5H11a1 1 0 011 1V11a1 1 0 01-1 1H3a1 1 0 01-1-1V4z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"/></svg>`,
  qr: `<svg width="13" height="13" viewBox="0 0 14 14" fill="none"><rect x="2" y="2" width="4" height="4" stroke="currentColor" stroke-width="1.2"/><rect x="8" y="2" width="4" height="4" stroke="currentColor" stroke-width="1.2"/><rect x="2" y="8" width="4" height="4" stroke="currentColor" stroke-width="1.2"/><path d="M8 8h2v2M12 8v4M8 12h2" stroke="currentColor" stroke-width="1.2"/></svg>`,
  shield: `<svg width="13" height="13" viewBox="0 0 14 14" fill="none"><path d="M7 1.5L2 3v4c0 3 2.5 5 5 5.5 2.5-.5 5-2.5 5-5.5V3l-5-1.5z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"/></svg>`,
  chevronDown: `<svg width="10" height="10" viewBox="0 0 10 10" fill="none"><path d="M2 4l3 3 3-3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>`
};
function avatarHue(letter) {
  return letter.charCodeAt(0) * 37 % 360;
}
export {
  avatarHue as a,
  icons as b,
  collections as c,
  contacts as d,
  inboxMessages as e,
  identity as i,
  navIcons as n,
  profile as p,
  sentMessages as s,
  toastMessage as t
};
