use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::mesh::{compute_normal, Exporter};
use crate::cxbin::CxbinMesh;

pub struct StlExporter;

impl Exporter for StlExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let mut file = File::create(output_path)?;
        file.write_all(&[0u8; 80])?; // 80 byte header

        let count = mesh.faces.len() as u32;
        file.write_all(&count.to_le_bytes())?;

        for face in &mesh.faces {
            let a = mesh.vertices[face[0] as usize];
            let b = mesh.vertices[face[1] as usize];
            let c = mesh.vertices[face[2] as usize];
            let normal = compute_normal(a, b, c);
            for v in &normal {
                file.write_all(&v.to_le_bytes())?;
            }
            for v in [a, b, c] {
                for coord in &v {
                    file.write_all(&coord.to_le_bytes())?;
                }
            }
            file.write_all(&[0u8; 2])?; // attribute byte count
        }

        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}
