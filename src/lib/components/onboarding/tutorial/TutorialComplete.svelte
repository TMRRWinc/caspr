<script lang="ts">
	import TutorialLayout from "./TutorialLayout.svelte";
  import {os} from "$lib/stores/stores";


</script>

<TutorialLayout subtitle="All Set">
  <div class="flex flex-col gap-3 h-full overflow-y-auto pr-4">
    <p class="text-sm font-thin">Congratulations on completing the Caspr tutorial!</p>
    <p class="text-sm font-thin">You are now ready to enjoy all the benefits of using Caspr as your ghostwriting assistant. From helping you generate new ideas to assisting with the writing process, Caspr is here to help you become a more efficient and effective writer.</p>
    <p class="text-sm font-thin">Don't forget to check out the quick menu and settings to further customize your experience.</p>
    <p class="text-sm font-thin">Happy writing!</p>
  </div>
</TutorialLayout>
