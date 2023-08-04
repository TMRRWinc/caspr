<script lang="ts">
	import Keycap from "../Keycap.svelte";
	import Autostart from "./Autostart.svelte";
</script>

<div class="flex flex-col gap-4">
  <Autostart />
  <div class="flex flex-col gap-2">
    <h2 class="border-b inner-borders">Hotkeys</h2>
    <Keycap shortcutKey="C" text="Open/Close Caspr" useCmdOrCtrl={true} shift />
    <p class="">Hotkey customization coming soon...</p>
  </div>
</div>

<style>


</style>