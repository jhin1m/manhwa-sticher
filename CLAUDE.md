# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Manhwa Sticher** is a desktop application for splitting manhwa images into multiple pages without losing information. Built with Tauri v2, SvelteKit, and Rust.

Purpose: Help creators and readers process manhwa images faster, more automatically, and more easily.

## Development Commands

### Frontend (SvelteKit + Svelte 5)
- `pnpm dev` - Start Vite development server (port 1420)
- `pnpm build` - Build SvelteKit frontend for production
- `pnpm preview` - Preview production build
- `pnpm check` - Run svelte-check for type checking
- `pnpm check:watch` - Run svelte-check in watch mode

### Tauri (Desktop App)
- `pnpm tauri dev` - Run Tauri app in development mode (starts frontend dev server automatically)
- `pnpm tauri build` - Build Tauri app for production
- `pnpm tauri` - Access Tauri CLI commands

### Backend (Rust)
From `src-tauri/` directory:
- `cargo build` - Build Rust backend
- `cargo test` - Run Rust tests
- `cargo check` - Check Rust code without building

## Architecture

### Tauri Desktop Application

This is a **hybrid desktop application** combining:
- **Frontend**: SvelteKit in SPA mode (SSR disabled)
- **Backend**: Rust via Tauri v2
- **Build**: Vite for frontend bundling

### Frontend Architecture

**Framework**: SvelteKit with Svelte 5 (runes syntax)
- **Adapter**: `@sveltejs/adapter-static` with fallback to `index.html` for SPA mode
- **SSR**: Disabled (`export const ssr = false` in `src/routes/+layout.ts`)
- **Dev Server**: Vite on port 1420 (required by Tauri, strictPort enabled)
- **State Management**: Svelte 5 runes (`$state`, `$derived`, etc.)

**Key Configuration**:
- SPA mode is required because Tauri doesn't support Node.js SSR
- Frontend builds to `build/` directory, consumed by Tauri
- Hot Module Replacement runs on port 1421 during development

### Backend Architecture

**Rust + Tauri v2**
- **Entry Point**: `src-tauri/src/main.rs` → calls `manhwa_sticher_lib::run()`
- **Library**: `src-tauri/src/lib.rs` contains Tauri app setup and commands
- **Crate Types**: `["staticlib", "cdylib", "rlib"]` for cross-platform compatibility

**Current Commands**:
- `greet(name: &str) -> String` - Example command demonstrating Tauri command invocation

**Plugins**:
- `tauri-plugin-opener` - For opening external links/files

### Communication Pattern

Frontend invokes Rust commands via `@tauri-apps/api/core`:
```typescript
import { invoke } from "@tauri-apps/api/core";
const result = await invoke("command_name", { args });
```

Rust commands are registered in `src-tauri/src/lib.rs`:
```rust
#[tauri::command]
fn command_name(args: Type) -> ReturnType { }

tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![command_name])
```

## Project Structure

```
manhwa-sticher/
├── src/                    # SvelteKit frontend
│   ├── routes/            # SvelteKit routes (SPA mode)
│   │   ├── +layout.ts    # Disables SSR
│   │   └── +page.svelte  # Main page
│   └── app.html          # HTML template
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs       # Binary entry point
│   │   └── lib.rs        # Tauri commands and setup
│   ├── tauri.conf.json   # Tauri configuration
│   ├── Cargo.toml        # Rust dependencies
│   └── build.rs          # Build script
├── static/                # Static assets
├── build/                 # Built frontend (generated)
├── svelte.config.js       # SvelteKit config
├── vite.config.js         # Vite config (Tauri-specific)
└── package.json           # Node dependencies
```

## Important Configuration Details

### Tauri Configuration (`src-tauri/tauri.conf.json`)
- **Product Name**: manhwa-sticher
- **Identifier**: com.jhin1m.manhwa-sticher
- **Dev Command**: `pnpm dev` (auto-run before Tauri dev)
- **Build Command**: `pnpm build` (auto-run before Tauri build)
- **Frontend Dist**: `../build` (SvelteKit output directory)
- **Dev URL**: http://localhost:1420

### Development Workflow

1. **Starting Development**:
   - Run `pnpm tauri dev` - automatically starts Vite dev server and Tauri window
   - OR run `pnpm dev` separately, then `pnpm tauri dev`

2. **Adding New Tauri Commands**:
   - Define command in `src-tauri/src/lib.rs` with `#[tauri::command]`
   - Register in `.invoke_handler(tauri::generate_handler![...])`
   - Invoke from frontend using `invoke("command_name", { args })`

3. **Building for Production**:
   - Run `pnpm tauri build` - builds both frontend and creates platform-specific bundles
   - Bundles created for all targets (configurable in `tauri.conf.json`)

### TypeScript Configuration

- Strict mode enabled
- Module resolution: `bundler`
- Path aliases handled by SvelteKit (`$lib` maps to `src/lib`)
- Extends `.svelte-kit/tsconfig.json` for SvelteKit-generated types

## Key Dependencies

**Frontend**:
- Svelte 5 (latest with runes)
- SvelteKit 2.x
- Vite 6.x
- TypeScript ~5.6

**Backend**:
- Tauri 2.x
- serde + serde_json for serialization

**Package Manager**: pnpm (check `pnpm-lock.yaml`)
