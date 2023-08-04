<script lang="ts">
	import TutorialIntro from "$lib/components/onboarding/tutorial/TutorialIntro.svelte";
	import Welcome from "$lib/components/onboarding/Welcome.svelte";
  import Setup from "$lib/components/onboarding/Setup.svelte";
	import Button from "$lib/components/Button.svelte";
	
	import { invoke } from "@tauri-apps/api";
	
	import TutorialLaunch from "$lib/components/onboarding/tutorial/TutorialLaunch.svelte";
  import { listen } from "@tauri-apps/api/event";
	import TutorialConversations from "$lib/components/onboarding/tutorial/TutorialConversations.svelte";
	import TutorialHistory from "$lib/components/onboarding/tutorial/TutorialHistory.svelte";
	import TutorialQuickMenu from "$lib/components/onboarding/tutorial/TutorialQuickMenu.svelte";
	import TutorialSettings from "$lib/components/onboarding/tutorial/TutorialSettings.svelte";
	import TutorialComplete from "$lib/components/onboarding/tutorial/TutorialComplete.svelte";
	import { appWindow } from "@tauri-apps/api/window";

  import { Store } from "tauri-plugin-store-api";

  import { ask, confirm } from '@tauri-apps/api/dialog';

  const store = new Store(".preferences.dat");

  // get query params
  const params = new URLSearchParams(window.location.search);
  const step = params.get("step");

  $: currentPage = step ? parseInt(step) : 0;
  
  let nextLoading = false;

  $: if (currentPage) {
    nextLoading = false;
  }

  const previous = () => {
    currentPage = currentPage - 1;
  }
  
  const next = () => {
    currentPage = currentPage + 1;
  }
  
  const close = () => {
    appWindow.close();
  };
  
  const openApp = () => {
    invoke("show_app");
  }
  
  const setOnboardingDone = async () => {
    await store.set("isOnboarded", true);
    store.save();
    close();
    openApp();
  }

  const handleSkip = async () => {
    // check if user wants to exit tutorial
    const exit = await confirm("Are you sure you want to exit the tutorial?");
    if (!exit) {
      return;
    }
    const showAgain = await ask("Would you like to see our quick tutorial when you open the app next time?");
    if (!showAgain) setOnboardingDone();
    close();
    openApp();
  }

  const pages = [Welcome, Setup, TutorialIntro, TutorialLaunch, TutorialConversations, TutorialHistory, TutorialQuickMenu, TutorialSettings, TutorialComplete]

  $: nextFunctions = [
    {title: "Get Started", function: next, hint: "Start Tutorial"},
    {title: "Continue", function: next, hint: "Continue to next page of tutorial"},
    {title: "Continue", function: next, hint: "Continue to next page of tutorial"},
    {title: "Continue", function: next, hint: "Continue to next page of tutorial"},
    {title: "Continue", function: next, hint: "Continue to next page of tutorial"},
    {title: "Continue", function: next, hint: "Continue to next page of tutorial"},
    {title: "Continue", function: next, hint: "Continue to next page of tutorial"},
    {title: "Launch Caspr", function: setOnboardingDone, hint: "Launch caspr app"},
  ]
  
  const previousFunctions = [
    {title: "Skip", function: handleSkip, hint: "Skip tutorial and launch app"},
    {title: "Previous", function: previous, hint: "Return to previous page of tutorial"},
    {title: "Previous", function: previous, hint: "Return to previous page of tutorial"},
    {title: "Previous", function: previous, hint: "Return to previous page of tutorial"},
    {title: "Previous", function: previous, hint: "Return to previous page of tutorial"},
    {title: "Previous", function: previous, hint: "Return to previous page of tutorial"},
    {title: "Previous", function: previous, hint: "Return to previous page of tutorial"},
    {title: "Previous", function: previous, hint: "Return to previous page of tutorial"},
  ]

  const handleNext = () => {
    nextFunctions[currentPage].function();
  }

  const handlePrevious = () => {
    previousFunctions[currentPage].function();
  }

  const handleClose = () => {
    close();
    openApp();
  };

</script>

<div id="onboarding-content" class="relative">
    <svelte:component this={pages[currentPage]} />
    <div class="w-full flex justify-between">
      <Button on:click={handlePrevious}  buttonStyle="plain" attributes={{class: "min-w-[130px]", title:previousFunctions[currentPage].hint}}>{previousFunctions[currentPage].title}</Button>
      <Button on:click={handleNext}  buttonStyle="standard" attributes={{class: "min-w-[130px]", title:nextFunctions[currentPage].hint}}>{nextFunctions[currentPage].title}</Button>
    </div>
    <button on:click={handleClose} class="opacity-50 hover:opacity-100 absolute top-0 right-0">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-6 stroke-accent_light dark:stroke-accent_dark" viewBox="0 0 24 24" stroke="">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
</div>

<style>
  #onboarding-content {
    display: grid;
    place-items: center;

    grid-template-rows: 1fr auto;
    gap: 1rem;

    height: 100%;
    max-height: 100%;
  }
</style>
