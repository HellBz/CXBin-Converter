use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct ObjExporter;

impl Exporter for ObjExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let base_dir = output_path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let stem = output_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("out");

        let obj_path = base_dir.join(format!("{}.obj", stem));
        let mtl_path = base_dir.join(format!("{}.mtl", stem));

        let mut files = vec![obj_path.to_string_lossy().to_string()];

        let mut obj_file = File::create(&obj_path)?;
        let mut obj = BufWriter::new(&mut obj_file);

        let has_textures = mesh
            .materials
            .as_ref()
            .map(|m| !m.textures.is_empty())
            .unwrap_or(false);
        let has_uvs = mesh.uvs.is_some() && !mesh.uvs.as_ref().unwrap().is_empty();
        let mut texture_path: Option<PathBuf> = None;

        if has_textures {
            let mtl_name = format!("{}.mtl", stem);
            writeln!(obj, "mtllib {}", mtl_name)?;
            writeln!(obj, "usemtl material_0")?;

            let tex = &mesh.materials.as_ref().unwrap().textures[0];
            let ext = if tex.is_png { "png" } else { "bin" };
            texture_path = Some(base_dir.join(format!("{}_texture.{}", stem, ext)));
            std::fs::write(texture_path.as_ref().unwrap(), &tex.data)?;
            files.push(texture_path.as_ref().unwrap().to_string_lossy().to_string());
        }

        // Vertices
        for v in &mesh.vertices {
            writeln!(obj, "v {} {} {}", v[0], v[1], v[2])?;
        }

        // Texture coordinates (per-vertex for simplicity)
        if has_uvs {
            for uv in mesh.uvs.as_ref().unwrap() {
                writeln!(obj, "vt {} {}", uv[0], uv[1])?;
            }
        }

        // Faces (OBJ uses 1-based indices)
        for f in &mesh.faces {
            if has_uvs {
                writeln!(
                    obj,
                    "f {}/{} {}/{} {}/{}",
                    f[0] + 1,
                    f[0] + 1,
                    f[1] + 1,
                    f[1] + 1,
                    f[2] + 1,
                    f[2] + 1
                )?;
            } else {
                writeln!(obj, "f {} {} {}", f[0] + 1, f[1] + 1, f[2] + 1)?;
            }
        }

        // Write MTL if we have a texture
        if let Some(tex_path) = texture_path {
            let mut mtl_file = File::create(&mtl_path)?;
            let mut mtl = BufWriter::new(&mut mtl_file);
            writeln!(mtl, "newmtl material_0")?;
            writeln!(mtl, "Ka 1.0 1.0 1.0")?;
            writeln!(mtl, "Kd 1.0 1.0 1.0")?;
            writeln!(mtl, "Ks 0.0 0.0 0.0")?;
            writeln!(mtl, "d 1.0")?;
            writeln!(mtl, "illum 1")?;
            writeln!(
                mtl,
                "map_Kd {}",
                tex_path.file_name().unwrap().to_string_lossy()
            )?;
            files.push(mtl_path.to_string_lossy().to_string());
        }

        Ok(files)
    }
}
