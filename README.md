# resize

A fast CLI tool that resizes JPG and PNG images for social media sharing. Built in Rust.

## What it does

- Scans a directory for JPG and PNG images
- Scales the longest side down to 1200px while maintaining aspect ratio
- Saves all output as PNG to a `resized/` subdirectory
- Skips images already 1200px or smaller (no upscaling)

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
resize [PATH]
```

### Examples

Resize all images in the current directory:

```bash
resize
```

Resize all images in a specific directory:

```bash
resize /home/user/Photos
```

Output is saved to a `resized/` subdirectory inside the target path:

```
/home/user/Photos/
├── photo1.jpg
├── photo2.png
├── vacation.jpeg
└── resized/
    ├── photo1.png
    ├── photo2.png
    └── vacation.png
```

### Supported formats

| Input | Output |
|-------|--------|
| `.png` | `.png` |
| `.jpg` | `.png` |
| `.jpeg` | `.png` |

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
