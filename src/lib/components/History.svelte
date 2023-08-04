<script lang='ts'>
	import { deleteAllConversations, deleteConversation, getConversation, getConversations } from "$lib/data/localdb";
	import { currentConversation, messages, isLoading } from "$lib/stores/stores";
	import { afterUpdate, onDestroy, onMount } from "svelte";

  let historyContainer : HTMLDivElement;

  onDestroy(() => {
    document.getElementById('prompt-field')?.focus();
  });

  // when isLoading transitions from true to false reload history
  $: if (!$isLoading) {
    setTimeout(() => {
      conversations = getConversations();
    }, 0);
  }

	let conversations = getConversations();
  
  const loadConversation = async (id: number) => {
    const conversation = await getConversation(id);
    messages.set(conversation.map((m) => ({ content: m.content, role: m.role })));
    currentConversation.set(id);
    const prompt = document.getElementById('prompt-field') as HTMLInputElement;
    prompt.focus();
    prompt.placeholder = "Continue the conversation...";
  }

  const handleKeydown = (e: KeyboardEvent) => {
    if (e.key === "ArrowUp") {
      e.preventDefault();
      const target = e.target as HTMLElement;
      if (target) {
        const prev = target.previousElementSibling as HTMLElement;
        if (prev) {
          prev.focus();
        } else {
          const lastElementChild  : HTMLElement = historyContainer.lastElementChild as HTMLElement;
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
          const firstElementChild  : HTMLElement = historyContainer.firstElementChild as HTMLElement;
          firstElementChild?.focus();
        }
      }
    } else if (e.key === "Delete") {
      const target = e.target as HTMLElement;
      if (target) {
        const id = target.getAttribute("data-id");
        if (id) {
          selectPreviousOrNextElement(target, parseInt(id) === $currentConversation);
          handleDelete(parseInt(id));
        }
      }
      
    }
  };

  const handleDelete = (id: number) => {
    if (id === $currentConversation) {
      currentConversation.set(null);
      messages.set([]);
    }
    deleteConversation(id);
    historyContainer.removeChild(historyContainer.querySelector("[data-id='"+id+"']") as HTMLElement);
  }

  const handleDeleteClick = (event: Event, id: number) => {
    event.stopPropagation();
    if (id === $currentConversation) {
      selectPreviousOrNextElement(historyContainer.querySelector("[data-id='"+id+"']") as HTMLElement, true);
    }
    handleDelete(id);
  }

  const selectPreviousOrNextElement = (target: HTMLElement, load: boolean) => {
    const prev = target.previousElementSibling as HTMLElement;
    if (prev) {
      prev.focus();
      if (load) {
        const prevId = prev.getAttribute("data-id")
        if (prevId) {
          loadConversation(parseInt(prevId));
        }
      }
    } else {
      const next = target.nextElementSibling as HTMLElement;
      if (next) {
        next.focus();
        if (load) {
          const nextId = next.getAttribute("data-id")
          if (nextId) {
            loadConversation(parseInt(nextId));
          }
        }
      } else {
        document.getElementById('prompt-field')?.focus();
      }
    }
  }


  const handleDeleteAll = () => {
    document.getElementById('prompt-field')?.focus();
    deleteAllConversations();
    currentConversation.set(null);
    messages.set([]);
    historyContainer.innerHTML = "";
  }

  afterUpdate(() => {
    if ($currentConversation) {
      const current = historyContainer.querySelector("[data-id='"+$currentConversation+"']") as HTMLElement;
      if (current) {
        current.focus();
      }
    }
  });

</script>

<div class="relative h-full overflow-hidden w-full" on:keydown={handleKeydown}>
  {#await conversations}
    <p class="text-center">loading...</p>
  {:then conversations}
  <div id="history-grid" class="absolute inset-0">
    <div id="history-list-container" bind:this={historyContainer} class="overflow-y-auto overflow-x-hidden w-full h-full">
      {#each conversations as conversation}
        <button 
          tabindex="0" 
          on:click={() => {loadConversation(conversation.id)}} 
          data-id={conversation.id}
          class={`history-item group/history-item w-[98%] relative color-transition text-left overflow-hidden truncate mx-1 px-2 py-1 my-[1px] rounded-md hover:bg-opacity-20 dark:hover:bg-opacity-20 focus:bg-opacity-20 dark:focus:bg-opacity-20 hover:bg-accent_light dark:hover:bg-accent_dark focus:bg-accent_light dark:focus:bg-accent_dark ${conversation.id === $currentConversation && "bg-accent_light bg-opacity-25 dark:bg-accent_dark dark:bg-opacity-25"}`} 
        >
          {conversation.title || "Untitled Conversation"}
          <button title="Delete history record" on:click={(e) => handleDeleteClick(e, conversation.id)} tabindex={-1} class="visible-transition invisible opacity-0 group-hover/history-item:visible group-hover/history-item:opacity-100 group absolute right-2 top-[50%] transform -translate-y-[50%] w-fit p-[2px] rounded-[50%] flex justify-center align-middle border border-border_light hover:border-error_light bg-bg_light dark:hover:border-error_dark dark:bg-bg_dark dark:bg-opacity-80 ">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" class="p-[1px] fill-accent_light group-hover:fill-error_light dark:fill-accent_dark dark:group-hover:fill-error_dark">
              <path fill="none" d="M0 0h24v24H0z"/>
              <path d="M7 4V2h10v2h5v2h-2v15a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V6H2V4h5zM6 6v14h12V6H6zm3 3h2v8H9V9zm4 0h2v8h-2V9z"/>
            </svg>
          </button>
        </button>
      {/each}
    </div>
    <div class="h-fit w-full flex justify-center p-1 border-t inner-borders bg-accent_light bg-opacity-5 dark:bg-accent_dark dark:bg-opacity-5">
      <button on:click={handleDeleteAll} title="Delete all history records" class="group gap-1 flex w-fit text-center text-xs hover:text-error_light focus-within:text-error_light dark:hover:text-error_dark dark:focus-within:text-error_dark">
        Clear History
        <svg tabindex="-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="12" height="12" class="self-center fill-accent_light group-hover:fill-error_light group-focus-within:fill-error_light dark:fill-accent_dark dark:group-hover:fill-error_dark dark:group-focus-within:fill-error_dark">
          <path fill="none" d="M0 0h24v24H0z"/>
          <path d="M7 4V2h10v2h5v2h-2v15a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V6H2V4h5zM6 6v14h12V6H6zm3 3h2v8H9V9zm4 0h2v8h-2V9z"/>
        </svg>
      </button>
    </div>
  </div>
  {/await}
</div>

<style>
  #history-grid {
    display: grid;
    grid-template-rows: 1fr auto;
    width: 100%;
  }
  .color-transition {
    transition: background-color 0.1s ease-in-out;
  }

  .visible-transition {
    transition: opacity 0.2s ease-in-out;
  }
</style>