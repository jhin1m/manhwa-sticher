<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';

  let inputFolder = $state<string | null>(null);
  let outputFolder = $state<string | null>(null);

  // Settings
  let splitHeight = $state(5000);
  let sensitivity = $state(90);
  let scanLineStep = $state(5);
  let ignorableBorder = $state(5);

  // UI State
  let activeTab = $state<'home' | 'settings'>('home');
  let selectedPreset = $state('Custom');

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

  const PRESETS = {
    'High Res Manhwa': { splitHeight: 7500, sensitivity: 95, scanLineStep: 3, ignorableBorder: 10 },
    'Manhua (SFX)': { splitHeight: 6000, sensitivity: 85, scanLineStep: 10, ignorableBorder: 15 },
    'Webtoon (Simple)': { splitHeight: 5000, sensitivity: 75, scanLineStep: 5, ignorableBorder: 5 },
  };

  function applyPreset(name: string) {
    selectedPreset = name;
    if (name !== 'Custom' && PRESETS[name as keyof typeof PRESETS]) {
      const p = PRESETS[name as keyof typeof PRESETS];
      splitHeight = p.splitHeight;
      sensitivity = p.sensitivity;
      scanLineStep = p.scanLineStep;
      ignorableBorder = p.ignorableBorder;
    }
  }

  function onSettingChange() {
    selectedPreset = 'Custom';
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

  <div class="tabs">
    <button 
      class="tab-btn {activeTab === 'home' ? 'active' : ''}" 
      onclick={() => activeTab = 'home'}
    >
      Home
    </button>
    <button 
      class="tab-btn {activeTab === 'settings' ? 'active' : ''}" 
      onclick={() => activeTab = 'settings'}
    >
      Settings
    </button>
  </div>

  <div class="tab-content">
    {#if activeTab === 'home'}
      <div class="card">
        <h2>1. Input</h2>
        <div class="row">
          <input type="text" value={inputFolder ?? ''} readonly placeholder="Select input folder..." />
          <button onclick={selectInputFolder}>Browse</button>
        </div>
      </div>

      <div class="card">
        <h2>2. Output</h2>
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
    {/if}

    {#if activeTab === 'settings'}
      <div class="card">
        <h2>Algorithm Settings</h2>
        
        <div class="setting-item">
          <label>
            <span>Preset</span>
            <select value={selectedPreset} onchange={(e) => applyPreset(e.currentTarget.value)}>
              <option value="Custom">Custom</option>
              {#each Object.keys(PRESETS) as preset}
                <option value={preset}>{preset}</option>
              {/each}
            </select>
          </label>
        </div>

        <div class="setting-item">
          <label>
            <span>Target Split Height (Max) (px)</span>
            <input type="number" bind:value={splitHeight} min="1000" step="100" oninput={onSettingChange} />
          </label>
          <p class="hint">Maximum height for output images. Actual height may be less to avoid cutting content.</p>
        </div>

        <div class="setting-item">
          <label>
            <span>Sensitivity ({sensitivity}%)</span>
            <input type="range" bind:value={sensitivity} min="0" max="100" oninput={onSettingChange} />
          </label>
          <p class="hint">Higher values mean stricter matching (less likely to cut through content).</p>
        </div>

        <div class="setting-item">
          <label>
            <span>Scan Line Step (px)</span>
            <input type="number" bind:value={scanLineStep} min="1" max="100" oninput={onSettingChange} />
          </label>
          <p class="hint">Step size for scanning split lines. Lower is more precise but slower.</p>
        </div>
        
         <div class="setting-item">
          <label>
            <span>Ignorable Border (px)</span>
            <input type="number" bind:value={ignorableBorder} min="0" max="50" oninput={onSettingChange} />
          </label>
          <p class="hint">Pixels to ignore at the edges of the image.</p>
        </div>
      </div>
    {/if}
  </div>
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
    margin-bottom: 20px;
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
    margin-bottom: 20px;
  }

  .setting-item label {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 5px;
  }

  .setting-item input[type="number"], .setting-item select {
    width: 120px;
    padding: 6px;
    border: 1px solid #ddd;
    border-radius: 4px;
  }
  
  .setting-item input[type="range"] {
    width: 150px;
  }

  .hint {
    font-size: 0.85rem;
    color: #666;
    margin: 0;
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

  /* Tabs */
  .tabs {
    display: flex;
    gap: 10px;
    margin-bottom: 10px;
    justify-content: center;
  }

  .tab-btn {
    background: transparent;
    border: 2px solid transparent;
    border-radius: 20px;
    padding: 8px 24px;
    font-weight: 600;
    color: #666;
  }

  .tab-btn:hover {
    background: rgba(0,0,0,0.05);
  }

  .tab-btn.active {
    background: white;
    color: #007bff;
    border-color: #007bff;
    box-shadow: 0 2px 8px rgba(0,123,255,0.1);
  }
</style>
