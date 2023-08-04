<script lang="ts">
	import { expanded, showHistory, showMenu } from "$lib/stores/stores";
	import { invoke } from "@tauri-apps/api";

	let container: HTMLDivElement;

	// invoke the tauri set_expand method to expand the app window 
	// add/remove closed class to prevent a double border when app window is closed
	$: if ($expanded) {
		invoke("set_expand", {open: true});
		container.classList.remove("closed");
	} else {
		invoke("set_expand", {open: false});
		if (container) {
			container.classList.add("closed");
		}
	}


</script>

<div id="app-container" class="main-container closed" bind:this={container} >
	<div id="prompt-wrapper"  class="border-b inner-borders" data-tauri-drag-region>
		<slot name="prompt" />
	</div>

	<div id="messages-wrapper" class="relative">
		{#if $showHistory}
			<slot name="history" />
		{/if}
		<slot name="messages" />
		{#if $showMenu}
			<slot name="menu" />
		{/if}
	</div>
	<div id="shortcuts-wrapper" class="border-t inner-borders">
		<slot name="shortcuts" />
	</div>
</div>

<style>
	#app-container {
		flex-direction: column;
		position: absolute;
		inset: 0;

		display: grid;
		grid-template-rows: min-content auto min-content;
	}
	
	#app-container.closed {
		grid-template-rows: min-content 0 min-content;
	}

	#prompt-wrapper {
		padding: 16px;
	}

	#messages-wrapper {
		overflow-y: auto;
		display: flex;
	}
	#shortcuts-wrapper {
		padding: 4px;
	}
</style>
