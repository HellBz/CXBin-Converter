pub mod amf;
pub mod dae;
pub mod dxf;
pub mod fbx;
pub mod glb;
pub mod gltf;
pub mod mesh;
pub mod msh;
pub mod obj;
pub mod off;
pub mod ply;
pub mod ply_binary;
pub mod stl;
pub mod threemf;
pub mod usdz;
pub mod vtk;
pub mod vrml;
pub mod x3d;
pub mod xyz;

use std::path::Path;

use crate::cxbin::CxbinMesh;
use amf::AmfExporter;
use dae::DaeExporter;
use dxf::DxfExporter;
use fbx::FbxExporter;
use glb::GlbExporter;
use gltf::GltfExporter;
use msh::MshExporter;
use obj::ObjExporter;
use off::OffExporter;
use ply::PlyExporter;
use ply_binary::PlyBinaryExporter;
use stl::StlExporter;
use threemf::ThreeMFExporter;
use usdz::UsdzExporter;
use vtk::VtkExporter;
use vrml::VrmlExporter;
use x3d::X3dExporter;
use xyz::XyzExporter;

pub use mesh::Exporter;

pub fn exporter_for(format: &str) -> Option<Box<dyn Exporter>> {
    match format.to_lowercase().as_str() {
        "stl" => Some(Box::new(StlExporter)),
        "ply" => Some(Box::new(PlyExporter)),
        "plyb" => Some(Box::new(PlyBinaryExporter)),
        "xyz" => Some(Box::new(XyzExporter)),
        "obj" => Some(Box::new(ObjExporter)),
        "off" => Some(Box::new(OffExporter)),
        "3mf" => Some(Box::new(ThreeMFExporter)),
        "amf" => Some(Box::new(AmfExporter)),
        "vrml" => Some(Box::new(VrmlExporter)),
        "x3d" => Some(Box::new(X3dExporter)),
        "dae" => Some(Box::new(DaeExporter)),
        "glb" => Some(Box::new(GlbExporter)),
        "gltf" => Some(Box::new(GltfExporter)),
        "vtk" => Some(Box::new(VtkExporter)),
        "msh" => Some(Box::new(MshExporter)),
        "dxf" => Some(Box::new(DxfExporter)),
        "fbx" => Some(Box::new(FbxExporter)),
        "usdz" => Some(Box::new(UsdzExporter)),
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
    vec!["stl", "ply", "plyb", "xyz", "obj", "off", "3mf", "amf", "vrml", "x3d", "dae", "glb", "gltf", "vtk", "msh", "dxf", "fbx", "usdz"]
}
