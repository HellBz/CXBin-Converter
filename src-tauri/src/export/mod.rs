pub mod amf;
pub mod dae;
pub mod glb;
pub mod gltf;
pub mod mesh;
pub mod obj;
pub mod off;
pub mod ply;
pub mod stl;
pub mod threemf;
pub mod vrml;
pub mod x3d;

use std::path::Path;

use crate::cxbin::CxbinMesh;
use amf::AmfExporter;
use dae::DaeExporter;
use glb::GlbExporter;
use gltf::GltfExporter;
use obj::ObjExporter;
use off::OffExporter;
use ply::PlyExporter;
use stl::StlExporter;
use threemf::ThreeMFExporter;
use vrml::VrmlExporter;
use x3d::X3dExporter;

pub use mesh::Exporter;

pub fn exporter_for(format: &str) -> Option<Box<dyn Exporter>> {
    match format.to_lowercase().as_str() {
        "stl" => Some(Box::new(StlExporter)),
        "ply" => Some(Box::new(PlyExporter)),
        "obj" => Some(Box::new(ObjExporter)),
        "off" => Some(Box::new(OffExporter)),
        "3mf" => Some(Box::new(ThreeMFExporter)),
        "amf" => Some(Box::new(AmfExporter)),
        "vrml" => Some(Box::new(VrmlExporter)),
        "x3d" => Some(Box::new(X3dExporter)),
        "dae" => Some(Box::new(DaeExporter)),
        "glb" => Some(Box::new(GlbExporter)),
        "gltf" => Some(Box::new(GltfExporter)),
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
    vec!["stl", "ply", "obj", "off", "3mf", "amf", "vrml", "x3d", "dae", "glb", "gltf"]
}
