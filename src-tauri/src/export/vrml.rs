use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct VrmlExporter;

impl Exporter for VrmlExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let file = File::create(output_path)?;
        let mut w = BufWriter::new(file);

        writeln!(w, "#VRML V2.0 utf8")?;
        writeln!(w, "Shape {{")?;
        writeln!(w, "  appearance Appearance {{")?;
        writeln!(w, "    material Material {{ diffuseColor 0.8 0.8 0.8 }}")?;
        writeln!(w, "  }}")?;
        writeln!(w, "  geometry IndexedFaceSet {{")?;
        writeln!(w, "    solid FALSE")?;
        writeln!(w, "    coord Coordinate {{")?;
        write!(w, "      point [")?;
        for (i, v) in mesh.vertices.iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            write!(w, "{} {} {}", v[0], v[1], v[2])?;
        }
        writeln!(w, "      ]")?;
        writeln!(w, "    }}")?;
        write!(w, "    coordIndex [")?;
        for (i, f) in mesh.faces.iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            write!(w, "{} {} {} -1", f[0], f[1], f[2])?;
        }
        writeln!(w, "    ]")?;
        writeln!(w, "  }}")?;
        writeln!(w, "}}")?;

        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}
