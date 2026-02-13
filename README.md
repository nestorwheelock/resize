# resize

A fast, parallel CLI tool that resizes images for social media sharing. Built in Rust with [rayon](https://github.com/rayon-rs/rayon) for multi-core processing.

## What it does

- Scans a directory for images (PNG, JPG, WebP, BMP, GIF, TIFF, AVIF)
- Scales the longest side down to 1200px while maintaining aspect ratio
- Saves output as PNG (default) or WebP to a `resized/` subdirectory
- Skips images already 1200px or smaller (no upscaling)
- Processes all images in parallel across CPU cores

## Download

Pre-built binaries are available on the [Releases](https://github.com/nestorwheelock/resize/releases) page:

| Platform | Binary |
|----------|--------|
| Linux x86_64 | `resize-linux-x86_64` |
| Windows x86_64 | `resize-windows-x86_64.exe` |

### Linux

```bash
curl -L https://github.com/nestorwheelock/resize/releases/latest/download/resize-linux-x86_64 -o resize
chmod +x resize
sudo mv resize /usr/local/bin/
```

### Windows

Download `resize-windows-x86_64.exe` from the [Releases](https://github.com/nestorwheelock/resize/releases) page and place it somewhere in your PATH.

## Usage

```
resize [OPTIONS] [PATH]
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `-f`, `--format <FORMAT>` | Output format: `png` or `webp` | `png` |
| `-h`, `--help` | Print help | |

### Examples

Resize all images in the current directory (output as PNG):

```bash
resize
```

Resize all images in a specific directory:

```bash
resize /home/user/Photos
```

Output as WebP instead of PNG:

```bash
resize -f webp /home/user/Photos
```

Output is saved to a `resized/` subdirectory inside the target path:

```
/home/user/Photos/
├── photo1.jpg
├── photo2.png
├── screenshot.webp
├── diagram.bmp
└── resized/
    ├── photo1.png
    ├── photo2.png
    ├── screenshot.png
    └── diagram.png
```

### Supported formats

| Input | Output |
|-------|--------|
| `.png` | `.png` or `.webp` |
| `.jpg` / `.jpeg` | `.png` or `.webp` |
| `.webp` | `.png` or `.webp` |
| `.bmp` | `.png` or `.webp` |
| `.gif` | `.png` or `.webp` |
| `.tiff` / `.tif` | `.png` or `.webp` |
| `.avif` | `.png` or `.webp` |

## Build from source

Requires [Rust](https://rustup.rs/) 1.85+.

```bash
git clone https://github.com/nestorwheelock/resize.git
cd resize
cargo build --release
```

The binary will be at `target/release/resize`.

### Cross-compile for Windows (from Linux)

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## License

GPL-3.0-or-later. See [LICENSE](LICENSE) for the full text.
