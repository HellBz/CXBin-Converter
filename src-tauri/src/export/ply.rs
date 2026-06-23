use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct PlyExporter;

impl Exporter for PlyExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let file = File::create(output_path)?;
        let mut w = BufWriter::new(file);

        let has_uvs = mesh.uvs.is_some() && mesh.uvs.as_ref().unwrap().len() == mesh.vertices.len();

        writeln!(w, "ply")?;
        writeln!(w, "format ascii 1.0")?;
        writeln!(w, "element vertex {}", mesh.vertices.len())?;
        writeln!(w, "property float x")?;
        writeln!(w, "property float y")?;
        writeln!(w, "property float z")?;
        if has_uvs {
            writeln!(w, "property float s")?;
            writeln!(w, "property float t")?;
        }
        writeln!(w, "element face {}", mesh.faces.len())?;
        writeln!(w, "property list uchar int vertex_indices")?;
        writeln!(w, "end_header")?;

        if has_uvs {
            let uvs = mesh.uvs.as_ref().unwrap();
            for (i, v) in mesh.vertices.iter().enumerate() {
                writeln!(w, "{} {} {} {} {}", v[0], v[1], v[2], uvs[i][0], uvs[i][1])?;
            }
        } else {
            for v in &mesh.vertices {
                writeln!(w, "{} {} {}", v[0], v[1], v[2])?;
            }
        }

        for f in &mesh.faces {
            writeln!(w, "3 {} {} {}", f[0], f[1], f[2])?;
        }

        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}
