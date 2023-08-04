<script lang="ts">
	import { onMount } from 'svelte';
	import { Store } from 'tauri-plugin-store-api';
	import Button from '../Button.svelte';
    import { relaunch } from "@tauri-apps/api/process";

	const store = new Store('.preferences.dat');

	let apiKey: string | null = null;

	onMount(async () => {
		// Get API Key if available
		let storedAPIKey: string | null = await store.get('apiKey');
		if (storedAPIKey) {
			apiKey = storedAPIKey;
		}
	});

	const setAPIKey = async () => {
		const apiKey = (document.getElementById('api-key') as HTMLInputElement).value;
		await store.set('apiKey', apiKey);
		await store.save();
    relaunch();
	};
</script>

<div class="flex flex-col gap-4 min-h-full relative">
	<p>Enter your OpenAI API key here:</p>
	<input id="api-key" class="rounded-lg border-2 disabled cursor-text text-sm mt-8 p-2" value={apiKey}/>
	<Button attributes={{ type: 'button', class: 'p-1 w-full h-fit' }} on:click={setAPIKey}>
		Set API Key
	</Button>
</div>
