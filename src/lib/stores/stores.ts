import type { Writable } from '@felte/common';
import { writable } from 'svelte/store';

export const messages: Writable<{ role: string; content: string }[]> = writable([]);
export const expanded: Writable<boolean> = writable(false);
export const maxHeight: Writable<number> = writable(800);

export const hideSuggestions = writable(false);
export const showShortcuts = writable(false);
export const showHistory = writable(false);
export const showMenu = writable(false);

export const responseErrors: Writable<Record<number, string>> = writable({});

// Waiting for the server to respond and finish responding
export const isLoading = writable(false);

export const os = writable('macos');
export const theme: Writable<'light' | 'dark'> = writable('light');

export const currentConversation: Writable<number | null> = writable(null);

export const app_pinned: Writable<boolean> = writable(false);