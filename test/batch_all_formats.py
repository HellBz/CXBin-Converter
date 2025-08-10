# batch_all_formats_json.py
# Comments in English only (as requested).
import argparse
import json
import subprocess
import sys
from pathlib import Path

CONVERTER = Path(__file__).parent.parent / "cxbin_converter" / "cxbin_converter.py"

def run(cmd: list[str]) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, capture_output=True, text=True, shell=False)

def list_formats() -> dict[str, list[str]]:
    """Call converter --list-formats and parse available formats."""
    proc = run([sys.executable, str(CONVERTER), "--list-formats"])
    tri, mio = [], []
    for line in proc.stdout.splitlines():
        s = line.strip().lower()
        if s.startswith("trimesh:"):
            tri = [x.strip() for x in s.split(":", 1)[1].split(",") if x.strip()]
        elif s.startswith("meshio (fallback):"):
            mio = [x.strip() for x in s.split(":", 1)[1].split(",") if x.strip()]
    return {"trimesh": tri, "meshio": mio}

def find_inputs(target: Path, recursive: bool) -> list[Path]:
    if target.is_file() and target.suffix.lower() == ".cxbin":
        return [target]
    if target.is_dir():
        pattern = "**/*.cxbin" if recursive else "*.cxbin"
        return sorted(target.glob(pattern))
    return []

def convert_one(file: Path, fmt: str, zip_mode: str) -> dict:
    """Call converter with --json to get machine-readable output."""
    args = [sys.executable, str(CONVERTER), str(file), "-f", fmt, "--json"]
    if zip_mode == "zip":
        args.append("--zip")
    elif zip_mode == "zip-only":
        args.append("--zip-only")
    proc = run(args)
    # If converter prints *only* JSON in --json mode, stdout should parse fine:
    try:
        data = json.loads(proc.stdout or "{}")
    except json.JSONDecodeError:
        data = {"results": [], "parse_error": proc.stdout}
    # Augment with stderr/returncode for diagnostics
    data["_stderr"] = proc.stderr
    data["_returncode"] = proc.returncode
    data["_format"] = fmt
    data["_file"] = str(file)
    return data

def main():
    p = argparse.ArgumentParser(description="Batch-convert CXBIN into all available formats via cxbin_converter.py (JSON-safe).")
    p.add_argument("input", help="Path to a .cxbin file or a folder.")
    p.add_argument("--recursive", "-r", action="store_true", help="Scan subfolders for .cxbin files.")
    p.add_argument("--zip", action="store_true", help="Zip multi-file outputs (OBJ/GLTF).")
    p.add_argument("--zip-only", action="store_true", help="Zip and remove the folder afterwards.")
    p.add_argument("--only", nargs="*", help="Only run these formats (space-separated), e.g. --only obj gltf glb.")
    p.add_argument("--out", default="batch_results.json", help="Path to write the JSON report.")
    args = p.parse_args()

    zip_mode = "zip-only" if args.zip_only else ("zip" if args.zip else "none")

    formats = list_formats()
    all_formats = sorted(set(formats["trimesh"]) | set(formats["meshio"]))
    if args.only:
        wanted = {f.lower() for f in args.only}
        formats_to_run = [f for f in all_formats if f in wanted]
    else:
        formats_to_run = all_formats

    inputs = find_inputs(Path(args.input), args.recursive)
    if not inputs:
        print("No .cxbin files found.", file=sys.stderr)
        sys.exit(2)

    report = {
        "converter": str(CONVERTER),
        "zip_mode": zip_mode,
        "formats_detected": formats,
        "formats_run": formats_to_run,
        "items": {}
    }

    print(f"[INFO] Formats to run: {formats_to_run}")
    for cx in inputs:
        print(f"[INFO] Processing: {cx}")
        per_file = []
        for fmt in formats_to_run:
            print(f"  -> {fmt}")
            res = convert_one(cx, fmt, zip_mode)
            per_file.append(res)
        report["items"][str(cx)] = per_file

    Path(args.out).write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"[DONE] Wrote report: {args.out}")

if __name__ == "__main__":
    main()
