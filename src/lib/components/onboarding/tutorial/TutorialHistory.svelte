<script lang="ts">
	import TutorialLayout from "./TutorialLayout.svelte";
  import {os} from "$lib/stores/stores";


</script>

<TutorialLayout subtitle="History">
  <div class="flex flex-col gap-3 h-full overflow-y-auto pr-4">
    <h2>Want to return to a previous conversation? No problem, well, so long as you have not deleted it. (more on that shortly)</h2>
    <ul class="list-disc list-outside ml-5 text-sm font-thin space-y-2">
      <li>To view a list of your previous conversations use the short-cut ctrl/CMD+Y or left click the "Show History" button located at the bottom of the caspr window.</li>
      <li>Use the keyboard up and down arrow keys to navigate to the conversation you would like to load and press enter. Alternatively use your mouse and left click the conversation you would like to load.</li>
      <li>After a conversation is loaded you can interact with it just as you would a new conversation.</li>
      <li>The history panel can be left open or closed, to close the panel use the shortcut ctrl/CMD+Y or click the "Hide History" button.</li>
      <li>To delete a single history, press the delete key while the history is highlighted in the history list or hover your mouse over the conversation listing you would like to delete and left click the garbage can button.</li>
      <li>If you would like to delete all conversation history you can tab to the Clear History button or left click it. The clear history button is located at the bottom of the history list.</li>
  </ul>
  </div>
</TutorialLayout>
