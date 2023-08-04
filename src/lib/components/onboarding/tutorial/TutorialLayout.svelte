<script lang="ts">
  export let subtitle: string | undefined = undefined;
</script>

<div id="how-to-content">
  <h1 class="text-xl">Tutorial{subtitle? `: ${subtitle}` : ""}</h1>
  <div class="flex flex-col gap-3 h-full w-full overflow-y-auto pr-4">
    <slot />
  </div>
</div>

<style>
  #how-to-content {
    padding-top: 2rem;
    place-items: center;
  
    display: grid;
    grid-template-rows: auto 1fr;
    gap: 1rem;

    width: 95%;
    height: 100%;

    max-height: 100%;
    overflow: hidden;
  }
</style>