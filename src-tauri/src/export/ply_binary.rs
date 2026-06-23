use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct PlyBinaryExporter;

impl Exporter for PlyBinaryExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let file = File::create(output_path)?;
        let mut w = BufWriter::new(file);

        let has_uvs = mesh.uvs.is_some() && mesh.uvs.as_ref().unwrap().len() == mesh.vertices.len();

        writeln!(w, "ply")?;
        writeln!(w, "format binary_little_endian 1.0")?;
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
                w.write_all(&v[0].to_le_bytes())?;
                w.write_all(&v[1].to_le_bytes())?;
                w.write_all(&v[2].to_le_bytes())?;
                w.write_all(&uvs[i][0].to_le_bytes())?;
                w.write_all(&uvs[i][1].to_le_bytes())?;
            }
        } else {
            for v in &mesh.vertices {
                w.write_all(&v[0].to_le_bytes())?;
                w.write_all(&v[1].to_le_bytes())?;
                w.write_all(&v[2].to_le_bytes())?;
            }
        }

        for f in &mesh.faces {
            w.write_all(&3u8.to_le_bytes())?;
            w.write_all(&f[0].to_le_bytes())?;
            w.write_all(&f[1].to_le_bytes())?;
            w.write_all(&f[2].to_le_bytes())?;
        }

        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}
