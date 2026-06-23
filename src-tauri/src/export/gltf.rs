use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct GltfExporter;

impl Exporter for GltfExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let parent = output_path
            .parent()
            .unwrap_or_else(|| Path::new("."));
        let stem = output_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("mesh");
        let out_dir = parent.join(format!("{}_gltf", stem));
        fs::create_dir_all(&out_dir)?;

        let gltf_path = out_dir.join(format!("{}.gltf", stem));
        let bin_path = out_dir.join(format!("{}.bin", stem));
        let bin_name = format!("{}.bin", stem);

        let mut bin_file = File::create(&bin_path)?;
        write_bin(mesh, &mut bin_file)?;

        let mut gltf_file = File::create(&gltf_path)?;
        gltf_file.write_all(gltf_json(mesh, &bin_name).as_bytes())?;

        Ok(vec![
            out_dir.to_string_lossy().to_string(),
            gltf_path.to_string_lossy().to_string(),
            bin_path.to_string_lossy().to_string(),
        ])
    }
}

fn write_bin(mesh: &CxbinMesh, writer: &mut File) -> anyhow::Result<()> {
    for v in &mesh.vertices {
        writer.write_all(&v[0].to_le_bytes())?;
        writer.write_all(&v[1].to_le_bytes())?;
        writer.write_all(&v[2].to_le_bytes())?;
    }
    for f in &mesh.faces {
        writer.write_all(&f[0].to_le_bytes())?;
        writer.write_all(&f[1].to_le_bytes())?;
        writer.write_all(&f[2].to_le_bytes())?;
    }
    Ok(())
}

fn gltf_json(mesh: &CxbinMesh, bin_uri: &str) -> String {
    let vertex_count = mesh.vertex_count();
    let index_count = mesh.face_count() * 3;
    let positions_byte_length = vertex_count * 3 * 4;
    let indices_byte_length = index_count * 4;

    format!(
        r#"{{
  "asset": {{"version": "2.0"}},
  "scene": 0,
  "scenes": [{{"nodes": [0]}}],
  "nodes": [{{"mesh": 0}}],
  "meshes": [{{
    "primitives": [{{
      "attributes": {{"POSITION": 0}},
      "indices": 1,
      "material": 0
    }}]
  }}],
  "materials": [{{"pbrMetallicRoughness": {{"baseColorFactor": [0.13, 0.77, 0.37, 1.0], "metallicFactor": 0.1, "roughnessFactor": 0.5}}}}],
  "accessors": [
    {{"bufferView": 0, "componentType": 5126, "count": {0}, "type": "VEC3"}},
    {{"bufferView": 1, "componentType": 5125, "count": {1}, "type": "SCALAR"}}
  ],
  "bufferViews": [
    {{"buffer": 0, "byteOffset": 0, "byteLength": {2}}},
    {{"buffer": 0, "byteOffset": {2}, "byteLength": {3}}}
  ],
  "buffers": [{{"uri": "{4}", "byteLength": {5}}}]
}}
"#,
        vertex_count,
        index_count,
        positions_byte_length,
        indices_byte_length,
        bin_uri,
        positions_byte_length + indices_byte_length
    )
}
