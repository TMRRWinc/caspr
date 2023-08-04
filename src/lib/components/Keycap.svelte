<script lang="ts">
     import { os } from "$lib/stores/stores";

export let shortcutKey: string;
export let text: string;
export let useCmdOrCtrl: boolean = true;
export let shift: boolean = false;

export let useHoverStyle = true;

</script>

<button type="button" class={`keycap-container focus:bg-opacity-20 focus:bg-accent_light ${useHoverStyle && "hover:bg-opacity-20 hover:dark:bg-opacity-20 hover:bg-accent_light hover:dark:bg-accent_dark"}`}>
  {#if useCmdOrCtrl}
    {#if $os === "macos"}
      <div class="keycap bg-accent_light dark:bg-accent_dark bg-opacity-20 dark:bg-opacity-20 text-text-medium-contrast_light dark:text-text-medium-contrast_dark">⌘</div>
    {:else}
      <div class="keycap bg-accent_light dark:bg-accent_dark bg-opacity-20 dark:bg-opacity-20 text-text-medium-contrast_light dark:text-text-medium-contrast_dark">ctrl</div>
    {/if}
  {/if}
  {#if shift}
    <div class="keycap bg-accent_light dark:bg-accent_dark bg-opacity-20 dark:bg-opacity-20 text-text-medium-contrast_light dark:text-text-medium-contrast_dark">⇧</div>
  {/if}
  <div>

    <div class={`keycap bg-accent_light dark:bg-accent_dark bg-opacity-20 dark:bg-opacity-20 text-text-medium-contrast_light dark:text-text-medium-contrast_dark`}>{shortcutKey}</div>
  </div>
  
  {text}
</button>
  
<style lang="postcss">
  .keycap-container {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    gap: 4px;
    border-radius: 6px;
    padding: 4px 8px;
    font-size: 12px;
  }

  .keycap {
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 4px;
    min-width: 18px;
    width: fit-content;
    padding: 2px;
    font-size: 10px;
  }
</style>
