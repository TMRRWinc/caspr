<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { register, unregister } from "@tauri-apps/api/globalShortcut";
	import { onDestroy, onMount } from "svelte";
	import { messages, os, theme, responseErrors } from "$lib/stores/stores";
	import { listen } from "@tauri-apps/api/event";
	import { Store } from "tauri-plugin-store-api";
	import { handleCompletionDone } from "$lib/data/openAIAPI";

	onMount(async () => {
		const store = new Store(".preferences.dat");
		const themePref = await store.get("theme");
		const osTheme = await store.get("os_theme");

		if (osTheme && (themePref === "system" || !themePref)) {
			if (osTheme === "dark") {
				document.body.classList.add("dark");
				theme.set("dark");
			} else {
				document.body.classList.remove("dark");
				theme.set("light");
			}
		}

		if (themePref) {
			if (themePref === "dark") {
				document.body.classList.add("dark");
				theme.set("dark");
			} else {
				document.body.classList.remove("dark");
				theme.set("light");
			}
		}
		// register a global shortcut to hide/show the app
		register("CommandOrControl+Shift+C", async () => {
			await invoke("toggle_app");
		});

		os.set(await invoke("get_os"));

		let currentRole: string | null = null;

		const unlistenStreamEvent = await listen<{type_: "start" | "finish" | "role" | "content" | "error", content: string, id: string }>("stream-event", (event) => {
			const {type_, content, id} = event.payload;
			
			switch (type_) {
				case "start":
					// console.log("stream has started");
					break;
				case "finish":
					// console.log(`stream has ended with reason: ${content}`);
					handleCompletionDone();
					break;
				case "error":
					responseErrors.update(n => {
						if ($messages.length > 0) {
								const index = $messages.length - 1;
								return {...n, [index] : content}
						} else {
							messages.set ([
								...$messages,
								{role: 'assistant', content}
							]);
						}
						return n;
					});
					handleCompletionDone();
					break;
				case "role":
					// console.log(`stream role: ${content}`);
					currentRole = content;
					break;
				case "content":
					if (content || currentRole) {
						messages.update((n) => {
							const current = n.at(-1);
							if (current && content) {
								current.content = current.content + content;
							}
							if (current && currentRole) {
								current.role = currentRole;
							}
							return n;
						});
					}
					break;
			}
		});

		// Listen for changes in theme
		const unlistenTheme = await listen<{theme: "light" | "dark"}>("theme-changed", (event) => {
			if (event.payload.theme === "dark") {
				document.body.classList.add("dark");
				theme.set("dark");
			} else {
				document.body.classList.remove("dark");
				theme.set("light");
			}
    	});

		// Listen for aptempted duplicate launches and show app
		const unlistenSingleIntance = await listen("single-instance", async () => {
			await invoke("show_app");
		});
			
		onDestroy(async () => {
			// unregister all shortcuts and listeners
			await unregister("CommandOrControl+Shift+C");
			unlistenTheme();
			unlistenSingleIntance();
			unlistenStreamEvent();
		});

	});
	
</script>

<slot />