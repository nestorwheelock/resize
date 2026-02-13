// Copyright (C) 2026  Nestor Wheelock
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{Parser, ValueEnum};
use image::ImageReader;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Png,
    Webp,
    Jpg,
    Bmp,
    Gif,
    Tiff,
    Avif,
}

const INPUT_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "webp", "bmp", "gif", "tiff", "tif", "avif",
];

#[derive(Parser)]
#[command(
    about = "Resize images to social-media-friendly dimensions",
    long_about = "Resize images to social-media-friendly dimensions.\n\n\
        Scans a directory for image files and resizes any with a longest side \
        greater than 1200px, maintaining the original aspect ratio. Images \
        already 1200px or smaller are saved without resizing. All images are \
        processed in parallel for speed. Original files are never modified.",
    after_long_help = "\
EXAMPLES:
    Resize all images in the current directory to PNG:
        resize

    Resize images in a specific folder:
        resize /home/user/Photos

    Output as WebP (smaller files, great for uploading):
        resize -f webp

    Output as JPG:
        resize -f jpg /home/user/Photos

    Combine options:
        resize -f avif /home/user/Vacation

SUPPORTED INPUT FORMATS:
    PNG (.png), JPEG (.jpg, .jpeg), WebP (.webp), BMP (.bmp),
    GIF (.gif), TIFF (.tiff, .tif), AVIF (.avif)

OUTPUT FORMATS:
    png (default), webp, jpg, bmp, gif, tiff, avif

HOW IT WORKS:
    1. Reads all supported image files from the given directory
    2. Resizes images with longest side > 1200px (Lanczos3 filter)
    3. Saves results to a 'resized/' subdirectory
    4. Original files are never touched

QUICK START:
    Open your file manager, navigate to a folder with images,
    right-click and select \"Open in Terminal\", then type: resize

    On Windows, open File Explorer, click the address bar,
    type 'cmd' and press Enter, then type: resize

HOMEPAGE:
    https://github.com/nestorwheelock/resize"
)]
struct Args {
    /// Directory containing images to resize (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Png)]
    format: OutputFormat,
}

const MAX_SIDE: u32 = 1200;

fn main() {
    let args = Args::parse();
    let source = &args.path;
    let ext = match args.format {
        OutputFormat::Png => "png",
        OutputFormat::Webp => "webp",
        OutputFormat::Jpg => "jpg",
        OutputFormat::Bmp => "bmp",
        OutputFormat::Gif => "gif",
        OutputFormat::Tiff => "tiff",
        OutputFormat::Avif => "avif",
    };

    if !source.is_dir() {
        eprintln!("Error: {} is not a directory", source.display());
        std::process::exit(1);
    }

    let resized_dir = source.join("resized");
    if let Err(e) = fs::create_dir_all(&resized_dir) {
        eprintln!("Error creating resized directory: {e}");
        std::process::exit(1);
    }

    let entries = match fs::read_dir(source) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory: {e}");
            std::process::exit(1);
        }
    };

    let image_paths: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| INPUT_EXTENSIONS.contains(&e.to_lowercase().as_str()))
                    .unwrap_or(false)
        })
        .collect();

    if image_paths.is_empty() {
        println!("No images found in {}", source.display());
        return;
    }

    let count = AtomicU32::new(0);

    image_paths.par_iter().for_each(|path| {
        let file_stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => return,
        };

        let reader = match ImageReader::open(path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Skipping {}: {e}", path.display());
                return;
            }
        };
        let img = match reader.decode() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Skipping {}: {e}", path.display());
                return;
            }
        };

        let (w, h) = (img.width(), img.height());
        let longest = w.max(h);

        let output = if longest > MAX_SIDE {
            let scale = MAX_SIDE as f64 / longest as f64;
            let new_w = (w as f64 * scale).round() as u32;
            let new_h = (h as f64 * scale).round() as u32;
            println!("{file_stem}: {w}x{h} -> {new_w}x{new_h} ({ext})");
            img.resize(new_w, new_h, image::imageops::FilterType::Lanczos3)
        } else {
            println!("{file_stem}: {w}x{h} (no resize needed, saved as {ext})");
            img
        };

        let out_path = resized_dir.join(format!("{file_stem}.{ext}"));
        if let Err(e) = output.save(&out_path) {
            eprintln!("Error saving {}: {e}", out_path.display());
            return;
        }

        count.fetch_add(1, Ordering::Relaxed);
    });

    let total = count.load(Ordering::Relaxed);
    println!("Done! {total} image(s) saved to {}", resized_dir.display());
}
