<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';

  // This function is called when the header (but not the drag handle) is double-clicked
  async function handleDoubleClick() {
    const appWindow = getCurrentWindow();
    const isVisible = await appWindow.isVisible();
    if (isVisible) {
      await appWindow.hide();
    } else {
      await appWindow.show();
      await appWindow.setFocus();
    }
  }

  // --- Mock Data for the UI ---
  // In a real app, this would come from a store or an API call.
  const todos = [
    { id: 1, text: "niche: ml/mlops + fullstack => deep-ml + geohot | langchain/rag/finetuning/ai agents/dag/knowledgegraphs/parallel/hybrid agents courses/workflow + full-stack project | klonos | medusa-tavern | finetuned manim/gsap" },
    { id: 2, text: "Qwenlo (ui adjustment) | current dev : twitter bot to post tweets | finetune | rag | deploy | chroma db | add mcp" },
    { id: 3, text: "train info deploy on custom mcp server | payment integration for ticket booking | blazing fast caching practice | mcp servers in detail (implement in web3 in coding tools and other things)" },
    { id: 4, text: "Most ai related tech basic implementation" }
  ];
</script>

<div id="app">
  <!-- Header is for double-click toggle, drag-indicator is for dragging -->
  <header class="app-header" role="button" tabindex="0" on:dblclick={handleDoubleClick}>
    <!-- This small div is the ONLY thing responsible for dragging -->
    <div class="drag-indicator" data-tauri-drag-region></div>
    <div class="header-content">
      <h1>gotta do</h1>
      <span class="timer">00:00:00</span>
    </div>
    <button class="add-btn" aria-label="Add new to-do">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
        <line x1="12" y1="5" x2="12" y2="19"></line>
        <line x1="5" y1="12" x2="19" y2="12"></line>
      </svg>
    </button>
  </header>

  <main class="todo-list">
    {#each todos as todo (todo.id)}
      <div class="todo-item">
        <div class="checkbox"></div>
        <span class="text">{todo.text}</span>
      </div>
    {/each}
  </main>
</div>

<style>
  /* The main #app container styling is now in the global CSS file */

  .app-header {
    position: relative; /* For the drag indicator positioning */
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px 8px;
    flex-shrink: 0; /* Prevents the header from shrinking */
  }

  .drag-indicator {
    position: absolute;
    top: 6px;
    left: 50%;
    transform: translateX(-50%);
    width: 40px;
    height: 4px;
    background-color: rgba(255, 255, 255, 0.3);
    border-radius: 2px;
    cursor: move; /* Cursor shows this is the draggable area */
  }

  .header-content {
    display: flex;
    align-items: baseline;
    gap: 12px;
  }

  .header-content h1 {
    font-size: 20px;
    font-weight: 600;
    margin: 0;
  }

  .timer {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.6);
    font-family: monospace;
  }

  .add-btn {
    background: var(--neon-green);
    color: black;
    border: none;
    border-radius: 50%;
    width: 28px;
    height: 28px;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    padding: 0;
    transition: all 0.2s ease;
  }

  .add-btn:hover {
    transform: scale(1.1);
    box-shadow: 0 4px 12px rgba(57, 255, 20, 0.3);
  }

  .todo-list {
    flex-grow: 1; /* Allows the list to take up remaining space */
    overflow-y: auto; /* Adds a scrollbar if content overflows */
    padding: 0 4px 0 16px; /* Padding for the list items */
  }

  /* Custom subtle scrollbar */
  .todo-list::-webkit-scrollbar {
    width: 4px;
  }
  .todo-list::-webkit-scrollbar-track {
    background: transparent;
  }
  .todo-list::-webkit-scrollbar-thumb {
    background-color: rgba(255, 255, 255, 0.2);
    border-radius: 2px;
  }

  .todo-item {
    display: flex;
    align-items: flex-start; /* Align to the top */
    padding: 12px 0;
    border-bottom: 1px solid var(--grey-line);
    line-height: 1.4;
  }
  
  .todo-item:last-child {
    border-bottom: none;
  }

  .checkbox {
    width: 20px;
    height: 20px;
    border: 2px solid var(--grey-line);
    border-radius: 50%;
    margin-right: 15px;
    flex-shrink: 0;
    margin-top: 1px; /* Align with the text */
  }

  .text {
    color: rgba(255, 255, 255, 0.9);
  }
</style>
