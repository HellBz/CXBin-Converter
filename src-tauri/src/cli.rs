use std::path::Path;
use std::process;

use clap::Parser;
use serde::Serialize;

use crate::cxbin::{load_cxbin, CxbinMesh};
use crate::export::{export_mesh, supported_formats};

#[derive(Parser)]
#[command(name = "cxbin-converter")]
#[command(about = "Convert Creality CXBin files to common 3D formats")]
struct CliArgs {
    /// Input .cxbin file or folder
    input: Option<String>,

    /// Target format (stl, ply, obj, off, 3mf, amf, vrml, x3d)
    #[arg(short, long, default_value = "stl")]
    format: String,

    /// Output directory
    #[arg(short, long)]
    output: Option<String>,

    /// Output name template. Use {stem} for input filename and {fmt} for format
    #[arg(long)]
    output_name: Option<String>,

    /// Recursively convert all .cxbin files in the input folder
    #[arg(short, long)]
    recursive: bool,

    /// Print result as JSON instead of plain text
    #[arg(long)]
    json: bool,

    /// Include geometry arrays in JSON output
    #[arg(long)]
    json_geometry: bool,

    /// Print all supported formats and exit
    #[arg(long)]
    list_formats: bool,

    /// Export the input to every supported format (useful for testing)
    #[arg(long)]
    all_formats: bool,
}

#[derive(Serialize)]
struct CliResult {
    success: bool,
    input: String,
    format: String,
    outputs: Vec<String>,
    stats: Stats,
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geometry: Option<GeometryJson>,
}

#[derive(Serialize)]
struct Stats {
    vertices: usize,
    faces: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    texture_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compressed_bytes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uncompressed_bytes: Option<usize>,
}

#[derive(Serialize)]
struct GeometryJson {
    vertices: Vec<[f32; 3]>,
    faces: Vec<[i32; 3]>,
}

/// If command-line arguments are present, run a headless conversion and exit.
/// This enables dragging a .cxbin file onto the EXE.
pub fn try_cli_mode() {
    let raw_args: Vec<String> = std::env::args().collect();
    if raw_args.len() <= 1 {
        return;
    }

    // Support legacy positional format argument: cxbin-converter.exe file.cxbin ply
    let args = if raw_args.len() == 3 && !raw_args[2].starts_with('-') {
        let mut extended = raw_args.clone();
        extended.insert(2, "--format".to_string());
        extended
    } else {
        raw_args.clone()
    };

    let cli = CliArgs::parse_from(args);

    if cli.list_formats {
        let formats = supported_formats();
        if cli.json {
            println!("{}", serde_json::to_string_pretty(&formats).unwrap());
        } else {
            println!("Unterstützte Formate:");
            for f in formats {
                println!("  - {}", f);
            }
        }
        process::exit(0);
    }

    let Some(ref input) = cli.input else {
        if cli.json {
            println!("{{\"error\":\"No input provided.\"}}");
        } else {
            println!("Fehler: Keine Eingabedatei angegeben.");
            println!("Nutze --help für alle Optionen.");
        }
        process::exit(1);
    };

    let input_path = Path::new(&input);
    let results = if input_path.is_dir() {
        convert_folder(&cli, input_path)
    } else if cli.all_formats {
        convert_all_formats(&cli, input_path)
    } else {
        vec![convert_one(&cli, input_path)]
    };

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
    } else {
        for r in &results {
            if r.success {
                println!("✅ Erfolgreich exportiert:");
                println!("   🔸 Format:        {}", r.format.to_uppercase());
                println!("   🔸 Ziel:          {}", r.outputs.join(", "));
                println!("   🔸 Vertices:      {}", r.stats.vertices);
                println!("   🔸 Faces:         {}", r.stats.faces);
                if let Some(t) = r.stats.texture_count {
                    println!("   🔸 Texturen:      {}", t);
                }
                if let Some(b) = r.stats.compressed_bytes {
                    println!("   🔸 Komprimiert:   {} Bytes", b);
                }
                if let Some(b) = r.stats.uncompressed_bytes {
                    println!("   🔸 Dekomprimiert: {} Bytes", b);
                }
            } else {
                eprintln!(
                    "❌ Fehler bei {}: {}",
                    r.input,
                    r.error.as_deref().unwrap_or("unbekannter Fehler")
                );
            }
        }
    }

    let exit_code = if results.iter().all(|r| r.success) { 0 } else { 1 };
    process::exit(exit_code);
}

