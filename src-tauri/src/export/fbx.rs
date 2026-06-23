use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::mesh::Exporter;
use crate::cxbin::CxbinMesh;

pub struct FbxExporter;

impl Exporter for FbxExporter {
    fn export(&self, mesh: &CxbinMesh, output_path: &Path) -> anyhow::Result<Vec<String>> {
        let mut file = File::create(output_path)?;
        file.write_all(fbx_ascii(mesh).as_bytes())?;
        Ok(vec![output_path.to_string_lossy().to_string()])
    }
}

fn fbx_ascii(mesh: &CxbinMesh) -> String {
    let vertex_count = mesh.vertex_count();
    let face_count = mesh.face_count();

    let positions: Vec<String> = mesh
        .vertices
        .iter()
        .map(|v| format!("{} {} {}", v[0], v[1], v[2]))
        .collect();
    let positions_str = positions.join(",");

    let indices: Vec<String> = mesh
        .faces
        .iter()
        .map(|f| format!("{},{},{}-", f[0], f[1], f[2]))
        .collect();
    let indices_str = indices.join(",");

    let normals: Vec<String> = mesh
        .faces
        .iter()
        .map(|f| {
            let a = mesh.vertices[f[0] as usize];
            let b = mesh.vertices[f[1] as usize];
            let c = mesh.vertices[f[2] as usize];
            let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let n = [
                ab[1] * ac[2] - ab[2] * ac[1],
                ab[2] * ac[0] - ab[0] * ac[2],
                ab[0] * ac[1] - ab[1] * ac[0],
            ];
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if len > 0.0 {
                format!(
                    "3 {},{},{},{},{},{},{},{},{}-",
                    n[0] / len,
                    n[1] / len,
                    n[2] / len,
                    n[0] / len,
                    n[1] / len,
                    n[2] / len,
                    n[0] / len,
                    n[1] / len,
                    n[2] / len
                )
            } else {
                format!("3 0,0,0,0,0,0,0,0,0-")
            }
        })
        .collect();
    let normals_str = normals.join(",");

    format!(
        r#"; FBX 7.5.0 project file
; Created by CXBin-Converter

FBXHeaderExtension:  {{
    FBXHeaderVersion: 1003
    FBXVersion: 7500
    CreationTimeStamp:  {{
        Version: 1000
        Year: 2024
        Month: 1
        Day: 1
        Hour: 0
        Minute: 0
        Second: 0
        Millisecond: 0
    }}
    Creator: "CXBin-Converter"
    SceneInfo: "SceneInfo::GlobalInfo", "UserData" {{
        Type: "UserData"
        Version: 100
        MetaData:  {{
            Version: 100
            Title: ""
            Subject: ""
            Author: ""
            Keywords: ""
            Revision: ""
            Comment: ""
        }}
    }}
}}

GlobalSettings:  {{
    Version: 1000
    Properties70:  {{
        P: "UpAxis", "int", "Integer", "",1
        P: "UpAxisSign", "int", "Integer", "",1
        P: "FrontAxis", "int", "Integer", "",2
        P: "FrontAxisSign", "int", "Integer", "",1
        P: "CoordAxis", "int", "Integer", "",0
        P: "CoordAxisSign", "int", "Integer", "",1
        P: "OriginalUpAxis", "int", "Integer", "",-1
        P: "OriginalUpAxisSign", "int", "Integer", "",1
        P: "UnitScaleFactor", "double", "Number", "",1
        P: "OriginalUnitScaleFactor", "double", "Number", "",1
    }}
}}

Documents:  {{
    Count: 1
    Document: 1000000000, "", "Scene" {{
        Properties70:  {{
            P: "SourceObject", "object", "", ""
            P: "ActiveAnimStackName", "KString", "", "", ""
        }}
        RootNode: 0
    }}
}}

References:  {{
}}

Definitions:  {{
    Version: 100
    Count: 3
    ObjectType: "GlobalSettings" {{
        Count: 1
    }}
    ObjectType: "Model" {{
        Count: 1
    }}
    ObjectType: "Geometry" {{
        Count: 1
    }}
}}

Objects:  {{
    Geometry: 1000000001, "Geometry::Mesh", "Mesh" {{
        Vertices: *{0} {{
            a: {1}
        }}
        PolygonVertexIndex: *{2} {{
            a: {3}
        }}
        GeometryVersion: 124
        LayerElementNormal: 0 {{
            Version: 101
            Name: ""
            MappingInformationType: "ByPolygon"
            ReferenceInformationType: "Direct"
            Normals: *{2} {{
                a: {4}
            }}
        }}
        LayerElementMaterial: 0 {{
            Version: 101
            Name: ""
            MappingInformationType: "AllSame"
            ReferenceInformationType: "IndexToDirect"
            Materials: *1 {{
                a: 0
            }}
        }}
        Layer: 0 {{
            Version: 100
            LayerElement:  {{
                Type: "LayerElementNormal"
                TypedIndex: 0
            }}
            LayerElement:  {{
                Type: "LayerElementMaterial"
                TypedIndex: 0
            }}
        }}
    }}
    Model: 1000000002, "Model::Mesh", "Mesh" {{
        Version: 232
        Properties70:  {{
            P: "Lcl Translation", "Lcl Translation", "", "A",0,0,0
            P: "Lcl Rotation", "Lcl Rotation", "", "A",0,0,0
            P: "Lcl Scaling", "Lcl Scaling", "", "A",1,1,1
        }}
        Shading: Y
        Culling: "CullingOff"
    }}
    Material: 1000000003, "Material::Default", "" {{
        Version: 102
        ShadingModel: "phong"
        MultiLayer: 0
        Properties70:  {{
            P: "DiffuseColor", "Color", "", "A",0.13,0.77,0.37
            P: "SpecularColor", "Color", "", "A",1,1,1
            P: "SpecularFactor", "double", "Number", "",0.5
        }}
    }}
}}

Connections:  {{
    C: "OO",1000000001,1000000002
    C: "OO",1000000002,0
    C: "OO",1000000003,1000000002
}}
"#,
        vertex_count * 3,
        positions_str,
        face_count,
        indices_str,
        normals_str
    )
}
