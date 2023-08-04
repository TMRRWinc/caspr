<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements"
  export let attributes: HTMLButtonAttributes = {};
  export let buttonStyle: "standard" | "plain" = "standard";

  let { class: buttonClass, ...rest } = attributes;
  const buttonStyles = {
    standard: "p-3 rounded-xl  bg-accent_light dark:bg-accent_dark bg-opacity-20 dark:bg-opacity-5 hover:bg-opacity-25 hover:dark:bg-opacity-10",
    plain: "p-3 rounded-xl hover:bg-opacity-5 hover:dark:bg-opacity-5 hover:bg-accent_light dark:hover:bg-accent_dark",
  }

</script>

<button
  class={buttonStyles[buttonStyle] + " " + buttonClass + " focus:border-2 focus:border-accent_light/20 dark:focus:border-accent_dark/20"}
  {...rest}
  on:click
  title={attributes.title}
>
  <slot/>
</button>
