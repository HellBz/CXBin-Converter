use std::fs::File;
use std::io::Write;
use std::path::Path;

use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Writer;
use zip::write::FileOptions;
use zip::ZipWriter;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct ThreeMFExporter;

impl Exporter for ThreeMFExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let file = File::create(output_path)?;
        let mut zip = ZipWriter::new(file);
        let options: FileOptions<()> = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .compression_level(Some(6));

        // [Content_Types].xml
        zip.start_file("[Content_Types].xml", options)?;
        zip.write_all(content_types_xml().as_bytes())?;

        // _rels/.rels
        zip.add_directory("_rels", options)?;
        zip.start_file("_rels/.rels", options)?;
        zip.write_all(rels_dot_rels().as_bytes())?;

        // 3D/3dmodel.model
        zip.add_directory("3D", options)?;
        zip.start_file("3D/3dmodel.model", options)?;
        zip.write_all(model_xml(mesh).as_bytes())?;

        // 3D/_rels/3dmodel.model.rels
        zip.add_directory("3D/_rels", options)?;
        zip.start_file("3D/_rels/3dmodel.model.rels", options)?;
        zip.write_all(model_rels().as_bytes())?;

        zip.finish()?;

        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}

fn content_types_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="model" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/>
</Types>
"#
    .to_string()
}

fn rels_dot_rels() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rel0" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel" Target="/3D/3dmodel.model"/>
</Relationships>
"#
    .to_string()
}

fn model_rels() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
</Relationships>
"#
    .to_string()
}

fn model_xml(mesh: &CxbinMesh) -> String {
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);

    // <?xml version="1.0" encoding="UTF-8"?>
    writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        None,
    ))).unwrap();

    // <model unit="millimeter" ...>
    let mut model = BytesStart::new("model");
    model.push_attribute(("unit", "millimeter"));
    model.push_attribute((
        "xmlns",
        "http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel",
    ));
    model.push_attribute((
        "xmlns:p",
        "http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodelxml",
    ));
    model.push_attribute((
        "xml:lang",
        "en-US",
    ));
    writer.write_event(Event::Start(model)).unwrap();

    // <resources>
    writer
        .write_event(Event::Start(BytesStart::new("resources")))
        .unwrap();

    // <object id="1" type="model">
    let mut object = BytesStart::new("object");
    object.push_attribute(("id", "1"));
    object.push_attribute(("type", "model"));
    writer.write_event(Event::Start(object)).unwrap();

    // <mesh>
    writer
        .write_event(Event::Start(BytesStart::new("mesh")))
        .unwrap();

    // <vertices>
    writer
        .write_event(Event::Start(BytesStart::new("vertices")))
        .unwrap();
    for v in &mesh.vertices {
        let mut vertex = BytesStart::new("vertex");
        vertex.push_attribute(("x", format!("{}", v[0]).as_str()));
        vertex.push_attribute(("y", format!("{}", v[1]).as_str()));
        vertex.push_attribute(("z", format!("{}", v[2]).as_str()));
        writer.write_event(Event::Empty(vertex)).unwrap();
    }
    writer
        .write_event(Event::End(BytesEnd::new("vertices")))
        .unwrap();

    // <triangles>
    writer
        .write_event(Event::Start(BytesStart::new("triangles")))
        .unwrap();
    for f in &mesh.faces {
        let mut triangle = BytesStart::new("triangle");
        triangle.push_attribute(("v1", format!("{}", f[0]).as_str()));
        triangle.push_attribute(("v2", format!("{}", f[1]).as_str()));
        triangle.push_attribute(("v3", format!("{}", f[2]).as_str()));
        writer.write_event(Event::Empty(triangle)).unwrap();
    }
    writer
        .write_event(Event::End(BytesEnd::new("triangles")))
        .unwrap();

    // </mesh>
    writer
        .write_event(Event::End(BytesEnd::new("mesh")))
        .unwrap();
    // </object>
    writer
        .write_event(Event::End(BytesEnd::new("object")))
        .unwrap();
    // </resources>
    writer
        .write_event(Event::End(BytesEnd::new("resources")))
        .unwrap();

    // <build>
    writer
        .write_event(Event::Start(BytesStart::new("build")))
        .unwrap();
    let mut item = BytesStart::new("item");
    item.push_attribute(("objectid", "1"));
    writer.write_event(Event::Empty(item)).unwrap();
    writer
        .write_event(Event::End(BytesEnd::new("build")))
        .unwrap();

    // </model>
    writer
        .write_event(Event::End(BytesEnd::new("model")))
        .unwrap();

    String::from_utf8(writer.into_inner()).unwrap()
}
