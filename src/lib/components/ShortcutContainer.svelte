
<script lang="ts">
  import Shortcut from "$lib/components/Shortcut.svelte";
  import { isLoading, showHistory, messages, responseErrors, currentConversation, expanded, showMenu } from "$lib/stores/stores";
	import Loading from "$lib/components/Loading.svelte";
	import { completion } from "$lib/data/openAIAPI";
	import MenuIcon from "./icons/MenuIcon.svelte";

  const handleToggleMenu = () => {
    showMenu.update((value) => { 
      if (value) {
				expanded.set($messages.length > 0 || $showHistory);
				document.getElementById('prompt-field')?.focus();
        return false;
      }
      expanded.set(true);
			return true;

    });
  }

  const getSelectedText = () => {
		const selection = window.getSelection();
		if (!selection) return null;
		const selectionText = selection.toString();
		if (!selectionText) return null;
		return selectionText;
	}

	const newPrompt = async () => {
    responseErrors.set({});
    messages.set([]);
    currentConversation.set(null);
    expanded.set($showHistory);
    const prompt = document.getElementById('prompt-field') as HTMLInputElement;
    prompt.focus();
    prompt.placeholder="Type a new prompt...";
  };

  const handleToggleHistory = () => {
    showHistory.update((value) => {
      const show = !value;
      if ($messages.length > 0 || $showMenu) {
        expanded.set(true);
      } else {
        expanded.set(show);
        if (!show) {
          document.getElementById('prompt-field')?.focus();
        } 
      }
      return show
    });  
  };

  const copyResponse = async () => {
		const selection = getSelectedText();
		if (selection) {
			navigator.clipboard.writeText(selection);
			return;
		}
		const lastResponse = $messages.at(-1);
		if (!lastResponse) {
			// TODO: add error message
			return;
		}
		navigator.clipboard.writeText(lastResponse.content);
	};

  const resubmit = async () => {
		const inputValue = $messages.at(-2)?.content;
		if (!inputValue) {
			responseErrors.update(n => {
				if ($messages.length > 0) {
	
					const index = $messages.length - 1;
					return {...n, [index] : "No prompt found to resubmit. Try using a new prompt."}
				}
				return n;
			});
			return;
		}

		// if error exists for the last response, remove it
		if ($messages.length > 0) {
			responseErrors.update(n => {
				const index = $messages.length - 1;
				if (n[index]) {
					const {[index]: _, ...rest} = n;
					return rest;
				}
				return n;
			});
		}
		
		// remove last two messages (prompt and response) TODO: may want to save this to a history for the user to access incase they want the previous response
		if ($messages.length >= 2) messages.update(n => [...n.slice(0, -2)]);
		completion(inputValue);
	};


  	$: historyButtonText = $showHistory? "Hide History" : "Show History";
</script>

<div id="shortcut-keys-container" class="flex" data-tauri-drag-region>
	{#if $isLoading}
    <div class="flex">
      <Loading loading={$isLoading} withMessage={false}/>
    </div>
	{:else}
		{#if $messages.length > 0}
			<Shortcut shortcutKey="R" text="Regenerate" useCmdOrCtrl={true} action={resubmit} onlyWhenVisible={true}/>
			<Shortcut shortcutKey="C" text="Copy" useCmdOrCtrl={true} action={copyResponse} onlyWhenVisible={true}/>
			<Shortcut shortcutKey="X" text="New Conversation" useCmdOrCtrl={true} action={newPrompt} onlyWhenVisible={true}/>
		{/if}
		<Shortcut shortcutKey="Y" text={historyButtonText} useCmdOrCtrl={true} action={handleToggleHistory} onlyWhenVisible={true}/>
	{/if}

	<button class="bg-transparent border-none ml-auto" type="button" on:click={handleToggleMenu}>
		<MenuIcon action={handleToggleMenu}/>
	</button>
</div>

<style>
</style>


