<script lang="ts">
	import TutorialLayout from "./TutorialLayout.svelte";
  import {os} from "$lib/stores/stores";


</script>

<TutorialLayout subtitle="Launch and Access">
  <div class="flex flex-col gap-3 h-full overflow-y-auto pr-4">
    <p class="text-sm font-thin"> If you enabled autostart in the previous steps, the Caspr app should already be running in the background whenever you start your computer. Otherwise, follow the manual start instructions shown below.</p>
    <h2>Launching Caspr Manually</h2>
    <ol class="list-outside list-decimal text-sm font-thin ml-5">
      {#if ($os === "windows")}
      <li>Click on the "Start" button or press the Windows key on your keyboard.</li>
      <li>Type "Caspr" in the search bar.</li>
      <li>Click on the Caspr app icon from the search results.</li>
      <li>The Caspr app should launch on your Windows computer.</li>
    {:else}
      <li>Click on the Launchpad icon in the Dock (it looks like a rocket ship).</li>
      <li>Search for "Caspr" using the search bar at the top.</li>
      <li>Click on the Caspr app icon from the search results.</li>
      <li>The Caspr app should launch on your Mac computer.</li>
    {/if}
    </ol>

    <div>
      <h2>Bringing Caspr to the foreground</h2>
      <p class="text-sm font-thin">There are two ways to bring Caspr to the foreground:</p>
    </div>

    <ul class="list-outside list-decimal text-sm font-thin ml-5">
      <li>Using the hotkey: Press Ctrl/CMD+Shift+C</li>
      <li>From the system tray:</li>
      <ul>
        {#if ($os === "windows")}
          <li>Left-click the Caspr icon shown in the system tray. You may need to click the up arrow to view hidden icons to find the Caspr icon.</li>
        {:else}
          <li>Left-click the Caspr icon shown on the menu bar.</li>
        {/if}
      </ul>
    </ul>
    
  </div>
</TutorialLayout>
