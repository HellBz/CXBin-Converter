import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open } from "@tauri-apps/plugin-dialog";
import {
  FileUp,
  FolderOpen,
  Loader2,
  Trash2,
  Eye,
  Sun,
  Moon,
  Monitor,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { useTheme } from "@/components/ThemeProvider";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import Viewer from "@/components/Viewer";

interface ConversionResult {
  success: boolean;
  input?: string;
  format?: string;
  outputs?: string[];
  stats?: {
    vertices: number;
    faces: number;
    compressed_bytes?: number;
    uncompressed_bytes?: number;
  };
  error?: string;
}

const FORMATS = ["stl", "ply", "obj", "off", "3mf", "amf", "vrml", "x3d", "dae", "glb", "gltf"];

function ThemeToggle() {
  const { theme, toggleTheme } = useTheme();
  return (
    <Button variant="outline" size="icon" onClick={toggleTheme} title={`Theme: ${theme}`}>
      {theme === "light" && <Sun className="h-4 w-4" />}
      {theme === "dark" && <Moon className="h-4 w-4" />}
      {theme === "system" && <Monitor className="h-4 w-4" />}
    </Button>
  );
}

export default function App() {
  const [files, setFiles] = useState<string[]>([]);
  const [format, setFormat] = useState("stl");
  const [running, setRunning] = useState(false);
  const [results, setResults] = useState<ConversionResult[]>([]);
  const [previewFile, setPreviewFile] = useState<string | null>(null);

  const addFiles = useCallback((paths: string[]) => {
    setFiles((prev) => Array.from(new Set([...prev, ...paths])));
    setResults([]);
  }, []);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    const setup = async () => {
      unlisten = await getCurrentWebview().onDragDropEvent((event) => {
        if (event.payload.type === "drop") {
          const paths = event.payload.paths.filter((p) =>
            p.toLowerCase().endsWith(".cxbin")
          );
          addFiles(paths);
        }
      });
    };
    setup();
    return () => {
      unlisten?.();
    };
  }, [addFiles]);

  const browseFiles = async () => {
    const selected = await open({
      multiple: true,
      filters: [{ name: "CXBin", extensions: ["cxbin"] }],
    });
    if (selected && Array.isArray(selected)) {
      addFiles(selected);
    }
  };

  const convert = async () => {
    if (files.length === 0) return;
    setRunning(true);
    setResults([]);
    const out: ConversionResult[] = [];
    for (const input of files) {
      try {
        const result = await invoke<ConversionResult>("convert_cxbin", {
          input,
          format,
        });
        out.push(result);
      } catch (e) {
        out.push({
          success: false,
          input,
          format,
          error: String(e),
        });
      }
    }
    setResults(out);
    setRunning(false);
  };

  return (
    <div className="min-h-screen bg-background p-6">
      <div className="mx-auto max-w-2xl space-y-6">
        <div className="text-center">
          <div className="flex items-center justify-center gap-3">
            <h1 className="text-3xl font-bold tracking-tight">
              <span className="text-primary">CX</span>Bin Converter
            </h1>
            <ThemeToggle />
          </div>
          <p className="text-muted-foreground">
            Tauri Desktop Rewrite basierend auf der Creality CXBin-Referenz
          </p>
        </div>

        <div className="flex flex-col items-center justify-center rounded-lg border-2 border-dashed border-border bg-card p-10 text-center hover:border-primary/50 transition-colors">
          <FileUp className="mb-3 h-10 w-10 text-muted-foreground" />
          <p className="text-sm font-medium">
            .cxbin-Dateien hierher ziehen oder über den Button auswählen
          </p>
          <Button variant="outline" className="mt-4" onClick={browseFiles}>
            <FolderOpen className="mr-2 h-4 w-4" />
            Dateien auswählen
          </Button>
        </div>

        {files.length > 0 && (
          <div className="rounded-lg border bg-card p-4">
            <div className="mb-2 flex items-center justify-between">
              <span className="font-medium">Dateien ({files.length})</span>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => {
                  setFiles([]);
                  setResults([]);
                }}
              >
                <Trash2 className="mr-1 h-4 w-4" />
                Leeren
              </Button>
            </div>
            <ul className="max-h-40 space-y-1 overflow-y-auto text-sm text-muted-foreground">
              {files.map((f) => (
                <li key={f} className="flex items-center justify-between gap-2">
                  <span className="truncate">{f}</span>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => setPreviewFile(f)}
                  >
                    <Eye className="mr-1 h-3 w-3" />
                    Vorschau
                  </Button>
                </li>
              ))}
            </ul>
          </div>
        )}

        <div className="flex items-center gap-4">
          <Select value={format} onValueChange={setFormat}>
            <SelectTrigger className="w-40">
              <SelectValue placeholder="Format" />
            </SelectTrigger>
            <SelectContent>
              {FORMATS.map((f) => (
                <SelectItem key={f} value={f}>
                  {f.toUpperCase()}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>

          <Button onClick={convert} disabled={files.length === 0 || running}>
            {running && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            Konvertieren
          </Button>
        </div>

        {results.length > 0 && (
          <div className="space-y-3">
            {results.map((r, i) => (
              <div
                key={i}
                className={`rounded-lg border p-4 ${
                  r.success
                    ? "border-green-500/30 bg-green-500/10 dark:border-green-400/30 dark:bg-green-400/10"
                    : "border-red-500/30 bg-red-500/10 dark:border-red-400/30 dark:bg-red-400/10"
                }`}
              >
                <div className="font-medium">
                  {r.success ? "Erfolg" : "Fehler"}
                </div>
                {r.input && (
                  <div className="text-sm text-muted-foreground truncate">
                    {r.input}
                  </div>
                )}
                {r.success && r.stats && (
                  <div className="mt-2 text-sm">
                    Vertices: {r.stats.vertices}, Faces: {r.stats.faces}
                    <br />
                    Ausgabe: {r.outputs?.join(", ")}
                  </div>
                )}
                {!r.success && r.error && (
                  <div className="mt-2 text-sm text-red-700 dark:text-red-300">{r.error}</div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>

      {previewFile && (
        <Viewer file={previewFile} onClose={() => setPreviewFile(null)} />
      )}
    </div>
  );
}
