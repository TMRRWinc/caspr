<script lang="ts">
	import Keycap from "../Keycap.svelte";
	import Autostart from "../settings/Autostart.svelte";
	import Theme from "../settings/Theme.svelte";

  const shortcuts = [
    {
      key: "C",
      cmd: true,
      shift: true,
      text: "Launch Caspr App"
    },
    {
      key: "S",
      cmd: true,
      shift: true,
      text: "Show Quick Menu"
    },
  ]
</script>

<div id="shortcuts-content">
  <h1 class="text-xl block">Setup: Customize how Caspr looks and works</h1>
  <div class="self-start w-full px-2 max-h-full overflow-y-auto">
    <table id="shortcuts-table" class="table-auto w-full text-center ml-auto mr-auto">
      <thead>
        <tr class="border-b inner-borders">
          <th>Hotkeys</th>
          <th>Action</th>
        </tr>
      </thead>
      <tbody class="">
        {#each shortcuts as shortcut}
          <tr>
            <td><Keycap useHoverStyle={false} useCmdOrCtrl={shortcut.cmd} shift={shortcut.shift} shortcutKey={shortcut.key} text=""/></td>
            <td>{shortcut.text}</td>
          </tr>
        {/each}
      </tbody>
    </table>
    <div class="mt-5">
      <Theme />
    </div>
    <div class="mt-5">
      <Autostart />
    </div>
  </div>
</div>

<style>
  #shortcuts-content {
    padding-top: 2rem;
    place-items: flex-start;
  
    display: grid;
    grid-template-rows: auto 1fr;
    gap: 1rem;

    max-width: 450px;
    width: 100%;
    height: 100%;
    max-height: 100%;
    overflow: hidden;
  }

  #shortcuts-table td, #shortcuts-table th {
    /* center content */
    text-align: left;
    align-content: center;
  }

  /* :global(#shortcuts-table td button) {
    margin-left: auto;
    margin-right: auto;
  } */

</style>