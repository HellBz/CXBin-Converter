use std::path::Path;

use serde::Serialize;
use tauri::command;

use crate::cxbin::load_cxbin;
use crate::export::export_mesh;

#[derive(Serialize)]
pub struct ConversionResult {
    success: bool,
    input: String,
    format: String,
    outputs: Vec<String>,
    stats: Stats,
    error: Option<String>,
}

#[derive(Serialize)]
pub struct Stats {
    vertices: usize,
    faces: usize,
    compressed_bytes: Option<usize>,
    uncompressed_bytes: Option<usize>,
}

#[command]
pub fn convert_cxbin(
    input: String,
    format: String,
    output_folder: Option<String>,
) -> Result<ConversionResult, String> {
    let input_path = Path::new(&input);
    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("out");
    let parent = output_folder
        .as_ref()
        .map(|f| Path::new(f).to_path_buf())
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| {
            input_path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf()
        });

    let format = format.to_lowercase();

    let output_path = if format == "obj" {
        let out_dir = parent.join(format!("{}_obj", stem));
        std::fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
        out_dir.join(format!("{}.obj", stem))
    } else {
        parent.join(format!("{}.{}", stem, format))
    };

    let mesh = match load_cxbin(input_path) {
        Ok(m) => m,
        Err(e) => {
            return Ok(ConversionResult {
                success: false,
                input,
                format,
                outputs: Vec::new(),
                stats: Stats {
                    vertices: 0,
                    faces: 0,
                    compressed_bytes: None,
                    uncompressed_bytes: None,
                },
                error: Some(e.to_string()),
            });
        }
    };

    let stats = Stats {
        vertices: mesh.vertex_count(),
        faces: mesh.face_count(),
        compressed_bytes: mesh.compressed_bytes,
        uncompressed_bytes: mesh.uncompressed_bytes,
    };

    match export_mesh(&mesh, &format, &output_path) {
        Ok(outputs) => Ok(ConversionResult {
            success: true,
            input,
            format,
            outputs,
            stats,
            error: None,
        }),
        Err(e) => Ok(ConversionResult {
            success: false,
            input,
            format,
            outputs: Vec::new(),
            stats,
            error: Some(e.to_string()),
        }),
    }
}

#[derive(Serialize)]
pub struct GeometryData {
    pub vertices: Vec<[f32; 3]>,
    pub faces: Vec<[i32; 3]>,
    pub vertex_count: usize,
    pub face_count: usize,
}

#[command]
pub fn get_geometry(input: String) -> Result<GeometryData, String> {
    let mesh = load_cxbin(&input).map_err(|e| e.to_string())?;
    Ok(GeometryData {
        vertex_count: mesh.vertex_count(),
        face_count: mesh.face_count(),
        vertices: mesh.vertices,
        faces: mesh.faces,
    })
}

#[command]
pub fn supported_formats() -> Vec<String> {
    crate::export::supported_formats()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}
