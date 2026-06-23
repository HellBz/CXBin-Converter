use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct MshExporter;

impl Exporter for MshExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let mut file = File::create(output_path)?;
        file.write_all(msh_text(mesh).as_bytes())?;
        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}

fn msh_text(mesh: &CxbinMesh) -> String {
    let vertex_count = mesh.vertex_count();
    let face_count = mesh.face_count();

    let mut out = String::new();
    out.push_str("$MeshFormat\n");
    out.push_str("2.2 0 8\n");
    out.push_str("$EndMeshFormat\n");
    out.push_str(&format!("$Nodes\n{}\n", vertex_count));
    for (i, v) in mesh.vertices.iter().enumerate() {
        out.push_str(&format!("{} {} {} {}\n", i + 1, v[0], v[1], v[2]));
    }
    out.push_str("$EndNodes\n");
    out.push_str(&format!("$Elements\n{}\n", face_count));
    for (i, f) in mesh.faces.iter().enumerate() {
        out.push_str(&format!(
            "{} 2 2 0 0 {} {} {}\n",
            i + 1,
            f[0] + 1,
            f[1] + 1,
            f[2] + 1
        ));
    }
    out.push_str("$EndElements\n");
    out
}
