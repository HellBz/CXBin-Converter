use std::fs::File;
use std::io::Write;
use std::path::Path;

use zip::write::SimpleFileOptions;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct UsdzExporter;

impl Exporter for UsdzExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let usda = usda_text(mesh);
        let file = File::create(output_path)?;
        let mut zip = zip::ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        zip.start_file("model.usda", options)?;
        zip.write_all(usda.as_bytes())?;
        zip.finish()?;
        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}

fn usda_text(mesh: &CxbinMesh) -> String {
    let positions: Vec<String> = mesh
        .vertices
        .iter()
        .map(|v| format!("({} {} {})", v[0], v[1], v[2]))
        .collect();
    let positions_str = positions.join(", ");

    let indices: Vec<String> = mesh
        .faces
        .iter()
        .map(|f| format!("({}, {}, {})", f[0], f[1], f[2]))
        .collect();
    let indices_str = indices.join(", ");

    format!(
        r#"#usda 1.0
(
    defaultPrim = "mesh"
    upAxis = "Y"
    metersPerUnit = 0.001
)


def Mesh "mesh"
{{
    uniform bool doubleSided = true
    int[] faceVertexCounts = [{}]
    int[] faceVertexIndices = [{}]
    point3f[] points = [{}]
    color3f[] primvars:displayColor = [(0.13, 0.77, 0.37)]
    uniform token subdivisionScheme = "none"
}}
"#,
        mesh
            .faces
            .iter()
            .map(|_| "3".to_string())
            .collect::<Vec<_>>()
            .join(", "),
        indices_str,
        positions_str
    )
}
