use std::path::Path;
use std::process;

use serde::Serialize;

use crate::cxbin::load_cxbin;
use crate::export::export_mesh;

#[derive(Serialize)]
struct CliResult {
    success: bool,
    input: String,
    format: String,
    outputs: Vec<String>,
    stats: Stats,
    error: Option<String>,
}

#[derive(Serialize)]
struct Stats {
    vertices: usize,
    faces: usize,
}

/// If command-line arguments are present, run a headless conversion and exit.
/// This enables dragging a .cxbin file onto the EXE.
pub fn try_cli_mode() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        return;
    }

    let input = &args[1];
    let format = args.get(2).map(|s| s.as_str()).unwrap_or("stl");

    match run_cli(input, format) {
        Ok(result) => {
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn run_cli(input: &str, format: &str) -> anyhow::Result<CliResult> {
    let input_path = Path::new(input);
    let mesh = load_cxbin(input_path)?;
    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("out");
    let parent = input_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    let format = format.to_lowercase();
    let output_path = if format == "obj" {
        let out_dir = parent.join(format!("{}_obj", stem));
        std::fs::create_dir_all(&out_dir)?;
        out_dir.join(format!("{}.obj", stem))
    } else {
        parent.join(format!("{}.{}", stem, format))
    };

    let outputs = export_mesh(&mesh, &format, &output_path)?;

    Ok(CliResult {
        success: true,
        input: input.to_string(),
        format,
        outputs,
        stats: Stats {
            vertices: mesh.vertex_count(),
            faces: mesh.face_count(),
        },
        error: None,
    })
}
