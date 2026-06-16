# MxMinimax — Tic-Tac-Toe with Minimax AI

A cross-platform Tic-Tac-Toe application powered by a pre-computed Minimax AI engine. Built with [Tauri v2](https://tauri.app/), featuring a **Rust** backend for AI logic and a **React/TypeScript** frontend for the UI. Runs on Windows, macOS, Linux, and Android.

---

## How It Works

The AI never loses. It uses the [Minimax algorithm](https://en.wikipedia.org/wiki/Minimax) to evaluate every possible game state and always plays optimally. To avoid expensive real-time computation (especially on mobile), the full game tree is **pre-computed and bundled** as compressed binary files (`maximizerTree.zip` / `minimizerTree.zip`). On startup, the Rust backend deserializes the relevant tree into memory and traverses it on each move.

```
User Move → React (invoke) → Tauri IPC → Rust (minimax.rs) → Optimal Move → React UI
```

---

## Tech Stack

| Layer      | Technology                          |
|------------|-------------------------------------|
| Frontend   | React 19, TypeScript, Tailwind CSS  |
| Backend    | Rust (Tauri v2)                     |
| IPC Bridge | Tauri Commands                      |
| AI Data    | `wincode` binary format + ZIP       |
| Build Tool | Vite 7                              |
| CI/CD      | GitHub Actions                      |

---

## Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [Rust](https://rustup.rs/) (stable toolchain)
- For **Linux**: `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`
- For **Android**: Android Studio with NDK, SDK Command-line Tools, and Rust mobile targets

---

## Getting Started

### Install dependencies

```bash
npm install
```

### Run in development mode

```bash
npm run tauri dev
```

This starts the Vite dev server on `http://localhost:1420` and compiles the Rust backend simultaneously, with hot-module reloading for the frontend.

### Build for desktop

```bash
npm run tauri build
```

Produces a platform-native installer (`.msi` on Windows, `.dmg` on macOS, `.deb`/`.AppImage` on Linux).

### Build for Android

```bash
npm run tauri android build
```

Requires Rust targets for Android architectures (e.g., `aarch64-linux-android`) installed via `rustup`.

---

## Project Structure

```
.
├── src/                        # React/TypeScript frontend
│   ├── App.tsx                 # Main UI component and game flow
│   ├── utils/board.ts          # Board logic, win-condition checking
│   └── components/             # Reusable UI elements
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs              # Tauri app entry point, state init
│   │   ├── commands.rs         # IPC command handlers (play_move, reset_bot)
│   │   ├── minimax.rs          # make_move() and move selection
│   │   ├── utils.rs            # Tree loading, compute_optimal()
│   │   └── val_types.rs        # Node, PlayerType data structures
│   ├── resources/
│   │   ├── maximizerTree.zip   # Pre-computed game tree (AI goes first)
│   │   └── minimizerTree.zip   # Pre-computed game tree (player goes first)
│   ├── icons/                  # App icons for all platforms
│   └── tauri.conf.json         # Tauri configuration
├── .github/workflows/
│   └── release.yml             # CI/CD: desktop + Android release pipeline
├── index.html
├── package.json
└── vite.config.ts
```

---

## IPC Commands

The frontend communicates with the Rust backend via three Tauri commands:

| Command      | Description                                      |
|--------------|--------------------------------------------------|
| `play_move`  | Submits the player's move; returns the AI's response move |
| `set_node`   | Syncs the backend game tree to a given board state |
| `reset_bot`  | Resets the backend state to the root of the game tree |

---

## CI/CD

Releases are triggered by pushing a `v*` tag (e.g., `v1.0.0`). GitHub Actions automatically:

1. **Desktop** — Builds for macOS, Windows, and Linux in parallel using `tauri-apps/tauri-action` and publishes a GitHub Release draft.
2. **Android** — Builds a universal APK, signs it using repository secrets, and uploads it to the same release.

---

## License

MIT license
