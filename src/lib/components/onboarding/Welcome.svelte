<script lang="ts">
	import { onMount } from "svelte";
	import GhostButton from "../GhostButton.svelte";
  import Prompt from "../Prompt.svelte";
	import ShortcutContainer from "../ShortcutContainer.svelte";
  import Logo from "../icons/Logo.svelte";

  import { tweened } from 'svelte/motion';
  import { linear } from 'svelte/easing';

  const typeProgress = tweened(0, {
		duration: 4000,
		easing: linear
	});

  let promptToType = '';
  let ghostButton: HTMLButtonElement;
  let promptEl: HTMLInputElement;

  const prompts = [
    "Write a blog post about the best way to make a cup of coffee.",
    "Write an excuse, why I can't attend Janes birthday party.",
    "Create a list of lifestyle changes I could make that would improve my health.",
    "Come up with 10 creative ways to use a paperclip.",
    "What are the best ways to improve my writing skills?",
    "Write a pros and cons list for exposing my personal life on social media.",
    "Am I too smart for my own good? ask me a series of questions to find out.",
    "I am drunk at work, should I ask my boss for a raise?"
  ]

  let promptIndex = 0;

  onMount(() => {
    console.log('mounted')
    prompts.sort(() => Math.random() - 0.5);
    promptToType = prompts[0];
    typeProgress.set(promptToType.length);
  })  

  $: if ($typeProgress) {
    promptEl.scrollLeft = promptEl.clientWidth;
  }

  $: if ($typeProgress > 0 && $typeProgress === promptToType.length) {
    setTimeout(() => {
      typeProgress.set(0);
    }, 2000)
  }

  $: if ($typeProgress === 0) {
    promptIndex = (promptIndex + 1) % prompts.length;
    promptToType = prompts[promptIndex];
    typeProgress.set(promptToType.length);
  }

  let blink = false;

  setInterval(() => {
    blink = !blink;
  }, 375)

</script>

<div id="welcome-content" class="h-full">
  <div class="mt-5">
    <Logo fill="fill-brand-accent-01 dark:fill-accent_dark" size="medium"/>
  </div>
  <div class="flex flex-col  items-center self-start gap-3">
    <h1 class="text-xl">Welcome to Caspr</h1>
    <p class="text-xs font-thin text-center">Thank you for choosing Caspr as your ghostwriting assistant. Our app is designed to help you write better and faster by generating high-quality content that's tailored to your needs.</p>
    <p class="text-xs font-thin text-center">To get started, we recommend that you click the "Get Started" button below. This will take you through a quick tutorial that will show you how to use Caspr and introduce you to some of our most useful keyboard shortcuts. If you prefer, you can skip the tutorial by clicking "Skip"</p>
  </div>
  <div class="w-[90%]">
    <!-- image of some sort here? or just space? -->
    <div id="fake-app" class="rounded-xl border inner-borders pointer-events-none">
      <div class="p-3 typewriter">
        <Prompt bind:inputEl={promptEl} value={promptToType.slice(0, $typeProgress) + `${blink? "|": ""}`} online={true}>
          <GhostButton  on:click={()=>{}} bind:buttonEl={ghostButton} />
        </Prompt>
      </div>
      <div class="border-t inner-borders justify-between pt-[2px] px-[2px]">
        <ShortcutContainer />
      </div>
    </div>
  </div>
</div>

<style>
  #welcome-content {
    place-items: center;
    display: grid;
    grid-template-rows: auto auto 1fr;
    gap: 1rem;
  }

  #fake-app {
    height: 85px;
    width: 100%;
    display: grid;
    grid-template-rows: 6fr 4fr;
    box-shadow: 2px 2px 10px 0px rgba(0,0,0,0.75);
  }

</style>
