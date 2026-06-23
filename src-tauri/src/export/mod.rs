pub mod mesh;
pub mod obj;
pub mod off;
pub mod ply;
pub mod stl;

use std::path::Path;

use crate::cxbin::CxbinMesh;
use obj::ObjExporter;
use off::OffExporter;
use ply::PlyExporter;
use stl::StlExporter;

pub use mesh::Exporter;

pub fn exporter_for(format: &str) -> Option<Box<dyn Exporter>> {
    match format.to_lowercase().as_str() {
        "stl" => Some(Box::new(StlExporter)),
        "ply" => Some(Box::new(PlyExporter)),
        "obj" => Some(Box::new(ObjExporter)),
        "off" => Some(Box::new(OffExporter)),
        _ => None,
    }
}

pub fn export_mesh(mesh: &CxbinMesh, format: &str, output_path: &Path) -> anyhow::Result<Vec<String>> {
    let exporter = exporter_for(format).ok_or_else(|| {
        anyhow::anyhow!("unsupported export format: {}", format)
    })?;
    exporter.export(mesh, output_path)
}

pub fn supported_formats() -> Vec<&'static str> {
    vec!["stl", "ply", "obj", "off"]
}
