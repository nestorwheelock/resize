// Copyright (C) 2026  nwheelo
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use image::ImageReader;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

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

    let image_paths: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| matches!(e.to_lowercase().as_str(), "png" | "jpg" | "jpeg"))
                    .unwrap_or(false)
        })
        .collect();

    if image_paths.is_empty() {
        println!("No JPG/PNG images found in {}", source.display());
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
            println!("{file_stem}: {w}x{h} -> {new_w}x{new_h}");
            img.resize(new_w, new_h, image::imageops::FilterType::Lanczos3)
        } else {
            println!("{file_stem}: {w}x{h} (no resize needed)");
            img
        };

        let out_path = resized_dir.join(format!("{file_stem}.png"));
        if let Err(e) = output.save(&out_path) {
            eprintln!("Error saving {}: {e}", out_path.display());
            return;
        }

        count.fetch_add(1, Ordering::Relaxed);
    });

    let total = count.load(Ordering::Relaxed);
    println!("Done! {total} image(s) saved to {}", resized_dir.display());
}
