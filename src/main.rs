// Copyright (C) 2026  nwheelo
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use image::ImageReader;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Resize JPG/PNG images to social-media-friendly dimensions (output always PNG)")]
struct Args {
    /// Directory containing images to resize (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,
}

const MAX_SIDE: u32 = 1200;

fn main() {
    let args = Args::parse();
    let source = &args.path;

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

    let mut count = 0;

    for entry in entries.flatten() {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        let is_image = matches!(ext.as_deref(), Some("png" | "jpg" | "jpeg"));
        if !is_image {
            continue;
        }

        let file_stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => continue,
        };

        let reader = match ImageReader::open(&path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Skipping {}: {e}", path.display());
                continue;
            }
        };
        let img = match reader.decode() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Skipping {}: {e}", path.display());
                continue;
            }
        };

        let (w, h) = (img.width(), img.height());
        let longest = w.max(h);

        let output = if longest > MAX_SIDE {
            let scale = MAX_SIDE as f64 / longest as f64;
            let new_w = (w as f64 * scale).round() as u32;
            let new_h = (h as f64 * scale).round() as u32;
            println!("{file_stem}: {w}x{h} -> {new_w}x{new_h}");
            img.resize(new_w, new_h, image::imageops::FilterType::Lanczos3)
        } else {
            println!("{file_stem}: {w}x{h} (no resize needed)");
            img
        };

        let out_path = resized_dir.join(format!("{file_stem}.png"));
        if let Err(e) = output.save(&out_path) {
            eprintln!("Error saving {}: {e}", out_path.display());
            continue;
        }

        count += 1;
    }

    if count == 0 {
        println!("No JPG/PNG images found in {}", source.display());
    } else {
        println!("Done! {count} image(s) saved to {}", resized_dir.display());
    }
}
