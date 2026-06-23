use std::fs::File;
use std::io::Write;
use std::path::Path;

use image::{ImageBuffer, Rgba};
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

        let thumbnail = render_thumbnail(mesh, 256, 256);

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

        // 3D/Thumbnails/thumbnail.png
        zip.add_directory("3D/Thumbnails", options)?;
        zip.start_file("3D/Thumbnails/thumbnail.png", options)?;
        zip.write_all(&thumbnail)?;

        zip.finish()?;

        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}

fn content_types_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="model" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/>
  <Default Extension="png" ContentType="image/png"/>
</Types>
"#
    .to_string()
}

fn rels_dot_rels() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rel0" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel" Target="/3D/3dmodel.model"/>
  <Relationship Id="rel1" Type="http://schemas.openxmlformats.org/package/2006/relationships/thumbnail" Target="/3D/Thumbnails/thumbnail.png"/>
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
    model.push_attribute(("xml:lang", "en-US"));
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

/// Render a simple 2D orthographic thumbnail of the mesh into a PNG.
fn render_thumbnail(mesh: &CxbinMesh, width: u32, height: u32) -> Vec<u8> {
    let mut img = ImageBuffer::from_pixel(width, height, Rgba([240, 240, 240, 255]));
    let (min, max) = bounding_box(mesh);
    let size = [
        (max[0] - min[0]).max(0.001),
        (max[1] - min[1]).max(0.001),
        (max[2] - min[2]).max(0.001),
    ];
    let center = [(min[0] + max[0]) / 2.0, (min[1] + max[1]) / 2.0, (min[2] + max[2]) / 2.0];
    let max_dim = size[0].max(size[1]).max(size[2]);
    let scale = ((width as f32 - 20.0) / max_dim).min((height as f32 - 20.0) / max_dim);

    // Project to screen: x horizontal, y vertical (flip y), ignore z
    let project = |v: &[f32; 3]| {
        let x = (v[0] - center[0]) * scale + (width as f32) / 2.0;
        let y = (height as f32) / 2.0 - (v[1] - center[1]) * scale;
        (x as i32, y as i32)
    };

    // Draw wireframe edges
    for f in &mesh.faces {
        let a = project(&mesh.vertices[f[0] as usize]);
        let b = project(&mesh.vertices[f[1] as usize]);
        let c = project(&mesh.vertices[f[2] as usize]);
        draw_line(&mut img, a, b, Rgba([59, 130, 246, 255]));
        draw_line(&mut img, b, c, Rgba([59, 130, 246, 255]));
        draw_line(&mut img, c, a, Rgba([59, 130, 246, 255]));
    }

    let mut out = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)
        .unwrap();
    out
}

fn bounding_box(mesh: &CxbinMesh) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];
    for v in &mesh.vertices {
        for i in 0..3 {
            if v[i] < min[i] {
                min[i] = v[i];
            }
            if v[i] > max[i] {
                max[i] = v[i];
            }
        }
    }
    (min, max)
}

fn draw_line(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, p0: (i32, i32), p1: (i32, i32), color: Rgba<u8>) {
    let (x0, y0) = p0;
    let (x1, y1) = p1;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && y >= 0 && x < img.width() as i32 && y < img.height() as i32 {
            img.put_pixel(x as u32, y as u32, color);
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
