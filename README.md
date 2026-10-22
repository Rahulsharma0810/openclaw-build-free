# OpenClaw Build Free (macOS arm64, unsigned)

A lightweight desktop GUI client for connecting to remote OpenClaw instances. Self-signed and installable without an Apple Developer ID.

## Features

- ✅ Connect to remote OpenClaw servers via HTTPS
- ✅ Clean, native macOS interface built with Tauri
- ✅ Daily automated builds
- ✅ Auto-update support
- ✅ Ad-hoc signed (no Developer ID needed)

## Installation

1. Download the latest `.dmg` from [Releases](https://github.com/Rahulsharma0810/openclaw-build-free/releases)
2. Open the DMG file
3. Drag `OpenClaw.app` to your Applications folder
4. **First launch**: Right-click → Open (to bypass Gatekeeper)
5. Subsequent launches work normally

## Usage

1. Launch OpenClaw.app
2. Enter your OpenClaw server URL (e.g., `https://rahuls-mac-mini.husky-tet.ts.net:18789`)
3. The app will connect and provide a GUI interface to your OpenClaw instance

## Auto-Updates

The app checks for updates automatically. When a new version is available, you'll be prompted to install it.

## Building Locally

```bash
# Install dependencies
npm install

# Development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Architecture

- **Frontend**: HTML/CSS/JavaScript with modern UI
- **Backend**: Tauri (Rust) for native functionality
- **Connectivity**: Secure HTTPS connections to remote OpenClaw servers

## Configuration

The app stores connection settings locally in your user preferences. You can configure:
- Server URL
- Connection timeout
- Auto-connect on launch

## License

MIT License

## Disclaimer

This is an unofficial GUI client for OpenClaw. It connects to remote instances and does not include the OpenClaw server itself.
