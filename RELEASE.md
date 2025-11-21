# 🚢 Release Guide

Hướng dẫn chi tiết về quy trình release cho Manhwa Sticher.

## 📝 Quy trình Release

### 1. Chuẩn bị Release

#### Kiểm tra code
```bash
# Chạy tests
pnpm check
cd src-tauri && cargo test

# Build thử local
pnpm tauri build
```

#### Cập nhật changelog (nếu có)
Ghi lại các thay đổi quan trọng trong version mới.

### 2. Bump Version

#### Tự động (khuyên dùng)
```bash
pnpm version:bump 0.2.0
```

Script sẽ tự động cập nhật version trong:
- `package.json`
- `src-tauri/tauri.conf.json`
- `src-tauri/Cargo.toml`

#### Manual
Nếu không dùng script, cập nhật version trong 3 file trên.

### 3. Commit và Tag

```bash
# Review changes
git status
git diff

# Commit
git add .
git commit -m "chore: bump version to 0.2.0"

# Tạo tag
git tag v0.2.0

# Push code
git push origin main

# Push tag (trigger GitHub Actions)
git push origin v0.2.0
```

### 4. GitHub Actions Build

Sau khi push tag, GitHub Actions sẽ tự động:

1. **Trigger workflow** `.github/workflows/release.yml`
2. **Build cho tất cả platforms**:
   - Windows x64 (.msi, .exe)
   - macOS Intel (.dmg, .app)
   - macOS Apple Silicon (.dmg, .app)
   - Linux x64 (.deb, .AppImage)
3. **Tạo draft release** trên GitHub với artifacts
4. **Upload các file build** lên release

### 5. Publish Release

1. Vào [GitHub Releases](https://github.com/jhin1m/manhwa-sticher/releases)
2. Tìm draft release mới nhất (tên: `Manhwa Sticher v0.2.0`)
3. Xem lại release notes và artifacts
4. Edit release notes nếu cần:
   ```markdown
   ## 🎉 What's New
   - Feature 1
   - Feature 2

   ## 🐛 Bug Fixes
   - Fix 1
   - Fix 2

   ## 📥 Downloads
   Chọn file tương ứng với hệ điều hành của bạn:
   - **Windows**: `.msi` hoặc `.exe`
   - **macOS**: `.dmg` (Intel hoặc Apple Silicon)
   - **Linux**: `.deb` hoặc `.AppImage`
   ```
5. Click **Publish release**

## 📦 Build Artifacts

### Windows
- `manhwa-sticher_0.2.0_x64_en-US.msi` - Windows Installer
- `manhwa-sticher_0.2.0_x64-setup.exe` - NSIS Installer

### macOS
- `manhwa-sticher_0.2.0_aarch64.dmg` - Apple Silicon (M1/M2/M3)
- `manhwa-sticher_0.2.0_x64.dmg` - Intel Macs

### Linux
- `manhwa-sticher_0.2.0_amd64.deb` - Debian/Ubuntu
- `manhwa-sticher_0.2.0_amd64.AppImage` - Universal Linux

## 🔄 Version Naming Convention

Sử dụng [Semantic Versioning](https://semver.org/):

- **MAJOR.MINOR.PATCH** (ví dụ: `1.2.3`)
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Ví dụ
- `0.1.0` → `0.2.0`: Thêm tính năng mới
- `0.2.0` → `0.2.1`: Fix bugs
- `0.2.1` → `1.0.0`: Release chính thức, có breaking changes
- `1.0.0` → `1.0.1-beta.1`: Pre-release version

## 🚨 Troubleshooting

### Build fails trên GitHub Actions

#### Kiểm tra logs
1. Vào tab **Actions** trên GitHub
2. Click vào workflow run bị failed
3. Xem logs của từng job (Windows/macOS/Linux)

#### Lỗi thường gặp

**1. Frontend build fails**
```
Error: TypeScript errors
```
**Fix**: Chạy `pnpm check` local và fix errors trước khi release

**2. Rust build fails**
```
Error: cargo build failed
```
**Fix**: Chạy `cd src-tauri && cargo test` local để check

**3. Tauri build fails**
```
Error: Failed to bundle application
```
**Fix**: Check `tauri.conf.json` và ensure icons tồn tại

### Tag đã push nhưng không trigger workflow

**Nguyên nhân**: Tag format không đúng (phải là `v*`)

**Fix**:
```bash
# Xóa tag sai
git tag -d v0.2.0
git push origin :refs/tags/v0.2.0

# Tạo lại tag đúng format
git tag v0.2.0
git push origin v0.2.0
```

### Build local thành công nhưng GitHub Actions fails

**Nguyên nhân**: Dependency versions khác nhau

**Fix**: Ensure `pnpm-lock.yaml` được commit và dependencies đầy đủ

## 🔐 Permissions

GitHub Actions cần permission để tạo releases:

- Repository Settings → Actions → General
- Workflow permissions: **Read and write permissions**
- ✅ Allow GitHub Actions to create and approve pull requests

## 📊 CI/CD Pipeline

### CI Workflow (`.github/workflows/ci.yml`)
Chạy trên mỗi push/PR:
- Type checking
- Frontend build
- Rust tests
- Test build (không publish)

### Release Workflow (`.github/workflows/release.yml`)
Chạy khi push tag `v*`:
- Build production cho tất cả platforms
- Tạo GitHub Release
- Upload artifacts

## 🎯 Best Practices

1. **Test kỹ trước khi release**
   - Chạy `pnpm tauri build` local
   - Test ứng dụng trên platform của bạn
   - Fix bugs nếu có

2. **Version bump có kế hoạch**
   - Không nên bump version liên tục
   - Gom nhiều changes vào 1 release
   - Có release notes rõ ràng

3. **Tag naming**
   - Luôn dùng prefix `v` (ví dụ: `v0.2.0`)
   - Không dùng tag khác format này

4. **Release notes**
   - Viết rõ ràng những gì thay đổi
   - Phân loại: Features, Bug Fixes, Breaking Changes
   - Hướng dẫn user cách download/install

5. **Backup**
   - Keep draft release cho đến khi verify artifacts
   - Test download và install trước khi publish
   - Có thể rollback nếu cần

## 📚 Resources

- [Tauri Documentation](https://tauri.app/v2/guides/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Semantic Versioning](https://semver.org/)
- [Tauri Action](https://github.com/tauri-apps/tauri-action)

## ❓ Questions

Nếu có vấn đề với release process, tạo issue tại:
https://github.com/jhin1m/manhwa-sticher/issues