fn convert_all_formats(cli: &CliArgs, input_path: &Path) -> Vec<CliResult> {
    let mut results = Vec::new();
    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("out");
    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));

    for fmt in supported_formats() {
        let args = CliArgs {
            input: Some(input_path.to_string_lossy().to_string()),
            format: fmt.to_string(),
            output: Some(parent.join(format!("{}_{}", stem, fmt)).to_string_lossy().to_string()),
            output_name: None,
            recursive: false,
            json: cli.json,
            json_geometry: cli.json_geometry,
            list_formats: false,
            all_formats: false,
        };
        results.push(convert_one(&args, input_path));
    }
    results
}

fn convert_folder(cli: &CliArgs, input_path: &Path) -> Vec<CliResult> {
    let input = input_path.to_string_lossy();
    let mut files = Vec::new();
    let pattern = format!("{}/*.cxbin", input);
    for entry in glob::glob(&pattern).unwrap_or_else(|_| glob::glob("*.cxbin").unwrap()) {
        if let Ok(path) = entry {
            files.push(path);
        }
    }
    if cli.recursive {
        let pattern = format!("{}/**/*.cxbin", input);
        let recursive: Vec<_> = glob::glob(&pattern)
            .unwrap_or_else(|_| glob::glob("*.cxbin").unwrap())
            .filter_map(|e| e.ok())
            .collect();
        for path in recursive {
            if !files.contains(&path) {
                files.push(path);
            }
        }
    }
    files.into_iter().map(|p| convert_one(cli, &p)).collect()
}

fn mesh_stats(mesh: &CxbinMesh) -> Stats {
    Stats {
        vertices: mesh.vertex_count(),
        faces: mesh.face_count(),
        texture_count: mesh.materials.as_ref().map(|m| m.textures.len()),
        compressed_bytes: mesh.compressed_bytes,
        uncompressed_bytes: mesh.uncompressed_bytes,
    }
}

fn convert_one(cli: &CliArgs, input_path: &Path) -> CliResult {
    let format = cli.format.to_lowercase();
    if !supported_formats().contains(&format.as_str()) {
        return CliResult {
            success: false,
            input: input_path.to_string_lossy().to_string(),
            format,
            outputs: Vec::new(),
            stats: Stats {
                vertices: 0,
                faces: 0,
                texture_count: None,
                compressed_bytes: None,
                uncompressed_bytes: None,
            },
            error: Some(format!("unsupported format: {}", cli.format)),
            geometry: None,
        };
    }

    let mesh = match load_cxbin(input_path) {
        Ok(m) => m,
        Err(e) => {
            return CliResult {
                success: false,
                input: input_path.to_string_lossy().to_string(),
                format,
                outputs: Vec::new(),
                stats: Stats {
                    vertices: 0,
                    faces: 0,
                    texture_count: None,
                    compressed_bytes: None,
                    uncompressed_bytes: None,
                },
                error: Some(e.to_string()),
                geometry: None,
            }
        }
    };

    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("out");
    let parent = cli
        .output
        .as_ref()
        .map(|o| Path::new(o).to_path_buf())
        .unwrap_or_else(|| {
            input_path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf()
        });

    let output_name = cli
        .output_name
        .as_ref()
        .map(|n| n.replace("{stem}", stem).replace("{fmt}", &format))
        .unwrap_or_else(|| stem.to_string());

    let output_path = if format == "obj" {
        let out_dir = parent.join(format!("{}_obj", output_name));
        std::fs::create_dir_all(&out_dir).ok();
        out_dir.join(format!("{}.obj", output_name))
    } else {
        parent.join(format!("{}.{}", output_name, format))
    };

    let geometry = if cli.json_geometry {
        Some(GeometryJson {
            vertices: mesh.vertices.clone(),
            faces: mesh.faces.clone(),
        })
    } else {
        None
    };

    match export_mesh(&mesh, &format, &output_path) {
        Ok(outputs) => CliResult {
            success: true,
            input: input_path.to_string_lossy().to_string(),
            format,
            outputs,
            stats: mesh_stats(&mesh),
            error: None,
            geometry,
        },
        Err(e) => CliResult {
            success: false,
            input: input_path.to_string_lossy().to_string(),
            format,
            outputs: Vec::new(),
            stats: mesh_stats(&mesh),
            error: Some(e.to_string()),
            geometry,
        },
    }
}
