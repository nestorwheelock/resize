# resize

A fast, parallel CLI tool that resizes images for social media sharing. Built in Rust with [rayon](https://github.com/rayon-rs/rayon) for multi-core processing.

## What it does

- Scans a directory for images (PNG, JPG, WebP, BMP, GIF, TIFF, AVIF)
- Scales the longest side down to 1200px while maintaining aspect ratio
- Saves output in your choice of format (PNG, WebP, JPG, BMP, GIF, TIFF, AVIF)
- Defaults to PNG output
- Skips images already 1200px or smaller (no upscaling)
- Processes all images in parallel across CPU cores

## Download

Pre-built binaries are available on the [Releases](https://github.com/nestorwheelock/resize/releases) page:

| Platform | Binary |
|----------|--------|
| Linux x86_64 | `resize-linux-x86_64` |
| Windows x86_64 | `resize-windows-x86_64.exe` |

### Installing on Linux

Open a terminal (you can use Ctrl+Alt+T on most Linux desktops) and paste these three lines:

```bash
curl -L https://github.com/nestorwheelock/resize/releases/latest/download/resize-linux-x86_64 -o resize
chmod +x resize
sudo mv resize /usr/local/bin/
```

That's it. The `resize` command is now available everywhere on your system.

### Installing on Windows

1. Go to the [Releases](https://github.com/nestorwheelock/resize/releases) page
2. Download `resize-windows-x86_64.exe`
3. Rename it to `resize.exe`
4. Move it to a folder that's in your PATH (for example `C:\Windows\` or create a `C:\Tools\` folder and [add it to your PATH](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/))

## Getting started — a quick guide

Don't worry if you're new to the terminal. Using `resize` is simpler than opening a photo editor and clicking through menus for each image. One command handles everything at once.

### Step 1: Open a terminal in your images folder

You don't need to memorize any paths. Just navigate to the folder where your images are and open a terminal right there:

**Linux (GNOME/Nautilus file manager):**
- Open your file manager and go to the folder with your images
- Right-click on an empty area in the folder
- Select **"Open in Terminal"**

**Linux (Dolphin/KDE file manager):**
- Open the folder with your images
- Right-click on an empty area
- Select **"Open Terminal Here"**

**Windows:**
- Open File Explorer and go to your images folder
- Click the address bar at the top, type `cmd`, and press Enter
- A terminal will open already pointed at that folder

### Step 2: Run resize

Now just type:

```bash
resize
```

That's it. The tool will find all your images, resize them, and save the results into a new `resized/` folder right next to your originals. Your original files are never modified.

You'll see output like this:

```
photo1: 4008x3008 -> 1200x901 (png)
photo2: 3015x1390 -> 1200x553 (png)
vacation: 800x600 (no resize needed, saved as png)
Done! 3 image(s) saved to /home/user/Photos/resized
```

### Step 3: Find your resized images

Open the `resized/` folder that appeared inside your images folder. All your social-media-ready images are there.

```
Your folder/
├── photo1.jpg          (original, untouched)
├── photo2.png          (original, untouched)
├── vacation.webp       (original, untouched)
└── resized/
    ├── photo1.png      (resized copy)
    ├── photo2.png      (resized copy)
    └── vacation.png    (resized copy)
```

## Examples

### Default (PNG output)

```bash
resize
resize /home/user/Photos
```

### WebP output (smaller files, great for uploading)

```bash
resize -f webp
resize -f webp /home/user/Photos
```

### JPG output

```bash
resize -f jpg
resize -f jpg /home/user/Vacation
```

### Other formats

```bash
resize -f avif /home/user/Photos     # AVIF (very small files)
resize -f tiff /home/user/Photos     # TIFF (lossless, large files)
resize -f bmp /home/user/Photos      # BMP (uncompressed)
resize -f gif /home/user/Photos      # GIF
```

### Re-process images (overwrite)

By default, `resize` skips images that already exist in the `resized/` folder. This makes it fast to run repeatedly — only new images get processed. To force re-processing:

```bash
resize --overwrite
resize -o /home/user/Photos
```

### Getting help

```bash
resize -h          # Short help summary
resize --help      # Detailed help with examples and supported formats
```

## Usage reference

```
resize [OPTIONS] [PATH]
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `-f`, `--format <FORMAT>` | Output format: `png`, `webp`, `jpg`, `bmp`, `gif`, `tiff`, `avif` | `png` |
| `-o`, `--overwrite` | Overwrite images that already exist in resized/ | off |
| `-h` | Print short help summary | |
| `--help` | Print detailed help with examples | |

### Supported formats

| Format | Input extensions | Available as output |
|--------|-----------------|-------------------|
| PNG | `.png` | yes (default) |
| JPEG | `.jpg`, `.jpeg` | yes (`-f jpg`) |
| WebP | `.webp` | yes (`-f webp`) |
| BMP | `.bmp` | yes (`-f bmp`) |
| GIF | `.gif` | yes (`-f gif`) |
| TIFF | `.tiff`, `.tif` | yes (`-f tiff`) |
| AVIF | `.avif` | yes (`-f avif`) |

## Man page

A man page is included for Linux. To install it:

```bash
sudo cp man/resize.1 /usr/local/share/man/man1/
sudo mandb -q
man resize
```

Or use the built-in detailed help:

```bash
resize --help
```

## Build from source

Requires [Rust](https://rustup.rs/) 1.85+.

```bash
git clone https://github.com/nestorwheelock/resize.git
cd resize
cargo build --release
```

The binary will be at `target/release/resize`.

### Using the Makefile

```bash
make              # Build Linux + Windows and update man page
make install      # Build, install binary and man page to system
make release      # Build and create GitHub release
make clean        # Clean build artifacts
```

### Cross-compile for Windows (from Linux)

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## License

GPL-3.0-or-later. See [LICENSE](LICENSE) for the full text.
