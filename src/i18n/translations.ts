export type Language = "en" | "de";

export const translations = {
  en: {
    app: {
      title: "CXBin Converter",
      subtitle: "Tauri desktop rewrite based on the Creality CXBin reference",
    },
    theme: {
      title: "Theme: {theme}",
    },
    dropzone: {
      hint: "Drop .cxbin files here or select them via the button",
      selectFiles: "Select files",
    },
    files: {
      title: "Files ({count})",
      clear: "Clear",
      preview: "Preview",
    },
    format: {
      placeholder: "Format",
    },
    convert: "Convert",
    result: {
      success: "Success",
      error: "Error",
      vertices: "Vertices",
      faces: "Faces",
      output: "Output",
    },
    viewer: {
      title: "Preview",
    },
    language: {
      title: "Language: {lang}",
      en: "English",
      de: "German",
    },
  },
  de: {
    app: {
      title: "CXBin Converter",
      subtitle: "Tauri Desktop Rewrite basierend auf der Creality CXBin-Referenz",
    },
    theme: {
      title: "Theme: {theme}",
    },
    dropzone: {
      hint: ".cxbin-Dateien hierher ziehen oder über den Button auswählen",
      selectFiles: "Dateien auswählen",
    },
    files: {
      title: "Dateien ({count})",
      clear: "Leeren",
      preview: "Vorschau",
    },
    format: {
      placeholder: "Format",
    },
    convert: "Konvertieren",
    result: {
      success: "Erfolg",
      error: "Fehler",
      vertices: "Vertices",
      faces: "Faces",
      output: "Ausgabe",
    },
    viewer: {
      title: "Vorschau",
    },
    language: {
      title: "Sprache: {lang}",
      en: "Englisch",
      de: "Deutsch",
    },
  },
};

export type Translations = typeof translations;
