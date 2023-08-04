<script lang="ts">
	import Prompt from '$lib/components/Prompt.svelte';
	import {appWindow} from '@tauri-apps/api/window';

	import {
		isLoading,
		responseErrors,
		messages,
		expanded,
		app_pinned,
		showShortcuts,
	} from '$lib/stores/stores';
	import GhostButton from '$lib/components/GhostButton.svelte';
	import ShortcutContainer from '$lib/components/ShortcutContainer.svelte';
	import { completion } from '$lib/data/openAIAPI';
	import AppContainer from '$lib/components/AppContainer.svelte';
	import Message from '$lib/components/Message.svelte';
	import History from '$lib/components/History.svelte';
	import Menu from '$lib/components/Menu.svelte';
	import { checkOnlineStatus } from '$lib/services/app';
	
	
	

	$: mouseDown = false;
	// Binding values
	let inputValue: string = '';
	let ghostButton: HTMLButtonElement;
	$: online = true;

	// hide the app window when the input field loses focus and mouse is not actively clicked on the window ie. resizing or dragging
	const onWindowBlur = async () => {
		if ($app_pinned) return;
		if (!mouseDown) {
			if (!$isLoading){
				await appWindow.hide();
			}	
		} else {
			mouseDown = false;
		} 
	};

	const onWindowFocus = async () => {
		await appWindow.setFocus();
		const promptEl: HTMLInputElement | null = document.querySelector('#prompt-field');
		if (promptEl) promptEl.focus();
		online = await checkOnlineStatus()
	};

	const handleSubmit = async () => {
		// Prevent prompt spamming
		if ($isLoading) return;

		// Prevent submitting empty prompt
		if (!inputValue) {
			responseErrors.update(n => {
				if ($messages.length > 0) {
	
					const index = $messages.length - 1;
					return {...n, [index] : "No prompt found to submit. Please type something in the prompt field."}
				} else {
					messages.set ([
						...$messages,
						{role: 'assistant', content: 'No prompt found to submit. Please type something in the prompt field.'}
					]);
				}
				return n;
			});
			return;
		};
		
		showShortcuts.set(false);
		
		expanded.set(true);

		try {
			await completion(inputValue);
			const prompt = document.getElementById('prompt-field') as HTMLInputElement;
			prompt.focus();
			inputValue = '';
			prompt.placeholder = 'Continue the conversation...';
			appWindow.setFocus();
		} catch (error: any) {
			responseErrors.update(n => {
				if ($messages.length > 0) {

					const index = $messages.length - 1;
					return {...n, [index] : error.message}
				}
				return n;
			});
			throw error;
		}
	};

	const handlePromptKeydown = async (event: KeyboardEvent) => {
		if (event.key === 'Enter') {
			event.preventDefault();
			await handleSubmit();
		}
		if (event.key == 'ArrowDown') {
			event.preventDefault();
			const firstHistoryEl: HTMLButtonElement | null = document.querySelector('#history-list-container > button');
			const firstMenuEl: HTMLButtonElement | null = document.querySelector('#menu-list-container > button');
			if (firstHistoryEl) {
				firstHistoryEl.focus();
			} else if (firstMenuEl) {
				firstMenuEl.focus();
			}
		}
	};

	// handle scrolling to bottom of response when data added to generatedResponse
	let bottomOfResponseRef: HTMLDivElement | null = null;
	$: $messages.length > 0 && bottomOfResponseRef?.scrollIntoView({ behavior: 'smooth' });

	let promptEl: HTMLInputElement;

	const handleMouseDown = (e: MouseEvent) => {
		mouseDown = true;
	};

</script>

<svelte:window
	on:blur={onWindowBlur}
	on:focus={onWindowFocus}
	on:mousedown={handleMouseDown}
	/>

<AppContainer>
	<!-- ! Prompt Slot -->
	<Prompt bind:inputEl={promptEl} bind:value={inputValue} bind:online={online} on:keydown={handlePromptKeydown} slot="prompt">
		<GhostButton on:click={handleSubmit} bind:buttonEl={ghostButton} />
	</Prompt>

	<!-- ! History Slot  -->
	<div id="history-container" slot="history" class="h-full max-h-full border-r inner-borders w-full overflow-hidden relative" >
		<History />
	</div>

	<div id="menu-container" slot="menu" class="h-full max-h-full border-l inner-borders w-full overflow-hidden relative bg-bg_light dark:bg-bg_dark">
		<Menu />
	</div>

	<!-- ! Response Slot  -->
	<div id="messages-container" slot="messages" class="w-full px-3">
		{#if $messages.length > 0}
			{#each $messages as {role,content}, index}
				{#if content || $responseErrors[index]}
					<Message role={role} content={content} error={$responseErrors[index]}/>
				{/if}
			{/each}
		{/if}

		{#if $messages.length > 0 || $isLoading}
			<div id="bottom-of-response" class="" bind:this={bottomOfResponseRef}/>
		{/if}
	</div>
		
	<!-- ! Footer Slot  -->
	<div slot="shortcuts" >
		<ShortcutContainer />
	</div>
</AppContainer>

<style>
	#messages-container {
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		align-items: flex-start;
	}

	#history-container {
		overflow-y: auto;
		flex-shrink: 0;
		width: 40vw;
		max-width: 400px;
	}
	
	#menu-container {
		position: absolute;
		right: 0;
		overflow-y: auto;
		flex-shrink: 0;
		width: 30vw;
		max-width: 300px;
	}

</style>
