<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';

  let inputFolder = $state<string | null>(null);
  let outputFolder = $state<string | null>(null);

  let splitHeight = $state(5000);
  let sensitivity = $state(90);
  let scanLineStep = $state(5);
  let ignorableBorder = $state(5);

  let isProcessing = $state(false);
  let progress = $state(0);
  let statusMessage = $state("");

  interface ProgressUpdate {
    current: number;
    total: number;
    percentage: number;
    message: string;
  }

  interface ProcessResult {
    success: boolean;
    message: string;
    outputFiles: string[];
    totalImages: number;
  }

  async function selectInputFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });
      if (selected) {
        inputFolder = selected as string;
        // Default output folder to input folder + "_output" if not set
        if (!outputFolder) {
          outputFolder = selected + "_output";
        }
      }
    } catch (err) {
      console.error("Failed to open dialog:", err);
    }
  }

  async function selectOutputFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });
      if (selected) {
        outputFolder = selected as string;
      }
    } catch (err) {
      console.error("Failed to open dialog:", err);
    }
  }

  async function startProcessing() {
    if (!inputFolder || !outputFolder) return;

    isProcessing = true;
    progress = 0;
    statusMessage = "Starting...";

    let unlisten: UnlistenFn | null = null;

    try {
      // Listen for progress updates
      unlisten = await listen<ProgressUpdate>('processing-progress', (event) => {
        const { percentage, message } = event.payload;
        progress = percentage;
        statusMessage = message;
      });

      // Invoke the Rust command
      const result = await invoke<ProcessResult>('process_images', {
        inputFolder,
        outputFolder,
        splitHeight,
        sensitivity,
        scanLineStep,
        ignorableBorder,
      });

      if (result.success) {
        statusMessage = `Success! Generated ${result.outputFiles.length} images from ${result.totalImages} source images.`;
        progress = 100;
      } else {
        statusMessage = `Failed: ${result.message}`;
      }
    } catch (e) {
      statusMessage = `Error: ${e}`;
      progress = 0;
    } finally {
      // Clean up event listener
      if (unlisten) {
        unlisten();
      }
      isProcessing = false;
    }
  }
</script>

<main class="container">
  <h1>Manhwa Stitcher</h1>

  <div class="card">
    <h2>1. Input</h2>
    <div class="row">
      <input type="text" value={inputFolder ?? ''} readonly placeholder="Select input folder..." />
      <button onclick={selectInputFolder}>Browse</button>
    </div>
  </div>

  <div class="card">
    <h2>2. Settings</h2>
    
    <div class="setting-item">
      <label>
        <span>Split Height (px)</span>
        <input type="number" bind:value={splitHeight} min="1000" step="100" />
      </label>
    </div>

    <div class="setting-item">
      <label>
        <span>Sensitivity ({sensitivity}%)</span>
        <input type="range" bind:value={sensitivity} min="0" max="100" />
      </label>
    </div>

    <div class="setting-item">
      <label>
        <span>Scan Line Step (px)</span>
        <input type="number" bind:value={scanLineStep} min="1" max="100" />
      </label>
    </div>
    
     <div class="setting-item">
      <label>
        <span>Ignorable Border (px)</span>
        <input type="number" bind:value={ignorableBorder} min="0" max="50" />
      </label>
    </div>
  </div>

  <div class="card">
    <h2>3. Output</h2>
    <div class="row">
      <input type="text" value={outputFolder ?? ''} readonly placeholder="Select output folder..." />
      <button onclick={selectOutputFolder}>Browse</button>
    </div>
  </div>

  <div class="actions">
    <button class="primary" onclick={startProcessing} disabled={isProcessing || !inputFolder || !outputFolder}>
      {isProcessing ? 'Processing...' : 'Start Process'}
    </button>
  </div>

  {#if isProcessing || progress > 0}
    <div class="status">
      <div class="progress-bar">
        <div class="progress-fill" style="width: {progress}%"></div>
      </div>
      <p>{statusMessage}</p>
    </div>
  {/if}
</main>

<style>
  :global(body) {
    font-family: system-ui, -apple-system, sans-serif;
    background: #f0f2f5;
    margin: 0;
    padding: 20px;
    color: #333;
  }
  
  .container {
    max-width: 600px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  h1 {
    text-align: center;
    color: #1a1a1a;
    margin-bottom: 10px;
  }

  h2 {
    font-size: 1.1rem;
    margin-top: 0;
    margin-bottom: 15px;
    color: #555;
    border-bottom: 1px solid #eee;
    padding-bottom: 10px;
  }

  .card {
    background: white;
    padding: 20px;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.05);
  }

  .row {
    display: flex;
    gap: 10px;
  }

  input[type="text"] {
    flex: 1;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 6px;
    background: #f9f9f9;
  }

  button {
    padding: 8px 16px;
    border: none;
    border-radius: 6px;
    background: #e0e0e0;
    cursor: pointer;
    font-weight: 500;
    transition: background 0.2s;
  }

  button:hover {
    background: #d0d0d0;
  }

  button.primary {
    background: #007bff;
    color: white;
    width: 100%;
    padding: 12px;
    font-size: 1.1rem;
  }

  button.primary:hover {
    background: #0069d9;
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .setting-item {
    margin-bottom: 15px;
  }

  .setting-item label {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .setting-item input[type="number"] {
    width: 80px;
    padding: 6px;
    border: 1px solid #ddd;
    border-radius: 4px;
  }
  
  .setting-item input[type="range"] {
    width: 150px;
  }

  .status {
    margin-top: 10px;
    text-align: center;
  }

  .progress-bar {
    height: 8px;
    background: #e0e0e0;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 8px;
  }

  .progress-fill {
    height: 100%;
    background: #007bff;
    transition: width 0.3s ease;
  }
</style>
