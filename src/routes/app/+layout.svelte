<script lang="ts">
	import { onDestroy, onMount } from "svelte";
  import {appWindow, PhysicalPosition, PhysicalSize} from "@tauri-apps/api/window";
  import type {Event} from "@tauri-apps/api/event";
	import { invoke } from "@tauri-apps/api";
  import { getMatches } from "@tauri-apps/api/cli";
  import { Store } from "tauri-plugin-store-api";


  // Handle window move event but only act on last event
  let timeout: any;
  
  const handleWindowMoved = (e: Event<PhysicalPosition>) => {
    clearTimeout(timeout);
    timeout = setTimeout(async () => {
      await invoke("window_moved");
    }, 100);
  }

  const handleWindowResized = (e: Event<PhysicalSize>) => {
    clearTimeout(timeout);
    timeout = setTimeout(async () => {
      await invoke("window_resized");
    }, 100);
  }

  onMount(async () => {
    // Check if user has been onboarded
    const store = new Store(".preferences.dat");

    let isOnboarded: boolean = false;
    
    try {
      isOnboarded = await store.get("isOnboarded") || false;

      if (!isOnboarded) {
        await invoke("open_onboarding", {step: 0});
      }
    } catch (e) {
      // if error, user has not save isOnboarded status yet
      await store.set("isOnboarded", false);
    }
    
    // check if the app was launched with the --hide-on-startup flag, show app otherwise
    const matches = await getMatches();
    if (matches.args["--hide-on-startup"] && isOnboarded) {
      if (matches.args["--hide-on-startup"].occurrences === 0) {
        await invoke("show_app");
      }
    }

    const removeOnMoved = await appWindow.onMoved(handleWindowMoved);
    const removeOnResized = await appWindow.onResized(handleWindowResized);
    
    
    onDestroy(() => {
      // remove event listeners
      removeOnMoved();
      removeOnResized();
    });
  });
  
</script>


<slot />

<style>
</style>
