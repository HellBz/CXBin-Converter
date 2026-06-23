use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct AmfExporter;

impl Exporter for AmfExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let file = File::create(output_path)?;
        let mut w = BufWriter::new(file);

        writeln!(w, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
        writeln!(w, "<amf unit=\"millimeter\" version=\"1.1\">")?;
        writeln!(w, "  <object id=\"1\">")?;
        writeln!(w, "    <mesh>")?;
        writeln!(w, "      <vertices>")?;
        for v in &mesh.vertices {
            writeln!(
                w,
                "        <vertex><coordinates><x>{}</x><y>{}</y><z>{}</z></coordinates></vertex>",
                v[0], v[1], v[2]
            )?;
        }
        writeln!(w, "      </vertices>")?;
        writeln!(w, "      <volume>")?;
        for f in &mesh.faces {
            writeln!(
                w,
                "        <triangle><v1>{}</v1><v2>{}</v2><v3>{}</v3></triangle>",
                f[0], f[1], f[2]
            )?;
        }
        writeln!(w, "      </volume>")?;
        writeln!(w, "    </mesh>")?;
        writeln!(w, "  </object>")?;
        writeln!(w, "</amf>")?;

        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}
