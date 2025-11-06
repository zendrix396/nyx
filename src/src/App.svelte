<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount, onDestroy } from 'svelte';

  let goal = '';
  const appWindow = getCurrentWindow();

  // Submitting the goal
  function handleSubmit() {
    if (!goal.trim()) return;
    console.log('Goal submitted:', goal);
    // TODO: Send goal to orchestrator
    goal = '';
  }

  // Handle Enter key for submission
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSubmit();
    }
  }

  onMount(() => {
    // Set always on top by default (like Krona-Lite)
    appWindow.setAlwaysOnTop(true).catch((error) => {
      console.error("Failed to set always-on-top:", error);
    });

    // CRITICAL: Prevent window from hiding on blur/unfocus  
    // Only F4 should hide the window (handled by Rust backend)
    // Strategy: Immediately re-show if hidden, but F4 will hide it again instantly
    // This creates a brief flash but keeps window visible for non-F4 hides
    let wasVisible = true;
    
    const preventUnexpectedHide = async () => {
      try {
        const isVisible = await appWindow.isVisible();
        
        // If window was visible and now it's hidden, show it immediately
        // F4 will hide it again immediately after, so it stays hidden
        // Other hides (blur, click outside) will be undone
        if (wasVisible && !isVisible) {
          await appWindow.show();
        }
        
        wasVisible = isVisible;
      } catch (error) {
        // Ignore errors
      }
    };

    // Check very frequently to catch hides immediately
    const intervalId = setInterval(preventUnexpectedHide, 50);

    return () => {
      clearInterval(intervalId);
    };
  });
</script>

<div class="app-container">
  <header data-tauri-drag-region>
    <!-- Drag handle at the top center (matches Krona-Lite design) -->
    <div class="drag-handle"></div>
    
    <div class="header-content">
      <h1 class="title">Nyx</h1>
    </div>
  </header>

  <main>
    <div class="content">
      <textarea
        bind:value={goal}
        on:keydown={handleKeydown}
        placeholder="What would you like me to do?"
        class="goal-input"
        rows="5"
        on:dblclick|stopPropagation
      ></textarea>
      <button
        class="submit-btn"
        on:click={handleSubmit}
        disabled={!goal.trim()}
        on:dblclick|stopPropagation
      >
        Execute
      </button>
    </div>
  </main>
</div>

<style>
  :global(#app) {
    height: 100vh;
    width: 100vw;
  }

  .app-container {
    background: rgba(0, 0, 0, 0.8);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border-radius: 12px;
    width: 100%;
    height: 100%;
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    display: flex;
    flex-direction: column;
    min-height: 80px;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px 8px;
    flex-shrink: 0;
    position: relative;
    cursor: move;
    -webkit-app-region: drag;
  }

  .drag-handle {
    position: absolute;
    top: 6px;
    left: 50%;
    transform: translateX(-50%);
    width: 40px;
    height: 4px;
    background-color: rgba(255, 255, 255, 0.3);
    border-radius: 2px;
    pointer-events: none;
  }

  /* Make interactive elements inside header non-draggable */
  header .header-content {
    -webkit-app-region: no-drag;
    pointer-events: auto;
  }

  .header-content {
    display: flex;
    align-items: baseline;
    gap: 12px;
    margin: 0 auto;
  }

  .title {
    font-size: 20px;
    font-weight: 600;
    margin: 0;
    user-select: none;
    color: white;
  }

  main {
    flex-grow: 1;
    overflow-y: auto;
  }

  .content {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 12px 20px 20px;
  }

  .goal-input {
    width: 100%;
    flex-grow: 1;
    padding: 12px;
    font-size: 1rem;
    font-family: var(--font-sans);
    background: transparent;
    border: none;
    outline: none;
    color: white;
    resize: none;
    min-height: 100px;
    user-select: text;
  }

  .goal-input::placeholder {
    color: rgba(255, 255, 255, 0.4);
  }

  .goal-input:focus {
    outline: none;
  }

  .submit-btn {
    padding: 12px;
    font-size: 1rem;
    font-weight: 500;
    background: var(--neon-green);
    color: black;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
    opacity: 0.8;
  }

  .submit-btn:hover:not(:disabled) {
    opacity: 1;
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(57, 255, 20, 0.3);
  }

  .submit-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
  }
</style>
