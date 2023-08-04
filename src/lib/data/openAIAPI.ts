import { showShortcuts, isLoading, messages, currentConversation } from '$lib/stores/stores';
import { get } from 'svelte/store';
import { invoke } from '@tauri-apps/api';
import { addConversation, updateConversation } from './localdb';

export const handleCompletionDone = () => {
	isLoading.set(false);
	showShortcuts.set(true);
	const currentId = get(currentConversation);
	if (currentId === null) {
		addConversation(get(messages).slice(-2)).then((result) =>
			currentConversation.set(result.lastInsertId)
		);
	} else {
		updateConversation(currentId, get(messages).slice(-2));
	}
};

export const completion = async (inputValue: string) => {
	isLoading.set(true);

	const inputEl: HTMLInputElement | null = document.querySelector('input#prompt-field');
	if (inputEl) {
		inputEl.value = '';
	}

	const reqMessages = [...get(messages), { role: 'user', content: inputValue }];

	messages.update((n) => [...n, { role: 'user', content: inputValue }]);
	messages.update((n) => [...n, { role: 'assistant', content: '' }]);

	await invoke('set_expand', { open: true });
	invoke('stream', { messages: reqMessages });
};
