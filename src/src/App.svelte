<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  let goal = '';
  let testStatus = '';
  let appState = 'IDLE';
  let taskProgress = '';
  const appWindow = getCurrentWindow();

  // Submitting the goal
  async function handleSubmit() {
    if (!goal.trim()) return;
    const taskDescription = goal.trim();
    goal = '';
    testStatus = 'Executing task...';
    
    try {
      const result = await invoke('execute_task_command', { task: taskDescription });
      testStatus = `✓ Task completed: ${result.message}`;
      setTimeout(() => testStatus = '', 5000);
    } catch (error) {
      testStatus = `✗ Error: ${error}`;
      console.error('Task execution failed:', error);
    }
  }

  // Handle Enter key for submission
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSubmit();
    }
  }

  // I/O Controller Test Functions
  async function testIO() {
    try {
      testStatus = 'Testing I/O controller...';
      await invoke('test_io');
      testStatus = '✓ Test complete! Mouse moved to (100, 100) and typed "hello"';
      setTimeout(() => testStatus = '', 3000);
    } catch (error) {
      testStatus = `✗ Error: ${error}`;
      console.error('Test failed:', error);
    }
  }

  async function testMouseMove() {
    try {
      testStatus = 'Moving mouse to (500, 500)...';
      await invoke('execute_mouse_move', { x: 500, y: 500 });
      testStatus = '✓ Mouse moved successfully';
      setTimeout(() => testStatus = '', 2000);
    } catch (error) {
      testStatus = `✗ Error: ${error}`;
      console.error('Mouse move failed:', error);
    }
  }

  async function testMouseClick() {
    try {
      testStatus = 'Clicking mouse...';
      await invoke('execute_mouse_click', { buttonStr: 'left' });
      testStatus = '✓ Mouse clicked successfully';
      setTimeout(() => testStatus = '', 2000);
    } catch (error) {
      testStatus = `✗ Error: ${error}`;
      console.error('Mouse click failed:', error);
    }
  }

  async function testTypeString() {
    try {
      testStatus = 'Typing "Hello from Nyx!"...';
      await invoke('execute_type_string', { text: 'Hello from Nyx!' });
      testStatus = '✓ Text typed successfully';
      setTimeout(() => testStatus = '', 2000);
    } catch (error) {
      testStatus = `✗ Error: ${error}`;
      console.error('Type string failed:', error);
    }
  }

  // Orchestrator Test Functions
  async function testExecuteTask() {
    try {
      testStatus = 'Executing test task...';
      const result = await invoke('execute_task_command', { 
        task: 'Test task: Move mouse and type hello' 
      });
      testStatus = `✓ Task completed: ${result.message}`;
      setTimeout(() => testStatus = '', 5000);
    } catch (error) {
      testStatus = `✗ Error: ${error}`;
      console.error('Execute task failed:', error);
    }
  }

  async function testGetAppState() {
    try {
      testStatus = 'Getting app state...';
      const state = await invoke('get_app_state_command');
      appState = JSON.parse(state);
      testStatus = `✓ Current state: ${appState}`;
      setTimeout(() => testStatus = '', 3000);
    } catch (error) {
      testStatus = `✗ Error: ${error}`;
      console.error('Get app state failed:', error);
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

    // Listen to orchestrator events
    const unlistenStateChanged = listen('app_state_changed', (event: any) => {
      appState = event.payload;
      console.log('App state changed:', appState);
    });

    const unlistenTaskProgress = listen('task_progress', (event: any) => {
      taskProgress = event.payload;
      testStatus = taskProgress;
      console.log('Task progress:', taskProgress);
    });

    return () => {
      clearInterval(intervalId);
      unlistenStateChanged.then(fn => fn());
      unlistenTaskProgress.then(fn => fn());
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

      <!-- Status Display -->
      {#if taskProgress || appState !== 'IDLE'}
        <div class="status-section">
          <div class="status-item">
            <span class="status-label">State:</span>
            <span class="status-value">{appState}</span>
          </div>
          {#if taskProgress}
            <div class="status-item">
              <span class="status-label">Progress:</span>
              <span class="status-value">{taskProgress}</span>
            </div>
          {/if}
        </div>
      {/if}

      <!-- I/O Controller Test Section -->
      <div class="test-section">
        <h3 class="test-title">I/O Controller Tests</h3>
        {#if testStatus}
          <div class="test-status">{testStatus}</div>
        {/if}
        <div class="test-buttons">
          <button class="test-btn" on:click={testIO} on:dblclick|stopPropagation>
            Test I/O (Full)
          </button>
          <button class="test-btn" on:click={testMouseMove} on:dblclick|stopPropagation>
            Move Mouse
          </button>
          <button class="test-btn" on:click={testMouseClick} on:dblclick|stopPropagation>
            Click Mouse
          </button>
          <button class="test-btn" on:click={testTypeString} on:dblclick|stopPropagation>
            Type Text
          </button>
        </div>
      </div>

      <!-- Orchestrator Test Section -->
      <div class="test-section">
        <h3 class="test-title">Orchestrator Tests</h3>
        <div class="test-buttons">
          <button class="test-btn" on:click={testExecuteTask} on:dblclick|stopPropagation>
            Execute Task
          </button>
          <button class="test-btn" on:click={testGetAppState} on:dblclick|stopPropagation>
            Get State
          </button>
        </div>
      </div>
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

  .test-section {
    margin-top: 20px;
    padding-top: 20px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
  }

  .test-title {
    font-size: 0.9rem;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.7);
    margin: 0 0 12px 0;
  }

  .test-status {
    padding: 8px 12px;
    margin-bottom: 12px;
    background: rgba(57, 255, 20, 0.1);
    border: 1px solid rgba(57, 255, 20, 0.3);
    border-radius: 6px;
    font-size: 0.85rem;
    color: var(--neon-green);
  }

  .test-buttons {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
  }

  .test-btn {
    padding: 8px 12px;
    font-size: 0.85rem;
    font-weight: 500;
    background: rgba(255, 255, 255, 0.1);
    color: white;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .test-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    border-color: rgba(255, 255, 255, 0.3);
    transform: translateY(-1px);
  }

  .status-section {
    margin-top: 20px;
    padding: 12px;
    background: rgba(57, 255, 20, 0.05);
    border: 1px solid rgba(57, 255, 20, 0.2);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .status-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.85rem;
  }

  .status-label {
    color: rgba(255, 255, 255, 0.6);
    font-weight: 500;
  }

  .status-value {
    color: var(--neon-green);
    font-weight: 600;
  }
</style>
