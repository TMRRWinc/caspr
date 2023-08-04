<script lang='ts'>
	import PinAppButton from "./PinAppButton.svelte";
	import { invoke } from "@tauri-apps/api";
  import {exit} from "@tauri-apps/api/process";
	import { onMount } from "svelte";

  onMount(()=> {
    const menuGrid : HTMLDivElement = document.querySelector("#menu-grid") as HTMLDivElement;
    menuGrid.focus();
  });

  let menuContainer : HTMLDivElement;

  const handleKeydown = (e: KeyboardEvent) => {
    if (e.key === "ArrowUp") {
      e.preventDefault();
      const target = e.target as HTMLElement;
      if (target) {
        const prev = target.previousElementSibling as HTMLElement;
        if (prev) {
          prev.focus();
        } else {
          const lastElementChild  : HTMLElement = menuContainer.lastElementChild as HTMLElement;
          lastElementChild?.focus();
        }
      }
    } else if (e.key === "ArrowDown") {
      const target = e.target as HTMLElement;
      e.preventDefault();
      if (target) {
        const next = target.nextElementSibling as HTMLElement;
        if (next) {
          next.focus();
        } else {
          const firstElementChild  : HTMLElement = menuContainer.firstElementChild as HTMLElement;
          firstElementChild?.focus();
        }
      }
    }
  };

  const handleQuit = () => {
    exit(0);
  }

  const handleOpenSettings = () => {
    invoke("open_settings");
  }

</script>

<div class="relative h-full overflow-hidden w-full" on:keydown={handleKeydown}>
  <div id="menu-grid" class="absolute inset-0">
    <div id="menu-list-container" bind:this={menuContainer} class="overflow-y-auto overflow-x-hidden h-full flex flex-col">
      <button 
        tabindex="0" 
        on:click={() => handleOpenSettings()}
        class={`history-item group/history-item relative color-transition text-center text-sm mx-1 px-2 py-1 my-[1px] rounded-md hover:bg-opacity-20 dark:hover:bg-opacity-20 focus:bg-opacity-20 dark:focus:bg-opacity-20 hover:bg-accent_light dark:hover:bg-accent_dark focus:bg-accent_light dark:focus:bg-accent_dark `} 
      >
        Open Settings
      </button>
    </div>
    <div class="h-fit w-full flex justify-center p-1 border-t inner-borders bg-accent_light bg-opacity-5 dark:bg-accent_dark dark:bg-opacity-5">
      <button on:click={handleQuit} title="Quit and close caspr application" class="group gap-1 flex w-fit text-center text-xs hover:text-error_light focus-within:text-error_light dark:hover:text-error_dark dark:focus-within:text-error_dark">
        Quit App
      </button>
    </div>
    <div class="h-fit w-full flex justify-end p-1 border-b inner-borders">
      <PinAppButton />
    </div>
  </div>
</div>

<style>
  #menu-grid {
    display: grid;
    grid-template-rows: 1fr auto auto;
  }
  .color-transition {
    transition: background-color 0.1s ease-in-out;
  }

</style>