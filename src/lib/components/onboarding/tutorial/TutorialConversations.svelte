<script lang="ts">
	import TutorialLayout from "./TutorialLayout.svelte";
  import {os} from "$lib/stores/stores";


</script>

<TutorialLayout subtitle="Conversations">
  <div class="flex flex-col gap-3 h-full overflow-y-auto pr-4">
    <h2>Start your first conversation</h2>
      <ol class="list-decimal list-outside text-sm font-thin ml-5 space-y-2 mb-2">
        <li>
          Open Caspr and start typing your prompt.
        </li>
        <li>
          Not sure where to start, here are some example prompts:
          <ul class="list-disc list-outside ml-5 text-sm font-extralight">
            <li>What are some effective strategies for brainstorming new ideas for a client's branding campaign?</li>
            <li>How can we improve our social media engagement and increase followers for our clients?</li>
            <li>How can I integrate stripe with my web app? (Example code in Python for Django Rest Framework)</li>
          </ul>
        </li>
        <li>
          To submit your prompt, press enter or left click the caspr logo at the far right of the prompt field.
        </li>
        <li>
          Caspr will briefly process your prompt and begin responding.
        </li>
      </ol>
    <h2>Working with Caspr's response</h2>
      <ul class="list-none text-sm font-thin space-y-2">
        <li>
          <h3>If you are not happy with Caspr's first response, you have a few options:</h3>
          <ul class="list-decimal list-outside text-sm font-thin ml-5 space-y-2 mt-2">
            <li>
              Continue a conversation
              <ol class="list-disc list-outside text-sm font-thin ml-5">
                <li>Caspr maintains context for the duration of the conversation so if you would like to request modification you can type them in the prompt at the top of the caspr window.</li>
                <li>To submit, press enter or left click the caspr logo at the far right of the prompt field.</li>
              </ol>              
            </li>
            <li>
              Generate a new response
              <ol class="list-disc list-outside text-sm font-thin ml-5">
                <li>Use the keyboard short-cut ctrl/CMD+R or left click the Regenerate button located at the bottom of the caspr window.</li>
              </ol>            
            </li>
            <li>
              Start a new conversation
              <ol class="list-disc list-outside text-sm font-thin ml-5">
                <li>
                  Use the hotkey ctrl/CMD+X or left-click the New Conversation button located at the bottom of the Caspr window.
                </li>
              </ol>              
            </li>
          </ul>
        </li>
        <li>
          <h3>Once happy with Caspr's response, here are some ways you can copy it</h3>
          <ul class="list-decimal list-outside text-sm font-thin space-y-2 ml-5 mt-2">
            <li>
              Copy entire response
              <ol class="list-disc list-outside text-sm font-thin ml-5">
                <li>
                  Press ctrl/CMD+C or left click the copy button located at the bottom of the caspr window.
                </li>
              </ol>            
            </li>
            <li>
              Copy part of a response
              <ol class="list-disc list-outside text-sm font-thin ml-5">
                <li>
                  Click and drag to highlight part of the response.
                </li>
                <li>
                  Press ctrl/CMD+C or left click the copy button located at the bottom of the caspr window.
                </li>
              </ol>
            </li>
            <li>
              Copy a codeblock
              <ol class="list-disc list-outside text-sm font-thin ml-5">
                <li>
                  Left click the copy button located at the top right corner of the code block.
                </li>
              </ol>
            </li>
          </ul>
        </li>
      </ul>
  </div>
</TutorialLayout>
