<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { onMount } from "svelte";
	import { Store } from "tauri-plugin-store-api";

	const store = new Store(".preferences.dat");
    let color = ''
    
    onMount(async () => {
        let themePref: "light" | "dark" | "system" | null = await store.get("theme");
        if (themePref) {
            if (themePref === "dark") {
                color = '#fff'
            } else {
                color = '#000'
            }
        } else {
            color = '#000'
        }
    });
</script>
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="-40 0 512 512" width="24px">
    <path
        d="m271 512h-191c-44.113281 0-80-35.886719-80-80v-271c0-44.113281 35.886719-80 80-80h191c44.113281 0 80 35.886719 80 80v271c0 44.113281-35.886719 80-80 80zm-191-391c-22.054688 0-40 17.945312-40 40v271c0 22.054688 17.945312 40 40 40h191c22.054688 0 40-17.945312 40-40v-271c0-22.054688-17.945312-40-40-40zm351 261v-302c0-44.113281-35.886719-80-80-80h-222c-11.046875 0-20 8.953125-20 20s8.953125 20 20 20h222c22.054688 0 40 17.945312 40 40v302c0 11.046875 8.953125 20 20 20s20-8.953125 20-20zm0 0"
        fill={color}
    />
</svg>