<script lang="ts">
	import TutorialLayout from "./TutorialLayout.svelte";
  import {os} from "$lib/stores/stores";


</script>

<TutorialLayout subtitle="Quick Menu">
  <div class="flex flex-col gap-3 h-full overflow-y-auto pr-4">
    <p class="text-sm font-thin">The quick menu in Caspr provides a way to easily modify window behavior and access important options such as settings and quitting the app.</p>
    <h2>About the quick menu:</h2>
    <ul class="list-disc list-outside ml-5 text-sm font-thin space-y-2">
      <li>To show the quick menu, use the short-cut ctrl/CMD+shift+S or left click the caspr settings icon located in the bottom right of the caspr window.</li>
      <li>At the bottom of the quick menu there is a series of icon buttons that can be used to quickly modify the caspr window behavior.</li>
      <li>
        Actions available in the quick menu:
        <ul class="list-disc list-outside ml-5 text-sm font-thin space-y-2">
          <li>Pin window - this setting prevents the caspr window from hiding when click outside it and keeps it on top of all other windows.</li>
          <li>Open settings</li>
          <li>Quit Caspr</li>
        </ul>
      </li>
      <li>
        The quick menu in future versions of Caspr will be expanded to include context specific shortcuts.
      </li>
    </ul>


  </div>
</TutorialLayout>
