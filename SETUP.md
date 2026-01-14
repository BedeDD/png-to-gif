# PNG to GIF Converter - Setup Instructions

## Prerequisites

### 1. Install Rust

#### macOS/Linux:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, restart your terminal or run:
```bash
source $HOME/.cargo/env
```

#### Windows:
1. Download and install Rust from https://rustup.rs/
2. Run the `rustup-init.exe` installer
3. Follow the installation prompts (default options work fine)
4. Restart your terminal/PowerShell after installation
5. Verify installation:
   ```powershell
   rustc --version
   ```

**Windows Prerequisites:**
- Visual Studio C++ Build Tools are required
- If not installed, the Rust installer will prompt you to install them
- Or download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/

### 2. Install Node.js

Download and install Node.js from https://nodejs.org/ (LTS version recommended)

Verify installation:
```bash
node --version
npm --version
```

### 3. Install FFmpeg

#### macOS:
```bash
brew install ffmpeg
```

#### Windows:

**Option 1: Using Scoop (Recommended)**
1. Install Scoop package manager:
   ```powershell
   Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
   irm get.scoop.sh | iex
   ```
2. Install FFmpeg:
   ```powershell
   scoop install ffmpeg
   ```

**Option 2: Using Chocolatey**
1. Install Chocolatey from https://chocolatey.org/install
2. Run as Administrator:
   ```powershell
   choco install ffmpeg
   ```

**Option 3: Manual Installation**
1. Download FFmpeg from https://ffmpeg.org/download.html#build-windows
2. Extract the ZIP file to `C:\ffmpeg`
3. Add to PATH:
   - Open "Environment Variables" (search in Windows)
   - Under "System variables", select "Path" and click "Edit"
   - Click "New" and add: `C:\ffmpeg\bin`
   - Click "OK" to save
4. Restart your terminal/PowerShell
5. Verify installation:
   ```powershell
   ffmpeg -version
   ```

#### Linux:
```bash
# Ubuntu/Debian
sudo apt install ffmpeg

# Fedora
sudo dnf install ffmpeg
```

## Development

### Install Dependencies
```bash
npm install
```

### Run in Development Mode
```bash
npm run tauri dev
```

### Build for Production
```bash
npm run tauri build
```

The compiled application will be in `src-tauri/target/release/`.

## Usage

1. Launch the application
2. Drag and drop PNG files with sequential numbering (e.g., `frame_001.png`, `frame_002.png`)
   - Or click the drop zone to browse for files
3. Adjust settings:
   - **Framerate**: Frames per second (default: 30)
   - **Width**: Output GIF width in pixels (default: 500)
   - **Loop**: Enable endless loop
4. Click "Convert to GIF"
5. Wait for the progress bar to complete

## File Naming Requirements

PNG files must:
- Have sequential numbering (no gaps)
- Use consistent padding (all `001, 002, 003` or all `1, 2, 3`)
- Share the same prefix
- Be in `.png` format

**Valid examples:**
- `frame_0001.png`, `frame_0002.png`, `frame_0003.png`
- `img_1.png`, `img_2.png`, `img_3.png`
- `render001.png`, `render002.png`, `render003.png`

**Invalid examples:**
- `frame_1.png`, `frame_02.png` (inconsistent padding)
- `frame_1.png`, `frame_3.png` (missing frame 2)
- Mixed prefixes or extensions

## Technical Details

- **Tech Stack**: Tauri (Rust + HTML/CSS/JS)
- **FFmpeg Integration**: External binary (not bundled)
- **Platform Support**: Windows, macOS, Linux
- **Output**: Animated GIF with Lanczos scaling filter
