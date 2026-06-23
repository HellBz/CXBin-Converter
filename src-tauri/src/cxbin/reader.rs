use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use flate2::read::ZlibDecoder;

use super::mesh::{CxbinMaterials, CxbinMesh, CxbinTexture};

const MAGIC: &[u8; 12] = b"\niwlskdfjad\0";

#[derive(Debug, thiserror::Error)]
pub enum CxbinError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Unexpected end of file")]
    Eof,
    #[error("Invalid magic bytes")]
    InvalidMagic,
    #[error("Decompressed size mismatch (got {got}, expected {expected})")]
    SizeMismatch { got: usize, expected: usize },
    #[error("PNG encoding failed: {0}")]
    Png(#[from] image::ImageError),
}

fn read_i32_le(f: &mut File) -> Result<i32, CxbinError> {
    let mut buf = [0u8; 4];
    f.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

fn read_exact(f: &mut File, n: usize) -> Result<Vec<u8>, CxbinError> {
    let mut buf = vec![0u8; n];
    f.read_exact(&mut buf)?;
    Ok(buf)
}

fn zlib_decompress(data: &[u8], expected: Option<usize>) -> Result<Vec<u8>, CxbinError> {
    let mut decoder = ZlibDecoder::new(data);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out)?;
    if let Some(expected) = expected {
        if out.len() != expected {
            return Err(CxbinError::SizeMismatch {
                got: out.len(),
                expected,
            });
        }
    }
    Ok(out)
}

fn decode_vertices(raw: &[u8], vert_num: usize) -> Vec<[f32; 3]> {
    let mut verts = Vec::with_capacity(vert_num);
    for i in 0..vert_num {
        let off = i * 12;
        verts.push([
            f32::from_le_bytes([raw[off], raw[off + 1], raw[off + 2], raw[off + 3]]),
            f32::from_le_bytes([raw[off + 4], raw[off + 5], raw[off + 6], raw[off + 7]]),
            f32::from_le_bytes([raw[off + 8], raw[off + 9], raw[off + 10], raw[off + 11]]),
        ]);
    }
    verts
}

fn decode_faces(raw: &[u8], face_num: usize) -> Vec<[i32; 3]> {
    let mut faces = Vec::with_capacity(face_num);
    for i in 0..face_num {
        let off = i * 12;
        faces.push([
            i32::from_le_bytes([raw[off], raw[off + 1], raw[off + 2], raw[off + 3]]),
            i32::from_le_bytes([raw[off + 4], raw[off + 5], raw[off + 6], raw[off + 7]]),
            i32::from_le_bytes([raw[off + 8], raw[off + 9], raw[off + 10], raw[off + 11]]),
        ]);
    }
    faces
}

fn decode_uvs(raw: &[u8], uv_num: usize) -> Vec<[f32; 2]> {
    let mut uvs = Vec::with_capacity(uv_num);
    for i in 0..uv_num {
        let off = i * 8;
        uvs.push([
            f32::from_le_bytes([raw[off], raw[off + 1], raw[off + 2], raw[off + 3]]),
            f32::from_le_bytes([raw[off + 4], raw[off + 5], raw[off + 6], raw[off + 7]]),
        ]);
    }
    uvs
}

fn decode_face_uvs(raw: &[u8], face_uv_num: usize) -> Vec<[i32; 3]> {
    let mut fuvs = Vec::with_capacity(face_uv_num);
    for i in 0..face_uv_num {
        let off = i * 12;
        fuvs.push([
            i32::from_le_bytes([raw[off], raw[off + 1], raw[off + 2], raw[off + 3]]),
            i32::from_le_bytes([raw[off + 4], raw[off + 5], raw[off + 6], raw[off + 7]]),
            i32::from_le_bytes([raw[off + 8], raw[off + 9], raw[off + 10], raw[off + 11]]),
        ]);
    }
    fuvs
}

