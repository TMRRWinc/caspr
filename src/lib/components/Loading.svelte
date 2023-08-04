<script lang="ts">
	import { LottiePlayer } from '@lottiefiles/svelte-lottie-player';
	import data from '../icons/7uME5L4OKV.json';
	export let loading = false;
	export let withMessage = true;
	export let customMessage = '';

	const loadingMessage = [
		"Loading words of wisdom... or not, we'll see.",
		"Writing something witty... or not, let's hope for the best.",
		'Composing a masterpiece... one typo at a time.',
		'Crafting a joke... no promises.',
		'Putting together some words... and hoping they make sense.',
		'Spinning a yarn... or just getting tangled up in my own words.',
		'Generating text... hold on, my keyboard is warming up.',
		'Loading inspiration... or maybe just a random word generator.',
		'Brewing a steam of consciousness... or just dribbling nonsense.',
		'Manifesting some creativity... or just making faces at the screen.',
		'Hatching a meaningful thought... or just laying an egg.',
		'Summoning my inner poet... or just Googling quotes.',
		'Gathering my thoughts... or just making things up as I go'
	];
</script>

<div class={`loader ${!loading && "hidden"}`}>
	<LottiePlayer src={data} autoplay={true} loop={true} width={32} />
	{#if withMessage}
		<p>{loadingMessage[Math.floor(Math.random() * loadingMessage.length)]}</p>
	{:else}
		<p>{customMessage}</p>
	{/if}
</div>


<style>
	.loader {
		display: flex;
		flex-direction: row;
		gap: 8px;
	}

	.hidden {
		display: none;
	}

	p {
		color: #777380;
		font-size: 16px;
		font-family: 'Segma-Regular';
	}
</style>
