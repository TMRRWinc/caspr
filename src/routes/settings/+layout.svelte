<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window';
	import General from '$lib/components/settings/General.svelte';
	import Usage from '$lib/components/settings/Usage.svelte';
	import Theme from '$lib/components/settings/Theme.svelte';
	import Preferences from '$lib/components/settings/Preferences.svelte';
	import Api from '$lib/components/settings/Api.svelte';
	import { onMount } from 'svelte';

	$: category = 'general';
	$: currentSetting = handleCategory(category);

	onMount(() => {
		appWindow.setAlwaysOnTop(true);
	});

	// List of settings Tabs
	// Define tab name and component to display
	const settings = [
		{ name: 'General', component: General },
		{ name: 'API', component: Api },
		{ name: 'Usage', component: Usage },
		{ name: 'Theme', component: Theme },
		{ name: 'Preferences', component: Preferences }
	];

	onMount(async () => {
		const path = window.location.pathname.split('/')[2];
		category = path;
	});
	const handleClose = async () => {
		await appWindow.close();
	};

	const handleCategory = (category: string | undefined) => {
		switch (category) {
			case 'general':
				return 0;
			case 'api':
				return 1;
			case 'usage':
				return 2;
			case 'theme':
				return 3;
			case 'preferences':
				return 4;
			default:
				return 0;
		}
	};

	const handleNavigation = (setting: number) => {
		currentSetting = setting;
	};
</script>

<svelte:window
	on:focus={() => {
		appWindow.setAlwaysOnTop(true);
	}}
	on:blur={() => {
		appWindow.setAlwaysOnTop(false);
	}}
/>
<div
	id="settings-container"
	class="main-container border border-border_light dark:border-border_dark border-opacity-50 dark:border-opacity-50 rounded-2xl"
>
	<header
		id="settings-header"
		class="border-b border-border_light dark:border-border_dark border-opacity-50 dark:border-opacity-50"
		data-tauri-drag-region
	>
		<h1 class="text-xl">Settings: {settings[currentSetting].name}</h1>
		<button on:click={handleClose} class="opacity-50 hover:opacity-100">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-6 stroke-accent_light dark:stroke-accent_dark"
				viewBox="0 0 24 24"
				stroke=""
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M6 18L18 6M6 6l12 12"
				/>
			</svg>
		</button>
	</header>
	<nav
		id="settings-nav"
		class="border-r border-border_light dark:border-border_dark border-opacity-50 dark:border-opacity-50"
	>
		<ul class="">
			{#each settings as setting, i (setting.name)}
				<li>
					<a
						href={`/settings/${setting.name.toLowerCase()}`}
						on:click={(e) => handleNavigation(i)}
						class={`block color-transition w-full py-3 px-4 hover:bg-opacity-20 hover:dark:bg-opacity-20 hover:bg-accent_light hover:dark:bg-accent_dark ${
							i === currentSetting &&
							'bg-accent_light bg-opacity-25 dark:bg-accent_dark dark:bg-opacity-25'
						}`}
					>
						{setting.name}
					</a>
				</li>
			{/each}
		</ul>
	</nav>
	<main id="settings-main">
		<slot />
	</main>
</div>

<style>
	#settings-container {
		display: grid;
		grid-template-columns: 1fr 2fr;
		grid-template-rows: 1fr 8fr;
		grid-template-areas:
			'header header'
			'nav main';

		overflow: hidden;
		inset: 5px;
		position: absolute;
	}
	#settings-header {
		grid-area: header;
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 100%;
		max-height: 100%;
		padding: 1rem;
	}
	#settings-nav {
		grid-area: nav;
		height: 100%;
		max-height: 100%;
		overflow-x: auto;
	}

	.color-transition {
		transition: background-color 0.2s ease-in-out;
	}

	#settings-main {
		grid-area: main;
		height: 100%;
		max-height: 100%;
		overflow-x: auto;
		padding: 1rem;
	}
</style>
