<script lang="ts">
	import { hideSuggestions } from '../stores/stores';
	export let inputEl: HTMLInputElement;
	export let value: string;
	export let online: boolean;

	$: isDisabled = !online;
</script>

<div id="prompt-container" class:disabled={isDisabled} >
	<input
		inputmode="text"
		autocomplete="off"
		id="prompt-field"
		class=""
		type="text"
		placeholder={online ? "Type a new prompt..." : "You're offline! Previous conversations are still available."}
		disabled={isDisabled}
		bind:value
		bind:this={inputEl}
		on:keydown
		on:focus={() => {
			hideSuggestions.set(true);
		}}
		on:blur={() => {
			hideSuggestions.set(false);
		}}
		
	/>
	<slot />
</div>

<style>
#prompt-container {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-content: center;
		margin: 0;
	}

	#prompt-field {
		background-color: transparent;
		width: 100%;
		padding-right: 10px;
	}

#prompt-container.disabled > #prompt-field {
		pointer-events: none;
		opacity: 0.5;
	}
</style>
