<!-- This component is specifically for Command or Control + 1 character type shortcuts. It can be modified to handle more shortcut type later -->
<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
	import { onDestroy, onMount } from "svelte";
  import {register, unregister} from "@tauri-apps/api/globalShortcut";
	import Keycap from "./Keycap.svelte";

  export let shortcutKey: string;
  export let text: string;
  export let useCmdOrCtrl: boolean = true;
  export let onlyWhenVisible: boolean = true;
  export let action: () => void;

  let shortcut = useCmdOrCtrl? `CommandOrControl+${shortcutKey[0].toUpperCase()}` : shortcutKey[0].toUpperCase();
  
  const registerShortcut = async () => {
    if (await appWindow.isVisible()) {
      register(shortcut, () => {
        action();
      });
    }
  }

  // When onlyWhenVisible is set true, Register shortcut only whem window is focused to prevent overlap with other global application shorcuts and
  // prevent the shortcut from being triggered when the window is not visible
  const handleWindowFocus = async () => {
    await registerShortcut();
  }
  const handleWindowBlur = async () => {
    if (onlyWhenVisible) unregister(shortcut);
  }
  
  // Register the shortcut when the component is mounted (used when shortcut is conditionally rendered)
  onMount(async () => {
    await registerShortcut();
  });

  onDestroy(() => {
    // unregister the shortcut when the component is destroyed
    unregister(shortcut); 
  });
  
</script>

<svelte:window on:focus={handleWindowFocus} on:blur={handleWindowBlur} />

<button tabindex="-1" type="button" class="flex after:border-r-2 after:inner-borders after:h-4 items-center last-of-type:after:border-none" on:click={() => action()} >
  <Keycap shortcutKey={shortcutKey} text={text} useCmdOrCtrl={useCmdOrCtrl}/>
</button>

<style>
</style>
