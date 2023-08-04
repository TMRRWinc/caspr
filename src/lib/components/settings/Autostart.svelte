<script lang="ts">
	import { onMount } from "svelte";
  import {disable as disableAS, enable as enableAS, isEnabled as isEnabledAS} from "tauri-plugin-autostart-api";
	import Button from "../Button.svelte";
  
  // Autostart Logic
  $: isAutostart = false;
  $: autoStartLoading = false;

  const isAutostartEnabled = async () => {
    autoStartLoading = true;
    isAutostart = await isEnabledAS();
    autoStartLoading = false;
  }

  const enableAutostart = async () => {
    await enableAS();
    isAutostartEnabled();
  }
  
  const disableAutostart = async () => {
    await disableAS();
    isAutostartEnabled();
  }

  // Lifecycle Hooks
  onMount(async () => {
    isAutostartEnabled();
  })

</script>

<div class="flex flex-col gap-2">
  <h2 class="border-b inner-borders">Autostart: {autoStartLoading? "-" : isAutostart? "Enabled" : "Disabled"}</h2>
  {#if isAutostart}
  <Button  on:click={disableAutostart} attributes={{class: "p-1"}}>Disable Autostart</Button>
  {:else}
  <Button  on:click={enableAutostart} attributes={{class: "p-1"}}>Enable Autostart</Button>
  {/if}
</div>

<style>


</style>