use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct OffExporter;

impl Exporter for OffExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let file = File::create(output_path)?;
        let mut w = BufWriter::new(file);

        writeln!(w, "OFF")?;
        writeln!(w, "{} {} 0", mesh.vertices.len(), mesh.faces.len())?;

        for v in &mesh.vertices {
            writeln!(w, "{} {} {}", v[0], v[1], v[2])?;
        }
        for f in &mesh.faces {
            writeln!(w, "3 {} {} {}", f[0], f[1], f[2])?;
        }

        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}
