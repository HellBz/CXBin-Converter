import sys
import struct
import zlib
import numpy as np
import trimesh
import os
import platform

SUPPORTED_FORMATS = ["stl", "ply", "obj", "glb", "gltf", "off", "dae", "3mf"]

def print_ascii_header():
    header = r"""
   ______  ______  _              ____                          _            
  / ___\ \/ / __ )(_)_ __        / ___|___  _ ____   _____ _ __| |_ ___ _ __ 
 | |    \  /|  _ \| | '_ \ _____| |   / _ \| '_ \ \ / / _ \ '__| __/ _ \ '__|
 | |___ /  \| |_) | | | | |_____| |__| (_) | | | \ V /  __/ |  | ||  __/ |   
  \____/_/\_\____/|_|_| |_|      \____\___/|_| |_|\_/ \___|_|   \__\___|_|   
                                                                             
                                                                             """
    print(header)

def parse_cxbin(file_path):
    with open(file_path, "rb") as f:
        f.read(16)  # skip header
        header = f.read(12)
        if len(header) < 12:
            print("❌ File too small or invalid.")
            return None, None
        total_size, vert_count, face_count = struct.unpack("<3i", header)
        comp_size = struct.unpack("<i", f.read(4))[0]
        compressed = f.read(comp_size)
        raw = zlib.decompress(compressed)
        if len(raw) != vert_count * 12 + face_count * 12:
            print("❌ Data length does not match:", len(raw))
            return None, None

        vertices = np.frombuffer(raw[:vert_count * 12], dtype=np.float32).reshape((vert_count, 3))
        faces = np.frombuffer(raw[vert_count * 12:], dtype=np.int32).reshape((face_count, 3))
        mesh = trimesh.Trimesh(vertices=vertices, faces=faces, process=False)

        metadata = {
            "vertices": vert_count,
            "faces": face_count,
            "compressed_bytes": comp_size,
            "uncompressed_bytes": len(raw)
        }
        return mesh, metadata

def convert(input_path, output_path=None, fmt=None):
    mesh, meta = parse_cxbin(input_path)
    if mesh is None:
        print("❌ Conversion failed.")
        return

    if output_path is None:
        output_path = os.path.splitext(input_path)[0] + ".stl"
        fmt = "stl"
    else:
        if fmt is None:
            _, ext = os.path.splitext(output_path)
            fmt = ext.lstrip(".").lower()

    try:
        if fmt in SUPPORTED_FORMATS:
            mesh.export(output_path, file_type=fmt)
        else:
            print(f"❌ Format '{fmt}' is not supported.")
            return

        print("✅ Successfully exported:")
        print(f"   🔸 Format:        {fmt.upper()}")
        print(f"   🔸 Target:          {output_path}")
        print(f"   🔸 Vertices:      {meta['vertices']}")
        print(f"   🔸 Faces:         {meta['faces']}")
        print(f"   🔸 Compressed:   {meta['compressed_bytes']} Bytes")
        print(f"   🔸 Decompressed: {meta['uncompressed_bytes']} Bytes")
    except Exception as e:
        print("❌ Error during export:", e)

def main():
    if len(sys.argv) < 2:
        script_name = os.path.basename(sys.argv[0])
        is_python_script = script_name.endswith(".py")
        interpreter = "python3" if platform.system() != "Windows" else "python"
        print("📦 Usage:")
        if is_python_script:
            print(f"  {interpreter} {script_name} <input.cxbin> [output.stl|.obj|.ply|...]")
        else:
            print(f"  {script_name} <input.cxbin> [output.stl|.obj|.ply|...]")
        print("  Or simply drag and drop a file on the script.\n")
        print("🛠️ Supported export formats:")
        print(" ", ", ".join(SUPPORTED_FORMATS))
        if platform.system() == "Windows":
            input("\n❎ Press [Enter] to exit...")
        return

    input_file = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) >= 3 else None
    convert(input_file, output_file)

if __name__ == "__main__":
    print_ascii_header()
    main()
