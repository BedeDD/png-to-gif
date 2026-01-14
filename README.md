# PNG to GIF Converter

A cross-platform desktop application for converting sequences of PNG images into animated GIFs. Built with Tauri and vanilla JavaScript.

## Features

- 🖱️ **Drag & Drop Support** - Simply drag PNG files into the app window
- 📁 **File Browser** - Click to select files using native file picker
- ⚙️ **Customizable Settings** - Adjust framerate, width, and loop options
- 📊 **Real-time Progress** - Track conversion progress as it happens
- 🔍 **Sequence Validation** - Automatically validates PNG sequences for gaps and consistency
- 🌍 **Cross-platform** - Works on macOS, Windows, and Linux

## Prerequisites

Before building or running the app, you need to install:

### 1. Node.js
Download and install from [nodejs.org](https://nodejs.org/) (LTS version recommended)

### 2. Rust
Install Rust using rustup:

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
1. Download and install from [rustup.rs](https://rustup.rs/)
2. Run the installer and follow prompts
3. Restart your terminal after installation

**Windows Additional Requirements:**
- Microsoft Visual C++ Build Tools
- The Rust installer will prompt you to install these if needed
- Or download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/

### 3. FFmpeg
The app requires FFmpeg for GIF conversion.

**macOS:**
```bash
brew install ffmpeg
```

**Windows:**

*Option 1: Using Scoop (Recommended)*
```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex
scoop install ffmpeg
```

*Option 2: Using Chocolatey*
```powershell
choco install ffmpeg
```

*Option 3: Manual Installation*
1. Download from [ffmpeg.org](https://ffmpeg.org/download.html#build-windows)
2. Extract to `C:\ffmpeg`
3. Add `C:\ffmpeg\bin` to your system PATH

**Linux:**
```bash
# Ubuntu/Debian
sudo apt install ffmpeg

# Fedora
sudo dnf install ffmpeg

# Arch
sudo pacman -S ffmpeg
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

The app will compile and open with hot-reload enabled. Changes to frontend code will automatically reload.

### Open Browser DevTools
While in development mode, press:
- **macOS:** `Cmd+Option+I`
- **Windows/Linux:** `Ctrl+Shift+I`

## Building for Production

### macOS
```bash
npm run tauri build
```

Output locations:
- **App Bundle:** `src-tauri/target/release/bundle/macos/png-gif-converter.app`
- **DMG Installer:** `src-tauri/target/release/bundle/dmg/png-gif-converter_*_aarch64.dmg` (Apple Silicon) or `*_x64.dmg` (Intel)

### Windows
```bash
npm run tauri build
```

Output locations:
- **MSI Installer:** `src-tauri/target/release/bundle/msi/png-gif-converter_*_x64_en-US.msi`
- **NSIS Installer:** `src-tauri/target/release/bundle/nsis/png-gif-converter_*_x64-setup.exe`
- **Executable:** `src-tauri/target/release/png-gif-converter.exe`

### Linux
```bash
npm run tauri build
```

Output locations:
- **AppImage:** `src-tauri/target/release/bundle/appimage/png-gif-converter_*_amd64.AppImage`
- **Deb Package:** `src-tauri/target/release/bundle/deb/png-gif-converter_*_amd64.deb`
- **Executable:** `src-tauri/target/release/png-gif-converter`

## Usage

1. Launch the application
2. **Add PNG files** using either:
   - Drag and drop files into the window
   - Click the drop zone to browse for files
3. The app will validate the sequence and show:
   - Detected pattern (e.g., `frame_%04d.png`)
   - Number of frames
   - Starting frame number
4. **Adjust settings** (optional):
   - **Framerate:** Frames per second (1-120, default: 30)
   - **Width:** Output GIF width in pixels (height auto-calculated)
   - **Loop:** Enable/disable endless loop
5. **Choose output location** by clicking "Browse"
6. Click **"Convert to GIF"** to start
7. Monitor progress in real-time
8. Your animated GIF will be saved to the chosen location

## File Requirements

PNG files must meet these requirements:
- Sequential numbering with no gaps (e.g., 001, 002, 003)
- Consistent zero-padding (all files must use same padding)
- Same filename prefix
- At least 2 files required

**Valid Examples:**
- `frame_0001.png`, `frame_0002.png`, `frame_0003.png`
- `img_1.png`, `img_2.png`, `img_3.png`
- `render001.png`, `render002.png`, `render003.png`

**Invalid Examples:**
- `frame_1.png`, `frame_02.png` (inconsistent padding)
- `frame_1.png`, `frame_3.png` (missing frame 2)
- Mixed prefixes or file extensions

## Technical Details

- **Framework:** Tauri v2
- **Frontend:** Vanilla JavaScript, HTML, CSS
- **Backend:** Rust
- **Video Processing:** FFmpeg with Lanczos scaling filter
- **Platforms:** macOS (Intel & Apple Silicon), Windows, Linux

## Troubleshooting

### FFmpeg Not Detected
The app checks for FFmpeg in:
- System PATH
- Common installation locations:
  - macOS: `/opt/homebrew/bin`, `/usr/local/bin`, `/usr/bin`
  - Windows: `C:\ffmpeg\bin`, `C:\Program Files\ffmpeg\bin`
  - Linux: `/usr/bin`, `/usr/local/bin`, `/snap/bin`

If FFmpeg isn't detected, ensure it's installed and either in your PATH or in one of the common locations above.

### Development Build Fails
1. Ensure Rust is installed: `rustc --version`
2. Ensure Node.js is installed: `node --version`
3. Clear build cache: `rm -rf src-tauri/target && npm run tauri build`

## License

See SETUP.md for detailed setup instructions.
