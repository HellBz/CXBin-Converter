use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct X3dExporter;

impl Exporter for X3dExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let file = File::create(output_path)?;
        let mut w = BufWriter::new(file);

        writeln!(w, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
        writeln!(w, "<X3D version=\"3.3\" profile=\"Immersive\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema-instance\">")?;
        writeln!(w, "  <Scene>")?;
        writeln!(w, "    <Shape>")?;
        writeln!(w, "      <Appearance>")?;
        writeln!(w, "        <Material diffuseColor=\"0.8 0.8 0.8\"/>")?;
        writeln!(w, "      </Appearance>")?;

        let coord_index = mesh
            .faces
            .iter()
            .map(|f| format!("{} {} {} -1", f[0], f[1], f[2]))
            .collect::<Vec<_>>()
            .join(" ");

        let points = mesh
            .vertices
            .iter()
            .map(|v| format!("{} {} {}", v[0], v[1], v[2]))
            .collect::<Vec<_>>()
            .join(" ");

        writeln!(
            w,
            "      <IndexedFaceSet coordIndex=\"{}\">",
            coord_index
        )?;
        writeln!(w, "        <Coordinate point=\"{}\" />", points)?;
        writeln!(w, "      </IndexedFaceSet>")?;
        writeln!(w, "    </Shape>")?;
        writeln!(w, "  </Scene>")?;
        writeln!(w, "</X3D>")?;

        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}
