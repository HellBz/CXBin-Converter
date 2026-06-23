use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct DxfExporter;

impl Exporter for DxfExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let mut file = File::create(output_path)?;
        file.write_all(dxf_text(mesh).as_bytes())?;
        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}

fn dxf_text(mesh: &CxbinMesh) -> String {
    let mut out = String::new();
    out.push_str("0\nSECTION\n2\nHEADER\n0\nENDSEC\n");
    out.push_str("0\nSECTION\n2\nTABLES\n0\nENDSEC\n");
    out.push_str("0\nSECTION\n2\nBLOCKS\n0\nENDSEC\n");
    out.push_str("0\nSECTION\n2\nENTITIES\n");

    for f in &mesh.faces {
        let a = mesh.vertices[f[0] as usize];
        let b = mesh.vertices[f[1] as usize];
        let c = mesh.vertices[f[2] as usize];
        out.push_str("0\n3DFACE\n");
        out.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", a[0], a[1], a[2]));
        out.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", b[0], b[1], b[2]));
        out.push_str(&format!("12\n{}\n22\n{}\n32\n{}\n", c[0], c[1], c[2]));
        out.push_str(&format!("13\n{}\n23\n{}\n33\n{}\n", c[0], c[1], c[2]));
    }

    out.push_str("0\nENDSEC\n");
    out.push_str("0\nSECTION\n2\nOBJECTS\n0\nENDSEC\n");
    out.push_str("0\nEOF\n");
    out
}
