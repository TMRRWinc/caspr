<script lang="ts">
	import { app_pinned } from "$lib/stores/stores";
  import {appWindow} from "@tauri-apps/api/window";

  const handleTogglePinned = () => {
    appWindow.setAlwaysOnTop(!$app_pinned);
    app_pinned.update((pinned) => !pinned);
  }

</script>


<button 
  title={$app_pinned ? "Unpin app window" : "Pin app window"} 
  on:click={handleTogglePinned}
  class="transform hover:scale-105 hover:-rotate-[20deg] transition-transform duration-200 ease-in-out"
>
  {#if $app_pinned}
    <svg 
      xmlns="http://www.w3.org/2000/svg" 
      width="16"
      height="16"
      viewBox="0 0 24 24"
      >
      <path
        class="fill-accent_light dark:fill-accent_dark opacity-80 max-w-full"
        d="M20.9711 17.1715L19.5568 18.5858L16.0223 15.0513L15.9486 15.125L15.2415 18.6605L13.8273 20.0747L9.58466 15.8321L4.63492 20.7818L3.2207 19.3676L8.17045 14.4179L3.92781 10.1752L5.34202 8.76101L8.87756 8.0539L8.95127 7.98019L5.4147 4.44362L6.82892 3.02941L20.9711 17.1715Z M18.8508 12.2228L20.1913 10.8823L20.8984 11.5894L22.3126 10.1752L13.8273 1.68994L12.4131 3.10416L13.1202 3.81126L11.7797 5.15176L18.8508 12.2228Z"
      >
    </path>
  </svg>
  {:else}
    <svg 
      xmlns="http://www.w3.org/2000/svg" 
      width="16"
      height="16"
      viewBox="0 0 24 24"
    >
      <path 
        class="fill-accent_light dark:fill-accent_dark opacity-80 max-w-full"
        d="M22.3126 10.1752L20.8984 11.5894L20.1913 10.8823L15.9486 15.125L15.2415 18.6605L13.8273 20.0747L9.58466 15.8321L4.63492 20.7818L3.2207 19.3676L8.17045 14.4179L3.92781 10.1752L5.34202 8.76101L8.87756 8.0539L13.1202 3.81126L12.4131 3.10416L13.8273 1.68994L22.3126 10.1752Z"
      ></path>
    </svg>
  {/if}
</button>
  