fn decode_texture_ids(raw: &[u8], tid_num: usize) -> Vec<i32> {
    let mut ids = Vec::with_capacity(tid_num);
    for i in 0..tid_num {
        let off = i * 4;
        ids.push(i32::from_le_bytes([raw[off], raw[off + 1], raw[off + 2], raw[off + 3]]));
    }
    ids
}

fn parse_material_block_new(
    raw: &[u8],
    uv_num: usize,
    face_uv_num: usize,
    texture_id_num: usize,
    material_sizes: &[i32],
    mtl_name_len: usize,
    map_type_count: i32,
) -> Result<CxbinMaterials, CxbinError> {
    let mut ptr = 0;
    let mut take = |n: usize| -> Result<&[u8], CxbinError> {
        if ptr + n > raw.len() {
            return Err(CxbinError::Eof);
        }
        let chunk = &raw[ptr..ptr + n];
        ptr += n;
        Ok(chunk)
    };

    let mut mats = CxbinMaterials::default();

    mats.uvs = if uv_num > 0 {
        Some(decode_uvs(take(uv_num * 8)?, uv_num))
    } else {
        None
    };
    mats.face_uvs = if face_uv_num > 0 {
        Some(decode_face_uvs(take(face_uv_num * 12)?, face_uv_num))
    } else {
        None
    };
    mats.texture_ids = if texture_id_num > 0 {
        Some(decode_texture_ids(take(texture_id_num * 4)?, texture_id_num))
    } else {
        None
    };

    let mut material_blocks = Vec::with_capacity(material_sizes.len());
    for &size in material_sizes {
        let size = size as usize;
        let block = if size > 0 { take(size)?.to_vec() } else { Vec::new() };
        material_blocks.push(block);
    }
    mats.material_blocks = material_blocks;

    if mtl_name_len > 1 {
        let name_bytes = take(mtl_name_len)?;
        let end = name_bytes.iter().position(|&b| b == 0).unwrap_or(name_bytes.len());
        mats.material_name = String::from_utf8(name_bytes[..end].to_vec()).ok();
    } else if mtl_name_len > 0 {
        ptr += mtl_name_len;
    }

    let mut textures = Vec::new();
    for _ in 0..map_type_count.max(0) {
        if ptr + 4 > raw.len() {
            break;
        }
        let buf_size = i32::from_le_bytes([
            raw[ptr], raw[ptr + 1], raw[ptr + 2], raw[ptr + 3],
        ]) as usize;
        ptr += 4;
        let end = (ptr + buf_size).min(raw.len());
        let buf = raw[ptr..end].to_vec();
        ptr = end;
        let is_png = buf.starts_with(b"\x89PNG\r\n\x1a\n");
        textures.push(CxbinTexture {
            data: buf,
            is_png,
            size_hint: None,
        });
    }
    mats.textures = textures;

    Ok(mats)
}

