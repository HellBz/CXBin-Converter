/// In-memory representation of a parsed CXBin mesh.
/// Mirrors the C++ `trimesh::TriMesh` data produced by the official
/// Creality `cxbin::loadCXBin` implementation.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct CxbinMesh {
    pub vertices: Vec<[f32; 3]>,
    pub faces: Vec<[i32; 3]>,
    pub uvs: Option<Vec<[f32; 2]>>,
    pub face_uvs: Option<Vec<[i32; 3]>>,
    pub materials: Option<CxbinMaterials>,
    pub cxbin_version: Option<i32>,
    pub compressed_bytes: Option<usize>,
    pub uncompressed_bytes: Option<usize>,
}

#[derive(Debug, Default)]
pub struct CxbinMaterials {
    pub material_name: Option<String>,
    pub material_blocks: Vec<Vec<u8>>,
    pub textures: Vec<CxbinTexture>,
    pub texture_ids: Option<Vec<i32>>,
    // Temporary storage used by the reader before the data is moved to CxbinMesh.
    pub(crate) uvs: Option<Vec<[f32; 2]>>,
    pub(crate) face_uvs: Option<Vec<[i32; 3]>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CxbinTexture {
    pub data: Vec<u8>,
    pub is_png: bool,
    pub size_hint: Option<(i32, i32)>,
}

impl CxbinMesh {
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }
}
