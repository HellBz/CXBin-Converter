use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct DaeExporter;

impl Exporter for DaeExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let mut file = File::create(output_path)?;
        file.write_all(dae_xml(mesh).as_bytes())?;
        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}

fn dae_xml(mesh: &CxbinMesh) -> String {
    let positions: Vec<String> = mesh
        .vertices
        .iter()
        .map(|v| format!("{} {} {}", v[0], v[1], v[2]))
        .collect();
    let positions_str = positions.join(" ");

    let indices: Vec<String> = mesh
        .faces
        .iter()
        .flat_map(|f| [f[0].to_string(), f[1].to_string(), f[2].to_string()])
        .collect();
    let indices_str = indices.join(" ");

    let vertex_count = mesh.vertex_count();
    let face_count = mesh.face_count();

    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<COLLADA xmlns=\"http://www.collada.org/2005/11/COLLADASchema\" version=\"1.4.1\">\n");
    xml.push_str("  <asset>\n");
    xml.push_str("    <created>2024-01-01T00:00:00</created>\n");
    xml.push_str("    <modified>2024-01-01T00:00:00</modified>\n");
    xml.push_str("    <unit name=\"millimeter\" meter=\"0.001\"/>\n");
    xml.push_str("    <up_axis>Z_UP</up_axis>\n");
    xml.push_str("  </asset>\n");
    xml.push_str("  <library_geometries>\n");
    xml.push_str("    <geometry id=\"mesh0\" name=\"mesh0\">\n");
    xml.push_str("      <mesh>\n");
    xml.push_str("        <source id=\"mesh0-positions\">\n");
    xml.push_str(&format!(
        "          <float_array id=\"mesh0-positions-array\" count=\"{}\">{}</float_array>\n",
        vertex_count * 3,
        positions_str
    ));
    xml.push_str("          <technique_common>\n");
    xml.push_str(&format!(
        "            <accessor source=\"#mesh0-positions-array\" count=\"{}\" stride=\"3\">\n",
        vertex_count
    ));
    xml.push_str("              <param name=\"X\" type=\"float\"/>\n");
    xml.push_str("              <param name=\"Y\" type=\"float\"/>\n");
    xml.push_str("              <param name=\"Z\" type=\"float\"/>\n");
    xml.push_str("            </accessor>\n");
    xml.push_str("          </technique_common>\n");
    xml.push_str("        </source>\n");
    xml.push_str("        <vertices id=\"mesh0-vertices\">\n");
    xml.push_str("          <input semantic=\"POSITION\" source=\"#mesh0-positions\"/>\n");
    xml.push_str("        </vertices>\n");
    xml.push_str(&format!(
        "        <triangles count=\"{}\">\n",
        face_count
    ));
    xml.push_str("          <input semantic=\"VERTEX\" source=\"#mesh0-vertices\" offset=\"0\"/>\n");
    xml.push_str(&format!("          <p>{}</p>\n", indices_str));
    xml.push_str("        </triangles>\n");
    xml.push_str("      </mesh>\n");
    xml.push_str("    </geometry>\n");
    xml.push_str("  </library_geometries>\n");
    xml.push_str("  <library_visual_scenes>\n");
    xml.push_str("    <visual_scene id=\"Scene\" name=\"Scene\">\n");
    xml.push_str("      <node id=\"mesh0-node\" name=\"mesh0\" type=\"NODE\">\n");
    xml.push_str("        <instance_geometry url=\"#mesh0\"/>\n");
    xml.push_str("      </node>\n");
    xml.push_str("    </visual_scene>\n");
    xml.push_str("  </library_visual_scenes>\n");
    xml.push_str("  <scene>\n");
    xml.push_str("    <instance_visual_scene url=\"#Scene\"/>\n");
    xml.push_str("  </scene>\n");
    xml.push_str("</COLLADA>\n");
    xml
}