fn parse_material_block_old(
    raw: &[u8],
    vert_num: usize,
    face_num: usize,
    uv_num: usize,
    face_uv_num: usize,
    texture_id_num: usize,
    material_sizes: &[i32],
    mtl_name_len: usize,
    map_type_count: i32,
) -> Result<(CxbinMaterials, Vec<[f32; 3]>, Vec<[i32; 3]>), CxbinError> {
    let mut ptr = 0;
    let mut take = |n: usize| -> Result<&[u8], CxbinError> {
        if ptr + n > raw.len() {
            return Err(CxbinError::Eof);
        }
        let chunk = &raw[ptr..ptr + n];
        ptr += n;
        Ok(chunk)
    };

    let vertices = if vert_num > 0 {
        decode_vertices(take(vert_num * 12)?, vert_num)
    } else {
        Vec::new()
    };
    let faces = if face_num > 0 {
        decode_faces(take(face_num * 12)?, face_num)
    } else {
        Vec::new()
    };

    let mut mats = CxbinMaterials::default();
    mats.uvs = if uv_num > 0 {
        Some(decode_uvs(take(uv_num * 8)?, uv_num))
    } else {
        None
    };
    mats.face_uvs = if face_uv_num > 0 {
        Some(decode_face_uvs(take(face_uv_num * 12)?, face_uv_num))
    } else {
        None
    };
    mats.texture_ids = if texture_id_num > 0 {
        Some(decode_texture_ids(take(texture_id_num * 4)?, texture_id_num))
    } else {
        None
    };

    let mut material_blocks = Vec::with_capacity(material_sizes.len());
    for &size in material_sizes {
        let size = size as usize;
        let block = if size > 0 { take(size)?.to_vec() } else { Vec::new() };
        material_blocks.push(block);
    }
    mats.material_blocks = material_blocks;

    if mtl_name_len > 1 {
        let name_bytes = take(mtl_name_len)?;
        let end = name_bytes.iter().position(|&b| b == 0).unwrap_or(name_bytes.len());
        mats.material_name = String::from_utf8(name_bytes[..end].to_vec()).ok();
    } else if mtl_name_len > 0 {
        ptr += mtl_name_len;
    }

    let mut textures = Vec::new();
    for _ in 0..map_type_count.max(0) {
        if ptr + 8 > raw.len() {
            break;
        }
        let width = i32::from_le_bytes([raw[ptr], raw[ptr + 1], raw[ptr + 2], raw[ptr + 3]]);
        ptr += 4;
        let height = i32::from_le_bytes([raw[ptr], raw[ptr + 1], raw[ptr + 2], raw[ptr + 3]]);
        ptr += 4;
        if width <= 0 || height <= 0 {
            textures.push(CxbinTexture {
                data: Vec::new(),
                is_png: false,
                size_hint: Some((width, height)),
            });
            continue;
        }
        let byte_count = (width as usize) * (height as usize) * 4;
        let end = (ptr + byte_count).min(raw.len());
        let rgba = raw[ptr..end].to_vec();
        ptr = end;
        let png = if rgba.len() == byte_count {
            encode_rgba_png(width as u32, height as u32, &rgba)?
        } else {
            rgba
        };
        textures.push(CxbinTexture {
            data: png,
            is_png: true,
            size_hint: Some((width, height)),
        });
    }
    mats.textures = textures;

    Ok((mats, vertices, faces))
}

fn encode_rgba_png(width: u32, height: u32, rgba: &[u8]) -> Result<Vec<u8>, CxbinError> {
    let img = image::RgbaImage::from_raw(width, height, rgba.to_vec())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid rgba buffer"))?;
    let mut out = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)?;
    Ok(out)
}

fn parse_mesh_block(f: &mut File) -> Result<(Vec<[f32; 3]>, Vec<[i32; 3]>, usize, usize), CxbinError> {
    let _total_num = read_i32_le(f)? as usize;
    let vert_num = read_i32_le(f)? as usize;
    let face_num = read_i32_le(f)? as usize;
    let compress_num = read_i32_le(f)? as usize;

    let cdata = read_exact(f, compress_num)?;
    let expected = vert_num * 12 + face_num * 12;
    let raw = zlib_decompress(&cdata, Some(expected))?;

    let vertices = decode_vertices(&raw[..vert_num * 12], vert_num);
    let faces = decode_faces(&raw[vert_num * 12..], face_num);

    Ok((vertices, faces, compress_num, raw.len()))
}

