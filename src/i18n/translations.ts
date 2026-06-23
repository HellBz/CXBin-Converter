export type Language = "en" | "de" | "fr" | "es" | "zh" | "ja";

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
      fr: "French",
      es: "Spanish",
      zh: "Chinese",
      ja: "Japanese",
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
      fr: "Französisch",
      es: "Spanisch",
      zh: "Chinesisch",
      ja: "Japanisch",
    },
  },
  fr: {
    app: {
      title: "CXBin Converter",
      subtitle: "Réécriture du bureau Tauri basée sur la référence Creality CXBin",
    },
    theme: {
      title: "Thème: {theme}",
    },
    dropzone: {
      hint: "Déposez les fichiers .cxbin ici ou sélectionnez-les via le bouton",
      selectFiles: "Sélectionner des fichiers",
    },
    files: {
      title: "Fichiers ({count})",
      clear: "Vider",
      preview: "Aperçu",
    },
    format: {
      placeholder: "Format",
    },
    convert: "Convertir",
    result: {
      success: "Succès",
      error: "Erreur",
      vertices: "Sommets",
      faces: "Faces",
      output: "Sortie",
    },
    viewer: {
      title: "Aperçu",
    },
    language: {
      title: "Langue: {lang}",
      en: "Anglais",
      de: "Allemand",
      fr: "Français",
      es: "Espagnol",
      zh: "Chinois",
      ja: "Japonais",
    },
  },
  es: {
    app: {
      title: "CXBin Converter",
      subtitle: "Reescritura de escritorio Tauri basada en la referencia Creality CXBin",
    },
    theme: {
      title: "Tema: {theme}",
    },
    dropzone: {
      hint: "Arrastra archivos .cxbin aquí o selecciónalos con el botón",
      selectFiles: "Seleccionar archivos",
    },
    files: {
      title: "Archivos ({count})",
      clear: "Vaciar",
      preview: "Vista previa",
    },
    format: {
      placeholder: "Formato",
    },
    convert: "Convertir",
    result: {
      success: "Éxito",
      error: "Error",
      vertices: "Vértices",
      faces: "Caras",
      output: "Salida",
    },
    viewer: {
      title: "Vista previa",
    },
    language: {
      title: "Idioma: {lang}",
      en: "Inglés",
      de: "Alemán",
      fr: "Francés",
      es: "Español",
      zh: "Chino",
      ja: "Japonés",
    },
  },
  zh: {
    app: {
      title: "CXBin Converter",
      subtitle: "基于 Creality CXBin 参考的 Tauri 桌面重写",
    },
    theme: {
      title: "主题: {theme}",
    },
    dropzone: {
      hint: "将 .cxbin 文件拖放到此处或通过按钮选择",
      selectFiles: "选择文件",
    },
    files: {
      title: "文件 ({count})",
      clear: "清空",
      preview: "预览",
    },
    format: {
      placeholder: "格式",
    },
    convert: "转换",
    result: {
      success: "成功",
      error: "错误",
      vertices: "顶点",
      faces: "面",
      output: "输出",
    },
    viewer: {
      title: "预览",
    },
    language: {
      title: "语言: {lang}",
      en: "英语",
      de: "德语",
      fr: "法语",
      es: "西班牙语",
      zh: "中文",
      ja: "日语",
    },
  },
  ja: {
    app: {
      title: "CXBin Converter",
      subtitle: "Creality CXBin リファレンスに基づく Tauri デスクトップ書き換え",
    },
    theme: {
      title: "テーマ: {theme}",
    },
    dropzone: {
      hint: ".cxbin ファイルをここにドロップするか、ボタンで選択してください",
      selectFiles: "ファイルを選択",
    },
    files: {
      title: "ファイル ({count})",
      clear: "クリア",
      preview: "プレビュー",
    },
    format: {
      placeholder: "形式",
    },
    convert: "変換",
    result: {
      success: "成功",
      error: "エラー",
      vertices: "頂点",
      faces: "面",
      output: "出力",
    },
    viewer: {
      title: "プレビュー",
    },
    language: {
      title: "言語: {lang}",
      en: "英語",
      de: "ドイツ語",
      fr: "フランス語",
      es: "スペイン語",
      zh: "中国語",
      ja: "日本語",
    },
  },
};

export type Translations = typeof translations;
