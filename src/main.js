const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const { open, save } = window.__TAURI_PLUGIN_DIALOG__;

// State
let sequenceInfo = null;
let isConverting = false;

// Elements
let elements = {};

// Initialize
window.addEventListener("DOMContentLoaded", async () => {
  // Get all elements
  elements = {
    ffmpegStatus: document.getElementById("ffmpeg-status"),
    dropZone: document.getElementById("drop-zone"),
    sequenceInfo: document.getElementById("sequence-info"),
    settings: document.getElementById("settings"),
    framerateInput: document.getElementById("framerate"),
    widthInput: document.getElementById("width"),
    loopCheckbox: document.getElementById("loop"),
    outputPath: document.getElementById("output-path"),
    browseBtn: document.getElementById("browse-btn"),
    convertBtn: document.getElementById("convert-btn"),
    progressContainer: document.getElementById("progress-container"),
    progressFill: document.getElementById("progress-fill"),
    progressText: document.getElementById("progress-text"),
    message: document.getElementById("message"),
  };

  // Check FFmpeg on startup
  await checkFfmpeg();

  // Set up event listeners
  setupEventListeners();

  // Set up Tauri event listeners
  await setupTauriEvents();
});

async function checkFfmpeg() {
  try {
    const ffmpegInfo = await invoke("check_ffmpeg_installed");

    if (ffmpegInfo.installed) {
      elements.ffmpegStatus.textContent = `FFmpeg installed: ${ffmpegInfo.version || "Unknown version"}`;
      elements.ffmpegStatus.className = "status-box success";
    } else {
      elements.ffmpegStatus.textContent = "FFmpeg not installed. Please install FFmpeg first.";
      elements.ffmpegStatus.className = "status-box error";
      elements.dropZone.style.pointerEvents = "none";
      elements.dropZone.style.opacity = "0.5";

      // Show installation instructions
      const instructions = document.createElement("div");
      instructions.style.marginTop = "10px";
      instructions.style.fontSize = "12px";

      const isMac = navigator.platform.toLowerCase().includes("mac");
      if (isMac) {
        instructions.innerHTML = 'Install with: <code>brew install ffmpeg</code>';
      } else {
        instructions.innerHTML = 'Download from: <a href="https://ffmpeg.org/download.html" target="_blank">ffmpeg.org</a>';
      }

      elements.ffmpegStatus.appendChild(instructions);
    }
  } catch (error) {
    showError("Failed to check FFmpeg: " + error);
  }
}

function setupEventListeners() {
  // Drop zone click
  elements.dropZone.addEventListener("click", async () => {
    const selected = await open({
      multiple: true,
      filters: [{ name: "PNG Images", extensions: ["png"] }],
    });

    if (selected && selected.length > 0) {
      await handleFiles(selected);
    }
  });

  // Browse button
  elements.browseBtn.addEventListener("click", async () => {
    const outputPath = await save({
      defaultPath: sequenceInfo ? `${sequenceInfo.prefix}_animated.gif` : "output.gif",
      filters: [{ name: "GIF Image", extensions: ["gif"] }],
    });

    if (outputPath) {
      elements.outputPath.value = outputPath;
    }
  });

  // Convert button
  elements.convertBtn.addEventListener("click", startConversion);
}

async function setupTauriEvents() {
  // File drop event
  await listen("tauri://drag-drop", async (event) => {
    await handleFiles(event.payload.paths);
  });

  // Drag enter
  await listen("tauri://drag-enter", () => {
    elements.dropZone.classList.add("drag-over");
  });

  // Drag leave
  await listen("tauri://drag-leave", () => {
    elements.dropZone.classList.remove("drag-over");
  });

  // Conversion progress
  await listen("conversion-progress", (event) => {
    const [frame, percent] = event.payload;
    updateProgress(frame, percent);
  });
}

async function handleFiles(paths) {
  elements.dropZone.classList.remove("drag-over");

  try {
    // Clear previous state
    hideMessage();

    // Analyze sequence
    sequenceInfo = await invoke("analyze_png_sequence", { paths });

    if (sequenceInfo.valid) {
      displaySequenceInfo(sequenceInfo);
      showSettings(sequenceInfo);
    } else {
      showError(sequenceInfo.error || "Invalid PNG sequence");
      hideSequenceInfo();
      hideSettings();
    }
  } catch (error) {
    showError("Failed to analyze sequence: " + error);
    hideSequenceInfo();
    hideSettings();
  }
}

function displaySequenceInfo(info) {
  elements.sequenceInfo.innerHTML = `
    <h3>Detected Sequence</h3>
    <p><strong>Pattern:</strong> ${info.pattern}</p>
    <p><strong>Frames:</strong> ${info.frame_count} (starting at ${info.start_number})</p>
    <p><strong>Directory:</strong> ${info.directory}</p>
  `;
  elements.sequenceInfo.classList.remove("hidden");
}

function hideSequenceInfo() {
  elements.sequenceInfo.classList.add("hidden");
}

function showSettings(info) {
  // Set default output path
  const defaultOutput = `${info.directory}/${info.prefix}_animated.gif`;
  elements.outputPath.value = defaultOutput;

  elements.settings.classList.remove("hidden");
}

function hideSettings() {
  elements.settings.classList.add("hidden");
}

async function startConversion() {
  if (!sequenceInfo || isConverting) return;

  const framerate = parseInt(elements.framerateInput.value);
  const width = parseInt(elements.widthInput.value);
  const loopForever = elements.loopCheckbox.checked;
  const outputPath = elements.outputPath.value;

  // Validate
  if (!outputPath) {
    showError("Please specify an output path");
    return;
  }

  if (framerate < 1 || framerate > 120) {
    showError("Framerate must be between 1 and 120");
    return;
  }

  if (width < 1 || width > 10000) {
    showError("Width must be between 1 and 10000");
    return;
  }

  try {
    isConverting = true;
    elements.convertBtn.disabled = true;
    hideMessage();
    showProgress();

    const request = {
      sequence_info: sequenceInfo,
      framerate,
      width,
      loop_forever: loopForever,
      output_path: outputPath,
    };

    await invoke("start_conversion", { request });

    hideProgress();
    showSuccess(`GIF created successfully at: ${outputPath}`);
  } catch (error) {
    hideProgress();
    showError("Conversion failed: " + error);
  } finally {
    isConverting = false;
    elements.convertBtn.disabled = false;
  }
}

function showProgress() {
  elements.progressContainer.classList.remove("hidden");
  elements.progressFill.style.width = "0%";
  elements.progressText.textContent = "Starting conversion...";
}

function hideProgress() {
  elements.progressContainer.classList.add("hidden");
}

function updateProgress(frame, percent) {
  elements.progressFill.style.width = percent + "%";
  elements.progressText.textContent = `Frame ${frame} / ${sequenceInfo.frame_count} (${percent.toFixed(1)}%)`;
}

function showMessage(text, type) {
  elements.message.textContent = text;
  elements.message.className = `message ${type}`;
  elements.message.classList.remove("hidden");
}

function hideMessage() {
  elements.message.classList.add("hidden");
}

function showSuccess(text) {
  showMessage(text, "success");
}

function showError(text) {
  showMessage(text, "error");
}
