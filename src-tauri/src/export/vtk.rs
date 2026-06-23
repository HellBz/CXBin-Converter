use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct VtkExporter;

impl Exporter for VtkExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let mut file = File::create(output_path)?;
        file.write_all(vtk_text(mesh).as_bytes())?;
        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}

fn vtk_text(mesh: &CxbinMesh) -> String {
    let vertex_count = mesh.vertex_count();
    let face_count = mesh.face_count();
    let triangle_count = face_count * 3;

    let mut out = String::new();
    out.push_str("# vtk DataFile Version 3.0\n");
    out.push_str("CXBin mesh export\n");
    out.push_str("ASCII\n");
    out.push_str("DATASET UNSTRUCTURED_GRID\n");
    out.push_str(&format!("POINTS {} float\n", vertex_count));
    for v in &mesh.vertices {
        out.push_str(&format!("{} {} {}\n", v[0], v[1], v[2]));
    }
    out.push_str(&format!(
        "CELLS {} {}\n",
        face_count,
        face_count + triangle_count
    ));
    for f in &mesh.faces {
        out.push_str(&format!("3 {} {} {}\n", f[0], f[1], f[2]));
    }
    out.push_str(&format!("CELL_TYPES {}\n", face_count));
    for _ in &mesh.faces {
        out.push_str("5\n"); // VTK_TRIANGLE
    }
    out
}