pub fn load_cxbin<P: AsRef<Path>>(path: P) -> Result<CxbinMesh, CxbinError> {
    let mut f = File::open(path)?;

    let head_code = read_i32_le(&mut f)?;
    let magic = read_exact(&mut f, 12)?;
    if &magic[..] != MAGIC {
        return Err(CxbinError::InvalidMagic);
    }

    if head_code == 1 {
        // Legacy v1: geometry + materials inside one compressed block.
        let total_num = read_i32_le(&mut f)? as usize;
        let vert_num = read_i32_le(&mut f)? as usize;
        let face_num = read_i32_le(&mut f)? as usize;
        let uv_num = read_i32_le(&mut f)? as usize;
        let face_uv_num = read_i32_le(&mut f)? as usize;
        let texture_id_num = read_i32_le(&mut f)? as usize;
        let material_num = read_i32_le(&mut f)? as usize;

        let mut material_sizes = Vec::with_capacity(material_num);
        for _ in 0..material_num {
            material_sizes.push(read_i32_le(&mut f)?);
        }
        let mtl_name_len = read_i32_le(&mut f)? as usize;
        let map_type_count = read_i32_le(&mut f)?;
        let compress_num = read_i32_le(&mut f)? as usize;

        let cdata = read_exact(&mut f, compress_num)?;
        let raw = zlib_decompress(&cdata, Some(total_num))?;

        let (mats, vertices, faces) = parse_material_block_old(
            &raw,
            vert_num,
            face_num,
            uv_num,
            face_uv_num,
            texture_id_num,
            &material_sizes,
            mtl_name_len,
            map_type_count,
        )?;

        let uvs = mats.uvs.clone();
        let face_uvs = mats.face_uvs.clone();
        let mut mats = mats;
        mats.uvs = None;
        mats.face_uvs = None;

        return Ok(CxbinMesh {
            vertices,
            faces,
            uvs,
            face_uvs,
            materials: Some(mats),
            cxbin_version: Some(1),
            compressed_bytes: None,
            uncompressed_bytes: None,
        });
    }

    // Newer path: geometry first, then optional versioned material block.
    let (vertices, faces, compressed, uncompressed) = parse_mesh_block(&mut f)?;

    let version = match read_i32_le(&mut f) {
        Ok(v) => v,
        Err(CxbinError::Io(e)) if e.kind() == io::ErrorKind::UnexpectedEof => {
            return Ok(CxbinMesh {
                vertices,
                faces,
                uvs: None,
                face_uvs: None,
                materials: None,
                cxbin_version: None,
                compressed_bytes: Some(compressed),
                uncompressed_bytes: Some(uncompressed),
            });
        }
        Err(e) => return Err(e),
    };

    let materials: Option<CxbinMaterials> = None;
    if version == 1 {
        let total_num = read_i32_le(&mut f)? as usize;
        let uv_num = read_i32_le(&mut f)? as usize;
        let face_uv_num = read_i32_le(&mut f)? as usize;
        let texture_id_num = read_i32_le(&mut f)? as usize;
        let material_num = read_i32_le(&mut f)? as usize;

        let mut material_sizes = Vec::with_capacity(material_num);
        for _ in 0..material_num {
            material_sizes.push(read_i32_le(&mut f)?);
        }
        let mtl_name_len = read_i32_le(&mut f)? as usize;
        let map_type_count = read_i32_le(&mut f)?;
        let compress_num = read_i32_le(&mut f)? as usize;

        let cdata = read_exact(&mut f, compress_num)?;
        let raw = zlib_decompress(&cdata, Some(total_num))?;

        let mut mats = parse_material_block_new(
            &raw,
            uv_num,
            face_uv_num,
            texture_id_num,
            &material_sizes,
            mtl_name_len,
            map_type_count,
        )?;
        let uvs = mats.uvs.clone();
        let face_uvs = mats.face_uvs.clone();
        mats.uvs = None;
        mats.face_uvs = None;

        return Ok(CxbinMesh {
            vertices,
            faces,
            uvs,
            face_uvs,
            materials: Some(mats),
            cxbin_version: Some(version),
            compressed_bytes: Some(compressed),
            uncompressed_bytes: Some(uncompressed),
        });
    }

    Ok(CxbinMesh {
        vertices,
        faces,
        uvs: None,
        face_uvs: None,
        materials,
        cxbin_version: Some(version),
        compressed_bytes: Some(compressed),
        uncompressed_bytes: Some(uncompressed),
    })
}

pub fn try_load_header<P: AsRef<Path>>(path: P) -> Result<bool, CxbinError> {
    let mut f = File::open(path)?;
    let mut version_buf = [0u8; 4];
    f.read_exact(&mut version_buf)?;
    let _version = i32::from_le_bytes(version_buf);
    let mut magic = [0u8; 12];
    f.read_exact(&mut magic)?;
    Ok(&magic == MAGIC)
}
