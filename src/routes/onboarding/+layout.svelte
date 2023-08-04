<script>
	import { appWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";


  onMount(() => {
    appWindow.setAlwaysOnTop(true);
  });
  
</script>

<svelte:window on:focus={()=>{appWindow.setAlwaysOnTop(true);}} on:close={()=>{appWindow.setAlwaysOnTop(false);}}/>
<div id="onboarding-container" class="main-container border border-border_light dark:border-border_dark border-opacity-50 dark:border-opacity-50 rounded-2xl p-5" data-tauri-drag-region>
  <slot />
</div>

<style>
  #onboarding-container {
    inset:3px;
    position:absolute;
  }
</style>