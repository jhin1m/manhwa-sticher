# 📚 Manhwa Sticher

**Manhwa Sticher** là một ứng dụng desktop dùng **Tauri v2**, **SvelteKit** và **Rust** để tạo công cụ cắt ảnh manhwa thành nhiều trang mà không bị mất thông tin.

Dự án hướng đến việc giúp creator và người đọc **xử lý ảnh manhwa nhanh hơn**, tự động hơn và dễ dùng hơn.

## 🚀 Tính năng

- ✂️ Tự động cắt ảnh manhwa thành nhiều trang
- 🎯 Thuật toán SmartStitch tối ưu hóa việc phát hiện điểm cắt
- 🖼️ Xử lý batch nhiều ảnh cùng lúc
- ⚙️ Giao diện tab cho trang chủ và cài đặt
- 🎨 Preset thuật toán có sẵn
- 💾 Lưu và quản lý file output dễ dàng

## 📋 Yêu cầu hệ thống

### Development

- **Node.js** >= 18.0
- **pnpm** >= 9.0
- **Rust** >= 1.70
- **Platform-specific dependencies**:
  - **Windows**: Microsoft Visual Studio C++ build tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: libwebkit2gtk-4.1-dev, libappindicator3-dev, librsvg2-dev, patchelf

### Linux Dependencies

Trên Ubuntu/Debian:
```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

Trên Fedora:
```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel
```

Trên Arch:
```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg
```

## 🛠️ Cài đặt và Development

### 1. Clone repository

```bash
git clone https://github.com/jhin1m/manhwa-sticher.git
cd manhwa-sticher
```

### 2. Cài đặt dependencies

```bash
pnpm install
```

### 3. Chạy development mode

```bash
pnpm tauri dev
```

Lệnh này sẽ:
- Tự động start Vite dev server (port 1420)
- Build Rust backend
- Mở ứng dụng desktop với hot-reload

### 4. Các lệnh khác

#### Frontend (SvelteKit)
```bash
pnpm dev              # Start Vite dev server
pnpm build            # Build frontend for production
pnpm preview          # Preview production build
pnpm check            # Type checking
pnpm check:watch      # Type checking (watch mode)
```

#### Backend (Rust)
```bash
cd src-tauri
cargo build           # Build Rust backend
cargo test            # Run tests
cargo check           # Check code without building
```

#### Tauri
```bash
pnpm tauri dev        # Development mode
pnpm tauri build      # Build production app
```

## 📦 Build Production

### Build cho platform hiện tại

```bash
pnpm tauri build
```

Build artifacts sẽ nằm trong `src-tauri/target/release/bundle/`:

- **Windows**: `.msi`, `.exe` trong `bundle/msi/` và `bundle/nsis/`
- **macOS**: `.app`, `.dmg` trong `bundle/macos/` và `bundle/dmg/`
- **Linux**: `.deb`, `.AppImage` trong `bundle/deb/` và `bundle/appimage/`

### Build cho các platform cụ thể

#### macOS (Universal Binary)
```bash
pnpm tauri build -- --target universal-apple-darwin
```

#### macOS (Apple Silicon)
```bash
pnpm tauri build -- --target aarch64-apple-darwin
```

#### macOS (Intel)
```bash
pnpm tauri build -- --target x86_64-apple-darwin
```

#### Linux (cross-compile)
```bash
# Cần cài đặt target trước
rustup target add x86_64-unknown-linux-gnu
pnpm tauri build -- --target x86_64-unknown-linux-gnu
```

## 🚢 Release Process

### Tự động release với GitHub Actions

Project đã được cấu hình với GitHub Actions để tự động build và release cho tất cả platforms.

#### 1. Cập nhật version

Sử dụng script tự động (khuyên dùng):
```bash
pnpm version:bump 0.2.0
```

Hoặc cập nhật manual trong 3 file:
- `package.json`: `"version": "0.2.0"`
- `src-tauri/tauri.conf.json`: `"version": "0.2.0"`
- `src-tauri/Cargo.toml`: `version = "0.2.0"`

#### 2. Commit và tag

```bash
git add .
git commit -m "chore: bump version to 0.2.0"
git tag v0.2.0
git push origin main
git push origin v0.2.0
```

#### 3. Tự động build

Khi push tag `v*`, GitHub Actions sẽ tự động:
- Build cho Windows (x64)
- Build cho macOS (Intel + Apple Silicon)
- Build cho Linux (x64)
- Tạo draft release với tất cả artifacts
- Upload các file build lên GitHub Releases

#### 4. Publish release

- Vào tab **Releases** trên GitHub
- Sửa release notes nếu cần
- Click **Publish release**

### Manual release (local)

Nếu muốn build manual:

```bash
# 1. Build cho platform hiện tại
pnpm tauri build

# 2. Tìm artifacts trong src-tauri/target/release/bundle/

# 3. Upload lên GitHub Releases manually
```

### Chi tiết Release Process

Xem hướng dẫn chi tiết tại [RELEASE.md](./RELEASE.md) để biết thêm về:
- Troubleshooting
- Best practices
- CI/CD pipeline
- Version naming conventions

## 📁 Cấu trúc project

```
manhwa-sticher/
├── .github/
│   └── workflows/
│       ├── ci.yml          # CI workflow (test on every push)
│       └── release.yml     # Release workflow (build on tag)
├── src/                    # SvelteKit frontend
│   ├── routes/
│   │   ├── +layout.ts     # Disable SSR
│   │   └── +page.svelte   # Main page
│   └── lib/               # Components, utilities
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Entry point
│   │   └── lib.rs         # Tauri commands
│   ├── tauri.conf.json    # Tauri config
│   └── Cargo.toml         # Rust dependencies
├── static/                # Static assets
└── CLAUDE.md              # Development guide
```

## 🔧 Kiến trúc

### Frontend
- **Framework**: SvelteKit + Svelte 5 (runes syntax)
- **Mode**: SPA (SSR disabled)
- **Dev Server**: Vite port 1420
- **Build Output**: `build/` directory

### Backend
- **Language**: Rust
- **Framework**: Tauri v2
- **Commands**: Defined in `src-tauri/src/lib.rs`
- **Image Processing**: `image` crate

### Communication
Frontend gọi Rust commands qua `@tauri-apps/api/core`:
```typescript
import { invoke } from "@tauri-apps/api/core";
const result = await invoke("command_name", { args });
```

## 📝 License

MIT

## 🤝 Contributing

Pull requests are welcome! Vui lòng tạo issue trước khi bắt đầu làm feature lớn.

## 📮 Contact

- GitHub: [@jhin1m](https://github.com/jhin1m)
- Issues: [GitHub Issues](https://github.com/jhin1m/manhwa-sticher/issues)
