use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct GlbExporter;

impl Exporter for GlbExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let mut file = File::create(output_path)?;
        write_glb(mesh, &mut file)?;
        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}

fn write_glb(mesh: &CxbinMesh, writer: &mut File) -> anyhow::Result<()> {
    let vertex_count = mesh.vertex_count();
    let index_count = mesh.face_count() * 3;

    // Build position buffer
    let mut positions = Vec::with_capacity(vertex_count * 3 * 4);
    for v in &mesh.vertices {
        positions.extend_from_slice(&v[0].to_le_bytes());
        positions.extend_from_slice(&v[1].to_le_bytes());
        positions.extend_from_slice(&v[2].to_le_bytes());
    }

    // Build index buffer
    let mut indices = Vec::with_capacity(index_count * 4);
    for f in &mesh.faces {
        indices.extend_from_slice(&f[0].to_le_bytes());
        indices.extend_from_slice(&f[1].to_le_bytes());
        indices.extend_from_slice(&f[2].to_le_bytes());
    }

    let bin_chunk = [positions.as_slice(), indices.as_slice()].concat();
    let bin_chunk = pad_to_4(bin_chunk);

    let json = format!(
        r#"{{"asset":{{"version":"2.0"}},"scene":0,"scenes":[{{"nodes":[0]}}],"nodes":[{{"mesh":0}}],"meshes":[{{"primitives":[{{"attributes":{{"POSITION":0}},"indices":1,"material":0}}]}}],"materials":[{{"pbrMetallicRoughness":{{"baseColorFactor":[0.13,0.77,0.37,1.0],"metallicFactor":0.1,"roughnessFactor":0.5}}}}],"accessors":[{{"bufferView":0,"componentType":5126,"count":{0},"type":"VEC3"}},{{"bufferView":1,"componentType":5125,"count":{1},"type":"SCALAR"}}],"bufferViews":[{{"buffer":0,"byteOffset":0,"byteLength":{2}}},{{"buffer":0,"byteOffset":{2},"byteLength":{3}}}],"buffers":[{{"byteLength":{4}}}]}}"#,
        vertex_count,
        index_count,
        positions.len(),
        indices.len(),
        bin_chunk.len()
    );

    let json_chunk = pad_to_4(json.into_bytes());

    let json_chunk_len = json_chunk.len() as u32;
    let bin_chunk_len = bin_chunk.len() as u32;
    let total_len = 12 + 8 + json_chunk_len + 8 + bin_chunk_len;

    // Header
    writer.write_all(&0x46546C67u32.to_le_bytes())?; // glTF magic
    writer.write_all(&2u32.to_le_bytes())?;            // version
    writer.write_all(&total_len.to_le_bytes())?;     // total length

    // JSON chunk
    writer.write_all(&json_chunk_len.to_le_bytes())?;
    writer.write_all(&0x4E4F534Au32.to_le_bytes())?; // JSON chunk type
    writer.write_all(&json_chunk)?;

    // BIN chunk
    writer.write_all(&bin_chunk_len.to_le_bytes())?;
    writer.write_all(&0x004E4942u32.to_le_bytes())?; // BIN chunk type
    writer.write_all(&bin_chunk)?;

    Ok(())
}

fn pad_to_4(mut data: Vec<u8>) -> Vec<u8> {
    while data.len() % 4 != 0 {
        data.push(0);
    }
    data
}
