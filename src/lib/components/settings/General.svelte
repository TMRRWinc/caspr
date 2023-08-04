<script lang="ts">
  import { page } from "$app/stores";
	import Button from "../Button.svelte";
	import { invoke } from "@tauri-apps/api";
	import { appWindow } from "@tauri-apps/api/window";

  const showTutorial = () => {
    appWindow.setAlwaysOnTop(false);
    invoke("open_onboarding", { step: 3 });
  }
</script>

<div class="flex flex-col gap-4 min-h-full relative">
  
  <h2 class="border-b inner-borders">General</h2>
  
  <Button attributes={{type: "button", class: "p-1 w-full h-fit"}} on:click={showTutorial}>
    Launch Tutorial
  </Button>

</div>

<style>
  
</style>