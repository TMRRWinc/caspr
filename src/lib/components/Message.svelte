<script lang="ts">
  export let content: string;
  export let role: string;
  export let error: string | null | undefined;
	import { theme } from "$lib/stores/stores";
  import {unified, type Transformer} from 'unified';
  import dockerfile from "highlight.js/lib/languages/dockerfile";
  import remarkParse from 'remark-parse';
  import remarkRehype from 'remark-rehype';
  import rehypeStringify from 'rehype-stringify';
  import rehypeHighlight from 'rehype-highlight';
  import remarkCodeExtra from 'remark-code-extra';
	import { afterUpdate, beforeUpdate, onDestroy } from "svelte";
 
  let errors: string[]= [];
	$: if (error) {
		error.split('\n').forEach((line) => {
			errors.push(line);
		});
	}

  let msgContainer: HTMLDivElement;

  const copyHandler = (e: Event) => {
    const currentTarget = e.currentTarget as HTMLElement;
    if (!currentTarget) return;
    
    const code = currentTarget.parentElement?.parentElement?.querySelector("code");
    if (!code) return;

    navigator.clipboard.writeText(code.innerText);
  }

  const wrapHandler = (e: Event) => {
    const currentTarget = e.currentTarget as HTMLElement;
    if (!currentTarget) return;
    
    const code = currentTarget.parentElement?.parentElement?.querySelector("code");
    if (!code) return;

    code.classList.toggle("wrap");
  }

 

  afterUpdate(() => {
    const copyButtons = msgContainer.querySelectorAll(".code-copy-button");
    copyButtons.forEach((button) => {
      button.addEventListener("click", copyHandler);
    })

    const wrapButtons = msgContainer.querySelectorAll(".code-wrap-button");
    wrapButtons.forEach((button) => {
      button.addEventListener("click", wrapHandler);
    })
  })

  onDestroy(() => {
    const buttons = msgContainer.querySelectorAll(".code-copy-button");
    buttons.forEach((button) => {
      button.removeEventListener("click", copyHandler);
    })
  })

  const setContent = async (content: string) => {
    const file = await unified()
      .use(remarkParse)
      .use(remarkCodeExtra, {
        transform: (node: any) => ({
          before: [
            
            
          {
            type: 'element',
            tagName: 'div',
            properties: {
              class: "code-buttons"
            },
            children: [
              {
          type: 'element',
          tagName: 'button',
          properties: {
            class: "code-wrap-button",
            title: "Wrap code"
          },
          children: [{
            type: 'element',
            tagName: 'svg',
            properties: {
              xmlns: "http://www.w3.org/2000/svg",
              width: "20",
              height: "20",
              viewBox: "0 0 24 24",
              class: "code-wrap-icon"
            },
            children: [{
              type: 'element',
              tagName: 'path',
              properties: {
                d: "M15 18H16.5C17.8807 18 19 16.8807 19 15.5C19 14.1193 17.8807 13 16.5 13H3V11H16.5C18.9853 11 21 13.0147 21 15.5C21 17.9853 18.9853 20 16.5 20H15V22L11 19L15 16V18ZM3 4H21V6H3V4ZM9 18V20H3V18H9Z",
              }
            }]
          }]
        },
              {
          type: 'element',
          tagName: 'button',
          properties: {
            class: "code-copy-button",
            title: "Copy to clipboard"
          },
          children: [{
            type: 'element',
            tagName: 'svg',
            properties: {
              xmlns: "http://www.w3.org/2000/svg",
              width: "20",
              height: "20",
              viewBox: "0 0 256 256",
              class: "code-copy-icon"
            },
            children: [{
              type: 'element',
              tagName: 'path',
              properties: {
                d: "M216 28H88a12 12 0 0 0-12 12v36H40a12 12 0 0 0-12 12v128a12 12 0 0 0 12 12h128a12 12 0 0 0 12-12v-36h36a12 12 0 0 0 12-12V40a12 12 0 0 0-12-12Zm-60 176H52V100h104Zm48-48h-24V88a12 12 0 0 0-12-12h-68V52h104Z",
                
              }
            }]
          }]
        },
            ]
          } 
          ]
        })
      })
      .use(remarkRehype)
      .use(rehypeStringify)
      .use(()=>rehypeHighlight({detect: true, languages: {dockerfile}, ignoreMissing: true}))
      .process(content);
    msgContainer.innerHTML = String(file);
  }

  $: setContent(content);
  
</script>

<div class={`message-container ${role} ${$theme === "dark"? "dark" : "light"}`}>
    <div bind:this={msgContainer}/>
    {#if error}
      <div class="">
        {#each errors as error}
          <p class="error">{error}</p>
        {/each}
      </div>
    {/if}
</div>

<style>
  .message-container {
    padding: 6px 0;
    justify-content: flex-start;
    width: 100%;
  }

  .message-container.light {
    @import 'node_modules/highlight.js/styles/github.css';
  }
  .message-container.dark {
    @import 'node_modules/highlight.js/styles/github-dark-dimmed.css';
  }

</style>