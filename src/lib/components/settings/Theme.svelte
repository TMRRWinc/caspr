<script lang="ts">
  import { emit } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
	import Button from "../Button.svelte";
  import { Store } from "tauri-plugin-store-api";
	import { invoke } from "@tauri-apps/api";

  let theme: "light" | "dark" | "system" = "system";

  const store = new Store(".preferences.dat");

  const addDarkClass = async () => {
    document.body.classList.add("dark");
    emit("theme-changed", { theme: "dark" });
    await store.set("theme", "dark");
    await store.save();
  };
  
  const removeDarkClass = async () => {
    document.body.classList.remove("dark");
    emit("theme-changed", { theme: "light" });
    await store.set("theme", "light");
    await store.save();
  };

  const handleThemeChange = async (theme: "light" | "dark" | "system") => {
    if (theme === "dark") {
      await addDarkClass();
    } else if (theme === "light") {
      await removeDarkClass();
    } else if (theme === "system") {
      try {
        const osTheme = await invoke("get_os_theme");
        if (osTheme === "dark") {
          await addDarkClass();
        } else {
          await removeDarkClass();
        }
      } catch (error) {
        console.error(error);
      }
    }
  };

  onMount(async () => {
    let themePref: "light" | "dark" | "system" | null = await store.get("theme");
    if (themePref) {
      theme = themePref;
    } 
  });
  
  // Passes button click to the label
  const handleButtonClick = (e: Event) => {
    const a = e.currentTarget as HTMLButtonElement;
    const parentNode = a.parentNode as HTMLLabelElement;
    parentNode.click();
  };

  $: handleThemeChange(theme);

</script>

<div class="flex flex-col gap-4">
  <div class="flex flex-col gap-2">
    <h2 class="border-b inner-borders">Theme</h2>
    <label for="light-mode">
      <Button attributes={{type: "button", class: "p-1 relative w-full"}} on:click={handleButtonClick} >
        <input type="radio" id="light-mode" name="theme" value="light" class="absolute left-2 top-[50%] transform -translate-y-[50%]" bind:group={theme} />
        Light Mode
      </Button>
    </label>
    <label for="dark-mode">
      <Button attributes={{type: "button", class: "p-1 relative w-full h-fit"}} on:click={handleButtonClick}>
        <input type="radio" id="dark-mode" name="theme" value="dark" class="absolute left-2 top-[50%] transform -translate-y-[50%]" bind:group={theme}  />
        Dark Mode
      </Button>
    </label>
    <label for="system-mode">
      <Button attributes={{type: "button", class: "p-1 relative w-full h-fit"}} on:click={handleButtonClick}>
        <input type="radio" id="system-mode" name="theme" value="system" class="absolute left-2 top-[50%] transform -translate-y-[50%]" bind:group={theme}  />
        System Mode
      </Button>
    </label>
  </div>
</div>

<style>
  
</style>