use image::{DynamicImage, GenericImageView, Rgba};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri::Emitter;
use walkdir::WalkDir;

// Data structures for communication with frontend
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessSettings {
    pub split_height: u32,
    pub sensitivity: u8,
    pub scan_line_step: u32,
    pub ignorable_border: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessResult {
    pub success: bool,
    pub message: String,
    pub output_files: Vec<String>,
    pub total_images: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressUpdate {
    pub current: usize,
    pub total: usize,
    pub percentage: f32,
    pub message: String,
}

// Helper function: List image files in a directory
fn list_images(folder_path: &Path) -> Result<Vec<PathBuf>, String> {
    let mut images = Vec::new();
    let valid_extensions = ["png", "jpg", "jpeg", "webp", "bmp"];

    for entry in WalkDir::new(folder_path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if let Some(ext_str) = ext.to_str() {
                    if valid_extensions.contains(&ext_str.to_lowercase().as_str()) {
                        images.push(entry.path().to_path_buf());
                    }
                }
            }
        }
    }

    // Sort images naturally by filename
    images.sort_by(|a, b| {
        a.file_name()
            .unwrap_or_default()
            .cmp(b.file_name().unwrap_or_default())
    });

    Ok(images)
}

// Helper function: Calculate pixel difference between two adjacent pixels
fn calculate_pixel_difference(p1: &Rgba<u8>, p2: &Rgba<u8>) -> u8 {
    let r_diff = (p1[0] as i16 - p2[0] as i16).abs();
    let g_diff = (p1[1] as i16 - p2[1] as i16).abs();
    let b_diff = (p1[2] as i16 - p2[2] as i16).abs();

    ((r_diff + g_diff + b_diff) / 3) as u8
}

// Helper function: Check if a horizontal line is safe to split (minimal content)
fn is_safe_line(img: &DynamicImage, y: u32, settings: &ProcessSettings) -> bool {
    let width = img.width();
    let threshold = (255 * (100 - settings.sensitivity as u32) / 100) as u8;

    // Check horizontal line from left border to right border (excluding ignorable borders)
    let start_x = settings.ignorable_border;
    let end_x = width.saturating_sub(settings.ignorable_border + 1);

    if start_x >= end_x {
        return true; // Image too narrow, consider safe
    }

    for x in start_x..end_x {
        let pixel1 = img.get_pixel(x, y);
        let pixel2 = img.get_pixel(x + 1, y);

        let diff = calculate_pixel_difference(&pixel1, &pixel2);

        if diff > threshold {
            return false; // Found content, not safe to split
        }
    }

    true // Safe to split
}

// Helper function: Find the best split position near target height
fn find_split_position(img: &DynamicImage, target_y: u32, settings: &ProcessSettings) -> u32 {
    let mut current_y = target_y;
    let max_attempts = 1000;
    let mut attempts = 0;

    while current_y < img.height() && attempts < max_attempts {
        if is_safe_line(img, current_y, settings) {
            return current_y;
        }
        current_y += settings.scan_line_step;
        attempts += 1;
    }

    // Fallback: return current position or image height
    current_y.min(img.height() - 1)
}

// Helper function: Smart split an image into multiple parts
fn smart_split(img: &DynamicImage, settings: &ProcessSettings) -> Result<Vec<DynamicImage>, String> {
    let mut result = Vec::new();
    let mut current_y = 0;

    while current_y < img.height() {
        let target_y = current_y + settings.split_height;

        let split_y = if target_y >= img.height() {
            img.height()
        } else {
            find_split_position(img, target_y, settings)
        };

        // Crop image from current_y to split_y
        let height = split_y - current_y;
        if height == 0 {
            break; // Avoid infinite loop
        }

        let cropped = img.crop_imm(0, current_y, img.width(), height);
        result.push(cropped);

        current_y = split_y;
    }

    Ok(result)
}

// Main Tauri command: Process images
#[tauri::command]
async fn process_images(
    app_handle: tauri::AppHandle,
    input_folder: String,
    output_folder: String,
    split_height: u32,
    sensitivity: u8,
    scan_line_step: u32,
    ignorable_border: u32,
) -> Result<ProcessResult, String> {
    let settings = ProcessSettings {
        split_height,
        sensitivity,
        scan_line_step,
        ignorable_border,
    };

    // Create output folder if it doesn't exist
    let output_path = Path::new(&output_folder);
    std::fs::create_dir_all(output_path)
        .map_err(|e| format!("Failed to create output folder: {}", e))?;

    // List all images in input folder
    let input_path = Path::new(&input_folder);
    let image_files = list_images(input_path)?;

    if image_files.is_empty() {
        return Ok(ProcessResult {
            success: false,
            message: "No images found in input folder".to_string(),
            output_files: Vec::new(),
            total_images: 0,
        });
    }

    let total_images = image_files.len();
    let mut output_files = Vec::new();
    let mut output_counter = 1;

    // Process each image
    for (idx, image_path) in image_files.iter().enumerate() {
        // Emit progress update
        let progress = ProgressUpdate {
            current: idx + 1,
            total: total_images,
            percentage: ((idx + 1) as f32 / total_images as f32) * 100.0,
            message: format!("Processing image {}/{}", idx + 1, total_images),
        };

        app_handle
            .emit("processing-progress", progress)
            .map_err(|e| format!("Failed to emit progress: {}", e))?;

        // Load image
        let img = image::open(image_path)
            .map_err(|e| format!("Failed to load image {:?}: {}", image_path, e))?;

        // Smart split the image
        let split_images = smart_split(&img, &settings)?;

        // Save each split part
        for split_img in split_images {
            let output_filename = format!("output_{:03}.png", output_counter);
            let output_file_path = output_path.join(&output_filename);

            split_img
                .save(&output_file_path)
                .map_err(|e| format!("Failed to save image: {}", e))?;

            output_files.push(output_file_path.to_string_lossy().to_string());
            output_counter += 1;
        }
    }

    // Emit final progress
    app_handle
        .emit(
            "processing-progress",
            ProgressUpdate {
                current: total_images,
                total: total_images,
                percentage: 100.0,
                message: "Processing complete!".to_string(),
            },
        )
        .ok();

    Ok(ProcessResult {
        success: true,
        message: format!("Successfully processed {} images", total_images),
        output_files,
        total_images,
    })
}

// Keep the example greet command for reference
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, process_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
